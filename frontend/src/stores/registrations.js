import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import registrationService from '@/services/registrationService'

export const useRegistrationStore = defineStore('registrations', () => {
  const registrations = ref([])
  const loading = ref(false)
  const error = ref(null)

  const pendingCount = computed(() =>
    registrations.value.filter((r) => r.status === 'pending').length
  )

  async function fetchRegistrations() {
    loading.value = true
    error.value = null
    try {
      const response = await registrationService.getRegistrations()
      registrations.value = response.data.registrations || response.data
    } catch (err) {
      error.value = err.response?.data?.error || 'Failed to fetch registrations'
    } finally {
      loading.value = false
    }
  }

  async function approve(id) {
    try {
      await registrationService.approveRegistration(id)
      await fetchRegistrations()
    } catch (err) {
      error.value = err.response?.data?.error || 'Failed to approve registration'
    }
  }

  async function reject(id) {
    try {
      await registrationService.rejectRegistration(id)
      await fetchRegistrations()
    } catch (err) {
      error.value = err.response?.data?.error || 'Failed to reject registration'
    }
  }

  function addRegistration(registration) {
    const exists = registrations.value.some((r) => r.id === registration.id)
    if (!exists) {
      registrations.value.unshift(registration)
    }
  }

  function updateRegistration(registration) {
    const idx = registrations.value.findIndex((r) => r.id === registration.id)
    if (idx !== -1) {
      registrations.value[idx] = { ...registrations.value[idx], ...registration }
    }
  }

  return {
    registrations,
    loading,
    error,
    pendingCount,
    fetchRegistrations,
    approve,
    reject,
    addRegistration,
    updateRegistration,
  }
})
