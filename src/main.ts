import { createApp } from "vue";
import App from "./App.vue";
import "./assets/global.css";
import "./assets/icons.css";

// 等待 DOM 和 Tauri 初始化
document.addEventListener('DOMContentLoaded', () => {
    createApp(App).mount("#app");
});
