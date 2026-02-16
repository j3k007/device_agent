from rest_framework.decorators import api_view, permission_classes
from rest_framework.response import Response
from rest_framework import status
from rest_framework.permissions import IsAuthenticated
from .models import Device, DeviceService, DeviceSoftware
from .serializers import DeviceSerializer, DeviceServiceSerializer, DeviceSoftwareSerializer
import logging

logger = logging.getLogger(__name__)


@api_view(['GET'])
@permission_classes([IsAuthenticated])
def list_devices(request):
    """
    List all devices.

    GET /api/devices/
    """
    devices = Device.objects.select_related('agent_token').all()
    serializer = DeviceSerializer(devices, many=True)
    return Response({
        'status': 'success',
        'count': devices.count(),
        'devices': serializer.data,
    })


@api_view(['GET'])
@permission_classes([IsAuthenticated])
def device_detail(request, pk):
    """
    Get a single device by ID.

    GET /api/devices/<pk>/
    """
    try:
        device = Device.objects.select_related('agent_token').get(pk=pk)
    except Device.DoesNotExist:
        return Response(
            {'error': 'Device not found'},
            status=status.HTTP_404_NOT_FOUND,
        )

    serializer = DeviceSerializer(device)
    return Response(serializer.data)


@api_view(['GET'])
@permission_classes([IsAuthenticated])
def device_services(request, pk):
    """
    List services for a device.

    GET /api/devices/<pk>/services/
    """
    try:
        device = Device.objects.get(pk=pk)
    except Device.DoesNotExist:
        return Response(
            {'error': 'Device not found'},
            status=status.HTTP_404_NOT_FOUND,
        )

    services = DeviceService.objects.filter(device=device)
    serializer = DeviceServiceSerializer(services, many=True)
    return Response({
        'status': 'success',
        'count': services.count(),
        'services': serializer.data,
    })


@api_view(['GET'])
@permission_classes([IsAuthenticated])
def device_software(request, pk):
    """
    List software for a device.

    GET /api/devices/<pk>/software/
    """
    try:
        device = Device.objects.get(pk=pk)
    except Device.DoesNotExist:
        return Response(
            {'error': 'Device not found'},
            status=status.HTTP_404_NOT_FOUND,
        )

    software = DeviceSoftware.objects.filter(device=device)
    serializer = DeviceSoftwareSerializer(software, many=True)
    return Response({
        'status': 'success',
        'count': software.count(),
        'software': serializer.data,
    })
