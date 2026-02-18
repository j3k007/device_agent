<script setup>
import { onMounted, computed } from 'vue'
import { useDeviceStore } from '@/stores/devices'
import { useRegistrationStore } from '@/stores/registrations'
import { useDashboardSocket } from '@/composables/useDashboardSocket'
import StatsCard from '@/components/dashboard/StatsCard.vue'

const deviceStore = useDeviceStore()
const registrationStore = useRegistrationStore()
const { connected, dashboardStats } = useDashboardSocket()

const totalDevices = computed(() =>
  dashboardStats.value?.total_devices ?? deviceStore.devices.length
)
const onlineDevices = computed(() =>
  dashboardStats.value?.online_devices ?? deviceStore.onlineDevices.length
)
const offlineDevices = computed(() =>
  dashboardStats.value?.offline_devices ?? deviceStore.offlineDevices.length
)
const pendingRegistrations = computed(() =>
  dashboardStats.value?.pending_registrations ?? registrationStore.pendingCount
)

onMounted(() => {
  deviceStore.fetchDevices()
  registrationStore.fetchRegistrations()
})
</script>

<template>
  <div class="dashboard">
    <div class="page-header">
      <h2 class="page-title">Dashboard</h2>
      <span class="ws-status" :class="connected ? 'ws-connected' : 'ws-disconnected'">
        {{ connected ? 'Live' : 'Connecting...' }}
      </span>
    </div>

    <div class="stats-grid">
      <StatsCard
        label="Total Devices"
        :value="totalDevices"
        :loading="deviceStore.loading"
      />
      <StatsCard
        label="Online"
        :value="onlineDevices"
        :loading="deviceStore.loading"
        variant="success"
      />
      <StatsCard
        label="Offline"
        :value="offlineDevices"
        :loading="deviceStore.loading"
        variant="danger"
      />
      <StatsCard
        label="Pending Registrations"
        :value="pendingRegistrations"
        :loading="registrationStore.loading"
        variant="warning"
      />
    </div>

    <div v-if="deviceStore.error" class="error-banner">
      {{ deviceStore.error }}
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  max-width: 1200px;
}

.page-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 24px;
}

.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.ws-status {
  font-size: 0.75rem;
  font-weight: 600;
  padding: 3px 10px;
  border-radius: 10px;
}

.ws-connected {
  background-color: var(--color-success-bg);
  color: var(--color-success);
}

.ws-disconnected {
  background-color: var(--color-warning-bg);
  color: var(--color-warning);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}

.error-banner {
  background-color: var(--color-danger-bg);
  color: var(--color-danger);
  padding: 12px 16px;
  border-radius: 8px;
  font-size: 0.9rem;
}
</style>
