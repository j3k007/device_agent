<script setup>
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useDeviceStore } from '@/stores/devices'
import deviceService from '@/services/deviceService'

const route = useRoute()
const deviceStore = useDeviceStore()
const services = ref([])
const software = ref([])

onMounted(async () => {
  const id = route.params.id
  await deviceStore.fetchDevice(id)

  try {
    const [servicesRes, softwareRes] = await Promise.all([
      deviceService.getDeviceServices(id),
      deviceService.getDeviceSoftware(id),
    ])
    services.value = servicesRes.data.services || servicesRes.data
    software.value = softwareRes.data.software || softwareRes.data
  } catch {
    // errors handled by store
  }
})

function formatBytes(bytes) {
  if (!bytes) return '0 B'
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`
}
</script>

<template>
  <div class="device-detail">
    <router-link to="/devices" class="back-link">&larr; Back to Devices</router-link>

    <div v-if="deviceStore.loading" class="loading">Loading device...</div>

    <div v-else-if="deviceStore.error" class="error-banner">
      {{ deviceStore.error }}
    </div>

    <template v-else-if="deviceStore.currentDevice">
      <div class="device-header">
        <h2 class="page-title">{{ deviceStore.currentDevice.hostname }}</h2>
        <span
          class="status-badge"
          :class="deviceStore.currentDevice.is_online ? 'online' : 'offline'"
        >
          {{ deviceStore.currentDevice.is_online ? 'Online' : 'Offline' }}
        </span>
      </div>

      <div class="info-grid">
        <div class="info-card">
          <h3>System Info</h3>
          <dl>
            <dt>OS</dt>
            <dd>{{ deviceStore.currentDevice.os_type }} {{ deviceStore.currentDevice.os_version }}</dd>
            <dt>CPU</dt>
            <dd>{{ deviceStore.currentDevice.cpu_info }}</dd>
            <dt>Memory</dt>
            <dd>
              {{ formatBytes(deviceStore.currentDevice.memory_available) }} available
              / {{ formatBytes(deviceStore.currentDevice.memory_total) }} total
            </dd>
          </dl>
        </div>

        <div class="info-card">
          <h3>Services ({{ services.length }})</h3>
          <ul v-if="services.length" class="item-list">
            <li v-for="svc in services" :key="svc.service_name">
              <span class="dot" :class="svc.is_active ? 'active' : 'inactive'"></span>
              {{ svc.service_name }}
            </li>
          </ul>
          <p v-else class="empty-text">No services tracked</p>
        </div>

        <div class="info-card">
          <h3>Software ({{ software.length }})</h3>
          <ul v-if="software.length" class="item-list">
            <li v-for="sw in software" :key="sw.software_name">
              <span class="dot" :class="sw.is_installed ? 'active' : 'inactive'"></span>
              {{ sw.software_name }}
            </li>
          </ul>
          <p v-else class="empty-text">No software tracked</p>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.device-detail {
  max-width: 1000px;
}

.back-link {
  color: var(--accent-color);
  text-decoration: none;
  font-size: 0.9rem;
  display: inline-block;
  margin-bottom: 16px;
}

.back-link:hover {
  text-decoration: underline;
}

.device-header {
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

.status-badge {
  font-size: 0.8rem;
  font-weight: 600;
  padding: 4px 12px;
  border-radius: 12px;
}

.status-badge.online {
  background-color: var(--color-success-bg);
  color: var(--color-success);
}

.status-badge.offline {
  background-color: var(--color-danger-bg);
  color: var(--color-danger);
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 16px;
}

.info-card {
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
}

.info-card h3 {
  margin: 0 0 12px 0;
  font-size: 1rem;
  color: var(--text-primary);
}

dl {
  margin: 0;
  display: grid;
  grid-template-columns: 100px 1fr;
  gap: 8px 12px;
}

dt {
  color: var(--text-secondary);
  font-size: 0.85rem;
}

dd {
  margin: 0;
  color: var(--text-primary);
  font-size: 0.85rem;
}

.item-list {
  list-style: none;
  padding: 0;
  margin: 0;
  max-height: 300px;
  overflow-y: auto;
}

.item-list li {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
  font-size: 0.85rem;
  color: var(--text-primary);
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.dot.active {
  background-color: var(--color-success);
}

.dot.inactive {
  background-color: var(--text-muted);
}

.empty-text {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin: 0;
}

.loading {
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
