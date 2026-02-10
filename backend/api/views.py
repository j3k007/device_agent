from rest_framework.decorators import api_view
from rest_framework.response import Response
from rest_framework import status
from datetime import datetime
import logging

# ✅ Get logger for this module
logger = logging.getLogger(__name__)

@api_view(['POST'])
def heartbeat(request):
    """Basic heartbeat endpoint"""
    
    logger.info("="*50)
    logger.info("Heartbeat received from agent")
    logger.info("="*50)
    data = request.data
    logger.info(f"Agent ID: {data.get('agent_id', 'N/A')}")
    logger.info(f"Agent Name: {data.get('agent_name', 'N/A')}")
    logger.info(f"Hostname: {data.get('hostname', 'N/A')}")
    logger.info(f"OS: {data.get('os_type', 'N/A')} {data.get('os_version', 'N/A')}")
    logger.info(f"CPU: {data.get('cpu_info', 'N/A')}")
    logger.debug(f"Memory Total: {data.get('memory_total', 'N/A')} bytes")
    logger.debug(f"Services Count: {len(data.get('services', []))}")
    logger.debug(f"Software Count: {len(data.get('installed_software', []))}")
    logger.debug(f"Collected At: {data.get('collected_at', 'N/A')}")

    logger.debug(f"Full heartbeat data: {data}")
    
    logger.info("="*50)
    
    return Response({
        'status': 'success',
        'message': 'Heartbeat received and logged',
        'timestamp': datetime.now(),
        'received_data': {
            'agent_id': data.get('agent_id'),
            'hostname': data.get('hostname'),
            'services_count': len(data.get('services', [])),
            'software_count': len(data.get('installed_software', [])),
        }
    }, status=status.HTTP_200_OK)

@api_view(['GET'])
def health_check(request):
    """Simple health check endpoint"""
    logger.debug("Health check requested")
    
    return Response({
        'status': 'healthy',
        'timestamp': datetime.now(),
    }, status=status.HTTP_200_OK)