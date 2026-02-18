import { ref, onUnmounted } from 'vue'
import { useWebSocket } from '@/services/websocket'
import { useDeviceStore } from '@/stores/devices'
import { useRegistrationStore } from '@/stores/registrations'

/**
 * Composable that connects a dashboard WebSocket and pipes events
 * into the Pinia stores.
 *
 * Returns reactive `connected` ref and manual `close()` for cleanup.
 */
export function useDashboardSocket() {
  const connected = ref(false)
  const dashboardStats = ref(null)
  const token = localStorage.getItem('auth_token')

  if (!token) return { connected, dashboardStats, close: () => {} }

  const deviceStore = useDeviceStore()
  const registrationStore = useRegistrationStore()
  const ws = useWebSocket('/ws/dashboard/', { token })

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

  ws.on('dashboard_stats', (data) => {
    if (data.stats) {
      dashboardStats.value = data.stats
    }
  })

  ws.on('registration_created', (data) => {
    if (data.registration) {
      registrationStore.addRegistration(data.registration)
    }
  })

  ws.on('registration_updated', (data) => {
    if (data.registration) {
      registrationStore.updateRegistration(data.registration)
    }
  })

  ws.connect()

  onUnmounted(() => {
    ws.close()
  })

  return { connected, dashboardStats, close: () => ws.close() }
}
