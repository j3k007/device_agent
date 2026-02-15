import api from './api'

export default {
  getDevices() {
    return api.get('/devices/')
  },

  getDevice(id) {
    return api.get(`/devices/${id}/`)
  },

  getDeviceServices(id) {
    return api.get(`/devices/${id}/services/`)
  },

  getDeviceSoftware(id) {
    return api.get(`/devices/${id}/software/`)
  },

  getDashboardStats() {
    return api.get('/dashboard/')
  },

  healthCheck() {
    return api.get('/health/')
  },
}
