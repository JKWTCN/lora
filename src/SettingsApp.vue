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
                                <h3>{{ $t('settings.about.appName') }}</h3>
                                <p class="version">{{ $t('common.version') }} {{ appVersion }}</p>
                                <p class="description">
                                    {{ $t('settings.about.description') }}
                                </p>
                            </div>
                        </div>

                        <div class="info-grid">
                            <div class="info-item">
                                <label>{{ $t('settings.about.developer') }}：</label>
                                <span>Lora Team</span>
                            </div>
                            <div class="info-item">
                                <label>{{ $t('settings.about.techStack') }}：</label>
                                <span>Tauri + Vue 3</span>
                            </div>
                            <div class="info-item">
                                <label>{{ $t('settings.about.license') }}：</label>
                                <span>MIT License</span>
                            </div>
                            <div class="info-item">
                                <label>{{ $t('settings.about.updateDate') }}：</label>
                                <span>{{ appUpdateDate }}</span>
                            </div>
                        </div>

                        <div class="links-section">
                            <button class="link-button" @click="openUrl('https://github.com/JKWTCN/lora')">
                                <i class="icon-github"></i>
                                {{ $t('settings.about.links.github') }}
                            </button>
                            <button class="link-button" @click="openUrl('https://github.com/JKWTCN/lora/issues')">
                                <i class="icon-bug"></i>
                                {{ $t('settings.about.links.reportIssue') }}
                            </button>
                            <button class="link-button" @click="openUrl('https://github.com/JKWTCN/lora/releases')">
                                <i class="icon-download"></i>
                                {{ $t('settings.about.links.checkUpdate') }}
                            </button>
                        </div>
                    </div>
                </div>

                <!-- 界面设置页面 -->
                <div v-if="activeTab === 'ui'" class="panel-content">
                    <div class="settings-group">
                        <h3>{{ $t('settings.ui.window.title') }}</h3>

                        <div class="setting-item">
                            <label>{{ $t('settings.ui.window.width') }}</label>
                            <div class="input-group">
                                <input type="number" v-model.number="localSettings.windowWidth" min="400" max="1920"
                                    @change="updateWindowSize" />
                                <span class="unit">px</span>
                            </div>
                        </div>

                        <div class="setting-item">
                            <label>{{ $t('settings.ui.window.height') }}</label>
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
                                {{ $t('settings.ui.window.preventAutoHide') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.ui.window.preventAutoHideDesc') }}
                            </p>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>{{ $t('settings.ui.appearance.title') }}</h3>

                        <div class="setting-item">
                            <label>{{ $t('settings.ui.appearance.theme') }}</label>
                            <select v-model="localSettings.theme" @change="updateTheme">
                                <option value="auto">{{ $t('settings.ui.appearance.auto') }}</option>
                                <option value="light">{{ $t('settings.ui.appearance.light') }}</option>
                                <option value="dark">{{ $t('settings.ui.appearance.dark') }}</option>
                            </select>
                        </div>

                        <div class="setting-item">
                            <label>{{ $t('settings.ui.appearance.iconSize') }}</label>
                            <div class="slider-group">
                                <input type="range" v-model.number="localSettings.iconSize" min="48" max="128" step="8"
                                    @input="updateIconSize" />
                                <span class="slider-value">{{ localSettings.iconSize }}px</span>
                            </div>
                        </div>

                        <div class="setting-item">
                            <label>{{ $t('settings.ui.appearance.sidebarWidth') }}</label>
                            <div class="slider-group">
                                <input type="range" v-model.number="localSettings.sidebarWidth" min="120" max="300"
                                    step="10" @input="updateSidebarWidth" />
                                <span class="slider-value">{{ localSettings.sidebarWidth }}px</span>
                            </div>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>{{ $t('settings.ui.animation.title') }}</h3>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.enableAnimations"
                                    @change="updateAnimations" />
                                {{ $t('settings.ui.animation.enableAnimations') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.ui.animation.enableAnimationsDesc') }}
                            </p>
                        </div>

                        <div class="setting-item" v-if="localSettings.enableAnimations">
                            <label>{{ $t('settings.ui.animation.speed') }}</label>
                            <select v-model="localSettings.animationSpeed" @change="updateAnimationSpeed">
                                <option value="slow">{{ $t('settings.ui.animation.slow') }}</option>
                                <option value="normal">{{ $t('settings.ui.animation.normal') }}</option>
                                <option value="fast">{{ $t('settings.ui.animation.fast') }}</option>
                            </select>
                        </div>
                    </div>
                </div>

                <!-- 功能设置页面 -->
                <div v-if="activeTab === 'features'" class="panel-content">
                    <div class="settings-group">
                        <h3>{{ $t('settings.features.startup.title') }}</h3>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.startWithSystem"
                                    @change="updateStartWithSystem" />
                                {{ $t('settings.features.startup.startWithSystem') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.features.startup.startWithSystemDesc') }}
                            </p>
                        </div>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.startMinimized"
                                    @change="updateStartMinimized" />
                                {{ $t('settings.features.startup.startMinimized') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.features.startup.startMinimizedDesc') }}
                            </p>
                        </div>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.autoHideAfterLaunch"
                                    @change="updateAutoHideAfterLaunch" />
                                {{ $t('settings.features.startup.autoHideAfterLaunch') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.features.startup.autoHideAfterLaunchDesc') }}
                            </p>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>{{ $t('settings.features.hotkey.title') }}</h3>

                        <div class="setting-item">
                            <label>{{ $t('settings.features.hotkey.toggleWindow') }}</label>
                            <div class="hotkey-input">
                                <input type="text" v-model="localSettings.toggleHotkey" @keydown="captureHotkey"
                                    :placeholder="$t('settings.features.hotkey.setHotkey')" readonly />
                                <button @click="clearHotkey" class="clear-button">{{ $t('settings.features.hotkey.clear') }}</button>
                            </div>
                        </div>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.globalHotkey"
                                    @change="updateGlobalHotkey" />
                                {{ $t('settings.features.hotkey.enableGlobalHotkey') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.features.hotkey.enableGlobalHotkeyDesc') }}
                            </p>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>{{ $t('settings.features.search.title') }}</h3>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.fuzzySearch"
                                    @change="updateFuzzySearch" />
                                {{ $t('settings.features.search.enableFuzzy') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.features.search.enableFuzzyDesc') }}
                            </p>
                        </div>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.searchInPath"
                                    @change="updateSearchInPath" />
                                {{ $t('settings.features.search.searchInPath') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.features.search.searchInPathDesc') }}
                            </p>
                        </div>

                        <div class="setting-item">
                            <label>{{ $t('settings.features.search.maxResults') }}</label>
                            <select v-model.number="localSettings.maxSearchResults" @change="updateMaxSearchResults">
                                <option :value="10">10 {{ $t('settings.features.search.results') }}</option>
                                <option :value="20">20 {{ $t('settings.features.search.results') }}</option>
                                <option :value="50">50 {{ $t('settings.features.search.results') }}</option>
                                <option :value="100">100 {{ $t('settings.features.search.results') }}</option>
                            </select>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>{{ $t('settings.features.data.title') }}</h3>

                        <div class="setting-item">
                            <label>
                                <input type="checkbox" v-model="localSettings.autoBackup" @change="updateAutoBackup" />
                                {{ $t('settings.features.data.autoBackup') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.features.data.autoBackupDesc') }}
                            </p>
                        </div>

                        <div class="setting-item" v-if="localSettings.autoBackup">
                            <label>{{ $t('settings.features.data.backupInterval') }}</label>
                            <select v-model="localSettings.backupInterval" @change="updateBackupInterval">
                                <option value="daily">{{ $t('settings.features.data.daily') }}</option>
                                <option value="weekly">{{ $t('settings.features.data.weekly') }}</option>
                                <option value="monthly">{{ $t('settings.features.data.monthly') }}</option>
                            </select>
                        </div>

                        <div class="setting-item">
                            <div class="button-group">
                                <button @click="exportData" class="action-button">
                                    <i class="icon-export"></i>
                                    {{ $t('settings.features.data.exportData') }}
                                </button>
                                <button @click="importData" class="action-button">
                                    <i class="icon-import"></i>
                                    {{ $t('settings.features.data.importData') }}
                                </button>
                                <button @click="resetData" class="action-button danger">
                                    <i class="icon-reset"></i>
                                    {{ $t('settings.features.data.resetData') }}
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
                    {{ $t('settings.footer.restoreDefaults') }}
                </button>
                <button @click="saveSettings" class="footer-button primary">
                    {{ $t('settings.footer.saveSettings') }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, reactive, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import { alertDialog, confirmDialog } from './utils/customDialog'

const { t } = useI18n()

// 标签页配置
const tabs = computed(() => [
    { id: 'about', name: t('settings.tabs.about'), icon: 'icon-info' },
    { id: 'ui', name: t('settings.tabs.ui'), icon: 'icon-display' },
    { id: 'features', name: t('settings.tabs.features'), icon: 'icon-settings' }
])

const activeTab = ref('about')
const isSaving = ref(false)
const lastSaved = ref(false)

// 应用版本号
const appVersion = ref('')
// 应用更新日期
const appUpdateDate = ref('')

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
    autoHideAfterLaunch: false,

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
    if (isSaving.value) return t('settings.saving')
    if (lastSaved.value) return t('settings.saved')
    return ''
})

// 监听设置变化
watch(localSettings, () => {
    lastSaved.value = false
}, { deep: true })

// 方法
const updateWindowSize = async () => {
    try {
        // 保存窗口大小到设置
        await invoke('save_window_size', {
            width: localSettings.windowWidth,
            height: localSettings.windowHeight
        })
        
        // 立即应用到主窗口
        const { WebviewWindow } = await import('@tauri-apps/api/window')
        const { LogicalSize } = await import('@tauri-apps/api/dpi')
        
        // 获取主窗口（不是设置窗口）
        try {
            const mainWindow = new WebviewWindow('main')
            await mainWindow.setSize(new LogicalSize(
                localSettings.windowWidth,
                localSettings.windowHeight
            ))
            console.log(`主窗口大小已更新: ${localSettings.windowWidth}x${localSettings.windowHeight}`)
        } catch (error) {
            console.error('无法获取主窗口:', error)
            // 如果无法获取主窗口，尝试通过后端获取窗口大小来验证主窗口是否存在
            try {
                await invoke('get_main_window_size')
                console.log('主窗口存在但无法直接操作')
            } catch (backendError) {
                console.error('主窗口不存在:', backendError)
            }
        }
    } catch (error) {
        console.error('更新窗口大小失败:', error)
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

const updateAutoHideAfterLaunch = async () => {
    try {
        await invoke('update_auto_hide_after_launch', { autoHideAfterLaunch: localSettings.autoHideAfterLaunch })
        console.log('运行应用后自动隐藏设置已更新')
    } catch (error) {
        console.error('更新运行应用后自动隐藏设置失败:', error)
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
        await alertDialog(t('settings.alert.exportSuccess'), { type: 'success' })
    } catch (error) {
        console.error('导出数据失败:', error)
        await alertDialog(t('settings.alert.exportFailed', { error: String(error) }), { type: 'error' })
    } finally {
        isSaving.value = false
    }
}

const importData = async () => {
    try {
        isSaving.value = true
        const result = await invoke('import_data')
        console.log('数据导入成功:', result)
        await alertDialog(t('settings.alert.importSuccess'), { type: 'success' })
        // 重新加载设置
        await loadSettings()
    } catch (error) {
        console.error('导入数据失败:', error)
        await alertDialog(t('settings.alert.importFailed', { error: String(error) }), { type: 'error' })
    } finally {
        isSaving.value = false
    }
}

const resetData = async () => {
    if (await confirmDialog(t('settings.confirm.resetData'))) {
        try {
            isSaving.value = true
            const result = await invoke('reset_data')
            console.log('数据重置成功:', result)
            await alertDialog(t('settings.alert.resetSuccess'), { type: 'success' })
            // 重新加载设置
            await loadSettings()
        } catch (error) {
            console.error('重置数据失败:', error)
            await alertDialog(t('settings.alert.resetFailed', { error: String(error) }), { type: 'error' })
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
    if (await confirmDialog(t('settings.confirm.resetSettings'))) {
        try {
            isSaving.value = true
            await invoke('reset_settings_to_default')
            // 重新加载设置
            await loadSettings()
            await alertDialog(t('settings.alert.restoreDefaultsSuccess'), { type: 'success' })
        } catch (error) {
            console.error('恢复默认设置失败:', error)
            await alertDialog(t('settings.alert.restoreDefaultsFailed', { error: String(error) }), { type: 'error' })
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
        localSettings.autoHideAfterLaunch = settings.auto_hide_after_launch || false
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
            autoHideAfterLaunch: localSettings.autoHideAfterLaunch,
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
onMounted(async () => {
    // 获取应用版本号和更新日期
    try {
        appVersion.value = await invoke('get_app_version')
        appUpdateDate.value = await invoke('get_app_update_date')
    } catch (error) {
        console.error('获取版本信息失败:', error)
        appVersion.value = '未知'
        appUpdateDate.value = '未知'
    }

    await loadSettings()
})
</script>

<style scoped>
.settings-app {
    display: grid;
    grid-template-columns: 160px minmax(0, 1fr);
    grid-template-rows: minmax(0, 1fr) auto;
    height: 100vh;
    background: #f6f8fb;
    color: #172033;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.settings-content {
    display: contents;
}

.settings-nav {
    width: 160px;
    grid-column: 1;
    grid-row: 1 / 3;
    background: #243447;
    border-right: 1px solid #1d2b3a;
    padding: 10px 0;
}

.nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 40px;
    padding: 0 16px;
    cursor: pointer;
    transition: background-color 0.18s ease, color 0.18s ease;
    color: #c9d4e2;
    font-size: 14px;
    border-left: 3px solid transparent;
}

.nav-item:hover {
    background: rgba(255, 255, 255, 0.08);
    color: #ffffff;
}

.nav-item.active {
    background: #2f9ae0;
    color: #ffffff;
    border-left-color: #7dd3fc;
}

.nav-item i {
    width: 16px;
    font-size: 15px;
    text-align: center;
}

.settings-panel {
    grid-column: 2;
    grid-row: 1;
    min-width: 0;
    min-height: 0;
    overflow-y: auto;
    background: #f6f8fb;
}

.panel-content {
    width: min(560px, 100%);
    padding: 20px 22px 28px;
}

/* 关于页面样式 */
.about-section {
    display: flex;
    flex-direction: column;
    gap: 14px;
}

.app-info {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 18px;
    border: 1px solid #d9e1ea;
    border-radius: 8px;
    background: #ffffff;
    box-shadow: 0 1px 2px rgba(15, 23, 42, 0.04);
}

.app-icon img {
    width: 52px;
    height: 52px;
    display: block;
}

.app-details h3 {
    margin: 0 0 6px 0;
    font-size: 20px;
    font-weight: 650;
    color: #172033;
}

.version {
    color: #64748b;
    margin: 0 0 8px 0;
    font-size: 13px;
}

.description {
    color: #64748b;
    margin: 0;
    line-height: 1.55;
    font-size: 13px;
}

.info-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0;
    overflow: hidden;
    border: 1px solid #d9e1ea;
    border-radius: 8px;
    background: #ffffff;
    box-shadow: 0 1px 2px rgba(15, 23, 42, 0.04);
}

.info-item {
    display: flex;
    justify-content: space-between;
    gap: 14px;
    min-height: 46px;
    padding: 12px 14px;
    border-bottom: 1px solid #edf1f5;
    font-size: 13px;
    background: #ffffff;
}

.info-item:nth-child(odd) {
    border-right: 1px solid #edf1f5;
}

.info-item:nth-last-child(-n + 2) {
    border-bottom: 0;
}

.info-item label {
    color: #64748b;
    font-weight: 500;
    white-space: nowrap;
}

.info-item span {
    color: #172033;
    text-align: right;
}

.links-section {
    display: flex;
    gap: 10px;
    justify-content: flex-start;
    flex-wrap: wrap;
    padding: 14px;
    border: 1px solid #d9e1ea;
    border-radius: 8px;
    background: #ffffff;
    box-shadow: 0 1px 2px rgba(15, 23, 42, 0.04);
}

.link-button {
    display: flex;
    align-items: center;
    gap: 6px;
    min-height: 34px;
    padding: 0 12px;
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    color: #334155;
    text-decoration: none;
    cursor: pointer;
    transition: background-color 0.18s ease, border-color 0.18s ease, color 0.18s ease;
    font-size: 12px;
    font-weight: 600;
}

.link-button:hover {
    background: #eef6ff;
    border-color: #2f9ae0;
    color: #1269a8;
}

/* 设置组样式 */
.settings-group {
    margin-bottom: 14px;
    padding: 16px 18px;
    border: 1px solid #d9e1ea;
    border-radius: 8px;
    background: #ffffff;
    box-shadow: 0 1px 2px rgba(15, 23, 42, 0.04);
}

.settings-group h3 {
    margin: 0 0 14px 0;
    font-size: 15px;
    font-weight: 650;
    color: #172033;
    border-bottom: 1px solid #edf1f5;
    padding-bottom: 10px;
}

.setting-item {
    display: grid;
    grid-template-columns: minmax(150px, 190px) minmax(0, 1fr);
    column-gap: 18px;
    row-gap: 6px;
    align-items: center;
    min-height: 40px;
    margin-bottom: 12px;
}

.setting-item label {
    font-weight: 500;
    color: #334155;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
}

.setting-item input[type="checkbox"] {
    margin: 0;
    accent-color: #2f9ae0;
}

.setting-item input[type="number"],
.setting-item input[type="text"],
.setting-item select {
    width: 100%;
    max-width: 220px;
    height: 34px;
    padding: 0 10px;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    background: #ffffff;
    color: #172033;
    font-size: 13px;
    transition: border-color 0.18s ease, box-shadow 0.18s ease;
}

.setting-item input:focus,
.setting-item select:focus {
    outline: none;
    border-color: #2f9ae0;
    box-shadow: 0 0 0 3px rgba(47, 154, 224, 0.16);
}

.setting-description {
    grid-column: 2;
    font-size: 11px;
    color: #64748b;
    margin: 0;
    line-height: 1.45;
}

.input-group {
    display: flex;
    align-items: center;
    gap: 6px;
}

.unit {
    color: #64748b;
    font-size: 12px;
}

.slider-group {
    display: flex;
    align-items: center;
    gap: 10px;
    max-width: 280px;
}

.slider-group input[type="range"] {
    flex: 1;
    accent-color: #2f9ae0;
}

.slider-value {
    min-width: 45px;
    text-align: right;
    color: #64748b;
    font-size: 12px;
}

.hotkey-input {
    display: flex;
    gap: 6px;
    max-width: 290px;
}

.hotkey-input input {
    flex: 1;
    font-size: 13px;
}

.clear-button {
    height: 34px;
    padding: 0 12px;
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    color: #64748b;
    cursor: pointer;
    transition: background-color 0.18s ease, color 0.18s ease, border-color 0.18s ease;
    font-size: 12px;
    font-weight: 600;
    white-space: nowrap;
}

.clear-button:hover {
    background: #eef6ff;
    border-color: #2f9ae0;
    color: #1269a8;
}

.button-group {
    display: flex;
    gap: 8px;
    flex-wrap: nowrap;
    grid-column: 1 / -1;
}

.action-button {
    display: flex;
    align-items: center;
    gap: 6px;
    min-height: 34px;
    padding: 0 12px;
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    color: #334155;
    cursor: pointer;
    transition: background-color 0.18s ease, border-color 0.18s ease, color 0.18s ease;
    font-size: 12px;
    font-weight: 600;
}

.action-button:hover {
    background: #eef6ff;
    border-color: #2f9ae0;
    color: #1269a8;
}

.action-button.danger {
    border-color: #fecaca;
    color: #dc2626;
}

.action-button.danger:hover {
    background: #fef2f2;
    border-color: #f87171;
}

.settings-footer {
    grid-column: 2;
    grid-row: 2;
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 62px;
    padding: 12px 20px;
    border-top: 1px solid #d9e1ea;
    background: #ffffff;
}

.save-status {
    font-size: 12px;
    color: #64748b;
    transition: color 0.18s ease;
}

.save-status.saving {
    color: #2f9ae0;
}

.save-status.saved {
    color: #059669;
}

.footer-right {
    display: flex;
    gap: 8px;
}

.footer-button {
    min-width: 86px;
    height: 36px;
    padding: 0 16px;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    cursor: pointer;
    transition: background-color 0.18s ease, border-color 0.18s ease, color 0.18s ease, box-shadow 0.18s ease;
    font-weight: 600;
    font-size: 13px;
}

.footer-button.secondary {
    background: #ffffff;
    color: #64748b;
}

.footer-button.secondary:hover {
    background: #f8fafc;
    color: #334155;
}

.footer-button.primary {
    background: #2f9ae0;
    color: #ffffff;
    border-color: #2f9ae0;
    box-shadow: 0 8px 18px rgba(47, 154, 224, 0.2);
}

.footer-button.primary:hover {
    background: #2384c4;
    border-color: #2384c4;
}

@media (max-width: 640px) {
    .settings-app {
        grid-template-columns: 148px minmax(0, 1fr);
    }

    .settings-nav {
        width: 148px;
    }

    .panel-content {
        padding: 16px;
    }

    .info-grid {
        grid-template-columns: 1fr;
    }

    .info-item:nth-child(odd) {
        border-right: 0;
    }

    .info-item:nth-last-child(2) {
        border-bottom: 1px solid #edf1f5;
    }

    .setting-item {
        grid-template-columns: 1fr;
    }

    .setting-description {
        grid-column: 1;
    }

    .button-group {
        flex-wrap: wrap;
    }
}
</style>
