from rest_framework.decorators import api_view, authentication_classes, permission_classes
from rest_framework.response import Response
from rest_framework import status
from rest_framework.permissions import AllowAny
from agents.authentication import AgentTokenAuthentication
from datetime import datetime
import logging

logger = logging.getLogger(__name__)

@api_view(['POST'])
@authentication_classes([AgentTokenAuthentication])
@permission_classes([AllowAny])
def heartbeat(request):
    """
    Heartbeat endpoint - receives system data from agents
    
    Authentication: Required
        Authorization: Bearer <agent_token>
    """
    
    # Get the authenticated agent token
    agent_token = request.auth
    
    if not agent_token:
        logger.warning("Heartbeat attempted without authentication")
        return Response(
            {
                'error': 'Authentication required',
                'message': 'Please provide a valid Bearer token in the Authorization header'
            },
            status=status.HTTP_401_UNAUTHORIZED
        )
    
    logger.info("="*70)
    logger.info(f"Heartbeat received from: {agent_token.agent_id}")
    logger.info("="*70)
    
    data = request.data
    
    # Validate that agent_id in data matches token
    data_agent_id = data.get('agent_id')
    if data_agent_id and data_agent_id != agent_token.agent_id:
        logger.warning(
            f"Agent ID mismatch! Token: {agent_token.agent_id}, Data: {data_agent_id}"
        )
        return Response(
            {
                'error': 'Agent ID mismatch',
                'message': f'Token is for agent {agent_token.agent_id} but data is from {data_agent_id}'
            },
            status=status.HTTP_400_BAD_REQUEST
        )
    
    # Log heartbeat details
    logger.info(f"Agent ID: {agent_token.agent_id}")
    logger.info(f"Agent Name: {agent_token.agent_name}")
    logger.info(f"Hostname: {data.get('hostname', 'N/A')}")
    logger.info(f"OS: {data.get('os_type', 'N/A')} {data.get('os_version', 'N/A')}")
    logger.info(f"CPU: {data.get('cpu_info', 'N/A')}")
    logger.debug(f"Memory Total: {data.get('memory_total', 'N/A')} bytes")
    logger.debug(f"Memory Available: {data.get('memory_available', 'N/A')} bytes")
    logger.debug(f"Services Count: {len(data.get('services', []))}")
    logger.debug(f"Software Count: {len(data.get('installed_software', []))}")
    logger.debug(f"Collected At: {data.get('collected_at', 'N/A')}")
    
    # Log IP addresses if present
    ip_addresses = data.get('ip_addresses', {})
    if ip_addresses:
        logger.debug(f"IP Addresses: {ip_addresses}")
    
    # Log full data at DEBUG level
    logger.debug(f"Full heartbeat data: {data}")
    
    logger.info("="*70)
    
    # Return success response
    return Response({
        'status': 'success',
        'message': 'Heartbeat received and authenticated',
        'timestamp': datetime.now(),
        'agent': {
            'agent_id': agent_token.agent_id,
            'agent_name': agent_token.agent_name,
        },
        'received_data': {
            'hostname': data.get('hostname'),
            'os_type': data.get('os_type'),
            'services_count': len(data.get('services', [])),
            'software_count': len(data.get('installed_software', [])),
        }
    }, status=status.HTTP_200_OK)

@api_view(['GET'])
def health_check(request):
    """Simple health check endpoint - no authentication required"""
    logger.debug("Health check requested")
    
    return Response({
        'status': 'healthy',
        'timestamp': datetime.now(),
        'version': '1.0.0',
    }, status=status.HTTP_200_OK)