from rest_framework import authentication
from rest_framework import exceptions
from django.utils import timezone
from .models import AgentToken
import logging

logger = logging.getLogger(__name__)

class AgentTokenAuthentication(authentication.BaseAuthentication):
    """
    Token-based authentication for device agents with device binding
    """
    
    keyword = 'Bearer'
    
    def authenticate(self, request):
        """Authenticate the request and return (user, token) tuple"""
        auth_header = request.META.get('HTTP_AUTHORIZATION', '').split()
        
        if not auth_header:
            return None
        
        if auth_header[0].lower() != self.keyword.lower():
            return None
        
        if len(auth_header) == 1:
            raise exceptions.AuthenticationFailed('Invalid token header. No credentials provided.')
        
        if len(auth_header) > 2:
            raise exceptions.AuthenticationFailed('Invalid token header. Token should not contain spaces.')
        
        token = auth_header[1]
        
        return self.authenticate_credentials(token, request)
    
    def authenticate_credentials(self, key, request):
        """Validate the token and device fingerprint"""
        try:
            agent_token = AgentToken.objects.get(token=key)
        except AgentToken.DoesNotExist:
            logger.warning("Authentication failed: Invalid token")
            raise exceptions.AuthenticationFailed('Invalid token')
        
        if not agent_token.is_active:
            logger.warning(f"Authentication failed: Token inactive (agent: {agent_token.agent_id})")
            raise exceptions.AuthenticationFailed('Token has been deactivated')
        
        # ✅ Validate device fingerprint
        device_fingerprint = request.data.get('device_fingerprint')
        
        if not device_fingerprint:
            logger.error(f"Authentication failed: No fingerprint provided (agent: {agent_token.agent_id})")
            raise exceptions.AuthenticationFailed('Device fingerprint required')
        
        # Check if token is bound to a device
        if agent_token.device_fingerprint:
            # Token is bound, validate fingerprint matches
            if not agent_token.validate_fingerprint(device_fingerprint):
                logger.error(
                    f"🚨 SECURITY ALERT: Fingerprint mismatch!\n"
                    f"   Agent: {agent_token.agent_id}\n"
                    f"   Expected: {agent_token.device_fingerprint[:16]}...\n"
                    f"   Received: {device_fingerprint[:16]}...\n"
                    f"   Bound Host: {agent_token.bound_hostname}\n"
                    f"   Request Host: {request.data.get('hostname', 'unknown')}\n"
                    f"   Mismatch Count: {agent_token.fingerprint_mismatch_count + 1}"
                )
                
                agent_token.record_fingerprint_mismatch()
                
                # Auto-disable after 5 mismatches
                if agent_token.fingerprint_mismatch_count >= 5:
                    agent_token.is_active = False
                    agent_token.save(update_fields=['is_active'])
                    logger.error(f"🚨 Token auto-disabled after {agent_token.fingerprint_mismatch_count} mismatches")
                
                raise exceptions.AuthenticationFailed(
                    'Device fingerprint mismatch. This token is bound to a different device.'
                )
        else:
            # Token not yet bound, bind it now
            hostname = request.data.get('hostname', 'unknown')
            agent_token.bind_to_device(device_fingerprint, hostname)
            logger.info(
                f"✓ Token bound to device:\n"
                f"   Agent: {agent_token.agent_id}\n"
                f"   Fingerprint: {device_fingerprint[:16]}...\n"
                f"   Hostname: {hostname}"
            )
        
        # Update last used
        agent_token.last_used = timezone.now()
        agent_token.save(update_fields=['last_used'])
        
        logger.info(f"✓ Authentication successful: {agent_token.agent_id}")
        
        return (None, agent_token)
    
    def authenticate_header(self, request):
        return self.keyword