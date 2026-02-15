<script setup>
defineProps({
  device: { type: Object, required: true },
})

function formatBytes(bytes) {
  if (!bytes) return '0 B'
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`
}
</script>

<template>
  <router-link :to="`/devices/${device.id}`" class="device-card">
    <div class="card-header">
      <span class="hostname">{{ device.hostname }}</span>
      <span class="status-dot" :class="device.is_online ? 'online' : 'offline'"></span>
    </div>

    <div class="card-body">
      <div class="info-row">
        <span class="label">OS</span>
        <span class="value">{{ device.os_type }} {{ device.os_version }}</span>
      </div>
      <div class="info-row">
        <span class="label">CPU</span>
        <span class="value truncate">{{ device.cpu_info }}</span>
      </div>
      <div class="info-row">
        <span class="label">Memory</span>
        <span class="value">
          {{ formatBytes(device.memory_available) }} / {{ formatBytes(device.memory_total) }}
        </span>
      </div>
    </div>
  </router-link>
</template>

<style scoped>
.device-card {
  display: block;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 16px 20px;
  text-decoration: none;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.device-card:hover {
  border-color: var(--accent-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.hostname {
  font-weight: 600;
  font-size: 1rem;
  color: var(--text-primary);
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-dot.online {
  background-color: var(--color-success);
}

.status-dot.offline {
  background-color: var(--text-muted);
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.82rem;
}

.label {
  color: var(--text-secondary);
  flex-shrink: 0;
}

.value {
  color: var(--text-primary);
  text-align: right;
}

.truncate {
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
