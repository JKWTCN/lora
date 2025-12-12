import { createApp } from "vue";
import App from "./App.vue";
import "./assets/global.css";
import "./assets/icons.css";
import i18n from "./i18n";

// 立即创建应用，无需等待 DOMContentLoaded
const app = createApp(App);

// 使用i18n插件
app.use(i18n);

// 优化的启动逻辑
if (document.readyState === 'loading') {
    // 如果DOM还在加载，等待加载完成
    document.addEventListener('DOMContentLoaded', () => {
        app.mount("#app");
    });
} else {
    // 如果DOM已经准备好，立即挂载
    app.mount("#app");
}
