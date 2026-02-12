from django.contrib import admin
from django.utils.html import format_html
from django.urls import reverse, path
from django.shortcuts import redirect
from django.contrib import messages
from django.utils import timezone
from .models import AgentToken, PendingRegistration

@admin.register(AgentToken)
class AgentTokenAdmin(admin.ModelAdmin):
    list_display = [
        'agent_id',
        'agent_name',
        'is_active',
        'bound_status',
        'mismatch_alerts',
        'created_at',
        'last_used',
    ]
    
    list_filter = [
        'is_active',
        'created_at',
        'last_used',
        'bound_at',
    ]
    
    search_fields = [
        'agent_id',
        'agent_name',
        'token',
        'bound_hostname',
    ]
    
    readonly_fields = [
        'token',
        'created_at',
        'last_used',
        'device_fingerprint',
        'bound_at',
        'bound_hostname',
        'fingerprint_mismatch_count',
        'last_fingerprint_mismatch',
    ]
    
    fieldsets = (
        ('Agent Information', {
            'fields': ('agent_id', 'agent_name')
        }),
        ('Authentication', {
            'fields': ('token', 'is_active')
        }),
        ('Device Binding', {
            'fields': (
                'device_fingerprint',
                'bound_hostname',
                'bound_at',
            ),
            'description': 'Token is automatically bound to the first device that uses it.'
        }),
        ('Security Alerts', {
            'fields': (
                'fingerprint_mismatch_count',
                'last_fingerprint_mismatch',
            ),
            'classes': ('collapse',)
        }),
        ('Metadata', {
            'fields': ('created_at', 'last_used')
        }),
    )
    
    def bound_status(self, obj):
        """Show if token is bound to a device"""
        if obj.device_fingerprint:
            return format_html(
                '<span style="color: green;" title="{}">● Bound to {}</span>',
                obj.device_fingerprint[:16] + '...',
                obj.bound_hostname or 'device'
            )
        return format_html('<span style="color: gray;">○ Not bound yet</span>')
    bound_status.short_description = 'Device Binding'
    
    def mismatch_alerts(self, obj):
        """Show mismatch count with warning"""
        if obj.fingerprint_mismatch_count == 0:
            return format_html('<span style="color: green;">✓ None</span>')
        elif obj.fingerprint_mismatch_count < 3:
            return format_html(
                '<span style="color: orange;">⚠ {} attempts</span>',
                obj.fingerprint_mismatch_count
            )
        else:
            return format_html(
                '<span style="color: red; font-weight: bold;">🚨 {} attempts</span>',
                obj.fingerprint_mismatch_count
            )
    mismatch_alerts.short_description = 'Security Alerts'


@admin.register(PendingRegistration)
class PendingRegistrationAdmin(admin.ModelAdmin):
    list_display = [
        'agent_name',
        'agent_id',
        'hostname',
        'os_info',
        'status_display',
        'requested_at',
        'action_buttons',
    ]
    
    list_filter = [
        'status',
        'os_type',
        'requested_at',
    ]
    
    search_fields = [
        'agent_id',
        'agent_name',
        'hostname',
        'device_fingerprint',
    ]
    
    readonly_fields = [
        'agent_id',
        'agent_name',
        'hostname',
        'os_type',
        'os_version',
        'device_fingerprint',
        'fingerprint_short',
        'requested_at',
        'approved_at',
        'approved_by',
        'token',
        'token_display',
    ]
    
    fieldsets = (
        ('Device Information', {
            'fields': (
                'agent_id',
                'agent_name',
                'hostname',
                'os_type',
                'os_version',
            )
        }),
        ('Device Fingerprint', {
            'fields': (
                'fingerprint_short',
                'device_fingerprint',
            ),
            'description': 'Unique hardware identifier for this device'
        }),
        ('Registration Status', {
            'fields': (
                'status',
                'requested_at',
                'approved_at',
                'approved_by',
            )
        }),
        ('Generated Token', {
            'fields': (
                'token',
                'token_display',
            )
        }),
    )
    
    def os_info(self, obj):
        """Display OS type and version"""
        return f"{obj.os_type} {obj.os_version}"
    os_info.short_description = 'Operating System'
    
    def status_display(self, obj):
        """Display status with color"""
        colors = {
            'pending': 'orange',
            'approved': 'green',
            'rejected': 'red',
        }
        color = colors.get(obj.status, 'gray')
        return format_html(
            '<span style="color: {}; font-weight: bold;">● {}</span>',
            color,
            obj.get_status_display()
        )
    status_display.short_description = 'Status'
    
    def fingerprint_short(self, obj):
        """Display shortened fingerprint"""
        if obj.device_fingerprint:
            return f"{obj.device_fingerprint[:16]}..."
        return "-"
    fingerprint_short.short_description = 'Fingerprint (short)'
    
    def action_buttons(self, obj):
        """Display approve/reject buttons"""
        if obj.status == 'pending':
            return format_html(
                '<a class="button" href="{}/approve/" style="background-color: #417690; color: white; padding: 5px 10px; text-decoration: none; border-radius: 3px;">Approve</a> '
                '<a class="button" href="{}/reject/" style="background-color: #ba2121; color: white; padding: 5px 10px; text-decoration: none; border-radius: 3px; margin-left: 5px;">Reject</a>',
                obj.id, obj.id
            )
        elif obj.status == 'approved':
            return format_html('<span style="color: green;">✓ Approved</span>')
        elif obj.status == 'rejected':
            return format_html('<span style="color: red;">✗ Rejected</span>')
        return "-"
    action_buttons.short_description = 'Actions'
    
    def token_display(self, obj):
        """Display token if approved"""
        if obj.token:
            return format_html(
                '<code style="background: #f0f0f0; padding: 5px; display: block; word-break: break-all;">{}</code>',
                obj.token.token
            )
        return "Not approved yet"
    token_display.short_description = 'API Token'
    
    def get_urls(self):
        """Add custom URLs for approve/reject"""
        urls = super().get_urls()
        custom_urls = [
            path('<int:registration_id>/approve/', self.admin_site.admin_view(self.approve_registration), name='approve_registration'),
            path('<int:registration_id>/reject/', self.admin_site.admin_view(self.reject_registration), name='reject_registration'),
        ]
        return custom_urls + urls
    
    def approve_registration(self, request, registration_id):
        """Approve a registration"""
        try:
            registration = PendingRegistration.objects.get(id=registration_id)
            
            if registration.status != 'pending':
                messages.error(request, f"Cannot approve: Registration is already {registration.status}")
            else:
                token = registration.approve(request.user)
                messages.success(
                    request,
                    f"✓ Registration approved! Token: {token.token}"
                )
        except PendingRegistration.DoesNotExist:
            messages.error(request, "Registration not found")
        except Exception as e:
            messages.error(request, f"Error approving registration: {e}")
        
        return redirect('admin:agents_pendingregistration_changelist')
    
    def reject_registration(self, request, registration_id):
        """Reject a registration"""
        try:
            registration = PendingRegistration.objects.get(id=registration_id)
            
            if registration.status != 'pending':
                messages.error(request, f"Cannot reject: Registration is already {registration.status}")
            else:
                registration.reject()
                messages.success(request, f"✓ Registration rejected: {registration.agent_id}")
        except PendingRegistration.DoesNotExist:
            messages.error(request, "Registration not found")
        except Exception as e:
            messages.error(request, f"Error rejecting registration: {e}")
        
        return redirect('admin:agents_pendingregistration_changelist')
    
    # Bulk actions
    actions = ['approve_selected', 'reject_selected']
    
    def approve_selected(self, request, queryset):
        """Bulk approve registrations"""
        count = 0
        for registration in queryset.filter(status='pending'):
            try:
                registration.approve(request.user)
                count += 1
            except Exception as e:
                self.message_user(
                    request,
                    f"Failed to approve {registration.agent_id}: {e}",
                    level=messages.ERROR
                )
        
        self.message_user(
            request,
            f"Successfully approved {count} registration(s)",
            level=messages.SUCCESS
        )
    approve_selected.short_description = "✓ Approve selected registrations"
    
    def reject_selected(self, request, queryset):
        """Bulk reject registrations"""
        count = queryset.filter(status='pending').update(status='rejected')
        self.message_user(
            request,
            f"Successfully rejected {count} registration(s)",
            level=messages.SUCCESS
        )
    reject_selected.short_description = "✗ Reject selected registrations"