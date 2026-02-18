"""
Helper functions to broadcast real-time events over WebSocket channel layers.
"""
import logging
from channels.layers import get_channel_layer
from asgiref.sync import async_to_sync
from devices.models import Device
from agents.models import PendingRegistration

logger = logging.getLogger(__name__)


def _send_to_group(group, message):
    channel_layer = get_channel_layer()
    if channel_layer is None:
        return
    try:
        async_to_sync(channel_layer.group_send)(group, message)
    except Exception as e:
        logger.warning(f"Failed to broadcast to {group}: {e}")


def broadcast_device_update(device):
    """Send device heartbeat data to dashboard and device detail groups."""
    payload = {
        'id': device.id,
        'hostname': device.hostname,
        'os_type': device.os_type,
        'os_version': device.os_version,
        'cpu_info': device.cpu_info,
        'memory_total': device.memory_total,
        'memory_available': device.memory_available,
        'memory_usage_percent': device.memory_usage_percent,
        'ip_addresses': device.ip_addresses,
        'is_online': device.is_online,
        'last_heartbeat': device.last_heartbeat.isoformat() if device.last_heartbeat else None,
        'agent_id': device.agent_token.agent_id,
        'agent_name': device.agent_token.agent_name,
        'services_count': device.services.filter(is_active=True).count(),
        'software_count': device.software.filter(is_installed=True).count(),
    }

    _send_to_group('dashboard', {
        'type': 'device_updated',
        'device': payload,
    })

    _send_to_group(f'device_{device.id}', {
        'type': 'device_updated',
        'device': payload,
    })


def broadcast_device_services(device):
    """Send updated service list to device detail group."""
    services = list(
        device.services.values('id', 'service_name', 'is_active', 'first_seen', 'last_seen')
    )
    for s in services:
        s['first_seen'] = s['first_seen'].isoformat() if s['first_seen'] else None
        s['last_seen'] = s['last_seen'].isoformat() if s['last_seen'] else None

    _send_to_group(f'device_{device.id}', {
        'type': 'services_updated',
        'services': services,
    })


def broadcast_device_software(device):
    """Send updated software list to device detail group."""
    software = list(
        device.software.values('id', 'software_name', 'is_installed', 'first_seen', 'last_seen')
    )
    for s in software:
        s['first_seen'] = s['first_seen'].isoformat() if s['first_seen'] else None
        s['last_seen'] = s['last_seen'].isoformat() if s['last_seen'] else None

    _send_to_group(f'device_{device.id}', {
        'type': 'software_updated',
        'software': software,
    })


def broadcast_dashboard_stats():
    """Send updated dashboard statistics."""
    total = Device.objects.count()
    online = Device.objects.filter(is_online=True).count()
    pending = PendingRegistration.objects.filter(status='pending').count()

    _send_to_group('dashboard', {
        'type': 'dashboard_stats',
        'stats': {
            'total_devices': total,
            'online_devices': online,
            'offline_devices': total - online,
            'pending_registrations': pending,
        },
    })


def broadcast_registration_created(registration):
    """Notify dashboard of a new pending registration."""
    _send_to_group('dashboard', {
        'type': 'registration_created',
        'registration': {
            'id': registration.id,
            'agent_id': registration.agent_id,
            'agent_name': registration.agent_name,
            'hostname': registration.hostname,
            'os_type': registration.os_type,
            'os_version': registration.os_version,
            'status': registration.status,
            'requested_at': registration.requested_at.isoformat(),
        },
    })
    broadcast_dashboard_stats()


def broadcast_registration_updated(registration):
    """Notify dashboard when a registration is approved/rejected."""
    _send_to_group('dashboard', {
        'type': 'registration_updated',
        'registration': {
            'id': registration.id,
            'agent_id': registration.agent_id,
            'agent_name': registration.agent_name,
            'hostname': registration.hostname,
            'os_type': registration.os_type,
            'status': registration.status,
            'approved_at': registration.approved_at.isoformat() if registration.approved_at else None,
        },
    })
    broadcast_dashboard_stats()
