from rest_framework.decorators import api_view, permission_classes
from rest_framework.response import Response
from rest_framework import status
from rest_framework.permissions import AllowAny, IsAdminUser
from .models import AgentToken, PendingRegistration
import logging

logger = logging.getLogger(__name__)

@api_view(['POST'])
@permission_classes([AllowAny])
def register_device(request):
    """
    Self-registration endpoint for new devices
    
    POST /api/agents/register/
    {
        "agent_id": "unique-device-id",
        "agent_name": "My Device Name",
        "hostname": "my-laptop",
        "os_type": "macos",
        "os_version": "14.0",
        "device_fingerprint": "abc123..."
    }
    """
    
    logger.info("="*70)
    logger.info("Device Registration Request")
    logger.info("="*70)
    
    # Validate required fields
    required_fields = ['agent_id', 'agent_name', 'hostname', 'os_type', 'os_version', 'device_fingerprint']
    missing_fields = [f for f in required_fields if not request.data.get(f)]
    
    if missing_fields:
        logger.warning(f"Registration failed: Missing fields {missing_fields}")
        return Response(
            {
                'error': 'Missing required fields',
                'missing': missing_fields
            },
            status=status.HTTP_400_BAD_REQUEST
        )
    
    agent_id = request.data['agent_id']
    device_fingerprint = request.data['device_fingerprint']
    
    logger.info(f"Agent ID: {agent_id}")
    logger.info(f"Hostname: {request.data['hostname']}")
    logger.info(f"OS: {request.data['os_type']} {request.data['os_version']}")
    logger.info(f"Fingerprint: {device_fingerprint[:16]}...")
    
    # Check if already registered
    if AgentToken.objects.filter(agent_id=agent_id).exists():
        logger.warning(f"Device already registered: {agent_id}")
        return Response(
            {
                'error': 'Device already registered',
                'message': 'This device is already registered. If you lost your token, contact your administrator.',
                'agent_id': agent_id
            },
            status=status.HTTP_409_CONFLICT
        )
    
    # Check if fingerprint already registered (prevents device spoofing)
    if AgentToken.objects.filter(device_fingerprint=device_fingerprint).exists():
        logger.error(f"🚨 SECURITY: Fingerprint already registered: {device_fingerprint[:16]}...")
        return Response(
            {
                'error': 'Device already registered',
                'message': 'This hardware is already registered with a different agent ID.',
            },
            status=status.HTTP_409_CONFLICT
        )
    
    # Check if registration already pending
    existing_pending = PendingRegistration.objects.filter(
        agent_id=agent_id,
        status='pending'
    ).first()
    
    if existing_pending:
        logger.info(f"Registration already pending: {agent_id}")
        return Response(
            {
                'status': 'pending',
                'message': 'Your registration request is pending admin approval',
                'agent_id': agent_id,
                'requested_at': existing_pending.requested_at.isoformat()
            },
            status=status.HTTP_202_ACCEPTED
        )
    
    # Check if fingerprint has pending registration
    if PendingRegistration.objects.filter(device_fingerprint=device_fingerprint, status='pending').exists():
        logger.error(f"🚨 SECURITY: Fingerprint has pending registration: {device_fingerprint[:16]}...")
        return Response(
            {
                'error': 'Registration pending',
                'message': 'This hardware already has a pending registration request.',
            },
            status=status.HTTP_409_CONFLICT
        )
    
    # Create pending registration
    try:
        registration = PendingRegistration.objects.create(
            agent_id=agent_id,
            agent_name=request.data['agent_name'],
            hostname=request.data['hostname'],
            os_type=request.data['os_type'],
            os_version=request.data['os_version'],
            device_fingerprint=device_fingerprint,
        )
        
        logger.info(f"✓ Registration request created: {agent_id}")
        logger.info("="*70)
        
        return Response(
            {
                'status': 'pending',
                'message': 'Registration request submitted successfully. Waiting for admin approval.',
                'registration_id': registration.id,
                'agent_id': agent_id,
                'agent_name': registration.agent_name,
                'instructions': 'Check status with: device-agent --check-status'
            },
            status=status.HTTP_202_ACCEPTED
        )
        
    except Exception as e:
        logger.exception(f"Failed to create registration: {e}")
        return Response(
            {
                'error': 'Registration failed',
                'details': str(e)
            },
            status=status.HTTP_500_INTERNAL_SERVER_ERROR
        )


@api_view(['GET'])
@permission_classes([AllowAny])
def check_registration(request, agent_id):
    """
    Check registration status and get token if approved
    
    GET /api/agents/register/<agent_id>/status/
    """
    
    logger.debug(f"Checking registration status for: {agent_id}")
    
    # Check if already registered and active
    try:
        token = AgentToken.objects.get(agent_id=agent_id, is_active=True)
        logger.info(f"Device already registered: {agent_id}")
        return Response({
            'status': 'approved',
            'message': 'Device is registered and active',
            'agent_id': agent_id,
            'agent_name': token.agent_name,
            'token': token.token,
            'bound_to_device': bool(token.device_fingerprint)
        })
    except AgentToken.DoesNotExist:
        pass
    
    # Check pending registration
    try:
        registration = PendingRegistration.objects.get(agent_id=agent_id)
        
        if registration.status == 'approved' and registration.token:
            logger.info(f"Registration approved: {agent_id}")
            return Response({
                'status': 'approved',
                'message': 'Registration approved! Token is ready.',
                'agent_id': agent_id,
                'agent_name': registration.agent_name,
                'token': registration.token.token,
                'approved_at': registration.approved_at.isoformat() if registration.approved_at else None
            })
        elif registration.status == 'pending':
            logger.debug(f"Registration pending: {agent_id}")
            return Response({
                'status': 'pending',
                'message': 'Registration is pending admin approval',
                'agent_id': agent_id,
                'requested_at': registration.requested_at.isoformat()
            })
        elif registration.status == 'rejected':
            logger.warning(f"Registration rejected: {agent_id}")
            return Response({
                'status': 'rejected',
                'message': 'Registration was rejected by administrator',
                'agent_id': agent_id
            })
            
    except PendingRegistration.DoesNotExist:
        logger.warning(f"No registration found: {agent_id}")
        return Response({
            'status': 'not_found',
            'message': 'No registration found for this device. Please register first.',
            'agent_id': agent_id
        }, status=status.HTTP_404_NOT_FOUND)


@api_view(['POST'])
@permission_classes([IsAdminUser])
def create_token(request):
    """
    Manual token creation by admin (legacy method)
    
    POST /api/agents/tokens/create/
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
    
    logger.info(f"Manual token created: {agent_id} by {request.user.username}")
    
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
                'bound_to_device': bool(token.device_fingerprint),
                'bound_hostname': token.bound_hostname,
            }
            for token in tokens
        ]
    }, status=status.HTTP_200_OK)