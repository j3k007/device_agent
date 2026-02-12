from rest_framework import serializers
from .models import Device, DeviceService, DeviceSoftware

class HeartbeatSerializer(serializers.Serializer):
    """
    Serializer for agent heartbeat data (Phase 3)
    """
    # Agent Info
    agent_id = serializers.CharField(max_length=255)
    agent_name = serializers.CharField(max_length=255)
    device_fingerprint = serializers.CharField(max_length=255)
    
    # Device Info
    hostname = serializers.CharField(max_length=255)
    os_type = serializers.CharField(max_length=50)
    os_version = serializers.CharField(max_length=100)
    
    # Hardware
    cpu_info = serializers.CharField()
    memory_total = serializers.IntegerField(min_value=0)
    memory_available = serializers.IntegerField(min_value=0)
    
    # Network - IPv4 → [IPv6...]
    ip_addresses = serializers.DictField(
        child=serializers.ListField(child=serializers.CharField()),
        required=False,
        default=dict
    )
    
    # Collections
    services = serializers.ListField(
        child=serializers.CharField(),
        required=False,
        default=list
    )
    installed_software = serializers.ListField(
        child=serializers.CharField(),
        required=False,
        default=list
    )
    
    # Metadata
    collected_at = serializers.DateTimeField()


class DeviceSerializer(serializers.ModelSerializer):
    """Serializer for Device model"""
    agent_id = serializers.CharField(source='agent_token.agent_id', read_only=True)
    agent_name = serializers.CharField(source='agent_token.agent_name', read_only=True)
    memory_usage_percent = serializers.SerializerMethodField()
    services_count = serializers.SerializerMethodField()
    software_count = serializers.SerializerMethodField()
    
    class Meta:
        model = Device
        fields = [
            'id', 'agent_id', 'agent_name', 'hostname', 'os_type', 'os_version',
            'cpu_info', 'memory_total', 'memory_available', 'memory_usage_percent',
            'ip_addresses', 'is_online', 'last_heartbeat', 'first_seen',
            'services_count', 'software_count'
        ]
    
    def get_memory_usage_percent(self, obj):
        return obj.memory_usage_percent
    
    def get_services_count(self, obj):
        return obj.services.filter(is_active=True).count()
    
    def get_software_count(self, obj):
        return obj.software.filter(is_installed=True).count()


class DeviceServiceSerializer(serializers.ModelSerializer):
    """Serializer for DeviceService model"""
    hostname = serializers.CharField(source='device.hostname', read_only=True)
    
    class Meta:
        model = DeviceService
        fields = ['id', 'device', 'hostname', 'service_name', 'is_active', 'first_seen', 'last_seen']


class DeviceSoftwareSerializer(serializers.ModelSerializer):
    """Serializer for DeviceSoftware model"""
    hostname = serializers.CharField(source='device.hostname', read_only=True)
    
    class Meta:
        model = DeviceSoftware
        fields = ['id', 'device', 'hostname', 'software_name', 'is_installed', 'first_seen', 'last_seen']