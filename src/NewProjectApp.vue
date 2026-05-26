<template>
    <div class="new-project-app">
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
            <!-- 设置面板 -->
            <div class="new-project-panel">
                <!-- 统一的项目设置区域 -->
                <div class="panel-section unified-section">
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

                        <!-- 第四行：项目描述（全宽） -->
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

        <!-- 底栏 -->
        <div class="new-project-footer">
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
                <button @click="cancelProject" class="footer-button secondary">
                    <i class="icon-close"></i>
                    {{ t('common.cancel') }}
                </button>
                <button @click="saveProject" class="footer-button primary" :disabled="!canSave">
                    <i class="icon-check"></i>
                    {{ t('newProject.createProject') }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, reactive, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import { alertDialog } from './utils/customDialog'

const { t } = useI18n()

const isSaving = ref(false)
const lastSaved = ref(false)

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
            await handlePathChange()
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

// 处理路径变化，自动获取图标
const handlePathChange = async () => {
    if (!projectData.targetPath.trim()) {
        return
    }

    // 对文件或 URL 类型自动获取图标
    if (projectData.targetType !== 'file' && projectData.targetType !== 'url') {
        return
    }

    try {
        // 尝试获取应用图标
        console.log('尝试获取应用图标:', projectData.targetPath)
        const iconBase64 = await invoke('get_app_icon', {
            filePath: projectData.targetPath
        })
        
        if (iconBase64 && iconBase64.startsWith('data:image/')) {
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

const saveProject = async () => {
    if (!canSave.value) {
        await alertDialog(t('newProject.fillRequiredFields'), { type: 'warning' })
        return
    }

    isSaving.value = true
    try {
        // 创建新的应用项
        const newApp = {
            id: Date.now(),
            name: projectData.name.trim(),
            category: projectData.category,
            icon: projectData.icon,
            path: projectData.targetPath,
            target_path: projectData.targetPath,
            is_shortcut: false,
            launch_args: projectData.launchArgs,
            target_type: projectData.targetType
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
            convertedCategories.unshift({ id: 'all', name: t('common.allApps'), icon: 'icon-apps', isDefault: true })
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

// 初始化
onMounted(async () => {
    await loadCategories()
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
    flex-direction: column;
    min-height: 0;
    overflow-y: auto;
}

/* 设置面板 */
.new-project-panel {
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
