from rest_framework.decorators import api_view, authentication_classes, permission_classes
from rest_framework.response import Response
from rest_framework import status
from rest_framework.permissions import AllowAny, IsAuthenticated
from rest_framework.authtoken.models import Token
from django.contrib.auth import authenticate
from agents.authentication import AgentTokenAuthentication
from agents.models import PendingRegistration
from devices.serializers import HeartbeatSerializer
from devices.models import Device, DeviceService, DeviceSoftware
from datetime import datetime
from django.utils import timezone
from api.broadcast import (
    broadcast_device_update, broadcast_device_services,
    broadcast_device_software, broadcast_dashboard_stats,
)
import logging

logger = logging.getLogger(__name__)

@api_view(['POST'])
@authentication_classes([AgentTokenAuthentication])
@permission_classes([AllowAny])
def heartbeat(request):
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
    
    # Validate data
    serializer = HeartbeatSerializer(data=request.data)
    if not serializer.is_valid():
        logger.error(f"Invalid heartbeat data: {serializer.errors}")
        return Response(
            {
                'error': 'Invalid data',
                'details': serializer.errors
            },
            status=status.HTTP_400_BAD_REQUEST
        )
    
    data = serializer.validated_data
    
    # Validate agent_id matches token
    if data['agent_id'] != agent_token.agent_id:
        logger.warning(
            f"Agent ID mismatch! Token: {agent_token.agent_id}, Data: {data['agent_id']}"
        )
        return Response(
            {
                'error': 'Agent ID mismatch',
                'message': f'Token is for agent {agent_token.agent_id} but data is from {data["agent_id"]}'
            },
            status=status.HTTP_400_BAD_REQUEST
        )
    
    # Process and store data
    try:
        device, services_stats, software_stats = process_heartbeat(agent_token, data)
        
        logger.info(f"✓ Device updated: {device.hostname}")
        logger.info(f"✓ Services: {services_stats['active']} active")
        logger.info(f"✓ Software: {software_stats['installed']} installed")
        logger.info("="*70)

        # Broadcast real-time updates
        broadcast_device_update(device)
        broadcast_device_services(device)
        broadcast_device_software(device)
        broadcast_dashboard_stats()

        return Response({
            'status': 'success',
            'message': 'Heartbeat received and stored',
            'timestamp': datetime.now(),
            'device_id': device.id,
            'device': {
                'hostname': device.hostname,
                'os': f"{device.os_type} {device.os_version}",
                'memory_usage_percent': device.memory_usage_percent,
                'services_count': services_stats['active'],
                'software_count': software_stats['installed'],
            }
        }, status=status.HTTP_200_OK)
        
    except Exception as e:
        logger.exception(f"Error processing heartbeat: {e}")
        return Response(
            {
                'error': 'Processing failed',
                'details': str(e)
            },
            status=status.HTTP_500_INTERNAL_SERVER_ERROR
        )


def process_heartbeat(agent_token, data):
    """
    Process heartbeat data and update database
    Returns: (device, services_stats, software_stats)
    """
    
    # Get or create device
    device, created = Device.objects.get_or_create(
        agent_token=agent_token,
        defaults={
            'hostname': data['hostname'],
            'os_type': data['os_type'],
            'os_version': data['os_version'],
            'cpu_info': data['cpu_info'],
            'memory_total': data['memory_total'],
            'memory_available': data['memory_available'],
            'ip_addresses': data.get('ip_addresses', {}),
            'is_online': True,
        }
    )
    
    if created:
        logger.info(f"✓ New device created: {device.hostname}")
    else:
        # Update existing device
        device.hostname = data['hostname']
        device.os_type = data['os_type']
        device.os_version = data['os_version']
        device.cpu_info = data['cpu_info']
        device.memory_total = data['memory_total']
        device.memory_available = data['memory_available']
        device.ip_addresses = data.get('ip_addresses', {})
        device.is_online = True
        device.last_heartbeat = timezone.now()
        device.save()
        logger.info(f"✓ Device updated: {device.hostname}")
    
    # Process services
    services_stats = process_services(device, data.get('services', []))
    
    # Process software
    software_stats = process_software(device, data.get('installed_software', []))
    
    return device, services_stats, software_stats


def process_services(device, service_names):
    """
    Process services list and update database
    Returns: {'active': count, 'inactive': count}
    """
    logger.debug(f"Processing {len(service_names)} services")
    
    # Track current services
    current_services = set(service_names)
    
    # Update or create services
    active_count = 0
    for service_name in current_services:
        service, created = DeviceService.objects.get_or_create(
            device=device,
            service_name=service_name,
            defaults={'is_active': True}
        )
        
        if not created:
            # Update existing service
            service.is_active = True
            service.last_seen = timezone.now()
            service.save(update_fields=['is_active', 'last_seen'])
        
        active_count += 1
    
    # Mark services NOT in current list as inactive
    inactive_count = DeviceService.objects.filter(
        device=device,
        is_active=True
    ).exclude(
        service_name__in=current_services
    ).update(is_active=False)
    
    logger.debug(f"✓ Services processed: {active_count} active, {inactive_count} deactivated")
    
    return {
        'active': active_count,
        'inactive': inactive_count
    }


def process_software(device, software_names):
    """
    Process software list and update database
    Returns: {'installed': count, 'uninstalled': count}
    """
    logger.debug(f"Processing {len(software_names)} software items")
    
    # Track current software
    current_software = set(software_names)
    
    # Update or create software
    installed_count = 0
    for software_name in current_software:
        software, created = DeviceSoftware.objects.get_or_create(
            device=device,
            software_name=software_name,
            defaults={'is_installed': True}
        )
        
        if not created:
            # Update existing software
            software.is_installed = True
            software.last_seen = timezone.now()
            software.save(update_fields=['is_installed', 'last_seen'])
        
        installed_count += 1
    
    # Mark software NOT in current list as uninstalled
    uninstalled_count = DeviceSoftware.objects.filter(
        device=device,
        is_installed=True
    ).exclude(
        software_name__in=current_software
    ).update(is_installed=False)
    
    logger.debug(f"✓ Software processed: {installed_count} installed, {uninstalled_count} uninstalled")
    
    return {
        'installed': installed_count,
        'uninstalled': uninstalled_count
    }


@api_view(['GET'])
def health_check(request):
    """Simple health check endpoint - no authentication required"""
    logger.debug("Health check requested")

    return Response({
        'status': 'healthy',
        'timestamp': datetime.utcnow().isoformat(),
        'version': '2.0.0',  # Phase 3
    }, status=status.HTTP_200_OK)


@api_view(['POST'])
@permission_classes([AllowAny])
def login(request):
    """
    Authenticate user and return a DRF token.

    POST /api/auth/login/
    {"username": "...", "password": "..."}
    """
    username = request.data.get('username')
    password = request.data.get('password')

    if not username or not password:
        return Response(
            {'error': 'Username and password are required'},
            status=status.HTTP_400_BAD_REQUEST,
        )

    user = authenticate(username=username, password=password)
    if user is None:
        return Response(
            {'error': 'Invalid credentials'},
            status=status.HTTP_401_UNAUTHORIZED,
        )

    token, _ = Token.objects.get_or_create(user=user)
    return Response({
        'token': token.key,
        'user': {
            'id': user.id,
            'username': user.username,
            'is_staff': user.is_staff,
        },
    })


@api_view(['POST'])
@permission_classes([IsAuthenticated])
def logout(request):
    """
    Delete the caller's DRF token.

    POST /api/auth/logout/
    """
    Token.objects.filter(user=request.user).delete()
    return Response({'status': 'success'})


@api_view(['GET'])
@permission_classes([IsAuthenticated])
def me(request):
    """
    Return the currently authenticated user.

    GET /api/auth/me/
    """
    return Response({
        'id': request.user.id,
        'username': request.user.username,
        'is_staff': request.user.is_staff,
    })


@api_view(['GET'])
@permission_classes([IsAuthenticated])
def dashboard_stats(request):
    """
    Aggregate statistics for the frontend dashboard.

    GET /api/dashboard/
    """
    total_devices = Device.objects.count()
    online_devices = Device.objects.filter(is_online=True).count()
    offline_devices = total_devices - online_devices
    pending_registrations = PendingRegistration.objects.filter(status='pending').count()

    return Response({
        'status': 'success',
        'total_devices': total_devices,
        'online_devices': online_devices,
        'offline_devices': offline_devices,
        'pending_registrations': pending_registrations,
    })