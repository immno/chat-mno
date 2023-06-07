import './assets/main.css'

import { createApp } from 'vue'
import App from '../src/App.vue'
import router from './routers/router'

createApp(App).use(router).mount('#app')