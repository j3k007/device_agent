<script setup>
import { onMounted } from 'vue'
import { useDeviceStore } from '@/stores/devices'
import { useRegistrationStore } from '@/stores/registrations'
import StatsCard from '@/components/dashboard/StatsCard.vue'

const deviceStore = useDeviceStore()
const registrationStore = useRegistrationStore()

onMounted(() => {
  deviceStore.fetchDevices()
  registrationStore.fetchRegistrations()
})
</script>

<template>
  <div class="dashboard">
    <h2 class="page-title">Dashboard</h2>

    <div class="stats-grid">
      <StatsCard
        label="Total Devices"
        :value="deviceStore.devices.length"
        :loading="deviceStore.loading"
      />
      <StatsCard
        label="Online"
        :value="deviceStore.onlineDevices.length"
        :loading="deviceStore.loading"
        variant="success"
      />
      <StatsCard
        label="Offline"
        :value="deviceStore.offlineDevices.length"
        :loading="deviceStore.loading"
        variant="danger"
      />
      <StatsCard
        label="Pending Registrations"
        :value="registrationStore.pendingCount"
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

.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 24px 0;
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
