<template>
    <div class="new-project-app" :class="newProjectAppClasses" @click="hideStartMenuContextMenu">
        <!-- 自定义标题栏 -->
        <!-- <div class="titlebar">
            <div class="titlebar-left" data-tauri-drag-region>
                <div class="app-icon">🚀</div>
                <span class="app-title" data-tauri-drag-region>新建项目</span>
            </div>
            <div class="titlebar-right">
                <button class="titlebar-button" @click="cancelProject" title="关闭">
                    <i class="icon-close"></i>
                </button>
            </div>
        </div> -->

        <!-- 新建项目内容 -->
        <div class="new-project-content">
            <aside class="project-type-nav">
                <button
                    v-for="type in projectTypes"
                    :key="type.id"
                    class="project-type-item"
                    :class="{ active: activeProjectType === type.id }"
                    type="button"
                    @click="activeProjectType = type.id">
                    <span class="project-type-icon">{{ type.icon }}</span>
                    <span>{{ type.name }}</span>
                </button>
            </aside>

            <!-- 设置面板 -->
            <div class="new-project-panel">
                <div v-if="activeProjectType !== 'custom'" class="preset-section">
                    <div v-if="activeProjectType === 'startMenu'" class="start-menu-search">
                        <input
                            v-model="startMenuSearchQuery"
                            type="text"
                            class="start-menu-search-input"
                            :placeholder="t('newProject.searchStartMenuPlaceholder')" />
                    </div>

                    <div class="preset-grid">
                        <button
                            v-for="preset in visiblePresets"
                            :key="preset.id"
                            class="preset-item"
                            :disabled="isSaving"
                            :title="getPresetTitle(preset)"
                            type="button"
                            @click.stop="createPresetProject(preset)"
                            @contextmenu.prevent.stop="showStartMenuContextMenu($event, preset)">
                            <span class="preset-icon">
                                <img
                                    v-if="preset.icon && (preset.icon.startsWith('data:image/') || preset.icon.startsWith('http'))"
                                    :src="preset.icon"
                                    :alt="preset.name" />
                                <span v-else>{{ preset.icon }}</span>
                            </span>
                            <span class="preset-name">{{ preset.name }}</span>
                        </button>
                    </div>

                    <div v-if="activeProjectType === 'startMenu' && isLoadingStartMenu" class="preset-state">
                        {{ t('common.loading') }}
                    </div>

                    <div v-if="activeProjectType === 'startMenu' && !isLoadingStartMenu && visiblePresets.length === 0" class="preset-state">
                        {{ t('newProject.noStartMenuItems') }}
                    </div>
                </div>

                <!-- 统一的项目设置区域 -->
                <div v-if="activeProjectType === 'custom'" class="panel-section unified-section">
                    <div class="settings-group">
                        <!-- 第一行：项目名称和所属分组 -->
                        <div class="settings-row">
                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('newProject.projectName') }}</span>
                                    <span class="label-required">*</span>
                                </label>
                                <div class="input-wrapper">
                                    <input type="text"
                                           v-model="projectData.name"
                                           :placeholder="t('newProject.projectNamePlaceholder')"
                                           class="setting-input" />
                                    <div class="input-icon">📋</div>
                                </div>
                            </div>

                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('newProject.category') }}</span>
                                    <span class="label-required">*</span>
                                </label>
                                <div class="input-wrapper">
                                    <select v-model="projectData.category" class="setting-input">
                                        <option v-for="category in categories" :key="category.id" :value="category.id">
                                            {{ category.name }}
                                        </option>
                                    </select>
                                    <div class="input-icon">📁</div>
                                </div>
                            </div>
                        </div>

                        <!-- 第二行：目标类型和目标路径/网址 -->
                        <div class="settings-row">
                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('newProject.targetType') }}</span>
                                    <span class="label-required">*</span>
                                </label>
                                <div class="input-wrapper">
                                    <select v-model="projectData.targetType"
                                            @change="handleTargetTypeChange"
                                            class="setting-input">
                                        <option value="file">{{ t('newProject.targetTypeFile') }}</option>
                                        <option value="folder">{{ t('newProject.targetTypeFolder') }}</option>
                                        <option value="url">{{ t('newProject.targetTypeUrl') }}</option>
                                    </select>
                                    <div class="input-icon">🎯</div>
                                </div>
                            </div>

                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text" v-if="projectData.targetType !== 'url'">{{ t('newProject.targetPath') }}</span>
                                    <span class="label-text" v-else>{{ t('newProject.url') }}</span>
                                    <span class="label-required">*</span>
                                </label>
                                <div class="input-wrapper">
                                    <input type="text"
                                           v-model="projectData.targetPath"
                                           :placeholder="projectData.targetType === 'file' ? t('newProject.filePathPlaceholder') : (projectData.targetType === 'folder' ? t('newProject.folderPathPlaceholder') : t('newProject.urlPlaceholder'))"
                                           class="setting-input"
                                           @input="handlePathChange" />
                                    <button v-if="projectData.targetType !== 'url'" class="browse-button" @click="browseTarget" type="button">
                                        <i class="icon-folder"></i>
                                        {{ t('common.browse') }}
                                    </button>
                                    <div v-else class="input-icon">🌐</div>
                                </div>
                            </div>
                        </div>

                        <!-- 第三行：项目图标和启动参数 -->
                        <div class="settings-row">
                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('newProject.projectIcon') }}</span>
                                    <span class="label-optional">{{ t('common.optional') }}</span>
                                </label>
                                <div class="icon-section compact">
                                    <div class="icon-preview-container">
                                        <div class="icon-preview">
                                            <img
                                                v-if="projectData.icon && (projectData.icon.startsWith('data:image/') || projectData.icon.startsWith('http'))"
                                                :src="projectData.icon" :alt="projectData.name" class="preview-icon" />
                                            <div
                                                v-else-if="projectData.icon && !projectData.icon.startsWith('data:image/') && !projectData.icon.startsWith('http')"
                                                class="file-type-icon preview-icon" :class="'file-type-' + projectData.icon">
                                                {{ getFileTypeIcon(projectData.icon) }}
                                            </div>
                                            <div v-else class="default-icon preview-icon">{{ projectData.name.charAt(0) || 'P' }}</div>
                                        </div>
                                    </div>
                                    <div class="icon-actions">
                                        <button class="browse-button icon-button primary" @click="selectIcon" type="button">
                                            <i class="icon-image"></i>
                                            {{ t('common.select') }}
                                        </button>
                                        <button v-if="projectData.icon" class="browse-button icon-button danger" @click="clearIcon"
                                            type="button">
                                            <i class="icon-close"></i>
                                            {{ t('common.clear') }}
                                        </button>
                                    </div>
                                </div>
                            </div>

                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('newProject.launchArgs') }}</span>
                                    <span class="label-optional">{{ t('common.optional') }}</span>
                                </label>
                                <div class="input-wrapper">
                                    <input type="text"
                                           v-model="projectData.launchArgs"
                                           :placeholder="t('newProject.launchArgsPlaceholder')"
                                           class="setting-input" />
                                    <div class="input-icon">⚙️</div>
                                </div>
                            </div>
                        </div>

                        <!-- 第四行：管理员启动 -->
                        <div v-if="projectData.targetType !== 'url'" class="settings-row">
                            <div class="setting-item full-width">
                                <label class="checkbox-setting">
                                    <input type="checkbox" v-model="projectData.runAsAdmin" />
                                    <span>{{ t('newProject.runAsAdmin') }}</span>
                                </label>
                            </div>
                        </div>

                        <!-- 第五行：项目描述（全宽） -->
                        <div class="settings-row">
                            <div class="setting-item full-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('newProject.projectDescription') }}</span>
                                    <span class="label-optional">{{ t('common.optional') }}</span>
                                </label>
                                <div class="input-wrapper">
                                    <textarea v-model="projectData.description"
                                              :placeholder="t('newProject.projectDescriptionPlaceholder')"
                                              class="setting-input textarea"></textarea>
                                    <div class="input-icon">💬</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <Teleport to="body">
            <div
                v-if="startMenuContextMenu.visible"
                class="new-project-context-menu"
                :style="{ left: startMenuContextMenu.x + 'px', top: startMenuContextMenu.y + 'px' }"
                @click.stop>
                <button class="context-menu-item" type="button" @click="openStartMenuItemLocation">
                    {{ t('main.contextMenu.openFileLocation') }}
                </button>
            </div>
        </Teleport>

        <!-- 底栏 -->
        <div class="new-project-footer">
            <div class="footer-left">
                <label v-if="activeProjectType !== 'custom'" class="footer-category">
                    <span>{{ t('newProject.category') }}</span>
                    <select v-model="projectData.category" class="footer-select">
                        <option v-for="category in categories.filter(category => category.id !== 'all')" :key="category.id" :value="category.id">
                            {{ category.name }}
                        </option>
                    </select>
                </label>
                <!-- <div class="save-status" :class="{ saving: isSaving, saved: lastSaved }">
                    <div class="status-icon">
                        <div v-if="isSaving" class="loading-spinner"></div>
                        <div v-else-if="lastSaved" class="success-icon">✓</div>
                        <div v-else class="idle-icon"></div>
                    </div>
                    <span class="status-text">{{ saveStatusText }}</span>
                </div> -->
            </div>
            <div class="footer-right">
                <button @click="cancelProject" class="footer-button secondary">
                    <i class="icon-close"></i>
                    {{ t('common.cancel') }}
                </button>
                <button v-if="activeProjectType === 'custom'" @click="saveProject" class="footer-button primary" :disabled="!canSave">
                    <i class="icon-check"></i>
                    {{ t('newProject.createProject') }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, reactive, computed, watch, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import { alertDialog } from './utils/customDialog'

const { t } = useI18n()

const isSaving = ref(false)
const lastSaved = ref(false)
const activeProjectType = ref('custom')
const appTheme = ref('auto')
const isLoadingStartMenu = ref(false)
const startMenuItems = ref([])
const startMenuSearchQuery = ref('')
const builtInIconMap = ref({})
const startMenuContextMenu = reactive({
    visible: false,
    x: 0,
    y: 0,
    item: null
})
let startMenuIconLoadId = 0
let iconFetchTimer = null
let iconFetchRequestId = 0

const projectTypes = computed(() => [
    { id: 'custom', name: t('newProject.customProject'), icon: '○' },
    { id: 'builtIn', name: t('newProject.builtInItems'), icon: '▦' },
    { id: 'startMenu', name: t('newProject.startMenuItems'), icon: '☰' }
])

const builtInPresets = computed(() => [
    { id: 'computer', name: t('newProject.presets.computer'), icon: builtInIconMap.value.computer || '', targetPath: 'C:\\Windows\\explorer.exe', launchArgs: 'shell:MyComputerFolder', targetType: 'file', iconType: builtInIconMap.value.computer || '' },
    { id: 'network', name: t('newProject.presets.network'), icon: builtInIconMap.value.network || '', targetPath: 'C:\\Windows\\explorer.exe', launchArgs: 'shell:NetworkPlacesFolder', targetType: 'file', iconType: builtInIconMap.value.network || '' },
    { id: 'recycleBin', name: t('newProject.presets.recycleBin'), icon: builtInIconMap.value.recycleBin || '', targetPath: 'C:\\Windows\\explorer.exe', launchArgs: 'shell:RecycleBinFolder', targetType: 'file', iconType: builtInIconMap.value.recycleBin || '' },
    { id: 'volumeMixer', name: t('newProject.presets.volumeMixer'), icon: builtInIconMap.value.volumeMixer || '', targetPath: 'C:\\Windows\\System32\\sndvol.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.volumeMixer || '' },
    { id: 'calculator', name: t('newProject.presets.calculator'), icon: builtInIconMap.value.calculator || '', targetPath: 'C:\\Windows\\System32\\calc.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.calculator || '' },
    { id: 'registryEditor', name: t('newProject.presets.registryEditor'), icon: builtInIconMap.value.registryEditor || '', targetPath: 'C:\\Windows\\regedit.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.registryEditor || '' },
    { id: 'groupPolicy', name: t('newProject.presets.groupPolicy'), icon: builtInIconMap.value.groupPolicy || '', targetPath: 'C:\\Windows\\System32\\gpedit.msc', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.groupPolicy || '' },
    { id: 'controlPanel', name: t('newProject.presets.controlPanel'), icon: builtInIconMap.value.controlPanel || '', targetPath: 'C:\\Windows\\System32\\control.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.controlPanel || '' },
    { id: 'taskManager', name: t('newProject.presets.taskManager'), icon: builtInIconMap.value.taskManager || '', targetPath: 'C:\\Windows\\System32\\Taskmgr.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.taskManager || '' },
    { id: 'cmd', name: t('newProject.presets.cmd'), icon: builtInIconMap.value.cmd || '', targetPath: 'C:\\Windows\\System32\\cmd.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.cmd || '' },
    { id: 'powershell', name: t('newProject.presets.powershell'), icon: builtInIconMap.value.powershell || '', targetPath: 'C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.powershell || '' },
    { id: 'notepad', name: t('newProject.presets.notepad'), icon: builtInIconMap.value.notepad || '', targetPath: 'C:\\Windows\\System32\\notepad.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.notepad || '' },
    { id: 'paint', name: t('newProject.presets.paint'), icon: builtInIconMap.value.paint || '', targetPath: 'C:\\Windows\\System32\\mspaint.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.paint || '' },
    { id: 'wordpad', name: t('newProject.presets.wordpad'), icon: builtInIconMap.value.wordpad || '', targetPath: 'C:\\Program Files\\Windows NT\\Accessories\\wordpad.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.wordpad || '' },
    { id: 'services', name: t('newProject.presets.services'), icon: builtInIconMap.value.services || '', targetPath: 'C:\\Windows\\System32\\services.msc', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.services || '' },
    { id: 'deviceManager', name: t('newProject.presets.deviceManager'), icon: builtInIconMap.value.deviceManager || '', targetPath: 'C:\\Windows\\System32\\devmgmt.msc', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.deviceManager || '' },
    { id: 'diskManagement', name: t('newProject.presets.diskManagement'), icon: builtInIconMap.value.diskManagement || '', targetPath: 'C:\\Windows\\System32\\diskmgmt.msc', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.diskManagement || '' },
    { id: 'eventViewer', name: t('newProject.presets.eventViewer'), icon: builtInIconMap.value.eventViewer || '', targetPath: 'C:\\Windows\\System32\\eventvwr.msc', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.eventViewer || '' },
    { id: 'systemInfo', name: t('newProject.presets.systemInfo'), icon: builtInIconMap.value.systemInfo || '', targetPath: 'C:\\Windows\\System32\\msinfo32.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.systemInfo || '' },
    { id: 'remoteDesktop', name: t('newProject.presets.remoteDesktop'), icon: builtInIconMap.value.remoteDesktop || '', targetPath: 'C:\\Windows\\System32\\mstsc.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.remoteDesktop || '' },
    { id: 'snippingTool', name: t('newProject.presets.snippingTool'), icon: builtInIconMap.value.snippingTool || '', targetPath: 'C:\\Windows\\System32\\SnippingTool.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.snippingTool || '' },
    { id: 'characterMap', name: t('newProject.presets.characterMap'), icon: builtInIconMap.value.characterMap || '', targetPath: 'C:\\Windows\\System32\\charmap.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.characterMap || '' },
    { id: 'magnifier', name: t('newProject.presets.magnifier'), icon: builtInIconMap.value.magnifier || '', targetPath: 'C:\\Windows\\System32\\Magnify.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.magnifier || '' },
    { id: 'osk', name: t('newProject.presets.osk'), icon: builtInIconMap.value.osk || '', targetPath: 'C:\\Windows\\System32\\osk.exe', launchArgs: '', targetType: 'file', iconType: builtInIconMap.value.osk || '' },
    { id: 'runDialog', name: t('newProject.presets.runDialog'), icon: builtInIconMap.value.runDialog || '', targetPath: 'C:\\Windows\\explorer.exe', launchArgs: 'shell:::{2559a1f3-21d7-11d4-bdaf-00c04f60b9f0}', targetType: 'file', iconType: builtInIconMap.value.runDialog || '' },
    { id: 'downloads', name: t('newProject.presets.downloads'), icon: builtInIconMap.value.downloads || '', targetPath: 'C:\\Windows\\explorer.exe', launchArgs: 'shell:Downloads', targetType: 'file', iconType: builtInIconMap.value.downloads || '' },
    { id: 'documents', name: t('newProject.presets.documents'), icon: builtInIconMap.value.documents || '', targetPath: 'C:\\Windows\\explorer.exe', launchArgs: 'shell:Personal', targetType: 'file', iconType: builtInIconMap.value.documents || '' },
    { id: 'pictures', name: t('newProject.presets.pictures'), icon: builtInIconMap.value.pictures || '', targetPath: 'C:\\Windows\\explorer.exe', launchArgs: 'shell:My Pictures', targetType: 'file', iconType: builtInIconMap.value.pictures || '' },
    { id: 'startupFolder', name: t('newProject.presets.startupFolder'), icon: builtInIconMap.value.startupFolder || '', targetPath: 'C:\\Windows\\explorer.exe', launchArgs: 'shell:Startup', targetType: 'file', iconType: builtInIconMap.value.startupFolder || '' },
    { id: 'shutdown', name: t('newProject.presets.shutdown'), icon: builtInIconMap.value.shutdown || '', targetPath: 'C:\\Windows\\System32\\shutdown.exe', launchArgs: '/s /t 0', targetType: 'file', iconType: builtInIconMap.value.shutdown || '' },
    { id: 'restart', name: t('newProject.presets.restart'), icon: builtInIconMap.value.restart || '', targetPath: 'C:\\Windows\\System32\\shutdown.exe', launchArgs: '/r /t 0', targetType: 'file', iconType: builtInIconMap.value.restart || '' },
    { id: 'sleep', name: t('newProject.presets.sleep'), icon: builtInIconMap.value.sleep || '', targetPath: 'C:\\Windows\\System32\\rundll32.exe', launchArgs: 'powrprof.dll,SetSuspendState 0,1,0', targetType: 'file', iconType: builtInIconMap.value.sleep || '' }
])

const visiblePresets = computed(() => {
    if (activeProjectType.value === 'builtIn') {
        return builtInPresets.value
    }

    const query = startMenuSearchQuery.value.trim().toLowerCase()
    if (!query) {
        return startMenuItems.value
    }

    return startMenuItems.value.filter(item => {
        return [item.name, item.targetPath]
            .some(value => (value || '').toLowerCase().includes(query))
    })
})

const resolvedTheme = computed(() => {
    if (appTheme.value === 'auto') {
        return window.matchMedia?.('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    }

    return appTheme.value === 'dark' ? 'dark' : 'light'
})

const newProjectAppClasses = computed(() => ({
    'theme-dark': resolvedTheme.value === 'dark',
    'theme-light': resolvedTheme.value === 'light'
}))

const applyRuntimeTheme = () => {
    const body = document.body
    body.classList.toggle('lora-theme-dark', resolvedTheme.value === 'dark')
    body.classList.toggle('lora-theme-light', resolvedTheme.value === 'light')
}

// 分类数据
const categories = ref([])

// 项目数据
const projectData = reactive({
    name: '',
    category: '',
    description: '',
    targetType: 'file',
    targetPath: '',
    launchArgs: '',
    runAsAdmin: false,
    icon: ''
})

// 保存状态文本
const saveStatusText = computed(() => {
    if (isSaving.value) return t('newProject.creating')
    if (lastSaved.value) return t('newProject.created')
    return ''
})

// 是否可以保存
const canSave = computed(() => {
    return projectData.name.trim() && projectData.category && projectData.targetPath.trim()
})

const isShortcutPath = (value) => /\.(lnk|url)$/i.test(value || '')


// 监听项目数据变化
watch(projectData, () => {
    lastSaved.value = false
}, { deep: true })

watch(resolvedTheme, applyRuntimeTheme)

watch(activeProjectType, async (type) => {
    hideStartMenuContextMenu()

    if (type === 'builtIn') {
        await loadBuiltInIcons()
    }

    if (type === 'startMenu' && startMenuItems.value.length === 0) {
        await loadStartMenuItems()
    }
})

// 方法
const handleTargetTypeChange = () => {
    // 当目标类型改变时，清空路径
    projectData.targetPath = ''
    clearPendingIconFetch()
}

const browseTarget = async () => {
    try {
        let selectedPath = ''
        
        if (projectData.targetType === 'file') {
            // 选择文件
            const filters = [
                [t('newProject.allFiles'), ['*']],
                [t('newProject.executableFiles'), ['exe', 'bat', 'cmd', 'msi']],
                [t('newProject.scriptFiles'), ['ps1', 'vbs', 'js', 'py']],
                [t('newProject.shortcutFiles'), ['lnk', 'url']]
            ]
            selectedPath = await invoke('open_file_dialog', {
                title: t('newProject.selectTargetFile'),
                filters: filters
            })
        } else if (projectData.targetType === 'folder') {
            // 选择文件夹
            selectedPath = await invoke('open_folder_dialog', {
                title: t('newProject.selectTargetFolder')
            })
        }

        if (selectedPath) {
            projectData.targetPath = selectedPath
            // 自动检测目标类型
            await detectTargetType()
            // 自动获取图标
            await fetchTargetIcon()
        }
    } catch (error) {
        console.error('浏览文件失败:', error)
        if (error !== t('newProject.userCancelled')) {
            await alertDialog(t('newProject.browseFileError') + ': ' + error, { type: 'error' })
        }
    }
}

// 自动检测目标类型
const detectTargetType = async () => {
    if (!projectData.targetPath.trim()) {
        return
    }

    try {
        const targetType = await invoke('detect_target_type', {
            targetPath: projectData.targetPath
        })
        // 这里不更新 projectData.targetType，因为用户可能已经选择了不同的类型
        console.log('检测到的目标类型:', targetType)
    } catch (error) {
        console.error('检测目标类型失败:', error)
    }
}

const clearPendingIconFetch = () => {
    if (iconFetchTimer) {
        clearTimeout(iconFetchTimer)
        iconFetchTimer = null
    }
}

const isValidHttpUrl = (value) => {
    try {
        const url = new URL(value)
        return url.protocol === 'http:' || url.protocol === 'https:'
    } catch {
        return false
    }
}

// 处理路径变化，自动获取图标
const handlePathChange = () => {
    clearPendingIconFetch()

    const targetPath = projectData.targetPath.trim()
    if (!targetPath) {
        return
    }

    if (projectData.targetType === 'url' && !isValidHttpUrl(targetPath)) {
        return
    }

    const delay = projectData.targetType === 'url' ? 800 : 300
    iconFetchTimer = setTimeout(() => {
        iconFetchTimer = null
        fetchTargetIcon()
    }, delay)
}

const fetchTargetIcon = async () => {
    if (!projectData.targetPath.trim()) {
        return
    }

    // 对文件或 URL 类型自动获取图标
    if (projectData.targetType !== 'file' && projectData.targetType !== 'url') {
        return
    }

    if (projectData.targetType === 'url' && !isValidHttpUrl(projectData.targetPath.trim())) {
        return
    }

    const requestId = ++iconFetchRequestId
    const targetPath = projectData.targetPath.trim()

    try {
        // 尝试获取应用图标
        console.log('尝试获取应用图标:', targetPath)
        const iconBase64 = await invoke('get_app_icon', {
            filePath: targetPath
        })
        
        if (requestId === iconFetchRequestId && iconBase64 && iconBase64.startsWith('data:image/')) {
            // 成功获取到图标，更新图标字段
            projectData.icon = iconBase64
            console.log('成功获取应用图标')
        } else {
            console.log('无法获取应用图标，将使用默认图标')
        }
    } catch (error) {
        console.log('获取应用图标失败:', error)
        // 获取图标失败时不做处理，保持默认图标
    }
}

const selectIcon = async () => {
    try {
        const filters = [
            [t('newProject.imageFiles'), ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'ico', 'svg']],
            [t('newProject.iconFiles'), ['ico', 'png']],
            [t('newProject.allFiles'), ['*']]
        ]
        const selectedPath = await invoke('open_file_dialog', {
            title: t('newProject.selectIconFile'),
            filters: filters
        })

        if (selectedPath) {
            // 尝试将图片转换为base64
            try {
                const iconBase64 = await invoke('get_app_icon', { filePath: selectedPath })
                if (iconBase64 && iconBase64.startsWith('data:image/')) {
                    projectData.icon = iconBase64
                } else {
                    // 如果无法转换为base64，直接使用文件路径
                    projectData.icon = selectedPath
                }
            } catch (iconError) {
                // 如果获取图标失败，直接使用文件路径
                projectData.icon = selectedPath
            }
        }
    } catch (error) {
        console.error('选择图标失败:', error)
        if (error !== t('newProject.userCancelled')) {
            await alertDialog(t('newProject.selectIconError') + ': ' + error, { type: 'error' })
        }
    }
}

const clearIcon = () => {
    projectData.icon = ''
}

const getFileTypeIcon = (fileType) => {
    const iconMap = {
        'exe': '🖥️',
        'installer': '📦',
        'shortcut': '🔗',
        'text': '📄',
        'pdf': '📕',
        'word': '📘',
        'excel': '📗',
        'powerpoint': '📙',
        'archive': '🗜️',
        'image': '🖼️',
        'audio': '🎵',
        'video': '🎬',
        'web': '🌐',
        'javascript': '📜',
        'python': '🐍',
        'java': '☕',
        'code': '💻',
        'json': '📋',
        'xml': '📰',
        'css': '🎨'
    }
    return iconMap[fileType] || '📁'
}

const cancelProject = async () => {
    try {
        await invoke('close_new_project_window')
    } catch (error) {
        console.error('关闭新建项目窗口失败:', error)
    }
}

const getPresetTitle = (preset) => {
    if (activeProjectType.value !== 'startMenu') {
        return preset.name
    }

    return `${preset.name}\n${t('common.path')}: ${preset.targetPath || ''}`
}

const showStartMenuContextMenu = (event, item) => {
    if (activeProjectType.value !== 'startMenu') {
        return
    }

    startMenuContextMenu.visible = true
    startMenuContextMenu.x = event.clientX
    startMenuContextMenu.y = event.clientY
    startMenuContextMenu.item = item
}

const hideStartMenuContextMenu = () => {
    startMenuContextMenu.visible = false
    startMenuContextMenu.item = null
}

const openStartMenuItemLocation = async () => {
    const item = startMenuContextMenu.item
    hideStartMenuContextMenu()

    if (!item?.targetPath) {
        return
    }

    try {
        await invoke('open_file_location', { filePath: item.targetPath })
    } catch (error) {
        console.error('打开开始菜单项目位置失败:', error)
        await alertDialog(t('main.alert.openFileLocationFailed', { error: String(error) }), { type: 'error' })
    }
}

const createProject = async (data) => {
    if (!data.name?.trim() || !data.category || !data.targetPath?.trim()) {
        await alertDialog(t('newProject.fillRequiredFields'), { type: 'warning' })
        return
    }

    isSaving.value = true
    try {
        // 创建新的应用项
        const newApp = {
            id: Date.now(),
            name: data.name.trim(),
            category: data.category,
            category_ids: [data.category],
            icon: data.icon || '',
            path: data.targetPath,
            target_path: data.targetPath,
            is_shortcut: isShortcutPath(data.targetPath),
            launch_args: data.launchArgs || '',
            target_type: data.targetType || 'file',
            run_as_admin: data.targetType !== 'url' && !!data.runAsAdmin,
            usage_count: 0,
            last_launched_at: null
        }

        // 调用后端保存应用数据
        await invoke('add_new_app', { app: newApp })
        
        // 通知主窗口刷新数据
        await invoke('notify_main_window_refresh')

        lastSaved.value = true
        setTimeout(() => {
            lastSaved.value = false
        }, 2000)

        // 延迟一秒后关闭窗口
        setTimeout(async () => {
            try {
                await invoke('close_new_project_window')
            } catch (error) {
                console.error('关闭新建项目窗口失败:', error)
            }
        }, 1000)
    } catch (error) {
        console.error('创建项目失败:', error)
        await alertDialog(t('newProject.createProjectError') + ': ' + error, { type: 'error' })
    } finally {
        isSaving.value = false
    }
}

const saveProject = async () => {
    if (!canSave.value) {
        await alertDialog(t('newProject.fillRequiredFields'), { type: 'warning' })
        return
    }

    await createProject({ ...projectData })
}

const createPresetProject = async (preset) => {
    let icon = preset.iconType || preset.icon || ''
    if (!icon && preset.targetPath) {
        try {
            icon = await invoke('get_shell_file_icon', { filePath: preset.targetPath })
        } catch (error) {
            console.error('获取 Shell 图标失败:', error)
        }
    }

    await createProject({
        name: preset.name,
        category: projectData.category,
        icon,
        targetPath: preset.targetPath || preset.path,
        launchArgs: preset.launchArgs || '',
        targetType: preset.targetType || 'file',
        runAsAdmin: false
    })
}

// 加载分类数据
const loadCategories = async () => {
    try {
        const storage = await invoke('load_app_data')
        // 转换后端的 is_default 为前端使用的 isDefault
        const categoriesFromBackend = storage.categories || []
        const convertedCategories = categoriesFromBackend.map(category => ({
            id: category.id,
            name: category.name,
            icon: category.icon,
            isDefault: category.is_default
        }))

        // 确保"全部应用"分组始终存在
        if (!convertedCategories.some(cat => cat.id === 'all')) {
            convertedCategories.unshift({ id: 'all', name: t('main.sidebar.allApps'), icon: 'icon-apps', isDefault: true })
        }

        categories.value = convertedCategories

        // 默认选择第一个非"全部应用"的分组
        const defaultCategory = convertedCategories.find(cat => !cat.isDefault)
        if (defaultCategory) {
            projectData.category = defaultCategory.id
        }
    } catch (error) {
        console.error('加载分类数据失败:', error)
    }
}

const loadStartMenuItems = async () => {
    const loadId = ++startMenuIconLoadId
    isLoadingStartMenu.value = true
    try {
        const items = await invoke('list_start_menu_items')
        startMenuItems.value = (items || []).map(item => ({
            id: item.id,
            name: item.name,
            icon: '',
            targetPath: item.path,
            launchArgs: item.launch_args || '',
            targetType: item.target_type || 'file',
            iconType: ''
        }))
        loadStartMenuIcons(loadId)
    } catch (error) {
        console.error('加载开始菜单项目失败:', error)
        startMenuItems.value = []
    } finally {
        isLoadingStartMenu.value = false
    }
}

const loadBuiltInIcons = async () => {
    const missingItems = builtInPresets.value.filter(item => !builtInIconMap.value[item.id])
    if (missingItems.length === 0) {
        return
    }

    const icons = await Promise.allSettled(missingItems.map(item => invoke('get_shell_file_icon', {
        filePath: item.targetPath
    })))

    const nextIconMap = { ...builtInIconMap.value }
    icons.forEach((result, index) => {
        if (result.status === 'fulfilled' && result.value) {
            nextIconMap[missingItems[index].id] = result.value
        }
    })
    builtInIconMap.value = nextIconMap
}

const loadStartMenuIcons = async (loadId) => {
    const batchSize = 12

    for (let index = 0; index < startMenuItems.value.length; index += batchSize) {
        if (loadId !== startMenuIconLoadId) {
            return
        }

        const batch = startMenuItems.value.slice(index, index + batchSize)
        const icons = await Promise.allSettled(batch.map(item => invoke('get_shell_file_icon', {
            filePath: item.targetPath
        })))

        if (loadId !== startMenuIconLoadId) {
            return
        }

        icons.forEach((result, batchIndex) => {
            if (result.status !== 'fulfilled' || !result.value) {
                return
            }

            const item = batch[batchIndex]
            const itemIndex = startMenuItems.value.findIndex(current => current.id === item.id)
            if (itemIndex !== -1) {
                startMenuItems.value[itemIndex] = {
                    ...startMenuItems.value[itemIndex],
                    icon: result.value,
                    iconType: result.value
                }
            }
        })
    }
}

const loadTheme = async () => {
    try {
        const settings = await invoke('load_app_settings')
        appTheme.value = settings.theme || 'auto'
        applyRuntimeTheme()
    } catch (error) {
        console.error('加载主题设置失败:', error)
        applyRuntimeTheme()
    }
}

// 初始化
onMounted(async () => {
    await loadTheme()
    await loadCategories()
    await loadBuiltInIcons()
})

onBeforeUnmount(() => {
    clearPendingIconFetch()
    iconFetchRequestId++
    startMenuIconLoadId++
})
</script>

<style scoped>
/* 全局样式 */
.new-project-app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    overflow: hidden;
    color: #2c3e50;
    color-scheme: light;
}

.new-project-app.theme-dark {
    background: linear-gradient(135deg, #111827 0%, #1f2937 100%);
    color: #e5e7eb;
    color-scheme: dark;
}

/* 自定义标题栏 */
.titlebar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 36px;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    padding: 0 16px;
    user-select: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
    z-index: 1000;
}

.titlebar-left {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: move;
}

.app-icon {
    font-size: 18px;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.app-title {
    font-size: 14px;
    font-weight: 600;
    color: #2c3e50;
}

.titlebar-right {
    display: flex;
    align-items: center;
}

.titlebar-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    background: transparent;
    border: none;
    color: #7f8c8d;
    cursor: pointer;
    border-radius: 6px;
    transition: all 0.2s ease;
}

.titlebar-button:hover {
    background: rgba(231, 76, 60, 0.1);
    color: #e74c3c;
}

/* 主内容区域 */
.new-project-content {
    flex: 1;
    display: flex;
    flex-direction: row;
    min-height: 0;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: rgba(52, 73, 94, 0.28) transparent;
}

.new-project-content::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

.new-project-content::-webkit-scrollbar-track {
    background: transparent;
}

.new-project-content::-webkit-scrollbar-thumb {
    border: 2px solid transparent;
    border-radius: 999px;
    background: rgba(52, 73, 94, 0.24);
    background-clip: padding-box;
}

.new-project-content::-webkit-scrollbar-thumb:hover {
    background: rgba(52, 73, 94, 0.38);
    background-clip: padding-box;
}

.project-type-nav {
    width: 150px;
    flex: 0 0 150px;
    padding: 0;
    background: rgba(222, 222, 222, 0.95);
    border-right: 1px solid rgba(0, 0, 0, 0.08);
}

.project-type-item {
    width: 100%;
    height: 38px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 18px;
    border: 0;
    background: transparent;
    color: #34495e;
    text-align: left;
    font-size: 14px;
    cursor: pointer;
    transition: background 0.2s ease, color 0.2s ease;
}

.project-type-item:hover {
    background: rgba(255, 255, 255, 0.45);
}

.project-type-item.active {
    background: rgba(255, 255, 255, 0.65);
    color: #1f2937;
}

.project-type-icon {
    width: 18px;
    text-align: center;
    color: #5b6ee1;
}

/* 设置面板 */
.new-project-panel {
    flex: 1;
    overflow-y: auto;
    background: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(5px);
    padding: 16px 0;
    scrollbar-width: thin;
    scrollbar-color: rgba(52, 73, 94, 0.28) transparent;
}

.new-project-panel::-webkit-scrollbar,
.preset-section::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

.new-project-panel::-webkit-scrollbar-track,
.preset-section::-webkit-scrollbar-track {
    background: transparent;
}

.new-project-panel::-webkit-scrollbar-thumb,
.preset-section::-webkit-scrollbar-thumb {
    border: 2px solid transparent;
    border-radius: 999px;
    background: rgba(52, 73, 94, 0.24);
    background-clip: padding-box;
}

.new-project-panel::-webkit-scrollbar-thumb:hover,
.preset-section::-webkit-scrollbar-thumb:hover {
    background: rgba(52, 73, 94, 0.38);
    background-clip: padding-box;
}

.preset-section {
    padding: 0;
}

.start-menu-search {
    padding: 0 10px 10px;
}

.start-menu-search-input {
    width: 100%;
    box-sizing: border-box;
    height: 34px;
    padding: 0 12px;
    border: 1px solid rgba(0, 0, 0, 0.12);
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.9);
    color: #2c3e50;
    font-size: 14px;
    outline: none;
}

.start-menu-search-input:focus {
    border-color: #3498db;
    box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.12);
}

.preset-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(145px, 1fr));
    gap: 2px 10px;
    align-content: start;
    padding: 0 10px 16px;
}

.preset-item {
    min-width: 0;
    height: 46px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border: 1px solid transparent;
    border-radius: 2px;
    background: transparent;
    color: #2c3e50;
    text-align: left;
    font-size: 14px;
    cursor: pointer;
    transition: background 0.2s ease, border-color 0.2s ease, transform 0.2s ease;
}

.preset-item:hover {
    background: rgba(255, 255, 255, 0.82);
    border-color: rgba(0, 0, 0, 0.12);
}

.preset-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 0 0 32px;
    font-size: 26px;
}

.preset-icon img {
    width: 32px;
    height: 32px;
    object-fit: contain;
}

.preset-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.start-menu-actions {
    display: flex;
    justify-content: flex-start;
    margin-top: 14px;
}

.preset-state {
    padding: 24px;
    color: #7f8c8d;
    font-size: 14px;
}

.new-project-context-menu {
    position: fixed;
    z-index: 3000;
    min-width: 150px;
    padding: 5px 0;
    border: 1px solid rgba(0, 0, 0, 0.12);
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.96);
    box-shadow: 0 10px 28px rgba(15, 23, 42, 0.18);
}

.new-project-context-menu .context-menu-item {
    width: 100%;
    height: 30px;
    display: flex;
    align-items: center;
    padding: 0 12px;
    border: 0;
    background: transparent;
    color: #2c3e50;
    font-size: 13px;
    text-align: left;
    cursor: pointer;
}

.new-project-context-menu .context-menu-item:hover {
    background: rgba(52, 152, 219, 0.12);
}

/* 面板区域 */
.panel-section {
    margin-bottom: 24px;
    padding: 0 24px;
    animation: fadeInUp 0.3s ease-out;
}

.panel-section:last-child {
    margin-bottom: 0;
}

/* 统一区域样式 */
.unified-section {
    margin-bottom: 0;
}

/* 设置行布局 */
.settings-row {
    display: flex;
    gap: 20px;
    margin-bottom: 20px;
}

.settings-row:last-child {
    margin-bottom: 0;
}

/* 半宽设置项 */
.setting-item.half-width {
    flex: 1;
    margin-bottom: 0;
}

/* 全宽设置项 */
.setting-item.full-width {
    width: 100%;
    margin-bottom: 0;
}

/* 紧凑图标区域 */
.icon-section.compact {
    display: flex;
    align-items: center;
    gap: 16px;
}

.icon-section.compact .icon-preview-container {
    margin-bottom: 0;
}

.icon-section.compact .icon-preview {
    width: 48px;
    height: 48px;
}

.icon-section.compact .preview-icon {
    width: 36px;
    height: 36px;
}

.icon-section.compact .preview-icon.file-type-icon {
    font-size: 24px;
}

.icon-section.compact .preview-icon.default-icon {
    font-size: 18px;
}

.icon-section.compact .icon-preview-label {
    display: none;
}

.icon-section.compact .icon-actions {
    flex-direction: row;
    gap: 8px;
}

.section-header {
    margin-bottom: 20px;
    text-align: center;
    padding-bottom: 16px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.section-icon {
    font-size: 28px;
    margin-bottom: 8px;
}

.section-title {
    font-size: 18px;
    font-weight: 600;
    color: #2c3e50;
    margin-bottom: 4px;
}

.section-description {
    font-size: 13px;
    color: #7f8c8d;
    line-height: 1.4;
}

/* 设置组 */
.settings-group {
    background: rgba(255, 255, 255, 0.8);
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 20px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.8);
}

.setting-item {
    margin-bottom: 20px;
}

.setting-item:last-child {
    margin-bottom: 0;
}

.setting-label {
    display: flex;
    align-items: center;
    margin-bottom: 8px;
    font-weight: 500;
    font-size: 14px;
    color: #2c3e50;
}

.label-text {
    margin-right: 6px;
}

.label-required {
    color: #e74c3c;
    font-size: 12px;
}

.label-optional {
    color: #95a5a6;
    font-size: 12px;
    font-style: italic;
}

.input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
}

.checkbox-setting {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    color: #2c3e50;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
}

.checkbox-setting input {
    appearance: none;
    display: inline-grid;
    place-content: center;
    width: 16px;
    height: 16px;
    margin: 0;
    border: 1px solid rgba(0, 0, 0, 0.18);
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.9);
    cursor: pointer;
    transition: background-color 0.18s ease, border-color 0.18s ease, box-shadow 0.18s ease;
}

.checkbox-setting input:checked {
    border-color: #3498db;
    background: #3498db;
    background-image: url("data:image/svg+xml,%3Csvg width='12' height='12' viewBox='0 0 12 12' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M2.3 6.15L4.8 8.65L9.7 3.35' stroke='white' stroke-width='1.8' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
    background-position: center;
    background-repeat: no-repeat;
}

.checkbox-setting input:focus-visible {
    outline: none;
    box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.14);
}

.setting-input {
    flex: 1;
    padding: 12px 16px 12px 40px;
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.9);
    color: #2c3e50;
    font-size: 14px;
    transition: all 0.3s ease;
    outline: none;
}

.setting-input:focus {
    border-color: #3498db;
    box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}

.setting-input.textarea {
    min-height: 80px;
    resize: vertical;
    padding-top: 12px;
    padding-bottom: 12px;
}

.input-icon {
    position: absolute;
    left: 12px;
    font-size: 16px;
    color: #95a5a6;
    pointer-events: none;
}

.browse-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 10px 16px;
    border: none;
    border-radius: 8px;
    background: #3498db;
    color: white;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.3s ease;
    margin-left: 8px;
}

.browse-button:hover {
    background: #2980b9;
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(52, 152, 219, 0.2);
}

.browse-button.primary {
    background: #3498db;
}

.browse-button.danger {
    background: #e74c3c;
}

.browse-button.danger:hover {
    background: #c0392b;
    box-shadow: 0 4px 8px rgba(231, 76, 60, 0.2);
}

/* 图标选择 */
.icon-section {
    display: flex;
    align-items: flex-start;
    gap: 20px;
}

.icon-preview-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
}

.icon-preview {
    width: 64px;
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px dashed rgba(0, 0, 0, 0.1);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.9);
    transition: all 0.3s ease;
}

.icon-preview:hover {
    border-color: #3498db;
    transform: scale(1.05);
}

.preview-icon {
    width: 48px;
    height: 48px;
}

.preview-icon.file-type-icon {
    font-size: 32px;
}

.preview-icon.default-icon {
    background: linear-gradient(135deg, #3498db, #2980b9);
    color: white;
    border-radius: 8px;
    font-size: 24px;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 4px 8px rgba(52, 152, 219, 0.2);
}

.icon-preview-label {
    font-size: 12px;
    color: #7f8c8d;
    font-weight: 500;
}

.icon-actions {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.icon-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 10px 16px;
    border: none;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
}

.icon-button:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

/* 底栏 */
.new-project-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 24px;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(10px);
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.05);
}

.save-status {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: #7f8c8d;
    transition: all 0.3s ease;
}

.save-status.saving {
    color: #3498db;
}

.save-status.saved {
    color: #27ae60;
}

.status-icon {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.loading-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(52, 152, 219, 0.2);
    border-top: 2px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

.success-icon {
    color: #27ae60;
    font-weight: bold;
}

.idle-icon {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #bdc3c7;
}

.footer-right {
    display: flex;
    gap: 12px;
}

.footer-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 10px 20px;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
}

.footer-button:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.footer-button.secondary {
    background: rgba(255, 255, 255, 0.8);
    color: #7f8c8d;
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.footer-button.secondary:hover {
    background: rgba(255, 255, 255, 0.9);
    color: #2c3e50;
}

.footer-button.primary {
    background: linear-gradient(135deg, #3498db, #2980b9);
    color: white;
    box-shadow: 0 4px 8px rgba(52, 152, 219, 0.2);
}

.footer-button.primary:hover {
    background: linear-gradient(135deg, #2980b9, #2573a7);
    box-shadow: 0 6px 12px rgba(52, 152, 219, 0.3);
}

.footer-button:disabled {
    background: #bdc3c7;
    color: #7f8c8d;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
}

.footer-button:disabled:hover {
    background: #bdc3c7;
    transform: none;
    box-shadow: none;
}

.footer-left {
    min-width: 0;
}

.footer-category {
    display: flex;
    align-items: center;
    gap: 8px;
    color: #52616f;
    font-size: 13px;
}

.footer-select {
    min-width: 150px;
    padding: 7px 28px 7px 10px;
    border: 1px solid rgba(0, 0, 0, 0.12);
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.9);
    color: #2c3e50;
    outline: none;
}

/* 深色模式 */
.new-project-app.theme-dark .project-type-nav {
    background: rgba(31, 41, 55, 0.96);
    border-right-color: rgba(255, 255, 255, 0.08);
}

.new-project-app.theme-dark .project-type-item {
    color: #cbd5e1;
}

.new-project-app.theme-dark .project-type-item:hover {
    background: rgba(255, 255, 255, 0.07);
}

.new-project-app.theme-dark .project-type-item.active {
    background: rgba(255, 255, 255, 0.12);
    color: #f8fafc;
}

.new-project-app.theme-dark .new-project-panel {
    background: rgba(17, 24, 39, 0.76);
    scrollbar-color: rgba(148, 163, 184, 0.42) transparent;
}

.new-project-app.theme-dark .new-project-content {
    scrollbar-color: rgba(148, 163, 184, 0.42) transparent;
}

.new-project-app.theme-dark .new-project-content::-webkit-scrollbar-thumb,
.new-project-app.theme-dark .new-project-panel::-webkit-scrollbar-thumb,
.new-project-app.theme-dark .preset-section::-webkit-scrollbar-thumb {
    background: rgba(148, 163, 184, 0.34);
    background-clip: padding-box;
}

.new-project-app.theme-dark .new-project-content::-webkit-scrollbar-thumb:hover,
.new-project-app.theme-dark .new-project-panel::-webkit-scrollbar-thumb:hover,
.new-project-app.theme-dark .preset-section::-webkit-scrollbar-thumb:hover {
    background: rgba(148, 163, 184, 0.5);
    background-clip: padding-box;
}

.new-project-app.theme-dark .preset-item {
    color: #e5e7eb;
}

.new-project-app.theme-dark .start-menu-search-input {
    background: rgba(15, 23, 42, 0.88);
    border-color: rgba(148, 163, 184, 0.22);
    color: #e5e7eb;
}

.new-project-app.theme-dark .start-menu-search-input::placeholder {
    color: #64748b;
}

.new-project-app.theme-dark .start-menu-search-input:focus {
    border-color: #60a5fa;
    box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.16);
}

.new-project-app.theme-dark .preset-item:hover {
    background: rgba(31, 41, 55, 0.9);
    border-color: rgba(96, 165, 250, 0.35);
}

.new-project-app.theme-dark .preset-state,
.new-project-app.theme-dark .footer-category {
    color: #94a3b8;
}

.new-project-app.theme-dark .footer-select {
    background: rgba(15, 23, 42, 0.88);
    border-color: rgba(148, 163, 184, 0.22);
    color: #e5e7eb;
}

.new-project-app.theme-dark .settings-group,
.new-project-app.theme-dark .new-project-footer {
    background: rgba(31, 41, 55, 0.88);
    border-color: rgba(255, 255, 255, 0.08);
    box-shadow: 0 4px 14px rgba(0, 0, 0, 0.22);
}

.new-project-app.theme-dark .setting-label,
.new-project-app.theme-dark .checkbox-setting,
.new-project-app.theme-dark .section-title {
    color: #f8fafc;
}

.new-project-app.theme-dark .label-optional,
.new-project-app.theme-dark .input-icon,
.new-project-app.theme-dark .save-status {
    color: #94a3b8;
}

.new-project-app.theme-dark .setting-input {
    background: rgba(15, 23, 42, 0.88);
    border-color: rgba(148, 163, 184, 0.22);
    color: #e5e7eb;
}

.new-project-app.theme-dark .checkbox-setting input {
    background: rgba(15, 23, 42, 0.88);
    border-color: rgba(148, 163, 184, 0.28);
}

.new-project-app.theme-dark .checkbox-setting input:checked {
    background: #60a5fa;
    border-color: #60a5fa;
}

.new-project-app.theme-dark .setting-input::placeholder {
    color: #64748b;
}

.new-project-app.theme-dark .setting-input:focus {
    border-color: #60a5fa;
    box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.16);
}

.new-project-app.theme-dark .icon-preview {
    background: rgba(15, 23, 42, 0.88);
    border-color: rgba(148, 163, 184, 0.28);
}

.new-project-app.theme-dark .footer-button.secondary {
    background: rgba(15, 23, 42, 0.84);
    color: #cbd5e1;
    border-color: rgba(148, 163, 184, 0.22);
}

.new-project-app.theme-dark .footer-button.secondary:hover {
    background: rgba(30, 41, 59, 0.95);
    color: #f8fafc;
}

.new-project-app.theme-dark .footer-button:disabled {
    background: #334155;
    color: #94a3b8;
}

:global(body.lora-theme-dark) .new-project-context-menu {
    background: rgba(31, 41, 55, 0.98);
    border-color: rgba(148, 163, 184, 0.22);
    box-shadow: 0 10px 28px rgba(0, 0, 0, 0.36);
}

:global(body.lora-theme-dark) .new-project-context-menu .context-menu-item {
    color: #e5e7eb;
}

:global(body.lora-theme-dark) .new-project-context-menu .context-menu-item:hover {
    background: rgba(96, 165, 250, 0.16);
}

/* 动画 */
@keyframes fadeInUp {
    from {
        opacity: 0;
        transform: translateY(10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }
    100% {
        transform: rotate(360deg);
    }
}

/* 响应式设计 */
@media (max-width: 768px) {
    .new-project-content {
        flex-direction: column;
    }

    .project-type-nav {
        width: 100%;
        flex: 0 0 auto;
        display: flex;
        border-right: 0;
        border-bottom: 1px solid rgba(0, 0, 0, 0.08);
    }

    .project-type-item {
        justify-content: center;
        padding: 0 10px;
    }

    .panel-section {
        padding: 0 16px;
    }

    .preset-section {
        padding: 0 16px 16px;
    }
    
    .settings-group {
        padding: 16px;
    }
    
    .settings-row {
        flex-direction: column;
        gap: 16px;
    }
    
    .icon-section {
        flex-direction: column;
        align-items: center;
        gap: 16px;
    }
    
    .icon-section.compact {
        flex-direction: column;
        align-items: center;
        gap: 16px;
    }
    
    .icon-actions {
        flex-direction: row;
        justify-content: center;
        width: 100%;
    }
    
    .icon-section.compact .icon-actions {
        flex-direction: row;
        justify-content: center;
        width: 100%;
    }
}
</style>
