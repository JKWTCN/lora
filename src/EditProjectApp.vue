<template>
    <div class="edit-project-app">
        <!-- 自定义标题栏 -->
        <!-- <div class="titlebar">
            <div class="titlebar-left" data-tauri-drag-region>
                <div class="app-icon">✏️</div>
                <span class="app-title" data-tauri-drag-region>编辑项目</span>
            </div>
            <div class="titlebar-right">
                <button class="titlebar-button" @click="cancelEdit" title="关闭">
                    <i class="icon-close"></i>
                </button>
            </div>
        </div> -->

        <!-- 加载状态指示器 -->
        <div v-if="isLoading" class="loading-overlay">
            <div class="loading-spinner-large"></div>
            <div class="loading-text">{{ loadingText }}</div>
        </div>

        <!-- 错误显示 -->
        <div v-if="hasError" class="error-overlay">
            <div class="error-container">
                <div class="error-icon">⚠️</div>
                <div class="error-title">{{ t('editProject.loadFailed') }}</div>
                <div class="error-message">{{ errorMessage }}</div>
                <div class="error-actions">
                    <button @click="retryLoading" class="retry-button">{{ t('common.retry') }}</button>
                    <button @click="cancelEdit" class="cancel-button">{{ t('common.close') }}</button>
                </div>
            </div>
        </div>

        <!-- 编辑项目内容 -->
        <div v-else class="edit-project-content">
            <!-- 设置面板 -->
            <div class="edit-project-panel">
                <!-- 统一的项目设置区域 -->
                <div class="panel-section unified-section">
                    <div class="settings-group">
                        <!-- 第一行：项目名称和所属分组 -->
                        <div class="settings-row">
                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('editProject.form.projectName') }}</span>
                                    <span class="label-required">*</span>
                                </label>
                                <div class="input-wrapper">
                                    <input type="text"
                                           v-model="projectData.name"
                                           :placeholder="t('editProject.form.projectNamePlaceholder')"
                                           class="setting-input" />
                                    <div class="input-icon">📋</div>
                                </div>
                            </div>

                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('editProject.form.category') }}</span>
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
                                    <span class="label-text">{{ t('editProject.form.targetType') }}</span>
                                    <span class="label-required">*</span>
                                </label>
                                <div class="input-wrapper">
                                    <select v-model="projectData.targetType"
                                            @change="handleTargetTypeChange"
                                            class="setting-input">
                                        <option value="file">{{ t('editProject.form.targetTypeFile') }}</option>
                                        <option value="folder">{{ t('editProject.form.targetTypeFolder') }}</option>
                                        <option value="url">{{ t('editProject.form.targetTypeUrl') }}</option>
                                    </select>
                                    <div class="input-icon">🎯</div>
                                </div>
                            </div>

                            <div class="setting-item half-width">
                                <label class="setting-label">
                                    <span class="label-text" v-if="projectData.targetType !== 'url'">{{ t('editProject.form.targetPath') }}</span>
                                    <span class="label-text" v-else>{{ t('editProject.url') }}</span>
                                    <span class="label-required">*</span>
                                </label>
                                <div class="input-wrapper">
                                    <input type="text"
                                           v-model="projectData.targetPath"
                                           :placeholder="projectData.targetType === 'file' ? t('editProject.filePathPlaceholder') : (projectData.targetType === 'folder' ? t('editProject.folderPathPlaceholder') : t('editProject.urlPlaceholder'))"
                                           class="setting-input" />
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
                                    <span class="label-text">{{ t('editProject.form.projectIcon') }}</span>
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
                                    <span class="label-text">{{ t('editProject.form.launchArgs') }}</span>
                                    <span class="label-optional">{{ t('common.optional') }}</span>
                                </label>
                                <div class="input-wrapper">
                                    <input type="text"
                                           v-model="projectData.launchArgs"
                                           :placeholder="t('editProject.form.launchArgsPlaceholder')"
                                           class="setting-input" />
                                    <div class="input-icon">⚙️</div>
                                </div>
                            </div>
                        </div>

                        <!-- 第四行：项目描述（全宽） -->
                        <div class="settings-row">
                            <div class="setting-item full-width">
                                <label class="setting-label">
                                    <span class="label-text">{{ t('editProject.form.projectDescription') }}</span>
                                    <span class="label-optional">{{ t('common.optional') }}</span>
                                </label>
                                <div class="input-wrapper">
                                    <textarea v-model="projectData.description"
                                              :placeholder="t('editProject.form.projectDescriptionPlaceholder')"
                                              class="setting-input textarea"></textarea>
                                    <div class="input-icon">💬</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- 底栏 -->
        <div class="edit-project-footer">
            <div class="footer-left">
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
                <button @click="cancelEdit" class="footer-button secondary">
                    <i class="icon-close"></i>
                    {{ t('common.cancel') }}
                </button>
                <button @click="saveProject" class="footer-button primary" :disabled="!canSave">
                    <i class="icon-check"></i>
                    {{ t('editProject.form.saveChanges') }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, reactive, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

// 调试日志函数
const debugLog = (message, data = null) => {
    const timestamp = new Date().toISOString()
    console.log(`[EditProjectApp ${timestamp}] ${message}`, data || '')
}

// 加载状态
const isLoading = ref(true)
const loadingText = ref(t('editProject.loadingData'))
const hasError = ref(false)
const errorMessage = ref('')

const isSaving = ref(false)
const lastSaved = ref(false)
const appId = ref(null)

// 分类数据
const categories = ref([])

// 项目数据
const projectData = reactive({
    id: null,
    name: '',
    category: '',
    description: '',
    targetType: 'file',
    targetPath: '',
    launchArgs: '',
    icon: ''
})

// 保存状态文本
const saveStatusText = computed(() => {
    if (isSaving.value) return t('editProject.saving')
    if (lastSaved.value) return t('editProject.saved')
    return ''
})

// 是否可以保存
const canSave = computed(() => {
    return projectData.name.trim() && projectData.category && projectData.targetPath.trim()
})

// 监听项目数据变化
watch(projectData, () => {
    lastSaved.value = false
}, { deep: true })

// 方法
const handleTargetTypeChange = () => {
    // 当目标类型改变时，清空路径
    projectData.targetPath = ''
}

const browseTarget = async () => {
    debugLog('开始浏览目标文件', { targetType: projectData.targetType })
    try {
        let selectedPath = ''
        
        if (projectData.targetType === 'file') {
            // 选择文件
            const filters = [
                [t('editProject.allFiles'), ['*']],
                [t('editProject.executableFiles'), ['exe', 'bat', 'cmd', 'msi']],
                [t('editProject.scriptFiles'), ['ps1', 'vbs', 'js', 'py']],
                [t('editProject.shortcutFiles'), ['lnk', 'url']]
            ]
            debugLog('调用 Tauri API: open_file_dialog', { title: t('editProject.selectTargetFile'), filters })
            selectedPath = await invoke('open_file_dialog', {
                title: t('editProject.selectTargetFile'),
                filters: filters
            })
        } else if (projectData.targetType === 'folder') {
            // 选择文件夹
            debugLog('调用 Tauri API: open_folder_dialog', { title: t('editProject.selectTargetFolder') })
            selectedPath = await invoke('open_folder_dialog', {
                title: t('editProject.selectTargetFolder')
            })
        }

        debugLog('选择的路径', selectedPath)
        if (selectedPath) {
            projectData.targetPath = selectedPath
            // 自动检测目标类型
            await detectTargetType()
        }
    } catch (error) {
        debugLog('浏览文件失败', error)
        console.error('浏览文件失败:', error)
        if (error !== t('editProject.userCancelled')) {
            alert(t('editProject.browseFileError') + ': ' + error)
        }
    }
}

// 自动检测目标类型
const detectTargetType = async () => {
    if (!projectData.targetPath.trim()) {
        debugLog('目标路径为空，跳过类型检测')
        return
    }

    try {
        debugLog('调用 Tauri API: detect_target_type', { targetPath: projectData.targetPath })
        const targetType = await invoke('detect_target_type', {
            targetPath: projectData.targetPath
        })
        debugLog('检测到的目标类型', targetType)
        // 这里不更新 projectData.targetType，因为用户可能已经选择了不同的类型
    } catch (error) {
        debugLog('检测目标类型失败', error)
        console.error('检测目标类型失败:', error)
    }
}

const selectIcon = async () => {
    debugLog('开始选择图标')
    try {
        const filters = [
            [t('editProject.imageFiles'), ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'ico', 'svg']],
            [t('editProject.iconFiles'), ['ico', 'png']],
            [t('editProject.allFiles'), ['*']]
        ]
        debugLog('调用 Tauri API: open_file_dialog', { title: t('editProject.selectIconFile'), filters })
        const selectedPath = await invoke('open_file_dialog', {
            title: t('editProject.selectIconFile'),
            filters: filters
        })

        debugLog('选择的图标路径', selectedPath)
        if (selectedPath) {
            // 尝试将图片转换为base64
            try {
                debugLog('调用 Tauri API: get_app_icon', { filePath: selectedPath })
                const iconBase64 = await invoke('get_app_icon', { filePath: selectedPath })
                debugLog('获取到的图标base64', iconBase64 ? '成功' : '失败')
                if (iconBase64 && iconBase64.startsWith('data:image/')) {
                    projectData.icon = iconBase64
                } else {
                    // 如果无法转换为base64，直接使用文件路径
                    projectData.icon = selectedPath
                    debugLog('使用文件路径作为图标')
                }
            } catch (iconError) {
                debugLog('获取图标base64失败', iconError)
                // 如果获取图标失败，直接使用文件路径
                projectData.icon = selectedPath
            }
        }
    } catch (error) {
        debugLog('选择图标失败', error)
        console.error('选择图标失败:', error)
        if (error !== t('editProject.userCancelled')) {
            alert(t('editProject.selectIconError') + ': ' + error)
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

const cancelEdit = async () => {
    debugLog('用户取消编辑')
    try {
        debugLog('调用 Tauri API: close_edit_project_window')
        await invoke('close_edit_project_window')
    } catch (error) {
        debugLog('关闭编辑项目窗口失败', error)
        console.error('关闭编辑项目窗口失败:', error)
    }
}

const saveProject = async () => {
    debugLog('开始保存项目', projectData)
    if (!canSave.value) {
        debugLog('验证失败，缺少必要信息')
        alert(t('editProject.fillRequiredFields'))
        return
    }

    isSaving.value = true
    try {
        // 创建更新后的应用项
        const updatedApp = {
            id: projectData.id,
            name: projectData.name.trim(),
            category: projectData.category,
            icon: projectData.icon,
            path: projectData.targetPath,
            target_path: projectData.targetPath,
            is_shortcut: false,
            launch_args: projectData.launchArgs,
            target_type: projectData.targetType
        }

        debugLog('调用 Tauri API: update_app', updatedApp)
        // 调用后端更新应用数据
        await invoke('update_app', { app: updatedApp })
        debugLog('项目保存成功')

        lastSaved.value = true
        setTimeout(() => {
            lastSaved.value = false
        }, 2000)

        // 延迟一秒后关闭窗口
        setTimeout(async () => {
            try {
                debugLog('调用 Tauri API: close_edit_project_window')
                await invoke('close_edit_project_window')
            } catch (error) {
                debugLog('关闭编辑项目窗口失败', error)
                console.error('关闭编辑项目窗口失败:', error)
            }
        }, 1000)
    } catch (error) {
        debugLog('保存项目失败', error)
        console.error('保存项目失败:', error)
        alert(t('editProject.saveProjectError') + ': ' + error)
    } finally {
        isSaving.value = false
    }
}

// 加载分类数据
const loadCategories = async () => {
    debugLog('开始加载分类数据')
    loadingText.value = t('editProject.loadingCategories')
    
    try {
        debugLog('调用 Tauri API: load_app_data')
        const storage = await invoke('load_app_data')
        debugLog('成功获取应用数据', storage)
        
        // 转换后端的 is_default 为前端使用的 isDefault
        const categoriesFromBackend = storage.categories || []
        debugLog('后端分类数据', categoriesFromBackend)
        
        const convertedCategories = categoriesFromBackend.map(category => ({
            id: category.id,
            name: category.name,
            icon: category.icon,
            isDefault: category.is_default
        }))

        // 确保"全部应用"分组始终存在
        if (!convertedCategories.some(cat => cat.id === 'all')) {
            debugLog('添加默认"全部应用"分组')
            convertedCategories.unshift({ id: 'all', name: t('common.allApps'), icon: 'icon-apps', isDefault: true })
        }

        categories.value = convertedCategories
        debugLog('分类数据加载完成', categories.value)
    } catch (error) {
        debugLog('加载分类数据失败', error)
        console.error('加载分类数据失败:', error)
        
        // 降级处理：提供默认分类
        categories.value = [
            { id: 'all', name: t('common.allApps'), icon: 'icon-apps', isDefault: true },
            { id: 'default', name: t('editProject.defaultCategory'), icon: 'icon-folder', isDefault: false }
        ]
        debugLog('使用默认分类数据', categories.value)
        
        throw new Error(`加载分类数据失败: ${error}`)
    }
}

// 加载应用数据
const loadAppData = async () => {
    debugLog('开始加载应用数据')
    loadingText.value = t('editProject.loadingAppData')
    
    try {
        // 从URL参数获取应用ID
        debugLog('获取URL参数')
        const urlParams = new URLSearchParams(window.location.search)
        const id = urlParams.get('id')
        debugLog('URL参数中的应用ID', id)
        
        if (!id) {
            debugLog('未找到应用ID参数，尝试从窗口标签获取')
            // 尝试从窗口标签获取应用ID
            try {
                const currentWindow = getCurrentWindow()
                const windowLabel = currentWindow.label
                debugLog('当前窗口标签', windowLabel)
                
                // 从窗口标签中提取ID（假设格式为 edit-project-{id}）
                const labelMatch = windowLabel.match(/edit-project-(\d+)/)
                if (labelMatch && labelMatch[1]) {
                    appId.value = parseInt(labelMatch[1])
                    debugLog('从窗口标签获取到应用ID', appId.value)
                } else {
                    throw new Error('无法从窗口标签获取应用ID')
                }
            } catch (labelError) {
                debugLog('从窗口标签获取ID失败', labelError)
                throw new Error(t('editProject.noAppIdError'))
            }
        } else {
            appId.value = parseInt(id)
            debugLog('从URL参数获取到应用ID', appId.value)
        }
        
        // 获取应用数据
        debugLog('调用 Tauri API: get_app_by_id', { appId: appId.value })
        const app = await invoke('get_app_by_id', { appId: appId.value })
        debugLog('成功获取应用数据', app)
        
        // 填充表单数据
        projectData.id = app.id
        projectData.name = app.name || ''
        projectData.category = app.category || 'default'
        projectData.description = app.description || ''
        projectData.targetType = app.target_type || 'file'
        projectData.targetPath = app.path || ''
        projectData.launchArgs = app.launch_args || ''
        projectData.icon = app.icon || ''
        
        debugLog('应用数据填充完成', projectData)
    } catch (error) {
        debugLog('加载应用数据失败', error)
        console.error('加载应用数据失败:', error)
        
        // 降级处理：提供默认值
        projectData.id = null
        projectData.name = ''
        projectData.category = categories.value.length > 0 ? categories.value[0].id : 'default'
        projectData.description = ''
        projectData.targetType = 'file'
        projectData.targetPath = ''
        projectData.launchArgs = ''
        projectData.icon = ''
        
        debugLog('使用默认应用数据', projectData)
        throw new Error(`加载应用数据失败: ${error}`)
    }
}

// 重试加载
const retryLoading = async () => {
    debugLog('用户点击重试')
    hasError.value = false
    errorMessage.value = ''
    isLoading.value = true
    await initializeApp()
}

// 初始化应用
const initializeApp = async () => {
    debugLog('开始初始化应用')
    try {
        await loadCategories()
        await loadAppData()
        debugLog('应用初始化成功')
    } catch (error) {
        debugLog('应用初始化失败', error)
        hasError.value = true
        errorMessage.value = error.message || t('editProject.unknownError')
    } finally {
        isLoading.value = false
    }
}

// 初始化
onMounted(async () => {
    debugLog('组件已挂载，开始初始化')
    await initializeApp()
})
</script>

<style scoped>
/* 全局样式 */
.edit-project-app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    overflow: hidden;
}

/* 加载状态 */
.loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(255, 255, 255, 0.9);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    z-index: 9999;
}

.loading-spinner-large {
    width: 48px;
    height: 48px;
    border: 4px solid rgba(52, 152, 219, 0.2);
    border-top: 4px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 16px;
}

.loading-text {
    font-size: 16px;
    color: #2c3e50;
    font-weight: 500;
}

/* 错误状态 */
.error-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(255, 255, 255, 0.95);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
}

.error-container {
    background: white;
    border-radius: 12px;
    padding: 32px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
    text-align: center;
    max-width: 400px;
    width: 90%;
}

.error-icon {
    font-size: 48px;
    margin-bottom: 16px;
}

.error-title {
    font-size: 20px;
    font-weight: 600;
    color: #2c3e50;
    margin-bottom: 12px;
}

.error-message {
    font-size: 14px;
    color: #7f8c8d;
    margin-bottom: 24px;
    line-height: 1.5;
}

.error-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
}

.retry-button {
    padding: 10px 20px;
    background: #3498db;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
}

.retry-button:hover {
    background: #2980b9;
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(52, 152, 219, 0.2);
}

.cancel-button {
    padding: 10px 20px;
    background: #e74c3c;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
}

.cancel-button:hover {
    background: #c0392b;
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(231, 76, 60, 0.2);
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
.edit-project-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    overflow-y: auto;
}

/* 设置面板 */
.edit-project-panel {
    flex: 1;
    overflow-y: auto;
    background: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(5px);
    padding: 16px 0;
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
.edit-project-footer {
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
    .panel-section {
        padding: 0 16px;
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