from django.contrib import admin
from django.utils.html import format_html
from django.urls import reverse
from django.db.models import Count
from .models import Device, DeviceService, DeviceSoftware
import json

@admin.register(Device)
class DeviceAdmin(admin.ModelAdmin):
    list_display = [
        'hostname',
        'agent_link',
        'os_info',
        'online_status',
        'memory_usage_display',
        'services_count',
        'software_count',
        'last_heartbeat_relative',
    ]
    
    list_filter = [
        'is_online',
        'os_type',
        'last_heartbeat',
        'first_seen',
    ]
    
    search_fields = [
        'hostname',
        'agent_token__agent_id',
        'agent_token__agent_name',
        'os_type',
        'os_version',
        'cpu_info',
    ]
    
    readonly_fields = [
        'agent_token',
        'hostname',
        'os_type',
        'os_version',
        'cpu_info',
        'memory_total_display',
        'memory_available_display',
        'memory_used_display',
        'memory_usage_percent',
        'ip_addresses_display',
        'first_seen',
        'last_heartbeat',
        'services_link',
        'software_link',
    ]
    
    fieldsets = (
        ('Device Information', {
            'fields': (
                'agent_token',
                'hostname',
                'os_type',
                'os_version',
                'cpu_info',
            )
        }),
        ('Memory Information', {
            'fields': (
                'memory_total_display',
                'memory_available_display',
                'memory_used_display',
                'memory_usage_percent',
            )
        }),
        ('Network Information', {
            'fields': ('ip_addresses_display',)
        }),
        ('Status', {
            'fields': (
                'is_online',
                'last_heartbeat',
                'first_seen',
            )
        }),
        ('Statistics', {
            'fields': (
                'services_link',
                'software_link',
            )
        }),
    )
    
    def agent_link(self, obj):
        """Link to agent token"""
        url = reverse('admin:agents_agenttoken_change', args=[obj.agent_token.id])
        return format_html(
            '<a href="{}">{}</a>',
            url,
            obj.agent_token.agent_id
        )
    agent_link.short_description = 'Agent'
    
    def os_info(self, obj):
        """Display OS type and version"""
        return f"{obj.os_type} {obj.os_version}"
    os_info.short_description = 'Operating System'
    
    def online_status(self, obj):
        """Display online status with color"""
        if obj.is_online:
            from django.utils import timezone
            time_diff = timezone.now() - obj.last_heartbeat
            
            if time_diff.total_seconds() < 120:  # Less than 2 minutes
                return format_html(
                    '<span style="color: green; font-weight: bold;">● Online</span>'
                )
            elif time_diff.total_seconds() < 600:  # Less than 10 minutes
                return format_html(
                    '<span style="color: orange; font-weight: bold;">● Stale</span>'
                )
        
        return format_html(
            '<span style="color: red; font-weight: bold;">● Offline</span>'
        )
    online_status.short_description = 'Status'
    
    def memory_usage_display(self, obj):
        """Display memory usage as progress bar"""
        percent = obj.memory_usage_percent
        
        if percent < 50:
            color = '#4caf50'  # Green
        elif percent < 80:
            color = '#ff9800'  # Orange
        else:
            color = '#f44336'  # Red
        
        return format_html(
            '<div style="width: 100px; background-color: #e0e0e0; border-radius: 3px;">'
            '<div style="width: {}%; background-color: {}; height: 20px; border-radius: 3px; text-align: center; color: white; font-size: 11px; line-height: 20px;">{}%</div>'
            '</div>',
            percent, color, percent
        )
    memory_usage_display.short_description = 'Memory Usage'
    
    def services_count(self, obj):
        """Display active services count"""
        count = obj.services.filter(is_active=True).count()
        return format_html('<strong>{}</strong>', count)
    services_count.short_description = 'Services'
    
    def software_count(self, obj):
        """Display installed software count"""
        count = obj.software.filter(is_installed=True).count()
        return format_html('<strong>{}</strong>', count)
    software_count.short_description = 'Software'
    
    def last_heartbeat_relative(self, obj):
        """Display last heartbeat as relative time"""
        from django.utils import timezone
        time_diff = timezone.now() - obj.last_heartbeat
        
        seconds = time_diff.total_seconds()
        if seconds < 60:
            return f"{int(seconds)}s ago"
        elif seconds < 3600:
            return f"{int(seconds / 60)}m ago"
        elif seconds < 86400:
            return f"{int(seconds / 3600)}h ago"
        else:
            return f"{int(seconds / 86400)}d ago"
    last_heartbeat_relative.short_description = 'Last Heartbeat'
    
    # Readonly field displays
    def memory_total_display(self, obj):
        return self._format_bytes(obj.memory_total)
    memory_total_display.short_description = 'Total Memory'
    
    def memory_available_display(self, obj):
        return self._format_bytes(obj.memory_available)
    memory_available_display.short_description = 'Available Memory'
    
    def memory_used_display(self, obj):
        return self._format_bytes(obj.memory_used)
    memory_used_display.short_description = 'Used Memory'
    
    def ip_addresses_display(self, obj):
        """Display IP addresses as formatted JSON"""
        if obj.ip_addresses:
            formatted = json.dumps(obj.ip_addresses, indent=2)
            return format_html('<pre style="margin: 0;">{}</pre>', formatted)
        return "-"
    ip_addresses_display.short_description = 'IP Addresses'
    
    def services_link(self, obj):
        """Link to services"""
        count = obj.services.filter(is_active=True).count()
        url = reverse('admin:devices_deviceservice_changelist') + f'?device__id__exact={obj.id}'
        return format_html(
            '<a href="{}" style="font-weight: bold;">{} active services</a>',
            url, count
        )
    services_link.short_description = 'Services'
    
    def software_link(self, obj):
        """Link to software"""
        count = obj.software.filter(is_installed=True).count()
        url = reverse('admin:devices_devicesoftware_changelist') + f'?device__id__exact={obj.id}'
        return format_html(
            '<a href="{}" style="font-weight: bold;">{} installed applications</a>',
            url, count
        )
    software_link.short_description = 'Software'
    
    @staticmethod
    def _format_bytes(bytes_value):
        """Format bytes to human-readable format"""
        for unit in ['B', 'KB', 'MB', 'GB', 'TB']:
            if bytes_value < 1024.0:
                return f"{bytes_value:.2f} {unit}"
            bytes_value /= 1024.0
        return f"{bytes_value:.2f} PB"


@admin.register(DeviceService)
class DeviceServiceAdmin(admin.ModelAdmin):
    list_display = [
        'service_name',
        'device_link',
        'is_active',
        'last_seen_relative',
    ]
    
    list_filter = [
        'is_active',
        'last_seen',
        'device__os_type',
    ]
    
    search_fields = [
        'service_name',
        'device__hostname',
        'device__agent_token__agent_id',
    ]
    
    readonly_fields = [
        'device',
        'service_name',
        'first_seen',
        'last_seen',
    ]
    
    def device_link(self, obj):
        """Link to device"""
        url = reverse('admin:devices_device_change', args=[obj.device.id])
        return format_html(
            '<a href="{}">{}</a>',
            url,
            obj.device.hostname
        )
    device_link.short_description = 'Device'
    
    def last_seen_relative(self, obj):
        """Display last seen as relative time"""
        from django.utils import timezone
        time_diff = timezone.now() - obj.last_seen
        
        seconds = time_diff.total_seconds()
        if seconds < 60:
            return f"{int(seconds)}s ago"
        elif seconds < 3600:
            return f"{int(seconds / 60)}m ago"
        elif seconds < 86400:
            return f"{int(seconds / 3600)}h ago"
        else:
            return f"{int(seconds / 86400)}d ago"
    last_seen_relative.short_description = 'Last Seen'


@admin.register(DeviceSoftware)
class DeviceSoftwareAdmin(admin.ModelAdmin):
    list_display = [
        'software_name',
        'device_link',
        'is_installed',
        'last_seen_relative',
    ]
    
    list_filter = [
        'is_installed',
        'last_seen',
        'device__os_type',
    ]
    
    search_fields = [
        'software_name',
        'device__hostname',
        'device__agent_token__agent_id',
    ]
    
    readonly_fields = [
        'device',
        'software_name',
        'first_seen',
        'last_seen',
    ]
    
    def device_link(self, obj):
        """Link to device"""
        url = reverse('admin:devices_device_change', args=[obj.device.id])
        return format_html(
            '<a href="{}">{}</a>',
            url,
            obj.device.hostname
        )
    device_link.short_description = 'Device'
    
    def last_seen_relative(self, obj):
        """Display last seen as relative time"""
        from django.utils import timezone
        time_diff = timezone.now() - obj.last_seen
        
        seconds = time_diff.total_seconds()
        if seconds < 60:
            return f"{int(seconds)}s ago"
        elif seconds < 3600:
            return f"{int(seconds / 60)}m ago"
        elif seconds < 86400:
            return f"{int(seconds / 3600)}h ago"
        else:
            return f"{int(seconds / 86400)}d ago"
    last_seen_relative.short_description = 'Last Seen'