<script setup>
defineProps({
  label: { type: String, required: true },
  value: { type: Number, default: 0 },
  loading: { type: Boolean, default: false },
  variant: { type: String, default: 'default' },
})
</script>

<template>
  <div class="stats-card" :class="`variant-${variant}`">
    <span class="stats-label">{{ label }}</span>
    <span v-if="loading" class="stats-value">--</span>
    <span v-else class="stats-value">{{ value }}</span>
  </div>
</template>

<style scoped>
.stats-card {
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stats-label {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stats-value {
  font-size: 2rem;
  font-weight: 700;
  color: var(--text-primary);
}

.variant-success .stats-value {
  color: var(--color-success);
}

.variant-danger .stats-value {
  color: var(--color-danger);
}

.variant-warning .stats-value {
  color: var(--color-warning);
}
</style>
