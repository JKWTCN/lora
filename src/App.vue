<script setup lang="ts">
import { ref, computed } from "vue";

const searchQuery = ref("");
const sidebarVisible = ref(false);
const selectedCategory = ref("All");

// 定义应用类型
interface App {
  id: number;
  name: string;
  icon: string;
  category: string;
}

// 模拟应用数据
const apps = ref<App[]>([
  { id: 1, name: "Safari", icon: "🌐", category: "Browser" },
  { id: 2, name: "VS Code", icon: "💻", category: "Development" },
  { id: 3, name: "Photoshop", icon: "🎨", category: "Design" },
  { id: 4, name: "Music", icon: "🎵", category: "Entertainment" },
  { id: 5, name: "Calendar", icon: "📅", category: "Productivity" },
  { id: 6, name: "Messages", icon: "💬", category: "Communication" },
  { id: 7, name: "Photos", icon: "📸", category: "Media" },
  { id: 8, name: "Notes", icon: "📝", category: "Productivity" },
  { id: 9, name: "Calculator", icon: "🔢", category: "Utilities" },
  { id: 10, name: "Settings", icon: "⚙️", category: "System" },
  { id: 11, name: "Mail", icon: "📧", category: "Communication" },
  { id: 12, name: "Maps", icon: "🗺️", category: "Navigation" },
  { id: 13, name: "Weather", icon: "🌤️", category: "Weather" },
  { id: 14, name: "Contacts", icon: "👥", category: "Productivity" },
  { id: 15, name: "Terminal", icon: "⚡", category: "Development" },
  { id: 16, name: "Finder", icon: "📁", category: "System" },
]);

// 获取所有分类
const categories = computed(() => {
  const cats = Array.from(new Set(apps.value.map(app => app.category))).sort();
  return ["All", ...cats];
});

// 过滤应用
const filteredApps = computed(() => {
  let result = apps.value;

  // 按分类过滤
  if (selectedCategory.value !== "All") {
    result = result.filter(app => app.category === selectedCategory.value);
  }

  // 按搜索词过滤
  if (searchQuery.value) {
    result = result.filter((app: App) =>
      app.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      app.category.toLowerCase().includes(searchQuery.value.toLowerCase())
    );
  }

  return result;
});

// 切换侧栏显示
const toggleSidebar = () => {
  sidebarVisible.value = !sidebarVisible.value;
};

// 选择分类
const selectCategory = (category: string) => {
  selectedCategory.value = category;
  sidebarVisible.value = false; // 选择后关闭侧栏
};

// 启动应用
const launchApp = (app: App) => {
  console.log(`Launching ${app.name}`);
  // 这里可以添加实际的应用启动逻辑
  // 比如调用 Tauri 的 API 来打开应用
};
</script>

<template>
  <main class="app-launcher">
    <!-- 汉堡菜单按钮 -->
    <button class="menu-button" @click="toggleSidebar" :class="{ active: sidebarVisible }">
      <span class="menu-line"></span>
      <span class="menu-line"></span>
      <span class="menu-line"></span>
    </button>

    <!-- 侧栏遮罩 -->
    <div v-if="sidebarVisible" class="sidebar-overlay" @click="toggleSidebar"></div>

    <!-- 侧栏 -->
    <aside class="sidebar" :class="{ visible: sidebarVisible }">
      <div class="sidebar-header">
        <h2 class="sidebar-title">分类</h2>
        <button class="close-btn" @click="toggleSidebar">✕</button>
      </div>

      <div class="sidebar-content">
        <div class="category-list">
          <button v-for="category in categories" :key="category" class="category-item"
            :class="{ active: selectedCategory === category }" @click="selectCategory(category)">
            <span class="category-icon">
              {{ category === 'All' ? '📱' :
                category === 'Browser' ? '🌐' :
                  category === 'Development' ? '💻' :
                    category === 'Design' ? '🎨' :
                      category === 'Entertainment' ? '🎵' :
                        category === 'Productivity' ? '📅' :
                          category === 'Communication' ? '💬' :
                            category === 'Media' ? '📸' :
                              category === 'Utilities' ? '🔧' :
                                category === 'System' ? '⚙️' :
                                  category === 'Navigation' ? '🗺️' :
                                    category === 'Weather' ? '🌤️' : '📁' }}
            </span>
            <span class="category-name">{{ category === 'All' ? '全部应用' : category }}</span>
            <span class="category-count">{{category === 'All' ? apps.length : apps.filter((app: App) => app.category
              === category).length}}</span>
          </button>
        </div>

        <div class="sidebar-footer">
          <div class="stats">
            <p class="stats-text">总计 {{ apps.length }} 个应用</p>
            <p class="stats-text">{{ filteredApps.length }} 个显示</p>
          </div>
        </div>
      </div>
    </aside>

    <!-- 主内容区域 -->
    <div class="main-content" :class="{ 'sidebar-open': sidebarVisible }">
      <!-- 搜索栏 -->
      <div class="search-container">
        <div class="search-box">
          <span class="search-icon">🔍</span>
          <input v-model="searchQuery" placeholder="搜索应用..." class="search-input" autocomplete="off" />
        </div>

        <!-- 当前分类显示 -->
        <div v-if="selectedCategory !== 'All'" class="current-category">
          <span class="current-category-text">{{ selectedCategory }}</span>
          <button class="clear-category" @click="selectCategory('All')">✕</button>
        </div>
      </div>

      <!-- 应用网格 -->
      <div class="apps-grid">
        <div v-for="app in filteredApps" :key="app.id" class="app-item" @click="launchApp(app)">
          <div class="app-icon">
            {{ app.icon }}
          </div>
          <div class="app-name">{{ app.name }}</div>
        </div>
      </div>

      <!-- 当没有搜索结果时显示 -->
      <div v-if="filteredApps.length === 0" class="no-results">
        <div class="no-results-icon">🔍</div>
        <div class="no-results-text">没有找到匹配的应用</div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.app-launcher {
  min-height: 100vh;
  display: flex;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  position: relative;
  overflow: hidden;
}

.app-launcher::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background:
    radial-gradient(circle at 20% 80%, rgba(120, 119, 198, 0.3) 0%, transparent 50%),
    radial-gradient(circle at 80% 20%, rgba(255, 119, 198, 0.3) 0%, transparent 50%),
    radial-gradient(circle at 40% 40%, rgba(120, 219, 255, 0.2) 0%, transparent 50%);
  pointer-events: none;
}

/* 汉堡菜单按钮 */
.menu-button {
  position: fixed;
  top: 2rem;
  left: 2rem;
  z-index: 1001;
  width: 50px;
  height: 50px;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  gap: 4px;
  opacity: 1;
  visibility: visible;
}

.menu-button:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: scale(1.05);
}

/* 当侧栏显示时隐藏菜单按钮 */
.menu-button.active {
  opacity: 0;
  visibility: hidden;
  transform: translateX(-10px);
}

.menu-line {
  width: 20px;
  height: 2px;
  background: white;
  border-radius: 1px;
  transition: all 0.3s ease;
  transform-origin: center;
}

/* 侧栏遮罩 */
.sidebar-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 999;
  backdrop-filter: blur(2px);
}

/* 侧栏 */
.sidebar {
  position: fixed;
  top: 0;
  left: -350px;
  width: 350px;
  height: 100vh;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(30px);
  border-right: 1px solid rgba(255, 255, 255, 0.2);
  z-index: 1000;
  transition: left 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  flex-direction: column;
}

.sidebar.visible {
  left: 0;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 2rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.sidebar-title {
  color: white;
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
}

.close-btn {
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  width: 40px;
  height: 40px;
  color: white;
  font-size: 1.2rem;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: scale(1.1);
}

.sidebar-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 1rem 0;
}

.category-list {
  flex: 1;
  padding: 0 1rem;
}

.category-item {
  width: 100%;
  display: flex;
  align-items: center;
  padding: 1rem 1.5rem;
  margin-bottom: 0.5rem;
  background: transparent;
  border: none;
  border-radius: 15px;
  color: white;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: left;
  position: relative;
}

.category-item:hover {
  transform: translateX(5px);
}

.category-item.active {
  transform: translateX(5px);
}

.category-item::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 6px;
  height: 6px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 50%;
  opacity: 0;
  transition: all 0.3s ease;
}

.category-item:hover::before {
  opacity: 0.6;
  width: 8px;
  height: 8px;
}

.category-item.active::before {
  opacity: 1;
  width: 10px;
  height: 10px;
  background: white;
}

.category-icon {
  font-size: 1.2rem;
  margin-right: 1rem;
  min-width: 24px;
}

.category-name {
  flex: 1;
  font-weight: 500;
}

.category-count {
  background: rgba(255, 255, 255, 0.2);
  padding: 0.3rem 0.8rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 600;
}

.sidebar-footer {
  padding: 1rem 2rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.stats-text {
  color: rgba(255, 255, 255, 0.7);
  font-size: 0.9rem;
  margin: 0.3rem 0;
}

/* 主内容区域 */
.main-content {
  flex: 1;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  transition: margin-left 0.3s ease;
  position: relative;
  z-index: 1;
}

/* 搜索容器 */
.search-container {
  position: relative;
  z-index: 10;
  margin-bottom: 3rem;
  width: 100%;
  max-width: 500px;
  margin-top: 4rem;
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 25px;
  padding: 0 20px;
  transition: all 0.3s ease;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.search-box:focus-within {
  background: rgba(255, 255, 255, 0.15);
  border-color: rgba(255, 255, 255, 0.3);
  transform: translateY(-2px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
}

.search-icon {
  font-size: 1.2rem;
  margin-right: 12px;
  opacity: 0.7;
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  padding: 16px 0;
  font-size: 1.1rem;
  color: white;
  outline: none;
  font-weight: 300;
}

.search-input::placeholder {
  color: rgba(255, 255, 255, 0.6);
}

/* 当前分类显示 */
.current-category {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 1rem;
  padding: 0.5rem 1rem;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 20px;
  width: fit-content;
  margin-left: auto;
  margin-right: auto;
}

.current-category-text {
  color: white;
  font-size: 0.9rem;
  font-weight: 500;
  margin-right: 0.5rem;
}

.clear-category {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  border-radius: 50%;
  width: 20px;
  height: 20px;
  color: white;
  font-size: 0.8rem;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.clear-category:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: scale(1.1);
}

/* 应用网格 */
.apps-grid {
  position: relative;
  z-index: 10;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
  gap: 2rem;
  max-width: 800px;
  width: 100%;
  padding: 0 1rem;
}

.app-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  padding: 1rem;
  border-radius: 20px;
  position: relative;
}

.app-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  opacity: 0;
  transition: all 0.3s ease;
}

.app-item:hover::before {
  opacity: 1;
}

.app-item:hover {
  transform: translateY(-8px) scale(1.05);
}

.app-item:active {
  transform: translateY(-4px) scale(1.02);
}

.app-icon {
  width: 80px;
  height: 80px;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2.5rem;
  margin-bottom: 0.8rem;
  transition: all 0.3s ease;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  position: relative;
  z-index: 1;
}

.app-item:hover .app-icon {
  background: rgba(255, 255, 255, 0.15);
  border-color: rgba(255, 255, 255, 0.3);
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.15);
}

.app-name {
  color: white;
  font-size: 0.9rem;
  font-weight: 500;
  text-align: center;
  opacity: 0.9;
  transition: all 0.3s ease;
  position: relative;
  z-index: 1;
}

.app-item:hover .app-name {
  opacity: 1;
  transform: translateY(-2px);
}

/* 无搜索结果 */
.no-results {
  position: relative;
  z-index: 10;
  text-align: center;
  color: rgba(255, 255, 255, 0.7);
  margin-top: 4rem;
}

.no-results-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.no-results-text {
  font-size: 1.2rem;
  font-weight: 300;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .sidebar {
    width: 300px;
    left: -300px;
  }
}

@media (max-width: 768px) {
  .main-content {
    padding: 1rem;
  }

  .apps-grid {
    grid-template-columns: repeat(auto-fit, minmax(80px, 1fr));
    gap: 1.5rem;
    max-width: 400px;
  }

  .app-icon {
    width: 60px;
    height: 60px;
    font-size: 2rem;
  }

  .search-container {
    margin-bottom: 2rem;
    margin-top: 2rem;
  }

  .menu-button {
    top: 1rem;
    left: 1rem;
    width: 45px;
    height: 45px;
  }

  .sidebar {
    width: 280px;
    left: -280px;
  }
}

@media (max-width: 480px) {
  .apps-grid {
    grid-template-columns: repeat(4, 1fr);
    gap: 1rem;
  }

  .app-icon {
    width: 50px;
    height: 50px;
    font-size: 1.5rem;
  }

  .app-name {
    font-size: 0.8rem;
  }

  .sidebar {
    width: 260px;
    left: -260px;
  }

  .sidebar-header {
    padding: 1.5rem;
  }

  .category-item {
    padding: 0.8rem 1rem;
    font-size: 0.9rem;
  }
}

/* 侧栏打开时的动画 */
@keyframes slideIn {
  from {
    transform: translateX(-100%);
  }

  to {
    transform: translateX(0);
  }
}

.sidebar.visible {
  animation: slideIn 0.3s ease-out;
}
</style>
<style>
:root {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  font-size: 16px;
  line-height: 1.5;
  font-weight: 400;
  color: #ffffff;
  background: #1a1a1a;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  overflow-x: hidden;
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.3);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.5);
}

/* 选中文本样式 */
::selection {
  background: rgba(255, 255, 255, 0.2);
  color: white;
}

/* 焦点样式 */
*:focus {
  outline: none;
}

/* 动画关键帧 */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes pulse {

  0%,
  100% {
    transform: scale(1);
  }

  50% {
    transform: scale(1.05);
  }
}

/* 暗色主题适配 */
@media (prefers-color-scheme: dark) {
  :root {
    color: #ffffff;
    background: #0a0a0a;
  }
}

/* 高对比度模式适配 */
@media (prefers-contrast: high) {
  .app-item::before {
    border-width: 2px;
  }

  .search-box {
    border-width: 2px;
  }
}

/* 减少动画模式适配 */
@media (prefers-reduced-motion: reduce) {

  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
</style>