import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '@/services/api'

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('auth_token') || null)
  const user = ref(JSON.parse(localStorage.getItem('auth_user') || 'null'))
  const loginError = ref(null)

  const isAuthenticated = computed(() => !!token.value)

  async function login(username, password) {
    loginError.value = null
    try {
      const response = await api.post('/auth/login/', { username, password })
      token.value = response.data.token
      user.value = response.data.user
      localStorage.setItem('auth_token', response.data.token)
      localStorage.setItem('auth_user', JSON.stringify(response.data.user))
      return true
    } catch (err) {
      loginError.value = err.response?.data?.error || 'Login failed'
      return false
    }
  }

  async function logout() {
    try {
      await api.post('/auth/logout/')
    } catch {
      // ignore — token may already be invalid
    }
    token.value = null
    user.value = null
    localStorage.removeItem('auth_token')
    localStorage.removeItem('auth_user')
  }

  return {
    token,
    user,
    loginError,
    isAuthenticated,
    login,
    logout,
  }
})
