import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import deviceService from '@/services/deviceService'

export const useDeviceStore = defineStore('devices', () => {
  const devices = ref([])
  const currentDevice = ref(null)
  const loading = ref(false)
  const error = ref(null)

  const onlineDevices = computed(() =>
    devices.value.filter((d) => d.is_online)
  )

  const offlineDevices = computed(() =>
    devices.value.filter((d) => !d.is_online)
  )

  async function fetchDevices() {
    loading.value = true
    error.value = null
    try {
      const response = await deviceService.getDevices()
      devices.value = response.data.devices || response.data
    } catch (err) {
      error.value = err.response?.data?.error || 'Failed to fetch devices'
    } finally {
      loading.value = false
    }
  }

  async function fetchDevice(id) {
    loading.value = true
    error.value = null
    try {
      const response = await deviceService.getDevice(id)
      currentDevice.value = response.data
    } catch (err) {
      error.value = err.response?.data?.error || 'Failed to fetch device'
    } finally {
      loading.value = false
    }
  }

  return {
    devices,
    currentDevice,
    loading,
    error,
    onlineDevices,
    offlineDevices,
    fetchDevices,
    fetchDevice,
  }
})
