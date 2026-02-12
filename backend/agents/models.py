from django.db import models
from django.contrib.auth.models import User
import secrets
from django.utils import timezone
import secrets

class AgentToken(models.Model):
    """API tokens for device agents"""
    
    token = models.CharField(max_length=64, unique=True, db_index=True)
    agent_id = models.CharField(max_length=255, unique=True)
    agent_name = models.CharField(max_length=255)
    
    created_at = models.DateTimeField(auto_now_add=True)
    is_active = models.BooleanField(default=True)
    last_used = models.DateTimeField(null=True, blank=True)
    
    # ✅ Device binding fields
    device_fingerprint = models.CharField(
        max_length=255,
        null=True,
        blank=True,
        db_index=True,
        help_text="Hardware fingerprint to bind token to specific device"
    )
    bound_at = models.DateTimeField(
        null=True,
        blank=True,
        help_text="When token was bound to a device"
    )
    bound_hostname = models.CharField(
        max_length=255,
        blank=True,
        help_text="Hostname when token was first bound"
    )
    
    # ✅ Security tracking
    fingerprint_mismatch_count = models.IntegerField(
        default=0,
        help_text="Number of times token was used with wrong fingerprint"
    )
    last_fingerprint_mismatch = models.DateTimeField(
        null=True,
        blank=True,
        help_text="Last time a fingerprint mismatch was detected"
    )
    
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
        
    def bind_to_device(self, fingerprint, hostname):
        """Bind token to a specific device"""
        if not self.device_fingerprint:
            self.device_fingerprint = fingerprint
            self.bound_hostname = hostname
            self.bound_at = timezone.now()
            self.save(update_fields=['device_fingerprint', 'bound_hostname', 'bound_at'])
            return True
        return False
    
    def validate_fingerprint(self, fingerprint):
        """Check if fingerprint matches bound device"""
        if not self.device_fingerprint:
            # Not bound yet, any device can use it
            return True
        
        return self.device_fingerprint == fingerprint
    
    def record_fingerprint_mismatch(self):
        """Record a fingerprint mismatch attempt"""
        self.fingerprint_mismatch_count += 1
        self.last_fingerprint_mismatch = timezone.now()
        self.save(update_fields=['fingerprint_mismatch_count', 'last_fingerprint_mismatch'])

class PendingRegistration(models.Model):
    """
    Pending device registration requests
    """
    # Device Info
    agent_id = models.CharField(max_length=255, unique=True, db_index=True)
    agent_name = models.CharField(max_length=255)
    hostname = models.CharField(max_length=255)
    os_type = models.CharField(max_length=50)
    os_version = models.CharField(max_length=100)
    device_fingerprint = models.CharField(max_length=255, db_index=True)
    
    # Registration Info
    requested_at = models.DateTimeField(auto_now_add=True)
    approved_at = models.DateTimeField(null=True, blank=True)
    approved_by = models.ForeignKey(
        User,
        on_delete=models.SET_NULL,
        null=True,
        blank=True,
        related_name='approved_registrations'
    )
    
    # Status
    status = models.CharField(
        max_length=20,
        choices=[
            ('pending', 'Pending'),
            ('approved', 'Approved'),
            ('rejected', 'Rejected'),
        ],
        default='pending',
        db_index=True
    )
    
    # Generated token (after approval)
    token = models.OneToOneField(
        AgentToken,
        on_delete=models.SET_NULL,
        null=True,
        blank=True,
        related_name='registration'
    )
    
    class Meta:
        db_table = 'pending_registrations'
        ordering = ['-requested_at']
        verbose_name = 'Pending Registration'
        verbose_name_plural = 'Pending Registrations'
    
    def __str__(self):
        return f"{self.agent_name} ({self.agent_id}) - {self.status}"
    
    def approve(self, user):
        """Approve registration and create token"""
        if self.status != 'pending':
            raise ValueError("Only pending registrations can be approved")
        
        # Create agent token with device fingerprint already bound
        token = AgentToken.objects.create(
            agent_id=self.agent_id,
            agent_name=self.agent_name,
            device_fingerprint=self.device_fingerprint,
            bound_hostname=self.hostname,
            bound_at=timezone.now()
        )
        
        self.token = token
        self.status = 'approved'
        self.approved_at = timezone.now()
        self.approved_by = user
        self.save()
        
        return token
    
    def reject(self):
        """Reject registration"""
        if self.status != 'pending':
            raise ValueError("Only pending registrations can be rejected")
        
        self.status = 'rejected'
        self.save()