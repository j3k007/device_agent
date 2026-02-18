/**
 * Reactive WebSocket service with auto-reconnect.
 *
 * Usage:
 *   import { useWebSocket } from '@/services/websocket'
 *   const ws = useWebSocket('/ws/dashboard/', { token })
 *   ws.on('device_updated', (data) => { ... })
 *   ws.connect()
 *   ws.close()
 */

const RECONNECT_DELAYS = [1000, 2000, 4000, 8000, 16000]

export function useWebSocket(path, { token } = {}) {
  let socket = null
  let reconnectAttempt = 0
  let reconnectTimer = null
  let intentionallyClosed = false
  const listeners = {}

  function getUrl() {
    const proto = window.location.protocol === 'https:' ? 'wss' : 'ws'
    const host = window.location.host
    const query = token ? `?token=${token}` : ''
    return `${proto}://${host}${path}${query}`
  }

  function on(event, callback) {
    if (!listeners[event]) listeners[event] = []
    listeners[event].push(callback)
  }

  function off(event, callback) {
    if (!listeners[event]) return
    listeners[event] = listeners[event].filter((cb) => cb !== callback)
  }

  function emit(event, data) {
    ;(listeners[event] || []).forEach((cb) => cb(data))
  }

  function connect() {
    if (socket && (socket.readyState === WebSocket.OPEN || socket.readyState === WebSocket.CONNECTING)) {
      return
    }
    intentionallyClosed = false

    const url = getUrl()
    socket = new WebSocket(url)

    socket.onopen = () => {
      reconnectAttempt = 0
      emit('connected')
    }

    socket.onclose = () => {
      emit('disconnected')
      if (!intentionallyClosed) {
        scheduleReconnect()
      }
    }

    socket.onerror = () => {
      // onclose will fire after this
    }

    socket.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data)
        if (data.type) {
          emit(data.type, data)
        }
        emit('message', data)
      } catch {
        // ignore non-JSON messages
      }
    }
  }

  function scheduleReconnect() {
    const delay = RECONNECT_DELAYS[Math.min(reconnectAttempt, RECONNECT_DELAYS.length - 1)]
    reconnectAttempt++
    reconnectTimer = setTimeout(() => {
      connect()
    }, delay)
  }

  function close() {
    intentionallyClosed = true
    clearTimeout(reconnectTimer)
    if (socket) {
      socket.close()
      socket = null
    }
  }

  function isConnected() {
    return socket && socket.readyState === WebSocket.OPEN
  }

  return { on, off, connect, close, isConnected }
}
