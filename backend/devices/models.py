from django.db import models
from agents.models import AgentToken

class Device(models.Model):
    """Device information"""
    # Link to agent token (one-to-one)
    agent_token = models.OneToOneField(
        AgentToken,
        on_delete=models.CASCADE,
        related_name='device'
    )
    
    # Basic Info
    hostname = models.CharField(max_length=255)
    os_type = models.CharField(max_length=50)
    os_version = models.CharField(max_length=100)
    
    # Hardware
    cpu_info = models.TextField()
    memory_total = models.BigIntegerField(help_text="Total memory in bytes")
    memory_available = models.BigIntegerField(help_text="Available memory in bytes")
    
    # Network (JSON field for IP addresses)
    ip_addresses = models.JSONField(default=dict, help_text="IPv4 → [IPv6...]")
    
    # Status
    is_online = models.BooleanField(default=True)
    last_heartbeat = models.DateTimeField(auto_now=True)
    first_seen = models.DateTimeField(auto_now_add=True)
    
    class Meta:
        db_table = 'devices'
        ordering = ['-last_heartbeat']
    
    def __str__(self):
        return f"{self.hostname} ({self.agent_token.agent_id})"
    
    @property
    def memory_used(self):
        """Calculate used memory"""
        return self.memory_total - self.memory_available
    
    @property
    def memory_usage_percent(self):
        """Calculate memory usage percentage"""
        if self.memory_total == 0:
            return 0
        return round((self.memory_used / self.memory_total) * 100, 2)


class DeviceService(models.Model):
    """Services running on device"""
    device = models.ForeignKey(
        Device,
        on_delete=models.CASCADE,
        related_name='services'
    )
    service_name = models.CharField(max_length=255)
    # Tracking
    first_seen = models.DateTimeField(auto_now_add=True)
    last_seen = models.DateTimeField(auto_now=True)
    is_active = models.BooleanField(default=True)
    class Meta:
        db_table = 'device_services'
        unique_together = [['device', 'service_name']]
        ordering = ['service_name']
    
    def __str__(self):
        return f"{self.service_name} on {self.device.hostname}"


class DeviceSoftware(models.Model):
    """Software installed on device"""
    device = models.ForeignKey(
        Device,
        on_delete=models.CASCADE,
        related_name='software'
    )
    software_name = models.CharField(max_length=255)
    # Tracking
    first_seen = models.DateTimeField(auto_now_add=True)
    last_seen = models.DateTimeField(auto_now=True)
    is_installed = models.BooleanField(default=True)
    class Meta:
        db_table = 'device_software'
        unique_together = [['device', 'software_name']]
        ordering = ['software_name']
    
    def __str__(self):
        return f"{self.software_name} on {self.device.hostname}"