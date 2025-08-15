<template>
    <div class="settings-app">
        <!-- 设置内容 -->
        <div class="settings-content">
            <!-- 左侧导航 -->
            <div class="settings-nav">
                <div v-for="tab in tabs" :key="tab.id" class="nav-item" :class="{ active: activeTab === tab.id }"
                    @click="activeTab = tab.id">
                    <i :class="tab.icon"></i>
                    <span>{{ tab.name }}</span>
                </div>
            </div>

            <!-- 右侧设置面板 -->
            <div class="settings-panel">
                <!-- 关于页面 -->
                <div v-if="activeTab === 'about'" class="panel-content">
                    <div class="about-section">
                        <div class="app-info">
                            <div class="app-icon">
                                <img src="/tauri.svg" alt="Lora" />
                            </div>
                            <div class="app-details">
                                <h3>Lora Launcher</h3>
                                <p class="version">版本 1.0.0</p>
                                <p class="description">
                                    一个简洁高效的应用启动器，让您快速访问常用应用程序。
                                </p>
                            </div>
                        </div>

                        <div class="info-grid">
                            <div class="info-item">
                                <label>开发者：</label>
                                <span>Lora Team</span>
                            </div>
                            <div class="info-item">
                                <label>技术栈：</label>
                                <span>Tauri + Vue 3</span>
                            </div>
                            <div class="info-item">
                                <label>许可证：</label>
                                <span>MIT License</span>
                            </div>
                            <div class="info-item">
                                <label>更新日期：</label>
                                <span>2025-01-15</span>
                            </div>
                        </div>

                        <div class="links-section">
                            <button class="link-button" @click="openUrl('https://github.com/JKWTCN/lora')">
                                <i class="icon-github"></i>
                                GitHub 仓库
                            </button>
                            <button class="link-button" @click="openUrl('https://github.com/JKWTCN/lora/issues')">
                                <i class="icon-bug"></i>
                                报告问题
                            </button>
                            <button class="link-button" @click="openUrl('https://github.com/JKWTCN/lora/releases')">
                                <i class="icon-download"></i>
                                检查更新
                            </button>
                        </div>
                    </div>
                </div>

                <!-- 界面设置页面 -->
                <div v-if="activeTab === 'ui'" class="panel-content">
                    <div class="settings-group">
                        <h3>窗口设置</h3>

                        <div class="setting-item">
                            <label>窗口宽度</label>
                            <div class="input-group">
                                <input type="number" v-model.number="localSettings.windowWidth" min="400" max="1920"
                                    @change="updateWindowSize" />
                                <span class="unit">px</span>
                            </div>
                        </div>

                        <div class="setting-item">
                            <label>窗口高度</label>
                            <div class="input-group">
                                <input type="number" v-model.number="localSettings.windowHeight" min="300" max="1080"
                                    @change="updateWindowSize" />
                                <span class="unit">px</span>
                            </div>
                        </div>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.preventAutoHide"
                                    @change="updatePreventAutoHide" />
                                阻止窗口自动隐藏
                            </label>
                            <p class="setting-description">
                                启用此选项后，窗口失去焦点时不会自动隐藏
                            </p>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>外观设置</h3>

                        <div class="setting-item">
                            <label>主题</label>
                            <select v-model="localSettings.theme" @change="updateTheme">
                                <option value="auto">自动</option>
                                <option value="light">浅色</option>
                                <option value="dark">深色</option>
                            </select>
                        </div>

                        <div class="setting-item">
                            <label>应用图标大小</label>
                            <div class="slider-group">
                                <input type="range" v-model.number="localSettings.iconSize" min="48" max="128" step="8"
                                    @input="updateIconSize" />
                                <span class="slider-value">{{ localSettings.iconSize }}px</span>
                            </div>
                        </div>

                        <div class="setting-item">
                            <label>侧栏宽度</label>
                            <div class="slider-group">
                                <input type="range" v-model.number="localSettings.sidebarWidth" min="120" max="300"
                                    step="10" @input="updateSidebarWidth" />
                                <span class="slider-value">{{ localSettings.sidebarWidth }}px</span>
                            </div>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>动画设置</h3>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.enableAnimations"
                                    @change="updateAnimations" />
                                启用动画效果
                            </label>
                            <p class="setting-description">
                                禁用动画可以提高性能，特别是在低配置设备上
                            </p>
                        </div>

                        <div class="setting-item" v-if="localSettings.enableAnimations">
                            <label>动画速度</label>
                            <select v-model="localSettings.animationSpeed" @change="updateAnimationSpeed">
                                <option value="slow">慢速</option>
                                <option value="normal">正常</option>
                                <option value="fast">快速</option>
                            </select>
                        </div>
                    </div>
                </div>

                <!-- 功能设置页面 -->
                <div v-if="activeTab === 'features'" class="panel-content">
                    <div class="settings-group">
                        <h3>启动设置</h3>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.startWithSystem"
                                    @change="updateStartWithSystem" />
                                开机自动启动
                            </label>
                            <p class="setting-description">
                                将 Lora 添加到系统启动项，开机时自动运行
                            </p>
                        </div>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.startMinimized"
                                    @change="updateStartMinimized" />
                                启动时最小化到托盘
                            </label>
                            <p class="setting-description">
                                程序启动时直接最小化到系统托盘，不显示主窗口
                            </p>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>快捷键设置</h3>

                        <div class="setting-item">
                            <label>显示/隐藏窗口</label>
                            <div class="hotkey-input">
                                <input type="text" v-model="localSettings.toggleHotkey" @keydown="captureHotkey"
                                    placeholder="点击设置快捷键" readonly />
                                <button @click="clearHotkey" class="clear-button">清除</button>
                            </div>
                        </div>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.globalHotkey"
                                    @change="updateGlobalHotkey" />
                                启用全局快捷键
                            </label>
                            <p class="setting-description">
                                在任何窗口下都能使用快捷键唤起 Lora
                            </p>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>搜索设置</h3>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.fuzzySearch"
                                    @change="updateFuzzySearch" />
                                启用模糊搜索
                            </label>
                            <p class="setting-description">
                                允许不完全匹配的搜索结果，提高搜索的容错性
                            </p>
                        </div>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.searchInPath"
                                    @change="updateSearchInPath" />
                                搜索文件路径
                            </label>
                            <p class="setting-description">
                                在搜索时包含文件路径信息
                            </p>
                        </div>

                        <div class="setting-item">
                            <label>最大搜索结果数</label>
                            <select v-model.number="localSettings.maxSearchResults" @change="updateMaxSearchResults">
                                <option :value="10">10 个</option>
                                <option :value="20">20 个</option>
                                <option :value="50">50 个</option>
                                <option :value="100">100 个</option>
                            </select>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>数据管理</h3>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.autoBackup" @change="updateAutoBackup" />
                                自动备份数据
                            </label>
                            <p class="setting-description">
                                定期自动备份应用数据，防止数据丢失
                            </p>
                        </div>

                        <div class="setting-item" v-if="localSettings.autoBackup">
                            <label>备份频率</label>
                            <select v-model="localSettings.backupInterval" @change="updateBackupInterval">
                                <option value="daily">每天</option>
                                <option value="weekly">每周</option>
                                <option value="monthly">每月</option>
                            </select>
                        </div>

                        <div class="setting-item">
                            <div class="button-group">
                                <button @click="exportData" class="action-button">
                                    <i class="icon-export"></i>
                                    导出数据
                                </button>
                                <button @click="importData" class="action-button">
                                    <i class="icon-import"></i>
                                    导入数据
                                </button>
                                <button @click="resetData" class="action-button danger">
                                    <i class="icon-reset"></i>
                                    重置数据
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 设置底栏 -->
        <div class="settings-footer">
            <div class="footer-left">
                <span class="save-status" :class="{ saving: isSaving, saved: lastSaved }">
                    {{ saveStatusText }}
                </span>
            </div>
            <div class="footer-right">
                <button @click="resetToDefaults" class="footer-button secondary">
                    恢复默认
                </button>
                <button @click="saveSettings" class="footer-button primary">
                    保存设置
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, reactive, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 标签页配置
const tabs = [
    { id: 'about', name: '关于', icon: 'icon-info' },
    { id: 'ui', name: '界面设置', icon: 'icon-display' },
    { id: 'features', name: '功能设置', icon: 'icon-settings' }
]

const activeTab = ref('about')
const isSaving = ref(false)
const lastSaved = ref(false)

// 本地设置状态
const localSettings = reactive({
    // 窗口设置
    windowWidth: 800,
    windowHeight: 600,
    preventAutoHide: false,

    // 外观设置
    theme: 'auto',
    iconSize: 64,
    sidebarWidth: 180,
    enableAnimations: true,
    animationSpeed: 'normal',

    // 启动设置
    startWithSystem: false,
    startMinimized: false,

    // 快捷键设置
    toggleHotkey: 'Ctrl+Space',
    globalHotkey: true,

    // 搜索设置
    fuzzySearch: true,
    searchInPath: false,
    maxSearchResults: 20,

    // 数据管理
    autoBackup: true,
    backupInterval: 'weekly'
})

// 保存状态文本
const saveStatusText = computed(() => {
    if (isSaving.value) return '正在保存...'
    if (lastSaved.value) return '已保存'
    return ''
})

// 监听设置变化
watch(localSettings, () => {
    lastSaved.value = false
}, { deep: true })

// 方法
const updateWindowSize = async () => {
    try {
        await invoke('save_window_size', {
            width: localSettings.windowWidth,
            height: localSettings.windowHeight
        })
    } catch (error) {
        console.error('保存窗口大小失败:', error)
    }
}

const updatePreventAutoHide = async () => {
    try {
        await invoke('update_prevent_auto_hide', {
            preventAutoHide: localSettings.preventAutoHide
        })
    } catch (error) {
        console.error('更新阻止自动隐藏设置失败:', error)
    }
}

const updateTheme = async () => {
    try {
        await invoke('update_theme', { theme: localSettings.theme })
        console.log('主题设置已更新')
    } catch (error) {
        console.error('更新主题设置失败:', error)
    }
}

const updateIconSize = async () => {
    try {
        await invoke('update_icon_size', { iconSize: localSettings.iconSize })
        console.log('图标大小已更新')
    } catch (error) {
        console.error('更新图标大小失败:', error)
    }
}

const updateSidebarWidth = async () => {
    try {
        await invoke('update_sidebar_width', { sidebarWidth: localSettings.sidebarWidth })
        console.log('侧栏宽度已更新')
    } catch (error) {
        console.error('更新侧栏宽度失败:', error)
    }
}

const updateAnimations = async () => {
    try {
        await invoke('update_animations', { enableAnimations: localSettings.enableAnimations })
        console.log('动画设置已更新')
    } catch (error) {
        console.error('更新动画设置失败:', error)
    }
}

const updateAnimationSpeed = async () => {
    try {
        await invoke('update_animation_speed', { animationSpeed: localSettings.animationSpeed })
        console.log('动画速度已更新')
    } catch (error) {
        console.error('更新动画速度失败:', error)
    }
}

const updateStartWithSystem = async () => {
    try {
        await invoke('update_start_with_system', { startWithSystem: localSettings.startWithSystem })
        console.log('开机自启动设置已更新')
    } catch (error) {
        console.error('更新开机自启动设置失败:', error)
    }
}

const updateStartMinimized = async () => {
    try {
        await invoke('update_start_minimized', { startMinimized: localSettings.startMinimized })
        console.log('启动最小化设置已更新')
    } catch (error) {
        console.error('更新启动最小化设置失败:', error)
    }
}

const captureHotkey = (event) => {
    event.preventDefault()
    const keys = []

    if (event.ctrlKey) keys.push('Ctrl')
    if (event.altKey) keys.push('Alt')
    if (event.shiftKey) keys.push('Shift')
    if (event.metaKey) keys.push('Meta')

    if (event.key && !['Control', 'Alt', 'Shift', 'Meta'].includes(event.key)) {
        keys.push(event.key.toUpperCase())
    }

    if (keys.length > 0) {
        localSettings.toggleHotkey = keys.join('+')
        // 立即保存快捷键设置
        updateToggleHotkey()
    }
}

const clearHotkey = () => {
    localSettings.toggleHotkey = ''
    updateToggleHotkey()
}

const updateToggleHotkey = async () => {
    try {
        await invoke('update_toggle_hotkey', { toggleHotkey: localSettings.toggleHotkey })
        console.log('快捷键设置已更新')
    } catch (error) {
        console.error('更新快捷键设置失败:', error)
    }
}

const updateGlobalHotkey = async () => {
    try {
        await invoke('update_global_hotkey', { globalHotkey: localSettings.globalHotkey })
        console.log('全局快捷键设置已更新')
    } catch (error) {
        console.error('更新全局快捷键设置失败:', error)
    }
}

const updateFuzzySearch = async () => {
    try {
        await invoke('update_fuzzy_search', { fuzzySearch: localSettings.fuzzySearch })
        console.log('模糊搜索设置已更新')
    } catch (error) {
        console.error('更新模糊搜索设置失败:', error)
    }
}

const updateSearchInPath = async () => {
    try {
        await invoke('update_search_in_path', { searchInPath: localSettings.searchInPath })
        console.log('路径搜索设置已更新')
    } catch (error) {
        console.error('更新路径搜索设置失败:', error)
    }
}

const updateMaxSearchResults = async () => {
    try {
        await invoke('update_max_search_results', { maxSearchResults: localSettings.maxSearchResults })
        console.log('最大搜索结果设置已更新')
    } catch (error) {
        console.error('更新最大搜索结果设置失败:', error)
    }
}

const updateAutoBackup = async () => {
    try {
        await invoke('update_auto_backup', { autoBackup: localSettings.autoBackup })
        console.log('自动备份设置已更新')
    } catch (error) {
        console.error('更新自动备份设置失败:', error)
    }
}

const updateBackupInterval = async () => {
    try {
        await invoke('update_backup_interval', { backupInterval: localSettings.backupInterval })
        console.log('备份间隔设置已更新')
    } catch (error) {
        console.error('更新备份间隔设置失败:', error)
    }
}

const exportData = async () => {
    try {
        isSaving.value = true
        const result = await invoke('export_data')
        console.log('数据导出成功:', result)
        alert('数据导出成功！')
    } catch (error) {
        console.error('导出数据失败:', error)
        alert('导出数据失败: ' + error)
    } finally {
        isSaving.value = false
    }
}

const importData = async () => {
    try {
        isSaving.value = true
        const result = await invoke('import_data')
        console.log('数据导入成功:', result)
        alert('数据导入成功！请重启应用以应用更改。')
        // 重新加载设置
        await loadSettings()
    } catch (error) {
        console.error('导入数据失败:', error)
        alert('导入数据失败: ' + error)
    } finally {
        isSaving.value = false
    }
}

const resetData = async () => {
    if (confirm('确定要重置所有数据吗？此操作不可撤销！')) {
        try {
            isSaving.value = true
            const result = await invoke('reset_data')
            console.log('数据重置成功:', result)
            alert('数据重置成功！请重启应用。')
            // 重新加载设置
            await loadSettings()
        } catch (error) {
            console.error('重置数据失败:', error)
            alert('重置数据失败: ' + error)
        } finally {
            isSaving.value = false
        }
    }
}

const openUrl = async (url) => {
    try {
        await invoke('open_url', { url })
    } catch (error) {
        console.error('打开链接失败:', error)
    }
}

const resetToDefaults = async () => {
    if (confirm('确定要恢复所有设置到默认值吗？')) {
        try {
            isSaving.value = true
            await invoke('reset_settings_to_default')
            // 重新加载设置
            await loadSettings()
            alert('设置已恢复到默认值')
        } catch (error) {
            console.error('恢复默认设置失败:', error)
            alert('恢复默认设置失败: ' + error)
        } finally {
            isSaving.value = false
        }
    }
}

const saveSettings = async () => {
    await saveAllSettings()
    
    // 延迟一秒后关闭设置窗口
    setTimeout(async () => {
        try {
            await invoke('close_settings_window')
        } catch (error) {
            console.error('关闭设置窗口失败:', error)
        }
    }, 1000)
}

// 加载设置
const loadSettings = async () => {
    try {
        const settings = await invoke('load_app_settings')
        
        // 将后端设置映射到前端本地设置
        localSettings.windowWidth = settings.window_width || 800
        localSettings.windowHeight = settings.window_height || 600
        localSettings.preventAutoHide = settings.prevent_auto_hide || false
        localSettings.theme = settings.theme || 'auto'
        localSettings.iconSize = settings.icon_size || 64
        localSettings.sidebarWidth = settings.sidebar_width || 180
        localSettings.enableAnimations = settings.enable_animations !== false
        localSettings.animationSpeed = settings.animation_speed || 'normal'
        localSettings.startMinimized = settings.start_minimized || false
        localSettings.toggleHotkey = settings.toggle_hotkey || 'Ctrl+Space'
        localSettings.globalHotkey = settings.global_hotkey !== false
        localSettings.fuzzySearch = settings.fuzzy_search !== false
        localSettings.searchInPath = settings.search_in_path || false
        localSettings.maxSearchResults = settings.max_search_results || 20
        localSettings.autoBackup = settings.auto_backup !== false
        localSettings.backupInterval = settings.backup_interval || 'weekly'

        // 检查实际的开机自启动状态
        try {
            const autoStartStatus = await invoke('check_auto_start_status')
            localSettings.startWithSystem = autoStartStatus
        } catch (error) {
            console.warn('检查开机自启动状态失败:', error)
            localSettings.startWithSystem = settings.start_with_system || false
        }
        
        console.log('设置加载成功')
    } catch (error) {
        console.error('加载设置失败:', error)
    }
}

// 批量保存所有设置
const saveAllSettings = async () => {
    isSaving.value = true
    try {
        // 构建设置对象（驼峰命名转换为下划线）
        const settingsToSave = {
            preventAutoHide: localSettings.preventAutoHide,
            windowWidth: localSettings.windowWidth,
            windowHeight: localSettings.windowHeight,
            theme: localSettings.theme,
            iconSize: localSettings.iconSize,
            sidebarWidth: localSettings.sidebarWidth,
            enableAnimations: localSettings.enableAnimations,
            animationSpeed: localSettings.animationSpeed,
            startWithSystem: localSettings.startWithSystem,
            startMinimized: localSettings.startMinimized,
            toggleHotkey: localSettings.toggleHotkey,
            globalHotkey: localSettings.globalHotkey,
            fuzzySearch: localSettings.fuzzySearch,
            searchInPath: localSettings.searchInPath,
            maxSearchResults: localSettings.maxSearchResults,
            autoBackup: localSettings.autoBackup,
            backupInterval: localSettings.backupInterval
        }

        await invoke('update_settings_batch', { settingsUpdate: settingsToSave })
        
        lastSaved.value = true
        setTimeout(() => {
            lastSaved.value = false
        }, 2000)
        
        console.log('所有设置保存成功')
    } catch (error) {
        console.error('保存设置失败:', error)
    } finally {
        isSaving.value = false
    }
}

// 初始化
onMounted(() => {
    loadSettings()
})
</script>

<style scoped>
.settings-app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #f5f5f5;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.settings-content {
    flex: 1;
    display: flex;
    min-height: 0;
}

.settings-nav {
    width: 160px;
    background: #34495e;
    border-right: 1px solid #2c3e50;
    padding: 8px 0;
}

.nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 16px;
    cursor: pointer;
    transition: all 0.2s;
    color: #bdc3c7;
    font-size: 14px;
}

.nav-item:hover {
    background: rgba(255, 255, 255, 0.1);
    color: white;
}

.nav-item.active {
    background: #3498db;
    color: white;
    border-right: 3px solid #2980b9;
}

.nav-item i {
    font-size: 14px;
    width: 14px;
}

.settings-panel {
    flex: 1;
    overflow-y: auto;
}

.panel-content {
    padding: 18px;
    max-width: 500px;
}

/* 关于页面样式 */
.about-section {
    text-align: center;
}

.app-info {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 24px;
    text-align: left;
}

.app-icon img {
    width: 48px;
    height: 48px;
}

.app-details h3 {
    margin: 0 0 6px 0;
    font-size: 20px;
    font-weight: 600;
    color: #2c3e50;
}

.version {
    color: #7f8c8d;
    margin: 0 0 8px 0;
    font-size: 13px;
}

.description {
    color: #7f8c8d;
    margin: 0;
    line-height: 1.4;
    font-size: 13px;
}

.info-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
    margin-bottom: 20px;
    text-align: left;
}

.info-item {
    display: flex;
    justify-content: space-between;
    padding: 8px 0;
    border-bottom: 1px solid #bdc3c7;
    font-size: 13px;
}

.info-item label {
    font-weight: 500;
    color: #7f8c8d;
}

.links-section {
    display: flex;
    gap: 8px;
    justify-content: center;
    flex-wrap: wrap;
}

.link-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: #ecf0f1;
    border: 1px solid #bdc3c7;
    border-radius: 6px;
    color: #2c3e50;
    text-decoration: none;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 12px;
}

.link-button:hover {
    background: #d5dbdb;
    border-color: #3498db;
}

/* 设置组样式 */
.settings-group {
    margin-bottom: 24px;
}

.settings-group h3 {
    margin: 0 0 12px 0;
    font-size: 15px;
    font-weight: 600;
    color: #2c3e50;
    border-bottom: 1px solid #bdc3c7;
    padding-bottom: 6px;
}

.setting-item {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 16px;
}

.setting-item label {
    font-weight: 500;
    color: #2c3e50;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
}

.setting-item input[type="checkbox"] {
    margin: 0;
}

.setting-item input[type="number"],
.setting-item input[type="text"],
.setting-item select {
    padding: 6px 10px;
    border: 1px solid #bdc3c7;
    border-radius: 4px;
    background: white;
    color: #2c3e50;
    max-width: 180px;
    font-size: 13px;
}

.setting-item input:focus,
.setting-item select:focus {
    outline: none;
    border-color: #3498db;
}

.setting-description {
    font-size: 11px;
    color: #7f8c8d;
    margin: 0;
    line-height: 1.3;
}

.input-group {
    display: flex;
    align-items: center;
    gap: 6px;
}

.unit {
    color: #7f8c8d;
    font-size: 12px;
}

.slider-group {
    display: flex;
    align-items: center;
    gap: 10px;
    max-width: 250px;
}

.slider-group input[type="range"] {
    flex: 1;
}

.slider-value {
    min-width: 45px;
    text-align: right;
    color: #7f8c8d;
    font-size: 12px;
}

.hotkey-input {
    display: flex;
    gap: 6px;
    max-width: 250px;
}

.hotkey-input input {
    flex: 1;
    font-size: 13px;
}

.clear-button {
    padding: 6px 10px;
    background: #ecf0f1;
    border: 1px solid #bdc3c7;
    border-radius: 4px;
    color: #7f8c8d;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 12px;
}

.clear-button:hover {
    background: #d5dbdb;
    color: #2c3e50;
}

.button-group {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
}

.action-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    background: #ecf0f1;
    border: 1px solid #bdc3c7;
    border-radius: 6px;
    color: #2c3e50;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 12px;
}

.action-button:hover {
    background: #d5dbdb;
    border-color: #3498db;
}

.action-button.danger {
    border-color: #e74c3c;
    color: #e74c3c;
}

.action-button.danger:hover {
    background: rgba(231, 76, 60, 0.1);
}

.settings-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    border-top: 1px solid #bdc3c7;
    background: #ecf0f1;
}

.save-status {
    font-size: 12px;
    color: #7f8c8d;
    transition: all 0.2s;
}

.save-status.saving {
    color: #3498db;
}

.save-status.saved {
    color: #27ae60;
}

.footer-right {
    display: flex;
    gap: 8px;
}

.footer-button {
    padding: 8px 16px;
    border: 1px solid #bdc3c7;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
    font-weight: 500;
    font-size: 13px;
}

.footer-button.secondary {
    background: white;
    color: #7f8c8d;
}

.footer-button.secondary:hover {
    background: #f8f9fa;
    color: #2c3e50;
}

.footer-button.primary {
    background: #3498db;
    color: white;
    border-color: #3498db;
}

.footer-button.primary:hover {
    background: #2980b9;
}
</style>
