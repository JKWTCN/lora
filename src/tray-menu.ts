import { createApp } from 'vue'
import TrayMenuApp from './TrayMenuApp.vue'
import './assets/context-menu.css'
import i18n from './i18n'

const app = createApp(TrayMenuApp)

app.use(i18n)
app.mount('#app')
