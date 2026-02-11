from rest_framework import authentication
from rest_framework import exceptions
from django.utils import timezone
from .models import AgentToken
import logging

logger = logging.getLogger(__name__)

class AgentTokenAuthentication(authentication.BaseAuthentication):
    """
    Token-based authentication for device agents
    
    Clients authenticate by passing the token in the Authorization header:
        Authorization: Bearer agt_xxxxxxxxxxxxxxxxxxx
    """
    
    keyword = 'Bearer'
    
    def authenticate(self, request):
        """
        Authenticate the request and return (user, token) tuple
        Returns None if authentication is not attempted
        """
        auth_header = request.META.get('HTTP_AUTHORIZATION', '').split()
        
        if not auth_header:
            # No Authorization header provided
            logger.debug("No Authorization header provided")
            return None
        
        if auth_header[0].lower() != self.keyword.lower():
            # Not a Bearer token
            logger.debug(f"Authentication method is not Bearer: {auth_header[0]}")
            return None
        
        if len(auth_header) == 1:
            msg = 'Invalid token header. No credentials provided.'
            logger.warning(msg)
            raise exceptions.AuthenticationFailed(msg)
        
        if len(auth_header) > 2:
            msg = 'Invalid token header. Token string should not contain spaces.'
            logger.warning(msg)
            raise exceptions.AuthenticationFailed(msg)
        
        try:
            token = auth_header[1]
        except UnicodeError:
            msg = 'Invalid token header. Token string contains invalid characters.'
            logger.warning(msg)
            raise exceptions.AuthenticationFailed(msg)
        
        return self.authenticate_credentials(token, request)
    
    def authenticate_credentials(self, key, request):
        """
        Validate the token and return (user, token) tuple
        """
        try:
            agent_token = AgentToken.objects.get(token=key)
        except AgentToken.DoesNotExist:
            logger.warning(f"Authentication failed: Invalid token (token not found)")
            raise exceptions.AuthenticationFailed('Invalid token')
        
        if not agent_token.is_active:
            logger.warning(f"Authentication failed: Token is inactive (agent_id: {agent_token.agent_id})")
            raise exceptions.AuthenticationFailed('Token has been deactivated')
        
        # Update last used timestamp
        agent_token.last_used = timezone.now()
        agent_token.save(update_fields=['last_used'])
        
        logger.info(f"✓ Authentication successful: {agent_token.agent_id} ({agent_token.agent_name})")
        
        # Return (user, token) - user is None for agent authentication
        # The token object is stored in request.auth
        return (None, agent_token)
    
    def authenticate_header(self, request):
        """
        Return a string to be used as the value of the `WWW-Authenticate`
        header in a `401 Unauthenticated` response.
        """
        return self.keyword