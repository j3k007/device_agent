from rest_framework.decorators import api_view, permission_classes
from rest_framework.response import Response
from rest_framework import status
from rest_framework.permissions import IsAdminUser
from .models import AgentToken
import logging

logger = logging.getLogger(__name__)

@api_view(['POST'])
@permission_classes([IsAdminUser])
def create_token(request):
    """
    Create a new agent token
    
    POST /api/agents/tokens/
    {
        "agent_id": "unique-agent-id",
        "agent_name": "My Device Agent"
    }
    """
    agent_id = request.data.get('agent_id')
    agent_name = request.data.get('agent_name')
    
    if not agent_id or not agent_name:
        return Response(
            {'error': 'Both agent_id and agent_name are required'},
            status=status.HTTP_400_BAD_REQUEST
        )
    
    # Check if agent_id already exists
    if AgentToken.objects.filter(agent_id=agent_id).exists():
        return Response(
            {'error': f'Agent with ID {agent_id} already exists'},
            status=status.HTTP_400_BAD_REQUEST
        )
    
    # Create token
    token = AgentToken.objects.create(
        agent_id=agent_id,
        agent_name=agent_name
    )
    
    logger.info(f"New agent token created: {agent_id} ({agent_name})")
    
    return Response({
        'status': 'success',
        'message': 'Agent token created successfully',
        'agent': {
            'agent_id': token.agent_id,
            'agent_name': token.agent_name,
            'token': token.token,
            'created_at': token.created_at,
            'is_active': token.is_active,
        }
    }, status=status.HTTP_201_CREATED)

@api_view(['GET'])
@permission_classes([IsAdminUser])
def list_tokens(request):
    """
    List all agent tokens
    
    GET /api/agents/tokens/
    """
    tokens = AgentToken.objects.all()
    
    return Response({
        'status': 'success',
        'count': tokens.count(),
        'agents': [
            {
                'agent_id': token.agent_id,
                'agent_name': token.agent_name,
                'is_active': token.is_active,
                'created_at': token.created_at,
                'last_used': token.last_used,
            }
            for token in tokens
        ]
    }, status=status.HTTP_200_OK)