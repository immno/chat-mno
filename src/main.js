import './assets/main.css'

import { createApp } from 'vue'

const app = createApp({
    data() {
        return {
            count: 0
        }
    }
})

app.mount('#app')

app.config.errorHandler = (err) => {
    /* 处理错误 */
}