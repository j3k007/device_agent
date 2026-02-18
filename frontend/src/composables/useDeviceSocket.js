import { ref, onUnmounted } from 'vue'
import { useWebSocket } from '@/services/websocket'
import { useDeviceStore } from '@/stores/devices'

/**
 * Composable that subscribes to real-time updates for a single device.
 *
 * Returns reactive refs for connection status, services, and software.
 */
export function useDeviceSocket(deviceId) {
  const connected = ref(false)
  const services = ref(null)
  const software = ref(null)
  const token = localStorage.getItem('auth_token')

  if (!token || !deviceId) return { connected, services, software, close: () => {} }

  const deviceStore = useDeviceStore()
  const ws = useWebSocket(`/ws/devices/${deviceId}/`, { token })

  ws.on('connected', () => {
    connected.value = true
  })

  ws.on('disconnected', () => {
    connected.value = false
  })

  ws.on('device_updated', (data) => {
    if (data.device) {
      deviceStore.applyDeviceUpdate(data.device)
    }
  })

  ws.on('services_updated', (data) => {
    if (data.services) {
      services.value = data.services
    }
  })

  ws.on('software_updated', (data) => {
    if (data.software) {
      software.value = data.software
    }
  })

  ws.connect()

  onUnmounted(() => {
    ws.close()
  })

  return { connected, services, software, close: () => ws.close() }
}
