import api from './api'

export default {
  getRegistrations() {
    return api.get('/agents/registrations/')
  },

  approveRegistration(id) {
    return api.post(`/agents/registrations/${id}/approve/`)
  },

  rejectRegistration(id) {
    return api.post(`/agents/registrations/${id}/reject/`)
  },
}
