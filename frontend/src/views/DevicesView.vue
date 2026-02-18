<script setup>
import { onMounted } from 'vue'
import { useDeviceStore } from '@/stores/devices'
import { useDashboardSocket } from '@/composables/useDashboardSocket'
import DeviceCard from '@/components/devices/DeviceCard.vue'

const deviceStore = useDeviceStore()
const { connected } = useDashboardSocket()

onMounted(() => {
  deviceStore.fetchDevices()
})
</script>

<template>
  <div class="devices-page">
    <div class="page-header">
      <h2 class="page-title">Devices</h2>
      <span class="ws-status" :class="connected ? 'ws-connected' : 'ws-disconnected'">
        {{ connected ? 'Live' : 'Connecting...' }}
      </span>
    </div>

    <div v-if="deviceStore.loading" class="loading">Loading devices...</div>

    <div v-else-if="deviceStore.error" class="error-banner">
      {{ deviceStore.error }}
    </div>

    <div v-else-if="deviceStore.devices.length === 0" class="empty-state">
      No devices registered yet.
    </div>

    <div v-else class="devices-grid">
      <DeviceCard
        v-for="device in deviceStore.devices"
        :key="device.id"
        :device="device"
      />
    </div>
  </div>
</template>

<style scoped>
.devices-page {
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

.devices-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 16px;
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
