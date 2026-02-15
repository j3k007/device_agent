import api from './api'

export default {
  getTokens() {
    return api.get('/agents/tokens/')
  },

  createToken(data) {
    return api.post('/agents/tokens/create/', data)
  },
}
