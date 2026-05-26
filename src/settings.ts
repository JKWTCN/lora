import { createApp } from 'vue'
import SettingsApp from './SettingsApp.vue'
import './assets/global.css'
import './assets/icons.css'
import i18n from './i18n'
import { installCustomDialogs } from './utils/customDialog'

const app = createApp(SettingsApp)
app.use(i18n)
installCustomDialogs(app, i18n)
app.mount('#app')
