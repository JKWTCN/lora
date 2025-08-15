<template>
    <div v-if="visible" class="settings-overlay" @click="closeSettings">
        <div class="settings-modal" @click.stop>
            <!-- 设置标题栏 -->
            <div class="settings-header">
                <h2>设置</h2>
                <button class="close-button" @click="closeSettings">
                    <i class="icon-close"></i>
                </button>
            </div>

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
                                    <input type="number" v-model.number="localSettings.windowHeight" min="300"
                                        max="1080" @change="updateWindowSize" />
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
                                    <input type="range" v-model.number="localSettings.iconSize" min="48" max="128"
                                        step="8" @input="updateIconSize" />
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
                                <select v-model.number="localSettings.maxSearchResults"
                                    @change="updateMaxSearchResults">
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
                                    <input type="checkbox" v-model="localSettings.autoBackup"
                                        @change="updateAutoBackup" />
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
    </div>
</template>

<script setup>
import { ref, reactive, computed, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
    visible: {
        type: Boolean,
        default: false
    }
})

const emit = defineEmits(['close', 'update:settings'])

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
const closeSettings = () => {
    emit('close')
}

const updateWindowSize = async () => {
    try {
        await invoke('save_window_size', {
            width: localSettings.windowWidth,
            height: localSettings.windowHeight
        })
        emit('update:settings', { ...localSettings })
    } catch (error) {
        console.error('保存窗口大小失败:', error)
    }
}

const updatePreventAutoHide = async () => {
    try {
        await invoke('update_prevent_auto_hide', {
            preventAutoHide: localSettings.preventAutoHide
        })
        emit('update:settings', { ...localSettings })
    } catch (error) {
        console.error('更新阻止自动隐藏设置失败:', error)
    }
}

const updateTheme = () => {
    emit('update:settings', { ...localSettings })
}

const updateIconSize = () => {
    emit('update:settings', { ...localSettings })
}

const updateSidebarWidth = () => {
    emit('update:settings', { ...localSettings })
}

const updateAnimations = () => {
    emit('update:settings', { ...localSettings })
}

const updateAnimationSpeed = () => {
    emit('update:settings', { ...localSettings })
}

const updateStartWithSystem = () => {
    // TODO: 实现开机自启动设置
    emit('update:settings', { ...localSettings })
}

const updateStartMinimized = () => {
    emit('update:settings', { ...localSettings })
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
    }
}

const clearHotkey = () => {
    localSettings.toggleHotkey = ''
}

const updateGlobalHotkey = () => {
    // TODO: 实现全局快捷键设置
    emit('update:settings', { ...localSettings })
}

const updateFuzzySearch = () => {
    emit('update:settings', { ...localSettings })
}

const updateSearchInPath = () => {
    emit('update:settings', { ...localSettings })
}

const updateMaxSearchResults = () => {
    emit('update:settings', { ...localSettings })
}

const updateAutoBackup = () => {
    emit('update:settings', { ...localSettings })
}

const updateBackupInterval = () => {
    emit('update:settings', { ...localSettings })
}

const exportData = async () => {
    try {
        // TODO: 实现数据导出功能
        console.log('导出数据')
    } catch (error) {
        console.error('导出数据失败:', error)
    }
}

const importData = async () => {
    try {
        // TODO: 实现数据导入功能
        console.log('导入数据')
    } catch (error) {
        console.error('导入数据失败:', error)
    }
}

const resetData = async () => {
    if (confirm('确定要重置所有数据吗？此操作不可撤销！')) {
        try {
            // TODO: 实现数据重置功能
            console.log('重置数据')
        } catch (error) {
            console.error('重置数据失败:', error)
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

const resetToDefaults = () => {
    if (confirm('确定要恢复所有设置到默认值吗？')) {
        Object.assign(localSettings, {
            windowWidth: 800,
            windowHeight: 600,
            preventAutoHide: false,
            theme: 'auto',
            iconSize: 64,
            sidebarWidth: 180,
            enableAnimations: true,
            animationSpeed: 'normal',
            startWithSystem: false,
            startMinimized: false,
            toggleHotkey: 'Ctrl+Space',
            globalHotkey: true,
            fuzzySearch: true,
            searchInPath: false,
            maxSearchResults: 20,
            autoBackup: true,
            backupInterval: 'weekly'
        })
    }
}

const saveSettings = async () => {
    isSaving.value = true
    try {
        await invoke('save_app_settings', { settings: localSettings })
        lastSaved.value = true
        setTimeout(() => {
            lastSaved.value = false
        }, 2000)
    } catch (error) {
        console.error('保存设置失败:', error)
    } finally {
        isSaving.value = false
    }
}

// 加载设置
const loadSettings = async () => {
    try {
        const settings = await invoke('load_app_settings')
        Object.assign(localSettings, settings)
    } catch (error) {
        console.error('加载设置失败:', error)
    }
}

// 初始化
loadSettings()
</script>

<style scoped>
.settings-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.settings-modal {
    background: #f5f5f5;
    border-radius: 10px;
    width: 85vw;
    max-width: 800px;
    height: 75vh;
    max-height: 600px;
    display: flex;
    flex-direction: column;
    box-shadow: 0 15px 35px rgba(0, 0, 0, 0.3);
    overflow: hidden;
}

.settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid #bdc3c7;
    background: #2c3e50;
    color: white;
}

.settings-header h2 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
}

.close-button {
    background: none;
    border: none;
    padding: 6px;
    border-radius: 4px;
    cursor: pointer;
    color: #bdc3c7;
    transition: all 0.2s;
    font-size: 14px;
}

.close-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: white;
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
