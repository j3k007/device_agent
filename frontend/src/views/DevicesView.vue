<script setup>
import { onMounted } from 'vue'
import { useDeviceStore } from '@/stores/devices'
import DeviceCard from '@/components/devices/DeviceCard.vue'

const deviceStore = useDeviceStore()

onMounted(() => {
  deviceStore.fetchDevices()
})
</script>

<template>
  <div class="devices-page">
    <h2 class="page-title">Devices</h2>

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

.page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 24px 0;
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
