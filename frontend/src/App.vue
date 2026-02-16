<script setup>
import { useRoute } from 'vue-router'
import AppLayout from '@/components/layout/AppLayout.vue'

const route = useRoute()
</script>

<template>
  <AppLayout v-if="route.name !== 'login'">
    <router-view />
  </AppLayout>
  <router-view v-else />
</template>
