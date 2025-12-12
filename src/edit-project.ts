import { createApp } from 'vue'
import EditProjectApp from './EditProjectApp.vue'
import './assets/global.css'
import './assets/icons.css'
import i18n from './i18n'

const app = createApp(EditProjectApp)
app.use(i18n)
app.mount('#app')