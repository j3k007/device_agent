<script setup>
import AppSidebar from './AppSidebar.vue'
</script>

<template>
  <div class="app-layout">
    <AppSidebar />
    <main class="main-content">
      <slot />
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  min-height: 100vh;
}

.main-content {
  flex: 1;
  padding: 24px 32px;
  background-color: var(--bg-primary);
  overflow-y: auto;
}
</style>
