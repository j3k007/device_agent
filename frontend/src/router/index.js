import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'dashboard',
    component: () => import('@/views/DashboardView.vue'),
  },
  {
    path: '/devices',
    name: 'devices',
    component: () => import('@/views/DevicesView.vue'),
  },
  {
    path: '/devices/:id',
    name: 'device-detail',
    component: () => import('@/views/DeviceDetailView.vue'),
    props: true,
  },
  {
    path: '/registrations',
    name: 'registrations',
    component: () => import('@/views/RegistrationsView.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
