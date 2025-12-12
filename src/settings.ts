import { createApp } from 'vue'
import SettingsApp from './SettingsApp.vue'
import './assets/global.css'
import './assets/icons.css'
import i18n from './i18n'

const app = createApp(SettingsApp)
app.use(i18n)
app.mount('#app')
