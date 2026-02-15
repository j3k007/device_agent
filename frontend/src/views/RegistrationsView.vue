<script setup>
import { onMounted } from 'vue'
import { useRegistrationStore } from '@/stores/registrations'

const registrationStore = useRegistrationStore()

onMounted(() => {
  registrationStore.fetchRegistrations()
})

function statusClass(status) {
  return {
    pending: 'badge-warning',
    approved: 'badge-success',
    rejected: 'badge-danger',
  }[status] || ''
}
</script>

<template>
  <div class="registrations-page">
    <h2 class="page-title">Registrations</h2>

    <div v-if="registrationStore.loading" class="loading">Loading registrations...</div>

    <div v-else-if="registrationStore.error" class="error-banner">
      {{ registrationStore.error }}
    </div>

    <div v-else-if="registrationStore.registrations.length === 0" class="empty-state">
      No registration requests.
    </div>

    <table v-else class="reg-table">
      <thead>
        <tr>
          <th>Agent ID</th>
          <th>Name</th>
          <th>Hostname</th>
          <th>OS</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="reg in registrationStore.registrations" :key="reg.id">
          <td class="mono">{{ reg.agent_id }}</td>
          <td>{{ reg.agent_name }}</td>
          <td>{{ reg.hostname }}</td>
          <td>{{ reg.os_type }}</td>
          <td>
            <span class="status-badge" :class="statusClass(reg.status)">
              {{ reg.status }}
            </span>
          </td>
          <td>
            <template v-if="reg.status === 'pending'">
              <button class="btn btn-approve" @click="registrationStore.approve(reg.id)">
                Approve
              </button>
              <button class="btn btn-reject" @click="registrationStore.reject(reg.id)">
                Reject
              </button>
            </template>
            <span v-else class="text-muted">--</span>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.registrations-page {
  max-width: 1200px;
}

.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 24px 0;
}

.reg-table {
  width: 100%;
  border-collapse: collapse;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
}

.reg-table th,
.reg-table td {
  text-align: left;
  padding: 12px 16px;
  font-size: 0.85rem;
}

.reg-table th {
  background-color: var(--bg-hover);
  color: var(--text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  font-size: 0.75rem;
  letter-spacing: 0.5px;
}

.reg-table td {
  border-top: 1px solid var(--border-color);
  color: var(--text-primary);
}

.mono {
  font-family: monospace;
  font-size: 0.8rem;
}

.status-badge {
  font-size: 0.75rem;
  font-weight: 600;
  padding: 3px 10px;
  border-radius: 10px;
  text-transform: capitalize;
}

.badge-warning {
  background-color: var(--color-warning-bg);
  color: var(--color-warning);
}

.badge-success {
  background-color: var(--color-success-bg);
  color: var(--color-success);
}

.badge-danger {
  background-color: var(--color-danger-bg);
  color: var(--color-danger);
}

.btn {
  padding: 5px 12px;
  border: none;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 600;
  cursor: pointer;
  margin-right: 6px;
}

.btn-approve {
  background-color: var(--color-success);
  color: #fff;
}

.btn-approve:hover {
  opacity: 0.85;
}

.btn-reject {
  background-color: var(--color-danger);
  color: #fff;
}

.btn-reject:hover {
  opacity: 0.85;
}

.text-muted {
  color: var(--text-muted);
}

.loading,
.empty-state {
  color: var(--text-secondary);
  padding: 40px 0;
  text-align: center;
}

.error-banner {
  background-color: var(--color-danger-bg);
  color: var(--color-danger);
  padding: 12px 16px;
  border-radius: 8px;
}
</style>
