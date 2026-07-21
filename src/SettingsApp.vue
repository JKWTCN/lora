<template>
    <div class="settings-app" :class="settingsAppClasses">
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
                                <img src="/app-icon.png" alt="Lora" />
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
                            <label>
                                <input type="checkbox" v-model="localSettings.preventAutoHide"
                                    @change="updatePreventAutoHide" />
                                {{ $t('settings.ui.window.preventAutoHide') }}
                            </label>
                            <p class="setting-description">
                                {{ $t('settings.ui.window.preventAutoHideDesc') }}
                            </p>
                        </div>

                        <div class="setting-item">
                            <label>{{ $t('settings.ui.window.layout') }}</label>
                            <select v-model="localSettings.windowLayout" @change="updateWindowLayout">
                                <option value="horizontal">{{ $t('settings.ui.window.layoutHorizontal') }}</option>
                                <option value="vertical">{{ $t('settings.ui.window.layoutVertical') }}</option>
                            </select>
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
                            <label>{{ $t('language.switch') }}</label>
                            <LanguageSwitch variant="settings" :dark="resolvedTheme === 'dark'" />
                        </div>

                        <div class="setting-item setting-note">
                            <p>{{ $t('settings.ui.appearance.gridCellSizeHint') }}</p>
                        </div>

                        <div class="setting-item">
                            <label>{{ $t('settings.ui.appearance.projectNamePosition') }}</label>
                            <select v-model="localSettings.projectNamePosition" @change="updateProjectNamePosition">
                                <option value="none">{{ $t('settings.ui.appearance.namePositionNone') }}</option>
                                <option value="top">{{ $t('settings.ui.appearance.namePositionTop') }}</option>
                                <option value="bottom">{{ $t('settings.ui.appearance.namePositionBottom') }}</option>
                                <option value="left">{{ $t('settings.ui.appearance.namePositionLeft') }}</option>
                                <option value="right">{{ $t('settings.ui.appearance.namePositionRight') }}</option>
                            </select>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>{{ $t('settings.ui.categories.title') }}</h3>

                        <p class="group-description">{{ $t('settings.ui.categories.description') }}</p>

                        <div v-if="customCategories.length > 0" class="category-visibility-list">
                            <label
                                v-for="category in customCategories"
                                :key="category.id"
                                class="category-visibility-item">
                                <span class="category-name">{{ category.name }}</span>
                                <span class="category-toggle">
                                    <input
                                        type="checkbox"
                                        v-model="category.hidden"
                                        @change="updateCategoryHidden(category)" />
                                    <span>{{ category.hidden ? $t('settings.ui.categories.hidden') : $t('settings.ui.categories.visible') }}</span>
                                </span>
                            </label>
                        </div>

                        <div v-else class="empty-state">
                            {{ $t('settings.ui.categories.empty') }}
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

                        <div class="setting-item">
                            <label>{{ $t('settings.features.search.sortOrder') }}</label>
                            <select v-model="localSettings.sortOrder" @change="updateSortOrder">
                                <option value="manual">{{ $t('settings.features.search.sortManual') }}</option>
                                <option value="name">{{ $t('settings.features.search.sortName') }}</option>
                                <option value="frequency">{{ $t('settings.features.search.sortFrequency') }}</option>
                            </select>
                            <p class="setting-description">
                                {{ $t('settings.features.search.sortOrderDesc') }}
                            </p>
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
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, reactive, computed, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { emit, emitTo, listen } from '@tauri-apps/api/event'
import { useI18n } from 'vue-i18n'
import { alertDialog, confirmDialog } from './utils/customDialog'
import LanguageSwitch from './components/LanguageSwitch.vue'

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
const categories = ref([])
let unlistenSettingsUpdated = null
let removeThemePreferenceListener = null

// 应用版本号
const appVersion = ref('')
// 应用更新日期
const appUpdateDate = ref('')

// 本地设置状态
const localSettings = reactive({
    // 窗口设置
    preventAutoHide: false,
    windowLayout: 'horizontal',

    // 外观设置
    theme: 'auto',
    projectNamePosition: 'bottom',

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
    sortOrder: 'manual',

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

const customCategories = computed(() => categories.value.filter(category => !category.isDefault))

const resolvedTheme = computed(() => {
    if (localSettings.theme === 'auto') {
        return window.matchMedia?.('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    }

    return localSettings.theme === 'dark' ? 'dark' : 'light'
})

const settingsAppClasses = computed(() => ({
    'theme-dark': resolvedTheme.value === 'dark',
    'theme-light': resolvedTheme.value === 'light'
}))

const applyRuntimeTheme = () => {
    const body = document.body
    body.classList.toggle('lora-theme-dark', resolvedTheme.value === 'dark')
    body.classList.toggle('lora-theme-light', resolvedTheme.value === 'light')
}

// 监听设置变化
watch(localSettings, () => {
    lastSaved.value = false
}, { deep: true })

watch(resolvedTheme, applyRuntimeTheme)

// 方法
const markSaved = () => {
    lastSaved.value = true
    setTimeout(() => {
        lastSaved.value = false
    }, 1500)
}

const notifySettingsUpdated = async () => {
    await Promise.allSettled([
        invoke('notify_main_settings_updated'),
        emitTo('main', 'settings-updated'),
        emit('settings-updated')
    ])
}

const notifyDataUpdated = async () => {
    await Promise.allSettled([
        invoke('notify_main_window_refresh'),
        emitTo('main', 'data-updated'),
        emit('data-updated')
    ])
}

const updatePreventAutoHide = async () => {
    try {
        await invoke('update_prevent_auto_hide', {
            preventAutoHide: localSettings.preventAutoHide
        })
        await notifySettingsUpdated()
        markSaved()
    } catch (error) {
        console.error('更新阻止自动隐藏设置失败:', error)
    }
}

const updateTheme = async () => {
    try {
        await invoke('update_theme', { theme: localSettings.theme })
        await notifySettingsUpdated()
        markSaved()
        console.log('主题设置已更新')
    } catch (error) {
        console.error('更新主题设置失败:', error)
    }
}

const updateWindowLayout = async () => {
    try {
        await invoke('update_window_layout', { windowLayout: localSettings.windowLayout })
        await notifySettingsUpdated()
        markSaved()
        console.log('窗口布局设置已更新')
    } catch (error) {
        console.error('更新窗口布局设置失败:', error)
    }
}

const updateProjectNamePosition = async () => {
    try {
        await invoke('update_project_name_position', { projectNamePosition: localSettings.projectNamePosition })
        await notifySettingsUpdated()
        markSaved()
        console.log('项目名称显示位置已更新')
    } catch (error) {
        console.error('更新项目名称显示位置失败:', error)
    }
}

const updateCategoryHidden = async (category) => {
    try {
        await invoke('update_category_hidden', {
            categoryId: category.id,
            hidden: !!category.hidden
        })
        await notifyDataUpdated()
        markSaved()
        console.log('分组显示状态已更新')
    } catch (error) {
        console.error('更新分组显示状态失败:', error)
        await loadCategories()
        await alertDialog(t('settings.alert.updateCategoryVisibilityFailed', { error: String(error) }), { type: 'error' })
    }
}

const updateStartWithSystem = async () => {
    try {
        await invoke('update_start_with_system', { startWithSystem: localSettings.startWithSystem })
        markSaved()
        console.log('开机自启动设置已更新')
    } catch (error) {
        console.error('更新开机自启动设置失败:', error)
    }
}

const updateStartMinimized = async () => {
    try {
        await invoke('update_start_minimized', { startMinimized: localSettings.startMinimized })
        markSaved()
        console.log('启动最小化设置已更新')
    } catch (error) {
        console.error('更新启动最小化设置失败:', error)
    }
}

const updateAutoHideAfterLaunch = async () => {
    try {
        await invoke('update_auto_hide_after_launch', { autoHideAfterLaunch: localSettings.autoHideAfterLaunch })
        await notifySettingsUpdated()
        markSaved()
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
        markSaved()
        console.log('快捷键设置已更新')
    } catch (error) {
        console.error('更新快捷键设置失败:', error)
    }
}

const updateGlobalHotkey = async () => {
    try {
        await invoke('update_global_hotkey', { globalHotkey: localSettings.globalHotkey })
        markSaved()
        console.log('全局快捷键设置已更新')
    } catch (error) {
        console.error('更新全局快捷键设置失败:', error)
    }
}

const updateFuzzySearch = async () => {
    try {
        await invoke('update_fuzzy_search', { fuzzySearch: localSettings.fuzzySearch })
        await notifySettingsUpdated()
        markSaved()
        console.log('模糊搜索设置已更新')
    } catch (error) {
        console.error('更新模糊搜索设置失败:', error)
    }
}

const updateSearchInPath = async () => {
    try {
        await invoke('update_search_in_path', { searchInPath: localSettings.searchInPath })
        await notifySettingsUpdated()
        markSaved()
        console.log('路径搜索设置已更新')
    } catch (error) {
        console.error('更新路径搜索设置失败:', error)
    }
}

const updateMaxSearchResults = async () => {
    try {
        await invoke('update_max_search_results', { maxSearchResults: localSettings.maxSearchResults })
        await notifySettingsUpdated()
        markSaved()
        console.log('最大搜索结果设置已更新')
    } catch (error) {
        console.error('更新最大搜索结果设置失败:', error)
    }
}

const updateSortOrder = async () => {
    try {
        await invoke('update_sort_order', { sortOrder: localSettings.sortOrder })
        await notifySettingsUpdated()
        markSaved()
        console.log('排序方式设置已更新')
    } catch (error) {
        console.error('更新排序方式设置失败:', error)
    }
}

const updateAutoBackup = async () => {
    try {
        await invoke('update_auto_backup', { autoBackup: localSettings.autoBackup })
        markSaved()
        console.log('自动备份设置已更新')
    } catch (error) {
        console.error('更新自动备份设置失败:', error)
    }
}

const updateBackupInterval = async () => {
    try {
        await invoke('update_backup_interval', { backupInterval: localSettings.backupInterval })
        markSaved()
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
        await notifySettingsUpdated()
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
            await notifySettingsUpdated()
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
            await notifySettingsUpdated()
            await alertDialog(t('settings.alert.restoreDefaultsSuccess'), { type: 'success' })
        } catch (error) {
            console.error('恢复默认设置失败:', error)
            await alertDialog(t('settings.alert.restoreDefaultsFailed', { error: String(error) }), { type: 'error' })
        } finally {
            isSaving.value = false
        }
    }
}

const loadCategories = async () => {
    try {
        const storage = await invoke('load_app_data')
        const categoriesFromBackend = storage.categories || []
        categories.value = categoriesFromBackend.map(category => ({
            id: category.id,
            name: category.name,
            icon: category.icon,
            isDefault: category.is_default,
            order: category.order,
            hidden: !!category.hidden
        }))
    } catch (error) {
        console.error('加载分组数据失败:', error)
        categories.value = []
    }
}

// 加载设置
const loadSettings = async () => {
    try {
        const settings = await invoke('load_app_settings')

        // 将后端设置映射到前端本地设置
        localSettings.preventAutoHide = settings.prevent_auto_hide || false
        localSettings.windowLayout = settings.window_layout || 'horizontal'
        localSettings.theme = settings.theme || 'auto'
        localSettings.projectNamePosition = settings.project_name_position || 'bottom'
        localSettings.startMinimized = settings.start_minimized || false
        localSettings.autoHideAfterLaunch = settings.auto_hide_after_launch || false
        localSettings.toggleHotkey = settings.toggle_hotkey || 'Ctrl+Space'
        localSettings.globalHotkey = settings.global_hotkey !== false
        localSettings.fuzzySearch = settings.fuzzy_search !== false
        localSettings.searchInPath = settings.search_in_path || false
        localSettings.maxSearchResults = settings.max_search_results || 20
        localSettings.sortOrder = settings.sort_order || 'manual'
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

    await loadCategories()
}

// 初始化
onMounted(async () => {
    const themePreference = window.matchMedia?.('(prefers-color-scheme: dark)')
    if (themePreference) {
        const handleThemePreferenceChange = () => applyRuntimeTheme()
        themePreference.addEventListener('change', handleThemePreferenceChange)
        removeThemePreferenceListener = () => {
            themePreference.removeEventListener('change', handleThemePreferenceChange)
        }
    }

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
    applyRuntimeTheme()

    unlistenSettingsUpdated = await listen('settings-updated', async () => {
        await loadSettings()
        applyRuntimeTheme()
    })
})

onUnmounted(() => {
    if (unlistenSettingsUpdated) {
        unlistenSettingsUpdated()
        unlistenSettingsUpdated = null
    }

    if (removeThemePreferenceListener) {
        removeThemePreferenceListener()
        removeThemePreferenceListener = null
    }
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
    color-scheme: light;
}

.settings-app.theme-dark {
    background: #111827;
    color: #e5edf7;
    color-scheme: dark;
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

.settings-app.theme-dark .settings-nav {
    background: #0f172a;
    border-right-color: #1f2937;
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
    scrollbar-width: thin;
    scrollbar-color: rgba(47, 154, 224, 0.28) transparent;
}

.settings-panel::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

.settings-panel::-webkit-scrollbar-track {
    background: transparent;
}

.settings-panel::-webkit-scrollbar-thumb {
    border: 2px solid transparent;
    border-radius: 999px;
    background: rgba(47, 154, 224, 0.24);
    background-clip: padding-box;
}

.settings-panel::-webkit-scrollbar-thumb:hover {
    background: rgba(47, 154, 224, 0.38);
    background-clip: padding-box;
}

.settings-app.theme-dark .settings-panel {
    background: #111827;
    scrollbar-color: rgba(56, 189, 248, 0.38) transparent;
}

.settings-app.theme-dark .settings-panel::-webkit-scrollbar-thumb {
    background: rgba(56, 189, 248, 0.32);
    background-clip: padding-box;
}

.settings-app.theme-dark .settings-panel::-webkit-scrollbar-thumb:hover {
    background: rgba(56, 189, 248, 0.46);
    background-clip: padding-box;
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

.settings-app.theme-dark .app-info,
.settings-app.theme-dark .info-grid,
.settings-app.theme-dark .links-section,
.settings-app.theme-dark .settings-group,
.settings-app.theme-dark .settings-footer {
    border-color: #2b3748;
    background: #172033;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.18);
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

.settings-app.theme-dark .app-details h3,
.settings-app.theme-dark .settings-group h3,
.settings-app.theme-dark .info-item span {
    color: #e5edf7;
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

.settings-app.theme-dark .version,
.settings-app.theme-dark .description,
.settings-app.theme-dark .info-item label,
.settings-app.theme-dark .setting-description,
.settings-app.theme-dark .group-description,
.settings-app.theme-dark .category-toggle,
.settings-app.theme-dark .empty-state,
.settings-app.theme-dark .setting-note p,
.settings-app.theme-dark .unit,
.settings-app.theme-dark .slider-value,
.settings-app.theme-dark .save-status {
    color: #94a3b8;
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

.settings-app.theme-dark .info-item {
    background: #172033;
    border-bottom-color: #253246;
}

.info-item:nth-child(odd) {
    border-right: 1px solid #edf1f5;
}

.settings-app.theme-dark .info-item:nth-child(odd) {
    border-right-color: #253246;
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

.settings-app.theme-dark .link-button,
.settings-app.theme-dark .clear-button,
.settings-app.theme-dark .action-button,
.settings-app.theme-dark .footer-button.secondary {
    background: #1f2937;
    border-color: #334155;
    color: #cbd5e1;
}

.link-button:hover {
    background: #eef6ff;
    border-color: #2f9ae0;
    color: #1269a8;
}

.settings-app.theme-dark .link-button:hover,
.settings-app.theme-dark .clear-button:hover,
.settings-app.theme-dark .action-button:hover,
.settings-app.theme-dark .footer-button.secondary:hover {
    background: #22364d;
    border-color: #38bdf8;
    color: #e0f2fe;
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

.settings-app.theme-dark .settings-group h3 {
    border-bottom-color: #253246;
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

.settings-app.theme-dark .setting-item label {
    color: #d8e2ef;
}

.setting-item input[type="checkbox"] {
    appearance: none;
    display: inline-grid;
    place-content: center;
    width: 16px;
    height: 16px;
    margin: 0;
    border: 1px solid #cbd5e1;
    border-radius: 4px;
    background: #ffffff;
    cursor: pointer;
    transition: background-color 0.18s ease, border-color 0.18s ease, box-shadow 0.18s ease;
}

.setting-item input[type="checkbox"]:checked {
    border-color: #2f9ae0;
    background: #2f9ae0;
    background-image: url("data:image/svg+xml,%3Csvg width='12' height='12' viewBox='0 0 12 12' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M2.3 6.15L4.8 8.65L9.7 3.35' stroke='white' stroke-width='1.8' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
    background-position: center;
    background-repeat: no-repeat;
}

.setting-item input[type="checkbox"]:focus-visible {
    outline: none;
    box-shadow: 0 0 0 3px rgba(47, 154, 224, 0.16);
}

.settings-app.theme-dark .setting-item input[type="checkbox"] {
    background: #111827;
    border-color: #334155;
}

.settings-app.theme-dark .setting-item input[type="checkbox"]:checked {
    background: #38bdf8;
    border-color: #38bdf8;
}

.settings-app.theme-dark .category-visibility-item {
    background: #111827;
    border-color: #253246;
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

.settings-app.theme-dark .setting-item input[type="number"],
.settings-app.theme-dark .setting-item input[type="text"],
.settings-app.theme-dark .setting-item select {
    background: #111827;
    border-color: #334155;
    color: #e5edf7;
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

.group-description {
    margin: -4px 0 12px;
    color: #64748b;
    font-size: 12px;
    line-height: 1.5;
}

.category-visibility-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.category-visibility-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    min-height: 38px;
    padding: 0 10px;
    border: 1px solid #edf1f5;
    border-radius: 6px;
    background: #f8fafc;
}

.category-name {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.category-toggle {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    color: #64748b;
    font-size: 12px;
    white-space: nowrap;
}

.empty-state {
    color: #64748b;
    font-size: 13px;
    line-height: 1.5;
}

.setting-note {
    display: block;
    min-height: 0;
}

.setting-note p {
    margin: 0;
    color: #64748b;
    font-size: 13px;
    line-height: 1.5;
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
    height: 18px;
    appearance: none;
    background: transparent;
    cursor: pointer;
}

.slider-group input[type="range"]:focus {
    outline: none;
}

.slider-group input[type="range"]::-webkit-slider-runnable-track {
    height: 6px;
    border-radius: 999px;
    background: #dbe6f1;
}

.slider-group input[type="range"]::-webkit-slider-thumb {
    appearance: none;
    width: 16px;
    height: 16px;
    margin-top: -5px;
    border: 2px solid #ffffff;
    border-radius: 50%;
    background: #2f9ae0;
    box-shadow: 0 1px 4px rgba(15, 23, 42, 0.22);
}

.slider-group input[type="range"]:focus-visible::-webkit-slider-thumb {
    box-shadow: 0 0 0 4px rgba(47, 154, 224, 0.18), 0 1px 4px rgba(15, 23, 42, 0.22);
}

.slider-group input[type="range"]::-moz-range-track {
    height: 6px;
    border: 0;
    border-radius: 999px;
    background: #dbe6f1;
}

.slider-group input[type="range"]::-moz-range-thumb {
    width: 16px;
    height: 16px;
    border: 2px solid #ffffff;
    border-radius: 50%;
    background: #2f9ae0;
    box-shadow: 0 1px 4px rgba(15, 23, 42, 0.22);
}

.settings-app.theme-dark .slider-group input[type="range"]::-webkit-slider-runnable-track {
    background: #334155;
}

.settings-app.theme-dark .slider-group input[type="range"]::-webkit-slider-thumb {
    border-color: #172033;
    background: #38bdf8;
}

.settings-app.theme-dark .slider-group input[type="range"]::-moz-range-track {
    background: #334155;
}

.settings-app.theme-dark .slider-group input[type="range"]::-moz-range-thumb {
    border-color: #172033;
    background: #38bdf8;
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

.settings-app.theme-dark .action-button.danger {
    background: #2a1b22;
    border-color: #7f1d1d;
    color: #fca5a5;
}

.action-button.danger:hover {
    background: #fef2f2;
    border-color: #f87171;
}

.settings-app.theme-dark .action-button.danger:hover {
    background: #3b1d25;
    border-color: #ef4444;
    color: #fecaca;
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

.settings-app.theme-dark .settings-footer {
    border-top-color: #2b3748;
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

/* Apple-inspired settings: translucent navigation and grouped controls. */
.settings-app {
    grid-template-columns: 184px minmax(0, 1fr);
    background: #f2f2f7;
    color: #1d1d1f;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI Variable", "Segoe UI", sans-serif;
}

.settings-app.theme-dark { background: #161618; color: #f5f5f7; }
.settings-nav {
    width: 184px;
    padding: 12px 9px;
    background: rgba(232, 232, 237, .82);
    backdrop-filter: blur(28px) saturate(170%);
    border-right: 1px solid rgba(60,60,67,.16);
}
.settings-app.theme-dark .settings-nav {
    background: rgba(36,36,38,.86);
    border-right-color: rgba(255,255,255,.1);
}
.nav-item {
    min-height: 36px;
    margin: 2px 0;
    padding: 0 11px;
    border: 0;
    border-radius: 9px;
    color: #3a3a3c;
    font-size: 13px;
    font-weight: 500;
    transition: background-color 140ms ease, transform 90ms ease;
}
.settings-app.theme-dark .nav-item { color: #d1d1d6; }
.nav-item:hover { color: inherit; background: rgba(118,118,128,.12); }
.nav-item:active { transform: scale(.97); }
.nav-item.active { color: #fff; background: #007aff; border: 0; }
.nav-item i { --icon-size: 17px; width: 17px; }
.settings-panel, .settings-app.theme-dark .settings-panel { background: transparent; }
.panel-content { width: min(680px, 100%); padding: 24px 28px 34px; }

.app-info, .info-grid, .links-section, .settings-group {
    border-color: rgba(60,60,67,.12);
    border-radius: 14px;
    background: rgba(255,255,255,.82);
    box-shadow: 0 1px 1px rgba(0,0,0,.03), 0 8px 30px rgba(0,0,0,.04);
}
.settings-app.theme-dark .app-info,
.settings-app.theme-dark .info-grid,
.settings-app.theme-dark .links-section,
.settings-app.theme-dark .settings-group {
    border-color: rgba(255,255,255,.1);
    background: rgba(44,44,46,.78);
}
.app-details h3 { font-size: 22px; font-weight: 650; letter-spacing: -.025em; }
.settings-group { padding: 17px 18px; margin-bottom: 16px; }
.settings-group h3 {
    padding-bottom: 11px;
    border-bottom-color: rgba(60,60,67,.12);
    font-size: 14px;
    letter-spacing: -.01em;
}
.setting-item { min-height: 42px; }
.setting-item label { color: #1d1d1f; font-size: 13px; font-weight: 500; }
.setting-description, .group-description { color: #6e6e73; }
.setting-item input[type="number"],
.setting-item input[type="text"],
.setting-item select,
.hotkey-input input {
    height: 36px;
    border-color: rgba(60,60,67,.18);
    border-radius: 9px;
    background: rgba(118,118,128,.08);
}
.setting-item input:focus, .setting-item select:focus {
    border-color: #007aff;
    box-shadow: 0 0 0 3px rgba(0,122,255,.16);
}
.setting-item input[type="checkbox"] { width: 18px; height: 18px; border-radius: 5px; }
.setting-item input[type="checkbox"]:checked { background-color: #007aff; border-color: #007aff; }
.category-visibility-item { min-height: 42px; border-radius: 9px; background: rgba(118,118,128,.07); }
.link-button, .clear-button, .action-button, .footer-button {
    min-height: 36px;
    border-radius: 10px;
    transition: background-color 140ms ease, color 140ms ease, transform 90ms ease;
}
.link-button i, .action-button i { --icon-size: 16px; }
.link-button:hover, .clear-button:hover, .action-button:hover { border-color: #007aff; color: #007aff; background: rgba(0,122,255,.08); }
.settings-footer {
    min-height: 64px;
    padding: 12px 22px;
    border-top-color: rgba(60,60,67,.14);
    background: rgba(246,246,246,.78);
    backdrop-filter: blur(24px) saturate(170%);
}
.settings-app.theme-dark .settings-footer { background: rgba(28,28,30,.82); border-top-color: rgba(255,255,255,.1); }
.footer-button.primary { background: #007aff; border-color: #007aff; box-shadow: 0 7px 18px rgba(0,122,255,.2); }

@media (prefers-reduced-transparency: reduce) {
    .settings-nav, .settings-footer { backdrop-filter: none; }
    .settings-nav { background: #e5e5ea; }
    .settings-footer { background: #f2f2f7; }
}

@media (max-width: 640px) {
    .settings-app { grid-template-columns: 148px minmax(0, 1fr); }
    .settings-nav { width: 148px; }
    .panel-content { padding: 18px; }
}
</style>
