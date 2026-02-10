from django.db import models
import secrets

class AgentToken(models.Model):
    """API tokens for device agents"""
    
    token = models.CharField(max_length=64, unique=True, db_index=True)
    agent_id = models.CharField(max_length=255, unique=True)
    agent_name = models.CharField(max_length=255)
    
    created_at = models.DateTimeField(auto_now_add=True)
    is_active = models.BooleanField(default=True)
    last_used = models.DateTimeField(null=True, blank=True)
    
    class Meta:
        db_table = 'agent_tokens'
        ordering = ['-created_at']
        verbose_name = 'Agent Token'
        verbose_name_plural = 'Agent Tokens'
    
    def __str__(self):
        return f"{self.agent_name} ({self.agent_id})"
    
    @staticmethod
    def generate_token():
        return f"agt_{secrets.token_urlsafe(32)}"
    
    def save(self, *args, **kwargs):
        if not self.token:
            self.token = self.generate_token()
        super().save(*args, **kwargs)