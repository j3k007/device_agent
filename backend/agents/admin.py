from django.contrib import admin
from .models import AgentToken

@admin.register(AgentToken)
class AgentTokenAdmin(admin.ModelAdmin):
    list_display = ['agent_id', 'agent_name', 'is_active', 'created_at', 'last_used']
    list_filter = ['is_active', 'created_at']
    search_fields = ['agent_id', 'agent_name', 'token']
    readonly_fields = ['token', 'created_at', 'last_used']
    
    fieldsets = (
        ('Agent Information', {
            'fields': ('agent_id', 'agent_name')
        }),
        ('Authentication', {
            'fields': ('token', 'is_active')
        }),
        ('Metadata', {
            'fields': ('created_at', 'last_used')
        }),
    )