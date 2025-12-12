import { createApp } from 'vue'
import NewProjectApp from './NewProjectApp.vue'
import './assets/global.css'
import './assets/icons.css'
import i18n from './i18n'

const app = createApp(NewProjectApp)
app.use(i18n)
app.mount('#app')