import { createRouter, createWebHistory } from 'vue-router'
import Chat from '@/layouts/Chat.vue'
import Prompt from '@/layouts/Prompt.vue'
import Model from '@/layouts/Model.vue'
import Plugins from '@/layouts/Plugins.vue'
import Settings from '@/layouts/Settings.vue'

const routes = [
  { path: '/chat', component: Chat },
  { path: '/prompt', component: Prompt },
  { path: '/model', component: Model },
  { path: '/plugins', component: Plugins },
  { path: '/settings', component: Settings }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
