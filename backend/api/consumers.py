import json
import logging
from channels.generic.websocket import AsyncJsonWebsocketConsumer
from django.contrib.auth.models import AnonymousUser

logger = logging.getLogger(__name__)


class DashboardConsumer(AsyncJsonWebsocketConsumer):
    """
    WebSocket consumer for the main dashboard.

    Broadcasts:
    - device_updated: when a heartbeat is received
    - device_online / device_offline: status changes
    - registration_created: new pending registration
    - registration_updated: registration approved/rejected
    - dashboard_stats: aggregated stat updates
    """

    async def connect(self):
        user = self.scope.get('user', AnonymousUser())
        if isinstance(user, AnonymousUser) or not user.is_authenticated:
            await self.close()
            return

        await self.channel_layer.group_add('dashboard', self.channel_name)
        await self.accept()
        logger.info(f"Dashboard WebSocket connected: {user.username}")

    async def disconnect(self, close_code):
        await self.channel_layer.group_discard('dashboard', self.channel_name)

    async def device_updated(self, event):
        await self.send_json(event)

    async def device_online(self, event):
        await self.send_json(event)

    async def device_offline(self, event):
        await self.send_json(event)

    async def registration_created(self, event):
        await self.send_json(event)

    async def registration_updated(self, event):
        await self.send_json(event)

    async def dashboard_stats(self, event):
        await self.send_json(event)


class DeviceDetailConsumer(AsyncJsonWebsocketConsumer):
    """
    WebSocket consumer for a single device's detail page.

    Clients join a group named 'device_<id>' and receive
    live heartbeat updates for that specific device.
    """

    async def connect(self):
        user = self.scope.get('user', AnonymousUser())
        if isinstance(user, AnonymousUser) or not user.is_authenticated:
            await self.close()
            return

        self.device_id = self.scope['url_route']['kwargs']['device_id']
        self.group_name = f'device_{self.device_id}'

        await self.channel_layer.group_add(self.group_name, self.channel_name)
        await self.accept()
        logger.info(f"Device detail WebSocket connected: device {self.device_id}")

    async def disconnect(self, close_code):
        if hasattr(self, 'group_name'):
            await self.channel_layer.group_discard(self.group_name, self.channel_name)

    async def device_updated(self, event):
        await self.send_json(event)

    async def services_updated(self, event):
        await self.send_json(event)

    async def software_updated(self, event):
        await self.send_json(event)
