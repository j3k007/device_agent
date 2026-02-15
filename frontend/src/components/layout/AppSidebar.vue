<script setup>
import { useRegistrationStore } from '@/stores/registrations'
import { computed } from 'vue'

const registrationStore = useRegistrationStore()
const pendingCount = computed(() => registrationStore.pendingCount)
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <h1 class="logo">Device Agent</h1>
    </div>

    <nav class="sidebar-nav">
      <router-link to="/" class="nav-item" exact-active-class="active">
        <span class="nav-icon">&#9632;</span>
        Dashboard
      </router-link>

      <router-link to="/devices" class="nav-item" active-class="active">
        <span class="nav-icon">&#9998;</span>
        Devices
      </router-link>

      <router-link to="/registrations" class="nav-item" active-class="active">
        <span class="nav-icon">&#9998;</span>
        Registrations
        <span v-if="pendingCount > 0" class="badge">{{ pendingCount }}</span>
      </router-link>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 240px;
  background-color: var(--bg-sidebar);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.sidebar-header {
  padding: 20px 16px;
  border-bottom: 1px solid var(--border-color);
}

.logo {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.sidebar-nav {
  padding: 12px 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 6px;
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 0.9rem;
  transition: background-color 0.15s, color 0.15s;
}

.nav-item:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.nav-item.active {
  background-color: var(--bg-active);
  color: var(--accent-color);
  font-weight: 600;
}

.nav-icon {
  font-size: 1rem;
  width: 20px;
  text-align: center;
}

.badge {
  margin-left: auto;
  background-color: var(--color-warning);
  color: #fff;
  font-size: 0.75rem;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 10px;
}
</style>
