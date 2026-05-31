<template>
  <div class="app-container" :class="appContainerClasses" :style="appContainerStyle"
    @mouseenter="isMouseInWindow = true" @mouseleave="isMouseInWindow = false">
    <!-- 加载指示器 -->
    <div v-if="isLoading" class="loading-overlay">
      <div class="loading-content">
        <div class="loading-spinner"></div>
        <div class="loading-text">{{ $t('app.loading') }}</div>
      </div>
    </div>

    <!-- 自定义标题栏 -->
    <div class="titlebar">
      <div class="titlebar-left" data-tauri-drag-region @mousedown="handleDragStart" @mouseup="handleDragEnd">
        <span class="app-title" data-tauri-drag-region>{{ $t('app.title') }}</span>
      </div>
      <div class="titlebar-right">
        <LanguageSwitch />
        <button class="titlebar-button" @click="toggleSearch" :title="$t('main.search.placeholder')">
          <i class="icon-search"></i>
        </button>
        <button class="titlebar-button" @click="toggleMenu" :title="$t('common.settings')">
          <i class="icon-menu"></i>
        </button>
        <button class="titlebar-button titlebar-close" @click="closeApp" :title="$t('common.close')">
          <i class="icon-close"></i>
        </button>
      </div>
    </div>

    <!-- 主启动器容器 -->
    <div class="launcher-container">
      <!-- 侧栏 -->
      <div class="sidebar" :style="sidebarStyle">
        <!-- <div class="sidebar-header">
        <h2>分类</h2>
      </div> -->
        <div class="sidebar-content" @contextmenu.prevent="showContextMenu($event, null)">
          <div v-for="category in categories" :key="category.id" class="category-item"
            :class="{ active: selectedCategory === category.id }" @click="selectCategory(category.id)"
            @contextmenu.prevent="showContextMenu($event, category)">
            <span>{{ category.name }}</span>
          </div>
        </div>
      </div>

      <!-- 右键菜单 -->
      <Teleport to="body">
        <div v-if="contextMenu.visible" class="context-menu"
          :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }" @click.stop>
          <div class="context-menu-item" @click="createNewCategory">
            <span>{{ $t('main.contextMenu.newCategory') }}</span>
          </div>
          <div v-if="contextMenu.category && !contextMenu.category.isDefault" class="context-menu-item"
            :class="{ 'context-menu-item-disabled': !contextMenu.category }" @click="renameCategory">
            <span>{{ $t('main.contextMenu.rename') }}</span>
          </div>
          <div v-if="contextMenu.category && !contextMenu.category.isDefault"
            class="context-menu-item context-menu-item-danger"
            :class="{ 'context-menu-item-disabled': !contextMenu.category }" @click="deleteCategory">
            <span>{{ $t('main.contextMenu.delete') }}</span>
          </div>
          <template v-if="!contextMenu.category">
            <div class="context-menu-item context-menu-item-disabled">
              <span>{{ $t('main.contextMenu.rename') }}</span>
            </div>
            <div class="context-menu-item context-menu-item-disabled">
              <span>{{ $t('main.contextMenu.delete') }}</span>
            </div>
          </template>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item context-menu-item-danger" @click="deleteAllCategories">
            <span>{{ $t('main.contextMenu.deleteAll') }}</span>
          </div>
        </div>
      </Teleport>

      <!-- 应用右键菜单 -->
      <Teleport to="body">
        <div v-if="appContextMenu.visible" class="context-menu"
          :style="{ left: appContextMenu.x + 'px', top: appContextMenu.y + 'px' }" @click.stop>
          <div v-if="appContextMenu.app?.target_type !== 'url'" class="context-menu-item" @click="runAsAdmin">
            <span>{{ $t('main.contextMenu.runAsAdmin') }}</span>
          </div>
          <div v-if="appContextMenu.app?.target_type !== 'url'" class="context-menu-divider"></div>
          <div v-if="appContextMenu.app?.target_type !== 'url'" class="context-menu-item" @click="openFileLocation">
            <span>{{ $t('main.contextMenu.openFileLocation') }}</span>
          </div>
          <div class="context-menu-item" @click="copyFullPath">
            <span>{{ $t('main.contextMenu.copyFullPath') }}</span>
          </div>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item" @click="showMoveToSubmenu">
            <span>{{ $t('main.contextMenu.moveTo') }}</span>
            <span class="arrow-right">▶</span>
          </div>
          <div class="context-menu-item" @click="showCopyToSubmenu">
            <span>{{ $t('main.contextMenu.copyTo') }}</span>
            <span class="arrow-right">▶</span>
          </div>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item" @click="editApp">
            <span>{{ $t('main.contextMenu.editApp') }}</span>
          </div>
          <div class="context-menu-item context-menu-item-danger" @click="deleteApp">
            <span>{{ $t('main.contextMenu.deleteApp') }}</span>
          </div>
          <div class="context-menu-item context-menu-item-danger" @click="deleteAllApps">
            <span>{{ $t('main.contextMenu.deleteAllApps') }}</span>
          </div>
        </div>
      </Teleport>

      <!-- 网格空白处右键菜单 -->
      <Teleport to="body">
        <div v-if="gridContextMenu.visible" class="context-menu"
          :style="{ left: gridContextMenu.x + 'px', top: gridContextMenu.y + 'px' }" @click.stop>
          <div class="context-menu-item" @click="createNewProject">
            <span>{{ $t('common.create') }} {{ $t('newProject.title') }}</span>
          </div>
        </div>
      </Teleport>

      <!-- 移动到子菜单 -->
      <Teleport to="body">
        <div v-if="moveToSubmenu.visible" class="context-menu submenu"
          :style="{ left: moveToSubmenu.x + 'px', top: moveToSubmenu.y + 'px' }" @click.stop>
          <div v-for="category in categories.filter(cat => cat.id !== selectedCategory)" :key="category.id"
            class="context-menu-item" @click="moveAppToCategory(category.id)">
            <span>{{ category.name }}</span>
          </div>
        </div>
      </Teleport>

      <!-- 复制到子菜单 -->
      <Teleport to="body">
        <div v-if="copyToSubmenu.visible" class="context-menu submenu"
          :style="{ left: copyToSubmenu.x + 'px', top: copyToSubmenu.y + 'px' }" @click.stop>
          <div v-for="category in categories.filter(cat => cat.id !== selectedCategory)" :key="category.id"
            class="context-menu-item" @click="copyAppToCategory(category.id)">
            <span>{{ category.name }}</span>
          </div>
        </div>
      </Teleport>

      <!-- 主菜单 -->
      <Teleport to="body">
        <div v-if="mainMenu.visible" class="context-menu main-menu"
          :style="{ left: mainMenu.x + 'px', top: mainMenu.y + 'px' }" @click.stop>
          <div class="context-menu-item" @click="togglePreventAutoHide">
            <span>{{ appSettings.preventAutoHide ? '✓' : '○' }} {{ $t('settings.ui.window.preventAutoHide') }}</span>
          </div>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item" @click="openSettings">
            <span>{{ $t('common.settings') }}</span>
          </div>
          <!-- <div class="context-menu-item" @click="openLanguageSettings">
            <span>{{ $t('language.switch') }}</span>
          </div> -->
          <div class="context-menu-divider"></div>
          <div class="context-menu-item context-menu-item-danger" @click="confirmExit">
            <span>{{ $t('main.confirm.exit') }}</span>
          </div>
        </div>
      </Teleport>

      <!-- 重命名对话框 -->
      <div v-if="renameDialog.visible" class="dialog-overlay" @click="cancelRename">
        <div class="dialog" @click.stop>
          <div class="dialog-header">
            <h3>{{ $t('main.dialog.renameCategory') }}</h3>
          </div>
          <div class="dialog-content">
            <input v-model="renameDialog.newName" type="text" class="dialog-input" :placeholder="$t('main.dialog.newName')"
              @keyup.enter="confirmRename" @keyup.escape="cancelRename" ref="renameInput">
          </div>
          <div class="dialog-actions">
            <button class="dialog-button dialog-button-secondary" @click="cancelRename">{{ $t('common.cancel') }}</button>
            <button class="dialog-button dialog-button-primary" @click="confirmRename">{{ $t('common.confirm') }}</button>
          </div>
        </div>
      </div>

      <!-- Toast 提示 -->
      <Teleport to="body">
        <div v-if="toast.visible" class="toast" :class="'toast-' + toast.type">
          <div class="toast-content">
            <span class="toast-message">{{ toast.message }}</span>
          </div>
        </div>
      </Teleport>

      <!-- 编辑应用对话框 -->
      <div v-if="editAppDialog.visible" class="dialog-overlay" @click="cancelEditApp">
        <div class="dialog large-dialog" @click.stop>
          <div class="dialog-header">
            <h3>{{ $t('main.dialog.editApp') }}</h3>
          </div>
          <div class="dialog-content">
            <div class="form-group">
              <label>{{ $t('main.dialog.appName') }}:</label>
              <input v-model="editAppDialog.editedName" type="text" class="dialog-input" :placeholder="$t('main.dialog.appName')"
                @keyup.enter="confirmEditApp" @keyup.escape="cancelEditApp">
            </div>
            <div class="form-group">
              <label>{{ $t('main.dialog.category') }}:</label>
              <select v-model="editAppDialog.editedCategory" class="dialog-select">
                <option v-for="category in categories.filter(cat => cat.id !== 'all')" :key="category.id"
                  :value="category.id">
                  {{ category.name }}
                </option>
              </select>
            </div>
            <div class="form-group">
              <label>{{ $t('main.dialog.targetPath') }}:</label>
              <div class="input-group">
                <input v-model="editAppDialog.editedTargetPath" type="text" class="dialog-input"
                  :placeholder="$t('main.dialog.filePlaceholder')" @blur="detectTargetType">
                <button class="browse-button" @click="browseTarget" type="button">
                  {{ $t('common.browse') }}
                </button>
              </div>
            </div>
            <div class="form-group">
              <label>{{ $t('main.dialog.launchArgs') }}:</label>
              <input v-model="editAppDialog.editedLaunchArgs" type="text" class="dialog-input"
                :placeholder="$t('main.dialog.launchArgsPlaceholder')">
            </div>
            <div class="form-group">
              <label>{{ $t('main.dialog.icon') }}:</label>
              <div class="icon-section">
                <div class="icon-preview">
                  <img
                    v-if="editAppDialog.editedIcon && (editAppDialog.editedIcon.startsWith('data:image/') || editAppDialog.editedIcon.startsWith('http'))"
                    :src="editAppDialog.editedIcon" :alt="editAppDialog.editedName" class="preview-icon" />
                  <div
                    v-else-if="editAppDialog.editedIcon && !editAppDialog.editedIcon.startsWith('data:image/') && !editAppDialog.editedIcon.startsWith('http')"
                    class="file-type-icon preview-icon" :class="'file-type-' + editAppDialog.editedIcon">
                    {{ getFileTypeIcon(editAppDialog.editedIcon) }}
                  </div>
                  <div v-else class="default-icon preview-icon">{{ editAppDialog.editedName.charAt(0) }}</div>
                </div>
                <div class="icon-actions">
                  <button class="browse-button icon-button" @click="selectIcon" type="button">
                    {{ $t('main.dialog.selectIcon') }}
                  </button>
                  <button v-if="editAppDialog.editedIcon" class="browse-button icon-button danger" @click="clearIcon"
                    type="button">
                    {{ $t('main.dialog.clearIcon') }}
                  </button>
                </div>
              </div>
            </div>
          </div>
          <div class="dialog-actions">
            <button class="dialog-button dialog-button-secondary" @click="cancelEditApp">{{ $t('common.cancel') }}</button>
            <button class="dialog-button dialog-button-primary" @click="confirmEditApp">{{ $t('common.confirm') }}</button>
          </div>
        </div>
      </div>

      <!-- 拖拽分隔线 -->
      <div class="resizer" @mousedown="startResize"></div>

      <!-- 主内容区域 -->
      <div class="main-content" :class="{ 'drag-over': isDragOver }">
        <div class="content-header" v-show="showSearchBox">
          <!-- <h1>{{ getCurrentCategoryName() }}</h1> -->
          <div class="search-box">
            <input v-model="searchQuery" type="text" :placeholder="$t('main.search.placeholder')" class="search-input" ref="searchInputRef"
              @keyup.escape="hideSearchBox" @keydown="handleSearchKeydown">
            <div v-if="searchQuery" class="search-info">
              {{ $t('main.search.results', { count: filteredApps.length }) }}
              <span v-if="filteredApps.length > 0" class="search-hint">
                • {{ $t('main.search.hint') }}
              </span>
            </div>
          </div>
        </div>

        <div class="app-grid" @wheel="handleLauncherWheel" @contextmenu.prevent="showGridContextMenu($event)">
          <div
            v-for="(app, index) in filteredApps"
            :key="app.id"
            class="app-item"
            :class="{ selected: selectedAppId === app.id }"
            :data-app-id="app.id"
            :data-app-index="index"
            @click="handleAppClick($event, app)"
            @dblclick="handleAppClick($event, app)"
            @contextmenu.prevent="showAppContextMenu($event, app)"
            @pointerdown="handleAppPointerDown($event, app, index)"
          >
            <div class="app-icon">
              <!-- 如果是 Base64 图标 (真实应用图标) -->
              <img :src="app.icon" :alt="app.name"
                v-if="app.icon && (app.icon.startsWith('data:image/') || app.icon.startsWith('http'))" />
              <!-- 如果是文件类型图标标识符 -->
              <div v-else-if="app.icon && !app.icon.startsWith('data:image/') && !app.icon.startsWith('http')"
                class="file-type-icon" :class="'file-type-' + app.icon">
                {{ getFileTypeIcon(app.icon) }}
              </div>
              <!-- 默认图标 (应用名称首字母) -->
              <div v-else class="default-icon">{{ app.name.charAt(0) }}</div>
            </div>
            <div class="app-name">{{ app.name }}</div>
          </div>
        </div>

        <!-- 拖拽提示覆盖层 -->
        <div v-if="isDragOver" class="drag-overlay">
          <div class="drag-message">
            <i class="icon-plus"></i>
            <p>{{ $t('main.drag.message') }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 设置模态框已移除，现在使用独立窗口 -->
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import LanguageSwitch from './components/LanguageSwitch.vue'
import { alertDialog, confirmDialog } from './utils/customDialog'

const { t } = useI18n()

// 定义数据接口
interface AppData {
  id: number
  name: string
  category: string
  icon: string
  path: string
  target_path?: string
  is_shortcut?: false
  launch_args?: string // 启动参数
  target_type?: 'file' | 'folder' | 'url' // 目标类型
  order?: number // 排序字段，用于图标拖拽排序
}

interface CategoryData {
  id: string
  name: string
  icon: string
  isDefault: boolean
}

// 响应式数据
const sidebarWidth = ref(0) // 初始化为0，将通过自适应计算
const windowWidth = ref(window.innerWidth)
const isResizing = ref(false)
const selectedCategory = ref('all')
const searchQuery = ref('')
const showSearchBox = ref(false) // 控制搜索框显示状态
const isLoading = ref(true) // 加载状态
const selectedAppId = ref<number | null>(null)
// 设置相关状态已移除，现在使用独立设置窗口

// 拖拽相关状态
const isDragOver = ref(false)
const dragCounter = ref(0)

// 图标拖拽排序状态
// 使用非响应式普通变量，避免 dragstart 期间触发 Vue 重渲染，防止 WebView2 丢失拖拽上下文
let isDraggingApp = false
let unlistenTauriFileDrop: (() => void) | null = null
let pointerDragState: {
  app: AppData
  sourceIndex: number
  targetIndex: number
  startX: number
  startY: number
  pointerId: number
  dragging: boolean
} | null = null
let suppressNextAppClick = false
const appSortDragThreshold = 6

// 右键菜单相关
const contextMenu = ref<{
  visible: boolean;
  x: number;
  y: number;
  category: any;
}>({
  visible: false,
  x: 0,
  y: 0,
  category: null
})

// 应用右键菜单相关
const appContextMenu = ref<{
  visible: boolean;
  x: number;
  y: number;
  app: any;
}>({
  visible: false,
  x: 0,
  y: 0,
  app: null
})

// 移动到子菜单
const moveToSubmenu = ref<{
  visible: boolean;
  x: number;
  y: number;
}>({
  visible: false,
  x: 0,
  y: 0
})

// 复制到子菜单
const copyToSubmenu = ref<{
  visible: boolean;
  x: number;
  y: number;
}>({
  visible: false,
  x: 0,
  y: 0
})

// 网格右键菜单相关
const gridContextMenu = ref<{
  visible: boolean;
  x: number;
  y: number;
}>({
  visible: false,
  x: 0,
  y: 0
})

// 主菜单相关
const mainMenu = ref<{
  visible: boolean;
  x: number;
  y: number;
}>({
  visible: false,
  x: 0,
  y: 0
})

const GRID_CELL_SIZE_DEFAULT = 88
const GRID_CELL_SIZE_MIN = 56
const GRID_CELL_SIZE_MAX = 144
const GRID_CELL_SIZE_STEP = 4
const SIDEBAR_WIDTH_MIN = 72

const clampGridCellSize = (value: unknown) => {
  const numericValue = Number(value)
  if (!Number.isFinite(numericValue)) {
    return GRID_CELL_SIZE_DEFAULT
  }

  return Math.min(GRID_CELL_SIZE_MAX, Math.max(GRID_CELL_SIZE_MIN, Math.round(numericValue)))
}

// 应用设置
const appSettings = ref({
  preventAutoHide: false, // 阻止自动隐藏
  windowWidth: undefined as number | undefined, // 窗口宽度
  windowHeight: undefined as number | undefined, // 窗口高度
  windowLayout: 'horizontal' as string, // 窗口布局
  theme: 'auto' as string, // 主题
  projectNamePosition: 'bottom' as string, // 项目名称显示位置
  gridCellSize: GRID_CELL_SIZE_DEFAULT as number, // 启动器格子大小
  sidebarWidth: 0 as number, // 侧栏宽度，0 表示自动
  enableAnimations: true as boolean, // 启用动画
  animationSpeed: 'normal' as string, // 动画速度
  gridViewEnabled: false as boolean, // 网格视图
  sortOrder: 'name' as string, // 排序方式
  showHiddenFiles: false as boolean, // 显示隐藏文件
  fuzzySearch: true as boolean, // 模糊搜索
  searchInPath: false as boolean, // 搜索路径
  maxSearchResults: 20 as number, // 最大搜索结果
  autoHideAfterLaunch: false as boolean, // 启动后自动隐藏
})

// 重命名对话框相关
const renameDialog = ref({
  visible: false,
  newName: '',
  categoryId: ''
})

// Toast 提示相关
const toast = ref({
  visible: false,
  message: '',
  type: 'info' // 'info', 'success', 'warning', 'error'
})

// 编辑应用对话框相关
const editAppDialog = ref({
  visible: false,
  app: null as AppData | null,
  editedName: '',
  editedCategory: '',
  editedIcon: '',
  editedTargetPath: '',
  editedLaunchArgs: '',
  editedTargetType: 'file' as 'file' | 'folder' | 'url'
})

const renameInput = ref(null)
const searchInputRef = ref(null)

// 分类数据
const categories = ref<CategoryData[]>([
  { id: 'all', name: t('main.sidebar.allApps'), icon: 'icon-apps', isDefault: true },
])

// 应用数据
const apps = ref<AppData[]>([])

// 确保"全部应用"分组始终存在
const ensureDefaultCategory = () => {
  const hasAllCategory = categories.value.some(cat => cat.id === 'all')
  if (!hasAllCategory) {
    categories.value.unshift({ id: 'all', name: t('main.sidebar.allApps'), icon: 'icon-apps', isDefault: true })
  }
}

// 获取合适的默认分组（用于新应用）
const getDefaultCategoryForNewApp = () => {
  // 如果当前选中的不是"全部应用"，则使用当前选中的分组
  if (selectedCategory.value !== 'all') {
    return selectedCategory.value
  }

  // 如果选中的是"全部应用"，则使用第一个非默认分组
  const nonDefaultCategory = categories.value.find(cat => !cat.isDefault)
  if (nonDefaultCategory) {
    return nonDefaultCategory.id
  }

  // 如果没有其他分组，提示用户先创建分组
  return null
}
const getFileTypeIcon = (fileType: string): string => {
  const iconMap: { [key: string]: string } = {
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

// 加载应用数据
const loadAppData = async () => {
  console.log('开始加载应用数据...')
  try {
    const storage = await invoke('load_app_data') as any
    console.log('从后端加载的数据:', storage)

    // 批量更新而不是逐个赋值
    Object.assign(apps, { value: storage.apps || [] })

    // 转换后端的 is_default 为前端使用的 isDefault
    const categoriesFromBackend = storage.categories || []
    const convertedCategories = categoriesFromBackend.map((category: any) => ({
      id: category.id,
      name: category.name,
      icon: category.icon,
      isDefault: category.is_default
    }))

    Object.assign(categories, { value: convertedCategories })

    // 确保"全部应用"分组始终存在
    ensureDefaultCategory()

    // 恢复选中的分组
    const targetCategory = storage.selected_category &&
      categories.value.some(cat => cat.id === storage.selected_category)
      ? storage.selected_category
      : 'all'

    selectedCategory.value = targetCategory

    console.log('应用数据加载成功')
  } catch (error) {
    console.error('加载应用数据失败:', error)
    // 使用默认数据
    categories.value = [
      { id: 'all', name: t('main.sidebar.allApps'), icon: 'icon-apps', isDefault: true }
    ]
    apps.value = []
    selectedCategory.value = 'all'
  }
}

// 保存应用数据
const saveAppData = async () => {
  console.log('开始保存应用数据...', {
    apps: apps.value,
    categories: categories.value,
    selectedCategory: selectedCategory.value
  })
  try {
    // 转换前端的 isDefault 为后端期望的 is_default
    const categoriesForBackend = categories.value.map(category => ({
      ...category,
      is_default: category.isDefault,
      isDefault: undefined // 移除前端字段
    })).map(({ isDefault, ...rest }) => rest) // 完全移除 isDefault 字段

    // 确保每个 app 包含后端期望的字段，避免缺少 is_shortcut 导致错误
    const appsForBackend = apps.value.map(a => ({
      ...a,
      is_shortcut: a.is_shortcut ?? false
    }))

    await invoke('save_app_data', {
      apps: appsForBackend,
      categories: categoriesForBackend,
      selectedCategory: selectedCategory.value
    })
    console.log('应用数据保存成功')
  } catch (error) {
    console.error('保存应用数据失败:', error)
  }
}

// 加载应用设置
const loadAppSettings = async () => {
  console.log('开始加载应用设置...')
  try {
    const settings = await invoke('load_app_settings') as any
    console.log('从后端加载的设置:', settings)

    // 批量更新设置，减少响应式触发次数
    const newSettings = {
      preventAutoHide: settings.prevent_auto_hide || false,
      windowWidth: settings.window_width,
      windowHeight: settings.window_height,
      windowLayout: settings.window_layout || 'horizontal',
      theme: settings.theme || 'auto',
      projectNamePosition: settings.project_name_position || 'bottom',
      gridCellSize: clampGridCellSize(settings.icon_size ?? GRID_CELL_SIZE_DEFAULT),
      sidebarWidth: settings.sidebar_width ?? 0,
      enableAnimations: settings.enable_animations !== false,
      animationSpeed: settings.animation_speed || 'normal',
      gridViewEnabled: settings.grid_view_enabled || false,
      sortOrder: settings.sort_order || 'name',
      showHiddenFiles: settings.show_hidden_files || false,
      fuzzySearch: settings.fuzzy_search !== false,
      searchInPath: settings.search_in_path || false,
      maxSearchResults: settings.max_search_results || 20,
      autoHideAfterLaunch: settings.auto_hide_after_launch || false,
    }

    // 一次性更新所有设置
    Object.assign(appSettings.value, newSettings)

    // 恢复界面状态
    if (settings.last_selected_category) {
      selectedCategory.value = settings.last_selected_category
    }
    if (settings.last_search_query) {
      searchQuery.value = settings.last_search_query
    }

    console.log('应用设置加载成功')
  } catch (error) {
    console.error('加载应用设置失败:', error)
  }
}

// 保存界面状态
const saveUIState = async () => {
  try {
    await invoke('save_ui_state', {
      activeTab: null, // 主窗口没有标签页概念
      lastSelectedCategory: selectedCategory.value,
      windowPositionX: null,
      windowPositionY: null,
      lastSearchQuery: searchQuery.value,
      gridViewEnabled: appSettings.value.gridViewEnabled,
      sortOrder: appSettings.value.sortOrder,
      showHiddenFiles: appSettings.value.showHiddenFiles,
    })
  } catch (error) {
    console.error('保存界面状态失败:', error)
  }
}

// 节流保存界面状态（避免频繁保存）
let saveUIStateTimer: number | null = null
const saveUIStateThrottled = () => {
  if (saveUIStateTimer) {
    clearTimeout(saveUIStateTimer)
  }
  saveUIStateTimer = setTimeout(() => {
    saveUIState()
  }, 1000) // 1秒后保存
}

// 监听状态变化并自动保存
watch(selectedCategory, () => {
  saveUIStateThrottled()
})

watch(searchQuery, () => {
  saveUIStateThrottled()
})

// 计算属性
const resolvedTheme = computed(() => {
  if (appSettings.value.theme === 'auto') {
    return window.matchMedia?.('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }

  return appSettings.value.theme === 'dark' ? 'dark' : 'light'
})

const appContainerClasses = computed(() => ({
  'theme-dark': resolvedTheme.value === 'dark',
  'theme-light': resolvedTheme.value === 'light',
  'layout-horizontal': appSettings.value.windowLayout !== 'vertical',
  'layout-vertical': appSettings.value.windowLayout === 'vertical',
  [`name-position-${appSettings.value.projectNamePosition || 'bottom'}`]: true,
  'animations-disabled': !appSettings.value.enableAnimations,
  [`animation-${appSettings.value.animationSpeed || 'normal'}`]: true,
}))

const sidebarStyle = computed(() => {
  if (appSettings.value.windowLayout === 'vertical') {
    return {}
  }

  return sidebarWidth.value > 0 ? { width: `${sidebarWidth.value}px` } : {}
})

const appContainerStyle = computed(() => {
  const cellSize = clampGridCellSize(appSettings.value.gridCellSize)
  const iconSize = Math.min(84, Math.max(32, Math.round(cellSize * 0.58)))
  const cellHeight = Math.max(cellSize, iconSize + 30)

  return {
    '--app-icon-size': `${iconSize}px`,
    '--app-cell-size': `${cellSize}px`,
    '--app-cell-height': `${cellHeight}px`,
  }
})

const applyRuntimeSettings = () => {
  const body = document.body
  body.classList.toggle('lora-theme-dark', resolvedTheme.value === 'dark')
  body.classList.toggle('lora-theme-light', resolvedTheme.value === 'light')
  body.classList.toggle('lora-animations-disabled', !appSettings.value.enableAnimations)
  body.classList.remove('lora-animation-slow', 'lora-animation-normal', 'lora-animation-fast')
  body.classList.add(`lora-animation-${appSettings.value.animationSpeed || 'normal'}`)
}

watch(
  () => [resolvedTheme.value, appSettings.value.enableAnimations, appSettings.value.animationSpeed],
  applyRuntimeSettings
)

const normalizeSearchText = (value?: string) => (value || '').toLowerCase().trim()

const fuzzyIncludes = (target: string, query: string) => {
  if (!query) return true

  let targetIndex = 0
  for (const queryChar of query) {
    targetIndex = target.indexOf(queryChar, targetIndex)
    if (targetIndex === -1) {
      return false
    }
    targetIndex += 1
  }

  return true
}

const matchesSearch = (app: AppData, query: string) => {
  const values = [app.name]
  if (appSettings.value.searchInPath) {
    values.push(app.path || '', app.target_path || '')
  }

  return values.some(value => {
    const normalizedValue = normalizeSearchText(value)
    return appSettings.value.fuzzySearch
      ? fuzzyIncludes(normalizedValue, query)
      : normalizedValue.includes(query)
  })
}

const filteredApps = computed(() => {
  console.log('计算filteredApps:', {
    totalApps: apps.value.length,
    selectedCategory: selectedCategory.value,
    searchQuery: searchQuery.value,
    apps: apps.value
  })

  let result = apps.value

  // 按分类筛选
  if (selectedCategory.value !== 'all') {
    result = result.filter(app => app.category === selectedCategory.value)
    console.log('按分类筛选后:', result)
  }

  // 按搜索词筛选
  const normalizedQuery = normalizeSearchText(searchQuery.value)
  if (normalizedQuery) {
    result = result.filter(app => matchesSearch(app, normalizedQuery))
    console.log('按搜索词筛选后:', result)
  }

  // 按 order 字段排序
  result = [...result].sort((a, b) => {
    const aOrder = a.order ?? Number.MAX_SAFE_INTEGER
    const bOrder = b.order ?? Number.MAX_SAFE_INTEGER
    return aOrder - bOrder
  })

  if (searchQuery.value) {
    result = result.slice(0, appSettings.value.maxSearchResults || 20)
  }

  console.log('最终结果:', result)
  return result
})

watch(filteredApps, currentApps => {
  if (currentApps.length === 0) {
    selectedAppId.value = null
    return
  }

  if (!currentApps.some(app => app.id === selectedAppId.value)) {
    selectedAppId.value = currentApps[0].id
  }
})

const getSelectedApp = () => filteredApps.value.find(app => app.id === selectedAppId.value) || filteredApps.value[0]

const getAppGridColumnCount = () => {
  const grid = document.querySelector('.app-grid') as HTMLElement | null
  if (!grid) {
    return 1
  }

  const computedStyle = window.getComputedStyle(grid)
  const renderedColumns = computedStyle.gridTemplateColumns
    .split(' ')
    .filter(column => column && column !== 'none')
    .length

  if (renderedColumns > 0) {
    return renderedColumns
  }

  const gap = parseFloat(computedStyle.columnGap || computedStyle.gap || '0') || 0
  const cellSize = clampGridCellSize(appSettings.value.gridCellSize)
  return Math.max(1, Math.floor((grid.clientWidth + gap) / (cellSize + gap)))
}

const moveSelectedApp = (offset: number) => {
  const currentApps = filteredApps.value
  if (currentApps.length === 0) {
    selectedAppId.value = null
    return
  }

  const currentIndex = Math.max(0, currentApps.findIndex(app => app.id === selectedAppId.value))
  const nextIndex = Math.min(currentApps.length - 1, Math.max(0, currentIndex + offset))
  selectedAppId.value = currentApps[nextIndex].id
}

// 方法
const selectCategory = async (categoryId: string) => {
  selectedCategory.value = categoryId

  // 自动保存选中的分组
  try {
    await invoke('save_selected_category', { categoryId: categoryId })
    console.log('选中分组已保存:', categoryId)
  } catch (error) {
    console.error('保存选中分组失败:', error)
  }
}

// Toast 提示功能
const showToast = (message: string, type: string = 'info') => {
  toast.value = {
    visible: true,
    message,
    type
  }

  // 3秒后自动隐藏
  setTimeout(() => {
    toast.value.visible = false
  }, 3000)
}

const getLaunchTargetPath = (app: AppData) => app.target_path || app.path

const hideAfterLaunchIfNeeded = async () => {
  if (!appSettings.value.autoHideAfterLaunch) {
    return
  }

  await new Promise(resolve => setTimeout(resolve, 300))
  try {
    await getCurrentWindow().hide()
  } catch (error) {
    console.error('启动后自动隐藏窗口失败:', error)
  }
}

const launchApp = async (app: any) => {
  console.log(`启动应用: ${app.name}`)

  selectedAppId.value = app.id
  const targetPath = getLaunchTargetPath(app)
  if (!targetPath) {
    console.error('应用路径不存在')
    await alertDialog(t('main.alert.appPathNotExist'), { type: 'warning' })
    return
  }

  try {
    // 根据目标类型选择不同的启动方式
    if (app.target_type === 'url') {
      // 打开网址
      await invoke('open_url', {
        url: targetPath,
        launchArgs: app.launch_args || ''
      })
    } else if (app.target_type === 'folder') {
      // 打开文件夹
      await invoke('open_folder', {
        folderPath: targetPath,
        launchArgs: app.launch_args || ''
      })
    } else {
      // 启动文件，使用支持自动隐藏的函数
      await invoke('launch_app_with_auto_hide', {
        appPath: targetPath,
        launchArgs: app.launch_args || ''
      })
    }
    await hideAfterLaunchIfNeeded()
    console.log('启动成功')
  } catch (error) {
    console.error('启动应用失败:', error)
    await alertDialog(t('main.alert.launchFailed', { error: String(error) }), { type: 'error' })
  }
}

const handleAppClick = (event: MouseEvent, app: AppData) => {
  if (suppressNextAppClick) {
    event.preventDefault()
    event.stopPropagation()
    return
  }

  selectedAppId.value = app.id
  void launchApp(app)
}

// 右键菜单相关方法
const showContextMenu = (e: MouseEvent, category: any) => {
  // 隐藏其他所有菜单
  hideAppContextMenu()
  hideMoveToSubmenu()
  hideCopyToSubmenu()

  // 获取屏幕坐标
  const x = e.clientX
  const y = e.clientY

  // 智能定位：确保菜单不超出屏幕边界
  const menuWidth = 120
  const menuHeight = 200 // 估算菜单高度
  const screenWidth = window.innerWidth
  const screenHeight = window.innerHeight

  let adjustedX = x
  let adjustedY = y

  // 如果菜单会超出右边界，则向左偏移
  if (x + menuWidth > screenWidth) {
    adjustedX = screenWidth - menuWidth - 10
  }

  // 如果菜单会超出下边界，则向上偏移
  if (y + menuHeight > screenHeight) {
    adjustedY = screenHeight - menuHeight - 10
  }

  contextMenu.value = {
    visible: true,
    x: adjustedX,
    y: adjustedY,
    category: category
  }

  // 点击其他地方时隐藏菜单
  document.addEventListener('click', hideContextMenu, { once: true })

  // 阻止默认的右键菜单
  e.preventDefault()
  e.stopPropagation()
}

const hideContextMenu = () => {
  contextMenu.value.visible = false
  // 同时隐藏子菜单
  hideMoveToSubmenu()
  hideCopyToSubmenu()
}

// 应用右键菜单相关方法
const showAppContextMenu = (e: MouseEvent, app: any) => {
  selectedAppId.value = app.id

  // 隐藏其他所有菜单
  hideContextMenu()
  hideMoveToSubmenu()
  hideCopyToSubmenu()

  // 获取屏幕坐标
  const x = e.clientX
  const y = e.clientY

  // 智能定位：确保菜单不超出屏幕边界
  const menuWidth = 150
  const menuHeight = 280 // 估算应用菜单高度
  const screenWidth = window.innerWidth
  const screenHeight = window.innerHeight

  let adjustedX = x
  let adjustedY = y

  // 如果菜单会超出右边界，则向左偏移
  if (x + menuWidth > screenWidth) {
    adjustedX = screenWidth - menuWidth - 10
  }

  // 如果菜单会超出下边界，则向上偏移
  if (y + menuHeight > screenHeight) {
    adjustedY = screenHeight - menuHeight - 10
  }

  appContextMenu.value = {
    visible: true,
    x: adjustedX,
    y: adjustedY,
    app: app
  }

  // 点击其他地方时隐藏菜单
  document.addEventListener('click', hideAppContextMenu, { once: true })

  // 阻止默认的右键菜单
  e.preventDefault()
  e.stopPropagation()
}

const hideAppContextMenu = () => {
  appContextMenu.value.visible = false
  // 同时隐藏子菜单
  hideMoveToSubmenu()
  hideCopyToSubmenu()
}

// 网格右键菜单相关方法
const showGridContextMenu = (e: MouseEvent) => {
  // 检查点击的是否为网格本身（空白处），而不是应用项
  const target = e.target as HTMLElement
  if (target.closest('.app-item')) {
    return // 如果点击的是应用项，不显示网格菜单
  }

  // 隐藏其他所有菜单
  hideContextMenu()
  hideAppContextMenu()
  hideMoveToSubmenu()
  hideCopyToSubmenu()

  // 获取屏幕坐标
  const x = e.clientX
  const y = e.clientY

  // 智能定位：确保菜单不超出屏幕边界
  const menuWidth = 120
  const menuHeight = 50 // 估算菜单高度
  const screenWidth = window.innerWidth
  const screenHeight = window.innerHeight

  let adjustedX = x
  let adjustedY = y

  // 如果菜单会超出右边界，则向左偏移
  if (x + menuWidth > screenWidth) {
    adjustedX = screenWidth - menuWidth - 10
  }

  // 如果菜单会超出下边界，则向上偏移
  if (y + menuHeight > screenHeight) {
    adjustedY = screenHeight - menuHeight - 10
  }

  gridContextMenu.value = {
    visible: true,
    x: adjustedX,
    y: adjustedY
  }

  // 点击其他地方时隐藏菜单
  document.addEventListener('click', hideGridContextMenu, { once: true })

  // 阻止默认的右键菜单
  e.preventDefault()
  e.stopPropagation()
}

const hideGridContextMenu = () => {
  gridContextMenu.value.visible = false
}

// 主菜单相关方法
const showMainMenu = (e: MouseEvent) => {
  // 隐藏其他所有菜单
  hideContextMenu()
  hideAppContextMenu()
  hideMoveToSubmenu()
  hideCopyToSubmenu()
  hideGridContextMenu()

  // 获取按钮位置
  const buttonRect = (e.target as HTMLElement).getBoundingClientRect()
  const x = buttonRect.left
  const y = buttonRect.bottom + 5 // 在按钮下方显示

  // 智能定位：确保菜单不超出屏幕边界
  const menuWidth = 150
  const menuHeight = 120 // 估算菜单高度
  const screenWidth = window.innerWidth
  const screenHeight = window.innerHeight

  let adjustedX = x
  let adjustedY = y

  // 如果菜单会超出右边界，则向左偏移
  if (x + menuWidth > screenWidth) {
    adjustedX = screenWidth - menuWidth - 10
  }

  // 如果菜单会超出下边界，则向上偏移到按钮上方
  if (y + menuHeight > screenHeight) {
    adjustedY = buttonRect.top - menuHeight - 5
  }

  mainMenu.value = {
    visible: true,
    x: adjustedX,
    y: adjustedY
  }

  // 点击其他地方时隐藏菜单
  document.addEventListener('click', hideMainMenu, { once: true })

  // 阻止事件冒泡
  e.preventDefault()
  e.stopPropagation()
}

const hideMainMenu = () => {
  mainMenu.value.visible = false
}

const togglePreventAutoHide = async () => {
  const newValue = !appSettings.value.preventAutoHide

  try {
    // 使用专门的更新命令
    await invoke('update_prevent_auto_hide', {
      preventAutoHide: newValue
    })

    // 更新前端状态
    appSettings.value.preventAutoHide = newValue

    // 更新托盘菜单
    await invoke('update_tray_menu', {
      preventAutoHide: newValue
    })

    console.log('阻止自动隐藏设置已更新:', newValue)

    // 显示状态反馈
    const message = newValue
      ? t('main.toast.preventAutoHideEnabled')
      : t('main.toast.preventAutoHideDisabled')
    showToast(message, 'success')
  } catch (error) {
    console.error('更新阻止自动隐藏设置失败:', error)
    showToast(t('main.toast.settingsUpdateFailed'), 'error')
  }

  hideMainMenu()
}

const openSettings = async () => {
  console.log('打开设置窗口')
  try {
    await invoke('open_settings_window')
  } catch (error) {
    console.error('打开设置窗口失败:', error)
    showToast(t('main.toast.openSettingsFailed'), 'error')
  }
  hideMainMenu()
}

const confirmExit = async () => {
  hideMainMenu()

  if (await confirmDialog(t('main.confirm.exit'))) {
    try {
      await invoke('quit_app')
    } catch (error) {
      console.error('退出应用失败:', error)
      showToast(t('main.toast.exitFailed'), 'error')
    }
  }
}


const createNewProject = async () => {
  console.log('新建项目')
  
  try {
    // 调用后端命令打开新建项目窗口
    await invoke('open_new_project_window')
    console.log('新建项目窗口已打开')
  } catch (error) {
    console.error('打开新建项目窗口失败:', error)
    showToast(t('main.toast.openNewProjectFailed'), 'error')
  }
  
  hideGridContextMenu()
}

// 应用右键菜单功能
const runAsAdmin = async () => {
  if (appContextMenu.value.app) {
    try {
      console.log(`以管理员权限运行: ${appContextMenu.value.app.name}`)
      const result = await invoke('run_as_admin', { appPath: getLaunchTargetPath(appContextMenu.value.app) })
      console.log('管理员权限运行结果:', result)
    } catch (error) {
      console.error('以管理员权限运行失败:', error)
      await alertDialog(t('main.alert.runAsAdminFailed', { error: String(error) }), { type: 'error' })
    }
  }
  hideAppContextMenu()
}

const openFileLocation = async () => {
  if (appContextMenu.value.app) {
    try {
      const targetPath = getLaunchTargetPath(appContextMenu.value.app)
      console.log(`打开文件位置: ${targetPath}`)
      const result = await invoke('open_file_location', { filePath: targetPath })
      console.log('打开文件位置结果:', result)
    } catch (error) {
      console.error('打开文件位置失败:', error)
      await alertDialog(t('main.alert.openFileLocationFailed', { error: String(error) }), { type: 'error' })
    }
  }
  hideAppContextMenu()
}

const copyFullPath = async () => {
  if (appContextMenu.value.app) {
    try {
      await navigator.clipboard.writeText(appContextMenu.value.app.path || '')
      console.log(`已复制路径: ${appContextMenu.value.app.path}`)
      // 可以添加一个临时提示
      showToast(t('main.toast.pathCopied'))
    } catch (error) {
      console.error('复制路径失败:', error)
      // 备用方法：创建临时文本区域
      const textArea = document.createElement('textarea')
      textArea.value = appContextMenu.value.app.path || ''
      document.body.appendChild(textArea)
      textArea.select()
      document.execCommand('copy')
      document.body.removeChild(textArea)
      showToast(t('main.toast.pathCopied'))
    }
  }
  hideAppContextMenu()
}

const showMoveToSubmenu = () => {
  hideCopyToSubmenu()

  // 计算子菜单位置，紧贴主菜单
  let submenuX = appContextMenu.value.x + 120 // 紧贴主菜单右侧
  let submenuY = appContextMenu.value.y

  // 智能定位：确保子菜单不超出屏幕边界
  const submenuWidth = 100
  const submenuHeight = 150 // 估算子菜单高度
  const screenWidth = window.innerWidth
  const screenHeight = window.innerHeight

  // 如果子菜单会超出右边界，则显示在主菜单左侧
  if (submenuX + submenuWidth > screenWidth) {
    submenuX = appContextMenu.value.x - submenuWidth
  }

  // 如果子菜单会超出下边界，则向上偏移
  if (submenuY + submenuHeight > screenHeight) {
    submenuY = screenHeight - submenuHeight - 10
  }

  moveToSubmenu.value = {
    visible: true,
    x: submenuX,
    y: submenuY
  }
}

const hideMoveToSubmenu = () => {
  moveToSubmenu.value.visible = false
}

const showCopyToSubmenu = () => {
  hideMoveToSubmenu()

  // 计算子菜单位置，紧贴主菜单
  let submenuX = appContextMenu.value.x + 120 // 紧贴主菜单右侧
  let submenuY = appContextMenu.value.y

  // 智能定位：确保子菜单不超出屏幕边界
  const submenuWidth = 100
  const submenuHeight = 150 // 估算子菜单高度
  const screenWidth = window.innerWidth
  const screenHeight = window.innerHeight

  // 如果子菜单会超出右边界，则显示在主菜单左侧
  if (submenuX + submenuWidth > screenWidth) {
    submenuX = appContextMenu.value.x - submenuWidth
  }

  // 如果子菜单会超出下边界，则向上偏移
  if (submenuY + submenuHeight > screenHeight) {
    submenuY = screenHeight - submenuHeight - 10
  }

  copyToSubmenu.value = {
    visible: true,
    x: submenuX,
    y: submenuY
  }
}

const hideCopyToSubmenu = () => {
  copyToSubmenu.value.visible = false
}

const moveAppToCategory = async (categoryId: string) => {
  if (appContextMenu.value.app) {
    const appIndex = apps.value.findIndex(app => app.id === appContextMenu.value.app.id)
    if (appIndex !== -1) {
      apps.value[appIndex].category = categoryId
      console.log(`已将 ${appContextMenu.value.app.name} 移动到分类: ${categoryId}`)

      // 保存数据
      await saveAppData()
    }
  }
  hideMoveToSubmenu()
  hideAppContextMenu()
}

const createUniqueAppId = () => {
  let id = Date.now() + Math.floor(Math.random() * 1000)

  while (apps.value.some(app => app.id === id)) {
    id += 1
  }

  return id
}

const copyAppToCategory = async (categoryId: string) => {
  if (appContextMenu.value.app) {
    const targetApps = apps.value.filter(app => app.category === categoryId)
    const maxOrder = targetApps.reduce((max, app) => Math.max(max, app.order ?? -1), -1)
    const copiedApp: AppData = {
      ...appContextMenu.value.app,
      id: createUniqueAppId(),
      category: categoryId,
      order: maxOrder + 1
    }

    apps.value.push(copiedApp)
    console.log(`已将 ${appContextMenu.value.app.name} 复制到分类: ${categoryId}`)

    // 保存数据
    await saveAppData()
  }

  hideCopyToSubmenu()
  hideAppContextMenu()
}

const editApp = async () => {
  if (appContextMenu.value.app) {
    console.log(`编辑应用: ${appContextMenu.value.app.name}`)
    
    try {
      // 调用后端命令打开编辑项目窗口
      await invoke('open_edit_project_window', {
        appId: appContextMenu.value.app.id
      })
      console.log('编辑项目窗口已打开')
    } catch (error) {
      console.error('打开编辑项目窗口失败:', error)
      showToast(t('main.toast.openEditProjectFailed'), 'error')
      
      // 如果打开新窗口失败，回退到对话框模式
      editAppDialog.value = {
        visible: true,
        app: appContextMenu.value.app,
        editedName: appContextMenu.value.app.name,
        editedCategory: appContextMenu.value.app.category,
        editedIcon: appContextMenu.value.app.icon || '',
        editedTargetPath: appContextMenu.value.app.target_path || appContextMenu.value.app.path,
        editedLaunchArgs: appContextMenu.value.app.launch_args || '',
        editedTargetType: appContextMenu.value.app.target_type || 'file'
      }

      // 如果没有目标类型，自动检测
      if (!appContextMenu.value.app.target_type && editAppDialog.value.editedTargetPath) {
        try {
          const targetType = await invoke('detect_target_type', {
            targetPath: editAppDialog.value.editedTargetPath
          }) as string
          editAppDialog.value.editedTargetType = targetType as 'file' | 'folder' | 'url'
        } catch (error) {
          console.error('检测目标类型失败:', error)
          editAppDialog.value.editedTargetType = 'file'
        }
      }
    }
  }
  hideAppContextMenu()
}

const deleteSpecificApp = async (appToDelete: AppData | null) => {
  if (appToDelete) {
    if (await confirmDialog(t('main.confirm.deleteApp', { name: appToDelete.name }))) {
      try {
        // 调用后端删除
        await invoke('delete_app', { appId: appToDelete.id })

        // 从前端数组中移除
        apps.value = apps.value.filter(app => app.id !== appToDelete.id)
        if (selectedAppId.value === appToDelete.id) {
          selectedAppId.value = filteredApps.value[0]?.id ?? null
        }
        console.log(`已删除应用: ${appToDelete.name}`)
      } catch (error) {
        console.error('删除应用失败:', error)
        await alertDialog(t('main.alert.deleteAppFailed'), { type: 'error' })
      }
    }
  }
}

const deleteApp = async () => {
  const appToDelete = appContextMenu.value.app
  hideAppContextMenu()
  await deleteSpecificApp(appToDelete)
}

const deleteAllApps = async () => {
  hideAppContextMenu()

  if (await confirmDialog(t('main.confirm.deleteAllApps'))) {
    try {
      // 获取要删除的应用列表
      const appsToDelete = selectedCategory.value === 'all'
        ? apps.value
        : apps.value.filter(app => app.category === selectedCategory.value)

      // 删除每个应用
      for (const app of appsToDelete) {
        await invoke('delete_app', { appId: app.id })
      }

      // 从前端数组中移除
      if (selectedCategory.value === 'all') {
        apps.value = []
      } else {
        apps.value = apps.value.filter(app => app.category !== selectedCategory.value)
      }

      console.log('已删除所有应用')
    } catch (error) {
      console.error('删除应用失败:', error)
      await alertDialog(t('main.alert.deleteAppFailed'), { type: 'error' })
    }
  }
}

const createNewCategory = async () => {
  const newId = Date.now().toString()
  const newCategory: CategoryData = {
    id: newId,
    name: t('main.contextMenu.newCategory'),
    icon: 'icon-apps',
    isDefault: false
  }

  categories.value.push(newCategory)
  hideContextMenu()

  // 保存数据
  await saveAppData()

  // 立即进入重命名模式
  setTimeout(() => {
    renameDialog.value = {
      visible: true,
      newName: newCategory.name,
      categoryId: newId
    }
    setTimeout(() => {
      if (renameInput.value) {
        (renameInput.value as HTMLInputElement).focus();
        (renameInput.value as HTMLInputElement).select()
      }
    }, 50)
  }, 100)
}

const renameCategory = () => {
  if (contextMenu.value.category && !contextMenu.value.category.isDefault) {
    renameDialog.value = {
      visible: true,
      newName: contextMenu.value.category.name,
      categoryId: contextMenu.value.category.id
    }
    hideContextMenu()

    setTimeout(() => {
      if (renameInput.value) {
        (renameInput.value as HTMLInputElement).focus();
        (renameInput.value as HTMLInputElement).select()
      }
    }, 50)
  }
}

const confirmRename = async () => {
  if (renameDialog.value.newName.trim()) {
    const categoryIndex = categories.value.findIndex(cat => cat.id === renameDialog.value.categoryId)
    if (categoryIndex !== -1) {
      categories.value[categoryIndex].name = renameDialog.value.newName.trim()

      // 保存数据
      await saveAppData()
    }
  }
  cancelRename()
}

const cancelRename = () => {
  renameDialog.value = {
    visible: false,
    newName: '',
    categoryId: ''
  }
}

// 编辑应用对话框相关函数
const confirmEditApp = async () => {
  if (editAppDialog.value.app && editAppDialog.value.editedName.trim()) {
    const appIndex = apps.value.findIndex(app => app.id === editAppDialog.value.app!.id)
    if (appIndex !== -1) {
      apps.value[appIndex].name = editAppDialog.value.editedName.trim()
      apps.value[appIndex].category = editAppDialog.value.editedCategory
      apps.value[appIndex].icon = editAppDialog.value.editedIcon
      apps.value[appIndex].target_path = editAppDialog.value.editedTargetPath
      apps.value[appIndex].launch_args = editAppDialog.value.editedLaunchArgs
      apps.value[appIndex].target_type = editAppDialog.value.editedTargetType

      // 如果目标路径改变，更新主路径
      if (editAppDialog.value.editedTargetPath !== apps.value[appIndex].path) {
        apps.value[appIndex].path = editAppDialog.value.editedTargetPath
      }

      // 保存数据
      await saveAppData()
      showToast(t('main.toast.appUpdated'), 'success')
    }
  }
  cancelEditApp()
}

const cancelEditApp = () => {
  editAppDialog.value = {
    visible: false,
    app: null,
    editedName: '',
    editedCategory: '',
    editedIcon: '',
    editedTargetPath: '',
    editedLaunchArgs: '',
    editedTargetType: 'file'
  }
}

// 浏览目标文件或文件夹
const browseTarget = async () => {
  try {
    // 显示选择对话框让用户选择文件或文件夹
    const choice = await confirmDialog(t('common.select'), {
      confirmText: t('common.selectFile'),
      cancelText: t('common.selectFolder')
    })

    let selectedPath = ''
    if (choice) {
      // 选择文件
      const filters = [
        [t('common.allFiles'), ['*']],
        [t('common.executableFiles'), ['exe', 'bat', 'cmd', 'msi']],
        [t('common.scriptFiles'), ['ps1', 'vbs', 'js', 'py']],
        [t('common.shortcutFiles'), ['lnk', 'url']]
      ]
      selectedPath = await invoke('open_file_dialog', {
        title: t('common.selectFile'),
        filters: filters
      }) as string
    } else {
      // 选择文件夹
      selectedPath = await invoke('open_folder_dialog', {
        title: t('common.selectFolder'),
      }) as string
    }

    if (selectedPath) {
      editAppDialog.value.editedTargetPath = selectedPath
      // 自动检测目标类型
      await detectTargetType()
    }
  } catch (error) {
    console.error('浏览文件失败:', error)
    if (error !== t('common.userCancelled')) {
      showToast(t('main.alert.browseFileFailed') + ': ' + error, 'error')
    }
  }
}

// 自动检测目标类型
const detectTargetType = async () => {
  if (!editAppDialog.value.editedTargetPath.trim()) {
    return
  }

  try {
    const targetType = await invoke('detect_target_type', {
      targetPath: editAppDialog.value.editedTargetPath
    }) as string
    editAppDialog.value.editedTargetType = targetType as 'file' | 'folder' | 'url'
  } catch (error) {
    console.error('检测目标类型失败:', error)
    // 默认设为文件类型
    editAppDialog.value.editedTargetType = 'file'
  }
}

// 选择图标
const selectIcon = async () => {
  try {
    const filters = [
      [t('common.imageFiles'), ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'ico', 'svg']],
      [t('common.iconFiles'), ['ico', 'png']],
      [t('common.allFiles'), ['*']]
    ]
    const selectedPath = await invoke('open_file_dialog', {
      title: t('common.selectIcon'),
      filters: filters
    }) as string

    if (selectedPath) {
      // 尝试将图片转换为base64
      try {
        const iconBase64 = await invoke('get_app_icon', { filePath: selectedPath }) as string
        if (iconBase64 && iconBase64.startsWith('data:image/')) {
          editAppDialog.value.editedIcon = iconBase64
        } else {
          // 如果无法转换为base64，直接使用文件路径
          editAppDialog.value.editedIcon = selectedPath
        }
      } catch (iconError) {
        // 如果获取图标失败，直接使用文件路径
        editAppDialog.value.editedIcon = selectedPath
      }
    }
  } catch (error) {
    console.error('选择图标失败:', error)
    if (error !== t('common.userCancelled')) {
      showToast(t('main.alert.selectIconFailed') + ': ' + error, 'error')
    }
  }
}

// 清除图标
const clearIcon = () => {
  editAppDialog.value.editedIcon = ''
}

const deleteCategory = async () => {
  const categoryToDelete = contextMenu.value.category
  hideContextMenu()

  if (categoryToDelete && !categoryToDelete.isDefault) {
    const categoryId = categoryToDelete.id

    // 确认删除操作
    const appsInCategory = apps.value.filter(app => app.category === categoryId)
    const confirmMessage = appsInCategory.length > 0
      ? t('main.confirm.deleteCategory', { name: categoryToDelete.name, count: appsInCategory.length })
      : t('main.confirm.deleteCategoryEmpty', { name: categoryToDelete.name })

    if (!(await confirmDialog(confirmMessage))) {
      return
    }

    // 删除该分类下的所有应用
    for (const app of appsInCategory) {
      try {
        await invoke('delete_app', { appId: app.id })
      } catch (error) {
        console.error('删除应用失败:', error)
      }
    }

    // 从前端数组中移除该分类下的应用
    apps.value = apps.value.filter(app => app.category !== categoryId)

    // 删除分类
    categories.value = categories.value.filter(cat => cat.id !== categoryId)

    // 如果当前选中的分类被删除，切换到"全部应用"
    if (selectedCategory.value === categoryId) {
      await selectCategory('all')
    }

    // 保存数据
    await saveAppData()
  }
}

const deleteAllCategories = async () => {
  hideContextMenu()

  const customCategories = categories.value.filter(cat => !cat.isDefault)
  if (customCategories.length === 0) {
    await alertDialog(t('main.alert.noCustomCategories'), { type: 'info' })
    return
  }

  // 计算要删除的应用数量
  const deletedCategoryIds = customCategories.map(cat => cat.id)
  const appsToDelete = apps.value.filter(app => deletedCategoryIds.includes(app.category))

  const confirmMessage = appsToDelete.length > 0
    ? t('main.confirm.deleteAllCategories', { groupCount: customCategories.length, appCount: appsToDelete.length })
    : t('main.confirm.deleteAllCategoriesEmpty', { groupCount: customCategories.length })

  if (!(await confirmDialog(confirmMessage))) {
    return
  }

  // 删除所有自定义分组下的应用
  for (const app of appsToDelete) {
    try {
      await invoke('delete_app', { appId: app.id })
    } catch (error) {
      console.error('删除应用失败:', error)
    }
  }

  // 从前端数组中移除被删除分组下的应用
  apps.value = apps.value.filter(app => !deletedCategoryIds.includes(app.category))

  // 只保留默认分组
  categories.value = categories.value.filter(cat => cat.isDefault)

  // 切换到"全部应用"
  await selectCategory('all')

  // 保存数据
  await saveAppData()
}

// 拖拽调整侧栏宽度
const startResize = (e: MouseEvent) => {
  if (appSettings.value.windowLayout === 'vertical') {
    return
  }

  isResizing.value = true

  // 如果当前是自适应状态，先获取当前实际宽度
  if (sidebarWidth.value === 0) {
    const sidebar = document.querySelector('.sidebar') as HTMLElement
    if (sidebar) {
      const rect = sidebar.getBoundingClientRect()
      sidebarWidth.value = rect.width
    }
  }

  document.addEventListener('mousemove', resize)
  document.addEventListener('mouseup', stopResize)
  e.preventDefault()
}

const resize = (e: MouseEvent) => {
  if (!isResizing.value) return

  const newWidth = e.clientX
  if (newWidth > SIDEBAR_WIDTH_MIN && newWidth < 200) {
    sidebarWidth.value = newWidth
  }
}

const stopResize = () => {
  isResizing.value = false
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
  invoke('update_sidebar_width', { sidebarWidth: Math.round(sidebarWidth.value) })
    .catch(error => console.error('保存侧栏宽度失败:', error))
}

const processDroppedPaths = async (paths: string[]) => {
  const validPaths = paths
    .map(path => path.trim())
    .filter(path => path.length > 0)

  if (validPaths.length === 0) {
    console.log('没有检测到有效文件路径')
    return
  }

  console.log('拖拽文件数量:', validPaths.length)

  for (const filePath of validPaths) {
    await handleFileDrop(filePath)
  }

  console.log('拖拽处理完成，当前应用总数:', apps.value.length)
}

const normalizeDroppedPath = (rawPath: string) => {
  let normalizedPath = rawPath.trim()

  if (!normalizedPath) {
    return ''
  }

  if (normalizedPath.startsWith('file:///')) {
    normalizedPath = decodeURIComponent(normalizedPath.slice('file:///'.length))
  } else if (normalizedPath.startsWith('file://')) {
    normalizedPath = `\\${decodeURIComponent(normalizedPath.slice('file://'.length)).replace(/\//g, '\\')}`
  }

  if (/^\/[A-Za-z]:\//.test(normalizedPath)) {
    normalizedPath = normalizedPath.slice(1)
  }

  if (/^[A-Za-z]:\//.test(normalizedPath)) {
    normalizedPath = normalizedPath.replace(/\//g, '\\')
  }

  return normalizedPath
}

const parseDroppedPathList = (rawValue: string) => {
  return rawValue
    .split(/\r?\n/)
    .map(line => line.trim())
    .filter(line => line.length > 0 && !line.startsWith('#'))
    .map(normalizeDroppedPath)
    .filter(path => path.length > 0)
}

const collectDroppedPaths = async (dragEvent: DragEvent) => {
  const files = dragEvent.dataTransfer?.files

  const directPaths = Array.from(files || [])
    .map(file => normalizeDroppedPath((file as File & { path?: string }).path || ''))
    .filter(path => path.length > 0)

  if (files && files.length > 0 && directPaths.length === files.length) {
    return directPaths
  }

  const rawPathValues: string[] = []
  const uriList = dragEvent.dataTransfer?.getData('text/uri-list') || ''
  const plainText = dragEvent.dataTransfer?.getData('text/plain') || ''

  if (uriList) {
    rawPathValues.push(...parseDroppedPathList(uriList))
  }

  if (plainText) {
    rawPathValues.push(...parseDroppedPathList(plainText))
  }

  const items = dragEvent.dataTransfer?.items
  if (items) {
    for (const item of Array.from(items)) {
      if (item.kind !== 'string') {
        continue
      }

      const value = await new Promise<string>(resolve => item.getAsString(resolve))
      rawPathValues.push(...parseDroppedPathList(value))
    }
  }

  const uniquePaths = [...new Set([...directPaths, ...rawPathValues])]
  return uniquePaths
}

const updateViewportWidth = () => {
  windowWidth.value = window.innerWidth
}

let saveGridCellSizeTimer: number | null = null

const saveGridCellSizeThrottled = () => {
  if (saveGridCellSizeTimer) {
    clearTimeout(saveGridCellSizeTimer)
  }

  saveGridCellSizeTimer = setTimeout(async () => {
    try {
      await invoke('update_icon_size', { iconSize: appSettings.value.gridCellSize })
      const { emit } = await import('@tauri-apps/api/event')
      await emit('settings-updated')
    } catch (error) {
      console.error('保存格子大小失败:', error)
    }
  }, 250)
}

const adjustGridCellSize = (delta: number) => {
  const nextSize = clampGridCellSize(appSettings.value.gridCellSize + delta)
  if (nextSize === appSettings.value.gridCellSize) {
    return
  }

  appSettings.value.gridCellSize = nextSize
  saveGridCellSizeThrottled()
}

const handleLauncherWheel = (event: WheelEvent) => {
  if (!event.ctrlKey) {
    return
  }

  event.preventDefault()
  adjustGridCellSize(event.deltaY < 0 ? GRID_CELL_SIZE_STEP : -GRID_CELL_SIZE_STEP)
}

// 生命周期
onMounted(async () => {
  console.log('开始初始化应用...')

  // 并行加载数据和设置，无需等待 500ms
  const [appDataResult, settingsResult] = await Promise.allSettled([
    loadAppData(),
    loadAppSettings()
  ])

  // 处理加载结果
  if (appDataResult.status === 'rejected') {
    console.error('加载应用数据失败:', appDataResult.reason)
  }
  if (settingsResult.status === 'rejected') {
    console.error('加载应用设置失败:', settingsResult.reason)
  }
  applyRuntimeSettings()

  // 数据加载完成，隐藏加载指示器
  isLoading.value = false
  console.log('应用初始化完成')

  // 异步设置窗口大小和监听器，不阻塞主流程
  setTimeout(async () => {
    // 恢复窗口大小
    if (appSettings.value.windowWidth && appSettings.value.windowHeight) {
      try {
        const window = getCurrentWindow()
        await window.setSize(new LogicalSize(
          appSettings.value.windowWidth,
          appSettings.value.windowHeight
        ))
        console.log(`恢复窗口大小: ${appSettings.value.windowWidth}x${appSettings.value.windowHeight}`)
      } catch (error) {
        console.error('恢复窗口大小失败:', error)
      }
    }

    // 窗口大小保存防抖变量
    let resizeTimeout: number | null = null

    // 监听窗口大小变化
    try {
      const window = getCurrentWindow()
      await window.listen('tauri://resize', async () => {
        try {
          // 获取后端转换为逻辑像素的主窗口大小，避免 DPI 缩放造成的物理/逻辑像素混淆
          const logicalSize = await invoke('get_main_window_size')

          // defensive parsing: server returns [width, height] or object
          let lw = null
          let lh = null
          if (Array.isArray(logicalSize) && logicalSize.length >= 2) {
            lw = logicalSize[0]
            lh = logicalSize[1]
          } else if (logicalSize && typeof logicalSize === 'object') {
            // logicalSize may be an object with numeric keys or width/height props
            const obj = logicalSize as Record<string, any>
            lw = (Array.isArray(logicalSize) ? logicalSize[0] : undefined) ?? obj.width ?? obj['0']
            lh = (Array.isArray(logicalSize) ? logicalSize[1] : undefined) ?? obj.height ?? obj['1']
          }

          if (lw == null || lh == null) {
            console.error('无法从后端获取逻辑窗口尺寸', logicalSize)
            return
          }

          windowWidth.value = Number(lw)

          // 防抖处理：清除之前的定时器，设置新的定时器
          if (resizeTimeout) {
            clearTimeout(resizeTimeout)
          }

          // 延迟500ms后保存窗口大小，避免频繁保存
          resizeTimeout = setTimeout(async () => {
            try {
              await invoke('save_window_size', {
                width: lw,
                height: lh
              })
              console.log(`保存窗口逻辑大小: ${lw}x${lh}`)
              // 通知前端其他组件（例如设置面板）窗口大小已改变（使用逻辑尺寸）
              try {
                if (typeof globalThis !== 'undefined' && typeof (globalThis as any).dispatchEvent === 'function') {
                  globalThis.dispatchEvent(new CustomEvent('window-size-changed', { detail: { width: lw, height: lh } }))
                }
              } catch (e) {
                console.error('触发 window-size-changed 事件失败:', e)
              }
            } catch (error) {
              console.error('保存窗口大小失败:', error)
            }
          }, 500)
        } catch (error) {
          console.error('处理窗口大小变化失败:', error)
        }
      })
      console.log('窗口大小监听器设置成功')
    } catch (error) {
      console.error('设置窗口大小监听器失败:', error)
    }

    // 监听来自托盘菜单的事件
    const { listen } = await import('@tauri-apps/api/event')
    await listen('toggle-prevent-auto-hide', () => {
      togglePreventAutoHide()
    })

    // 监听来自托盘的设置变化事件
    await listen('prevent-auto-hide-changed', (event: any) => {
      appSettings.value.preventAutoHide = event.payload
      console.log('从托盘菜单接收到设置变化:', event.payload)

      // 显示状态反馈
      const message = event.payload
        ? t('main.toast.preventAutoHideEnabled')
        : t('main.toast.preventAutoHideDisabled')
      showToast(message, 'success')
    })

    // 监听数据更新事件
    await listen('data-updated', () => {
      console.log('收到数据更新通知，重新加载数据')
      loadAppData()
    })

    await listen('settings-updated', async () => {
      console.log('收到设置更新通知，重新加载设置')
      await loadAppSettings()
      applyRuntimeSettings()
    })
  }, 100) // 延迟100ms执行，让界面先渲染

  // 立即计算侧栏宽度和设置事件监听器
  nextTick(() => {
    if (appSettings.value.sidebarWidth > 0) {
      sidebarWidth.value = appSettings.value.sidebarWidth
    } else {
      // 计算侧栏的自然宽度
      const sidebar = document.querySelector('.sidebar') as HTMLElement
      if (sidebar) {
      // 先让侧栏自适应，然后获取其实际宽度
        sidebar.style.width = 'auto'
        const rect = sidebar.getBoundingClientRect()
        sidebarWidth.value = Math.max(rect.width, SIDEBAR_WIDTH_MIN)
      }
    }

    // 添加全局点击监听，点击搜索框外部时隐藏搜索框
    const handleClickOutside = (event: Event) => {
      const target = event.target as HTMLElement

      // 任何点击都算用户交互
      handleFirstUserInteraction()

      if (showSearchBox.value &&
        !target.closest('.content-header') &&
        !target.closest('.titlebar-button')) {
        hideSearchBox()
      }

      // 隐藏右键菜单 - 统一处理所有菜单的隐藏
      if (!target.closest('.context-menu')) {
        if (contextMenu.value.visible) {
          hideContextMenu()
        }
        if (appContextMenu.value.visible) {
          hideAppContextMenu()
        }
        if (moveToSubmenu.value.visible) {
          hideMoveToSubmenu()
        }
        if (copyToSubmenu.value.visible) {
          hideCopyToSubmenu()
        }
        if (gridContextMenu.value.visible) {
          hideGridContextMenu()
        }
      }
    }
    document.addEventListener('click', handleClickOutside)

    // 添加全局键盘监听，实现直接输入搜索
    document.addEventListener('keydown', handleGlobalKeydown)

    // 全局禁用右键菜单
    document.addEventListener('contextmenu', disableContextMenu)

    // 添加窗口失焦监听，自动隐藏到托盘
    window.addEventListener('blur', handleWindowBlur)

    // 添加窗口聚焦事件监听器
    window.addEventListener('focus', handleWindowFocus)

    // 跟随窗口宽度变化重新计算网格列数
    window.addEventListener('resize', updateViewportWidth)

    // 添加鼠标移动和离开事件监听器
    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseleave', handleMouseLeave)

    // 添加全局 mouseup 事件监听器，确保拖拽结束能被正确检测
    document.addEventListener('mouseup', handleDragEnd)

    // 异步设置拖拽功能，不阻塞主流程
    setTimeout(() => {
      setupDragAndDrop()
    }, 200)
  })

  // 异步更新托盘菜单，不阻塞主流程
  setTimeout(async () => {
    try {
      await invoke('update_tray_menu', {
        preventAutoHide: appSettings.value.preventAutoHide
      })
      console.log('托盘菜单已更新以反映当前设置')
    } catch (error) {
      console.error('更新托盘菜单失败:', error)
    }
  }, 300)
})

// 禁用右键菜单的函数
const disableContextMenu = (e: Event) => {
  e.preventDefault()
  return false
}

// 全局键盘事件处理函数
const handleGlobalKeydown = (event: KeyboardEvent) => {
  // 任何键盘操作都算用户交互
  handleFirstUserInteraction()

  // 检查是否正在编辑状态（对话框打开、输入框聚焦等）
  const isEditing = document.activeElement?.tagName === 'INPUT' ||
    document.activeElement?.tagName === 'TEXTAREA' ||
    document.activeElement?.tagName === 'SELECT' ||
    document.querySelector('.dialog-overlay') ||
    contextMenu.value.visible ||
    appContextMenu.value.visible ||
    moveToSubmenu.value.visible ||
    copyToSubmenu.value.visible ||
    gridContextMenu.value.visible

  // 如果正在编辑，不处理
  if (isEditing) {
    return
  }

  // ESC键隐藏搜索框
  if (event.key === 'Escape') {
    if (showSearchBox.value) {
      hideSearchBox()
      event.preventDefault()
    }
    return
  }

  // Ctrl+F 或 F3 快捷键打开搜索
  if ((event.ctrlKey && event.key === 'f') || event.key === 'F3') {
    if (!showSearchBox.value) {
      toggleSearch()
    }
    event.preventDefault()
    return
  }

  if (event.key === 'ArrowRight') {
    moveSelectedApp(1)
    event.preventDefault()
    return
  }

  if (event.key === 'ArrowLeft') {
    moveSelectedApp(-1)
    event.preventDefault()
    return
  }

  if (event.key === 'ArrowDown') {
    moveSelectedApp(getAppGridColumnCount())
    event.preventDefault()
    return
  }

  if (event.key === 'ArrowUp') {
    moveSelectedApp(-getAppGridColumnCount())
    event.preventDefault()
    return
  }

  if (event.key === 'Enter') {
    const selectedApp = getSelectedApp()
    if (selectedApp) {
      void launchApp(selectedApp)
      event.preventDefault()
    }
    return
  }

  if (event.key === 'Delete') {
    const selectedApp = getSelectedApp()
    if (selectedApp) {
      void deleteSpecificApp(selectedApp)
      event.preventDefault()
    }
    return
  }

  // 检查是否是可打印字符（字母、数字、部分符号等）
  const isPrintableChar = event.key.length === 1 &&
    !event.ctrlKey &&
    !event.altKey &&
    !event.metaKey &&
    // 排除一些特殊字符
    !/[\s\t\n\r]/.test(event.key)

  if (isPrintableChar) {
    // 如果搜索框未显示，显示它并添加字符
    if (!showSearchBox.value) {
      showSearchBox.value = true
      searchQuery.value = event.key

      // 聚焦到搜索框
      nextTick(() => {
        if (searchInputRef.value) {
          const input = searchInputRef.value as HTMLInputElement
          input.focus()
          // 将光标移到末尾
          input.setSelectionRange(1, 1)
        }
      })

      event.preventDefault()
    }
  }
}

// 鼠标位置追踪
const mousePosition = ref({ x: 0, y: 0 })
const isMouseInWindow = ref(true)
const isDraggingWindow = ref(false)
const windowJustShown = ref(false) // 追踪窗口是否刚刚显示
const userInteracted = ref(false) // 追踪用户是否已经与窗口交互过

// 处理用户首次点击窗口
const handleFirstUserInteraction = () => {
  if (!userInteracted.value) {
    userInteracted.value = true
    windowJustShown.value = false // 用户交互后，立即允许自动隐藏
    console.log('用户首次交互，开始监听失去焦点事件')
  }
}

// 追踪鼠标位置
const handleMouseMove = (event: MouseEvent) => {
  mousePosition.value = { x: event.clientX, y: event.clientY }
  isMouseInWindow.value = true
  // 鼠标移动也算用户交互
  handleFirstUserInteraction()
}

// 鼠标离开窗口
const handleMouseLeave = () => {
  isMouseInWindow.value = false
}

// 处理窗口拖拽开始
const handleDragStart = () => {
  isDraggingWindow.value = true
  console.log('开始拖拽窗口')
}

// 处理窗口拖拽结束
const handleDragEnd = () => {
  // 延迟重置拖拽状态，确保拖拽完全结束
  setTimeout(() => {
    isDraggingWindow.value = false
    console.log('结束拖拽窗口')
  }, 200)
}

// 窗口聚焦处理函数
const handleWindowFocus = () => {
  console.log('窗口获得焦点')
  // 重置用户交互状态，需要等待新的用户交互
  userInteracted.value = false
  windowJustShown.value = true
  console.log('窗口显示，等待用户交互')
}

// 窗口失焦处理函数
const handleWindowBlur = async () => {
  // 检查设置窗口是否打开
  let isSettingsOpen = false
  try {
    isSettingsOpen = await invoke('is_settings_window_open')
  } catch (error) {
    console.error('检查设置窗口状态失败:', error)
  }

  // 如果设置窗口打开，则不执行自动隐藏
  if (isSettingsOpen) {
    console.log('设置窗口打开中，不执行自动隐藏')
    return
  }

  // 只有在没有阻止自动隐藏且（用户已交互或窗口刚刚显示）的情况下才隐藏窗口
  // 这保证了通过快捷键显示窗口后，用户点击窗口外部能立即触发自动隐藏
  if (!appSettings.value.preventAutoHide && (userInteracted.value || windowJustShown.value)) {
    // 延迟检查，给鼠标事件时间更新状态
    const delay = 100

    setTimeout(async () => {
      // 只有当鼠标不在窗口内且不在拖动窗口时才隐藏窗口
      if (!isMouseInWindow.value && !isDraggingWindow.value) {
        try {
          console.log('窗口失去焦点且鼠标不在窗口内且未拖动窗口，隐藏到托盘')
          const { getCurrentWindow } = await import('@tauri-apps/api/window')
          const currentWindow = getCurrentWindow()
          await currentWindow.hide()
        } catch (error) {
          console.error('隐藏窗口失败:', error)
        }
      } else {
        console.log('窗口失去焦点但鼠标仍在窗口内或正在拖动窗口，不隐藏窗口')
      }
    }, delay)
  } else if (!userInteracted.value) {
    console.log('用户尚未交互，不执行自动隐藏')
  }
}

onUnmounted(() => {
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
  // 清理全局右键菜单禁用监听器
  document.removeEventListener('contextmenu', disableContextMenu)

  // 清理拖拽功能
  cleanupDragAndDrop()

  // 清理全局键盘监听器
  document.removeEventListener('keydown', handleGlobalKeydown)

  // 清理窗口失焦监听器
  window.removeEventListener('blur', handleWindowBlur)

  // 清理窗口聚焦监听器
  window.removeEventListener('focus', handleWindowFocus)

  // 清理窗口尺寸监听器
  window.removeEventListener('resize', updateViewportWidth)

  if (saveGridCellSizeTimer) {
    clearTimeout(saveGridCellSizeTimer)
  }

  // 清理鼠标事件监听器
  document.removeEventListener('mousemove', handleMouseMove)
  document.removeEventListener('mouseleave', handleMouseLeave)
  document.removeEventListener('mouseup', handleDragEnd)
})

// 标题栏相关方法
const toggleSearch = () => {
  // 切换搜索框的显示/隐藏
  showSearchBox.value = !showSearchBox.value

  // 如果显示搜索框，则聚焦到输入框
  if (showSearchBox.value) {
    setTimeout(() => {
      if (searchInputRef.value) {
        (searchInputRef.value as HTMLInputElement).focus()
      }
    }, 50)
  } else {
    // 隐藏时清空搜索内容
    searchQuery.value = ''
  }
}

const hideSearchBox = () => {
  showSearchBox.value = false
  searchQuery.value = ''
}

// 搜索框键盘事件处理
const handleSearchKeydown = (event: KeyboardEvent) => {
  // Enter键启动第一个搜索结果
  if (event.key === 'Enter') {
    const firstApp = filteredApps.value[0]
    if (firstApp) {
      selectedAppId.value = firstApp.id
      launchApp(firstApp)
      hideSearchBox()
    }
    event.preventDefault()
  }
  if (event.key === 'ArrowRight') {
    moveSelectedApp(1)
    event.preventDefault()
  }
  if (event.key === 'ArrowLeft') {
    moveSelectedApp(-1)
    event.preventDefault()
  }
  if (event.key === 'ArrowDown') {
    moveSelectedApp(getAppGridColumnCount())
    event.preventDefault()
  }
  if (event.key === 'ArrowUp') {
    moveSelectedApp(-getAppGridColumnCount())
    event.preventDefault()
  }
}

const toggleMenu = (e: MouseEvent) => {
  // 显示主菜单
  showMainMenu(e)
}

// 设置相关方法已移除，现在使用独立设置窗口

const closeApp = async () => {
  console.log('关闭/隐藏 应用 被调用，调用后端切换窗口可见性')
  try {
    await invoke('toggle_window_visibility')
    console.log('已请求后端切换窗口可见性')
  } catch (error) {
    console.error('请求切换窗口可见性失败:', error)
    showToast(t('main.toast.toggleVisibilityFailed'), 'error')
  }
}

// 拖拽处理函数
const handleDragEnter = (e: Event) => {
  e.preventDefault()

  if (!isExternalFileDrag(e)) {
    return
  }

  console.log('拖拽进入事件触发')
  dragCounter.value++
  isDragOver.value = true
}

const handleDragLeave = (e: Event) => {
  if (!isExternalFileDrag(e)) {
    return
  }

  console.log('拖拽离开事件触发')
  e.preventDefault()
  dragCounter.value--
  if (dragCounter.value === 0) {
    isDragOver.value = false
  }
}

const handleDragOver = (e: Event) => {
  // 无论内外部拖拽都必须 preventDefault，否则浏览器显示禁止光标
  e.preventDefault()

  if (!isExternalFileDrag(e)) {
    return
  }

  console.log('拖拽悬停事件触发')
}

const handleDrop = async (e: Event) => {
  if (!isExternalFileDrag(e)) {
    return
  }

  e.preventDefault()
  isDragOver.value = false
  dragCounter.value = 0

  console.log('拖拽释放事件触发')

  const dragEvent = e as DragEvent
  const droppedPaths = await collectDroppedPaths(dragEvent)
  if (droppedPaths.length === 0) {
    console.log('没有检测到文件')
    return
  }

  await processDroppedPaths(droppedPaths)
}

const setupTauriFileDrop = async () => {
  if (unlistenTauriFileDrop) {
    return
  }

  try {
    unlistenTauriFileDrop = await getCurrentWebview().onDragDropEvent(async (event) => {
      if (isDraggingApp) {
        return
      }

      const payload = event.payload

      if (payload.type === 'enter') {
        console.log('Tauri 文件拖拽进入事件触发', payload.paths)
        dragCounter.value = 1
        isDragOver.value = true
        return
      }

      if (payload.type === 'over') {
        isDragOver.value = true
        return
      }

      if (payload.type === 'leave') {
        console.log('Tauri 文件拖拽离开事件触发')
        dragCounter.value = 0
        isDragOver.value = false
        return
      }

      console.log('Tauri 文件拖拽释放事件触发', payload.paths)
      dragCounter.value = 0
      isDragOver.value = false

      const droppedPaths = payload.paths
        .map(normalizeDroppedPath)
        .filter(path => path.length > 0)

      if (droppedPaths.length === 0) {
        console.log('Tauri 没有检测到文件')
        return
      }

      await processDroppedPaths(droppedPaths)
    })

    console.log('Tauri 文件拖拽监听器设置成功')
  } catch (error) {
    console.error('设置 Tauri 文件拖拽监听器失败:', error)
  }
}

// 处理单个文件拖拽的函数
const handleFileDrop = async (filePath: string) => {
  console.log('处理文件:', filePath)
  try {
    // 检查 invoke 函数是否可用
    console.log('检查 invoke 函数可用性...')
    if (typeof invoke !== 'function') {
      throw new Error('invoke 函数不可用')
    }

    // 调用 Rust 后端获取文件信息
    console.log('调用get_file_info API...')
    const fileInfo = await invoke('get_file_info', { filePath: filePath }) as any
    console.log('文件信息获取成功:', fileInfo)

    // 创建新的应用项
    const defaultCategory = getDefaultCategoryForNewApp()
    const newApp: AppData = {
      id: Date.now() + Math.floor(Math.random() * 1000), // 避免ID冲突，使用整数
      name: fileInfo.name,
      category: selectedCategory.value === 'all' ? (defaultCategory || 'all') : selectedCategory.value,
      icon: fileInfo.icon || '', // 使用后端返回的图标标识符
      path: fileInfo.path,
      target_path: fileInfo.target_path,
      is_shortcut: fileInfo.is_shortcut,
      launch_args: '', // 默认无启动参数
      target_type: 'file' // 默认为文件类型
    }

    console.log('创建新应用项:', newApp)
    apps.value.push(newApp)
    console.log('应用已添加到数组，当前应用数量:', apps.value.length)

    // 尝试获取真实应用图标
    try {
      console.log('尝试获取应用真实图标...')
      // 对于快捷方式，使用目标路径获取图标；对于普通文件，使用原路径
      const iconPath = fileInfo.is_shortcut && fileInfo.target_path ? fileInfo.target_path : filePath
      console.log('获取图标的路径:', iconPath)

      const realIcon = await invoke('get_app_icon', { filePath: iconPath }) as string
      if (realIcon && realIcon.startsWith('data:image/png;base64,')) {
        // 更新应用图标
        const appIndex = apps.value.findIndex(app => app.id === newApp.id)
        if (appIndex !== -1) {
          apps.value[appIndex].icon = realIcon
          console.log('应用图标已更新为真实图标')
        }
      }
    } catch (iconError) {
      console.log('获取真实图标失败，使用默认图标:', iconError)
    }

    // 自动保存数据
    console.log('开始保存数据...')
    await saveAppData()
    console.log('数据保存完成')

    // 拖入行为仅添加到列表并保存，不自动打开编辑对话框。
    // 只有通过右键新建或在应用上选择编辑时才会打开编辑对话框。

  } catch (error) {
    console.error('处理文件失败:', error)
    // 可以显示错误提示
    await alertDialog(`${t('app.cannotAddFile', { path: filePath })}: ${error}`, { type: 'error' })
  }
}

// 在 onMounted 中添加拖拽事件监听器
const setupAppItemDragEvents = () => {
  console.log('图标排序使用 pointer 事件，无需绑定 HTML5 drop 事件')
}

const isExternalFileDrag = (e: Event) => {
  if (isDraggingApp) {
    return false
  }

  const dragEvent = e as DragEvent
  const types = dragEvent.dataTransfer?.types

  if (!types) {
    return false
  }

  return Array.from(types).includes('Files')
}

const setupDragAndDrop = () => {
  console.log('设置拖拽功能...')

  // 设置图标拖拽排序的原生事件监听
  setupAppItemDragEvents()

  // Tauri 文件拖放事件能提供真实本地路径；HTML5 drop 仅作为兜底。
  void setupTauriFileDrop()

  // DOM 拖拽事件（外部文件拖入 + 兜底）
  const appContainer = document.querySelector('.app-container') as HTMLElement
  const launcherContainer = document.querySelector('.launcher-container') as HTMLElement
  const mainContent = document.querySelector('.main-content') as HTMLElement

  console.log('DOM元素查找结果:', {
    appContainer: !!appContainer,
    launcherContainer: !!launcherContainer,
    mainContent: !!mainContent
  })

  // 优先绑定到主内容区域，如果不存在则绑定到应用容器
  const targetElement = mainContent || launcherContainer || appContainer

  if (targetElement) {
    console.log('绑定传统拖拽事件到:', targetElement.className)
    targetElement.addEventListener('dragenter', handleDragEnter)
    targetElement.addEventListener('dragleave', handleDragLeave)
    targetElement.addEventListener('dragover', handleDragOver)
    targetElement.addEventListener('drop', handleDrop)
  } else {
    console.error('未找到合适的DOM元素来绑定拖拽事件')
  }
}

// 在 onUnmounted 中清理拖拽事件监听器
const cleanupDragAndDrop = () => {
  console.log('清理拖拽功能...')

  if (unlistenTauriFileDrop) {
    unlistenTauriFileDrop()
    unlistenTauriFileDrop = null
  }

  const appContainer = document.querySelector('.app-container') as HTMLElement
  const launcherContainer = document.querySelector('.launcher-container') as HTMLElement
  const mainContent = document.querySelector('.main-content') as HTMLElement

  // 从所有可能绑定过事件的元素上移除监听器
  const elements = [mainContent, launcherContainer, appContainer].filter(Boolean)

  elements.forEach(element => {
    if (element) {
      element.removeEventListener('dragenter', handleDragEnter)
      element.removeEventListener('dragleave', handleDragLeave)
      element.removeEventListener('dragover', handleDragOver)
      element.removeEventListener('drop', handleDrop)
    }
  })
}

// ===== 图标拖拽排序相关函数 =====

const saveAppsOrder = async () => {
  try {
    await invoke('save_apps_order', { apps: apps.value })
    console.log('应用排序已保存')
  } catch (error) {
    console.error('保存应用排序失败:', error)
  }
}

const reorderApps = async (sourceApp: AppData, sourceIndex: number, targetIndex: number) => {
  if (sourceIndex === targetIndex || targetIndex < 0) {
    return
  }

  const currentApps = selectedCategory.value === 'all'
    ? [...apps.value]
    : apps.value.filter(app => app.category === selectedCategory.value)

  if (sourceIndex < 0 || sourceIndex >= currentApps.length || targetIndex >= currentApps.length) {
    return
  }

  // 从源位置移除
  currentApps.splice(sourceIndex, 1)
  // 插入到目标位置
  currentApps.splice(targetIndex, 0, sourceApp)

  // 更新所有相关应用的 order 值
  currentApps.forEach((app, idx) => {
    const originalApp = apps.value.find(a => a.id === app.id)
    if (originalApp) {
      originalApp.order = idx
    }
  })

  await saveAppsOrder()
}

const getAppItemIndexAtPoint = (x: number, y: number) => {
  const target = document.elementFromPoint(x, y) as HTMLElement | null
  const appItem = target?.closest('.app-item') as HTMLElement | null
  if (!appItem) {
    return -1
  }

  return parseInt(appItem.dataset.appIndex || '-1')
}

const updatePointerDragTarget = (targetIndex: number) => {
  document.querySelectorAll('.app-item.drag-target').forEach(el => {
    el.classList.remove('drag-target')
  })

  if (targetIndex < 0 || targetIndex === pointerDragState?.sourceIndex) {
    return
  }

  const targetItem = document.querySelector(`.app-item[data-app-index="${targetIndex}"]`)
  targetItem?.classList.add('drag-target')
}

const startPointerAppSort = (event: PointerEvent) => {
  if (!pointerDragState || pointerDragState.dragging) {
    return
  }

  pointerDragState.dragging = true
  isDraggingApp = true
  suppressNextAppClick = true

  const appItem = document.querySelector(`.app-item[data-app-index="${pointerDragState.sourceIndex}"]`)
  appItem?.classList.add('dragging')

  document.body.classList.add('app-sort-dragging')
  event.preventDefault()
  console.log('指针拖拽排序开始', {
    app: pointerDragState.app.name,
    index: pointerDragState.sourceIndex
  })
}

const handleAppPointerMove = (event: PointerEvent) => {
  if (!pointerDragState || pointerDragState.pointerId !== event.pointerId) {
    return
  }

  const distance = Math.hypot(
    event.clientX - pointerDragState.startX,
    event.clientY - pointerDragState.startY
  )

  if (!pointerDragState.dragging && distance >= appSortDragThreshold) {
    startPointerAppSort(event)
  }

  if (!pointerDragState.dragging) {
    return
  }

  event.preventDefault()
  const targetIndex = getAppItemIndexAtPoint(event.clientX, event.clientY)
  pointerDragState.targetIndex = targetIndex
  updatePointerDragTarget(targetIndex)
}

const handleAppPointerUp = async (event: PointerEvent) => {
  if (!pointerDragState || pointerDragState.pointerId !== event.pointerId) {
    return
  }

  const dragState = pointerDragState

  if (dragState.dragging) {
    event.preventDefault()
    event.stopPropagation()
    await reorderApps(dragState.app, dragState.sourceIndex, dragState.targetIndex)
  }

  clearDragState()
}

const handleAppPointerDown = (event: PointerEvent, app: AppData, index: number) => {
  if (event.button !== 0) {
    return
  }

  pointerDragState = {
    app,
    sourceIndex: index,
    targetIndex: index,
    startX: event.clientX,
    startY: event.clientY,
    pointerId: event.pointerId,
    dragging: false
  }

  document.addEventListener('pointermove', handleAppPointerMove)
  document.addEventListener('pointerup', handleAppPointerUp)
  document.addEventListener('pointercancel', handleAppPointerUp)
}

// 清除拖拽状态
const clearDragState = () => {
  isDraggingApp = false
  pointerDragState = null

  document.removeEventListener('pointermove', handleAppPointerMove)
  document.removeEventListener('pointerup', handleAppPointerUp)
  document.removeEventListener('pointercancel', handleAppPointerUp)
  document.body.classList.remove('app-sort-dragging')

  document.querySelectorAll('.app-item.dragging').forEach(el => {
    el.classList.remove('dragging')
  })
  document.querySelectorAll('.app-item.drag-target').forEach(el => {
    el.classList.remove('drag-target')
  })

  setTimeout(() => {
    suppressNextAppClick = false
  }, 0)
}
</script>

<style scoped>
.app-container {
  --app-bg: #f5f5f5;
  --surface-bg: #ffffff;
  --surface-hover: #f8fafc;
  --text-color: #2c3e50;
  --muted-text: #7f8c8d;
  --border-color: #e0e0e0;
  --accent-color: #3498db;
  --titlebar-bg: #2c3e50;
  --sidebar-bg: #2c3e50;
  --sidebar-hover: #34495e;
  --shadow-color: rgba(0, 0, 0, 0.12);
  --transition-speed: 0.2s;
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  /* 减少重排 */
  contain: layout;
}

.app-container.theme-dark {
  --app-bg: #111827;
  --surface-bg: #1f2937;
  --surface-hover: #263244;
  --text-color: #e5e7eb;
  --muted-text: #9ca3af;
  --border-color: #374151;
  --accent-color: #60a5fa;
  --titlebar-bg: #0f172a;
  --sidebar-bg: #111827;
  --sidebar-hover: #1f2937;
  --shadow-color: rgba(0, 0, 0, 0.35);
}

.app-container.animation-slow {
  --transition-speed: 0.35s;
}

.app-container.animation-fast {
  --transition-speed: 0.1s;
}

.app-container.animations-disabled *,
.app-container.animations-disabled *::before,
.app-container.animations-disabled *::after {
  transition: none !important;
  animation: none !important;
}

/* 自定义标题栏样式 */
.titlebar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 32px;
  background: var(--titlebar-bg);
  color: white;
  padding: 0 16px;
  user-select: none;
  -webkit-user-select: none;
  position: relative;
  z-index: 1000;
  /* 使用 transform 而不是改变位置 */
  will-change: transform;
}

.titlebar-left {
  display: flex;
  align-items: center;
  flex: 1;
  cursor: move;
}

.app-title {
  font-size: 14px;
  font-weight: 500;
}

.titlebar-right {
  display: flex;
  align-items: center;
  gap: 4px;
}

.titlebar-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  color: white;
  cursor: pointer;
  transition: background-color var(--transition-speed) ease;
  font-size: 12px;
  position: relative;
  z-index: 1001;
  pointer-events: auto;
  /* 优化动画性能 */
  will-change: background-color;
}

.titlebar-button:hover {
  background: rgba(255, 255, 255, 0.1);
}

.titlebar-close:hover {
  background: #e74c3c;
}

.launcher-container {
  display: flex;
  flex: 1;
  background: var(--app-bg);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  position: relative;
  /* Ensure children with overflow can scroll inside this flex container */
  overflow: hidden;
  min-height: 0;
  /* 减少重排 */
  contain: layout;
}

.layout-vertical .launcher-container {
  flex-direction: column;
}

/* 侧栏样式 */
.sidebar {
  background: var(--sidebar-bg);
  color: white;
  width: auto;
  min-width: 72px;
  max-width: 200px;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
  /* 优化性能 */
  contain: layout;
  will-change: width;
}

.layout-vertical .sidebar {
  width: 100%;
  min-width: 0;
  max-width: none;
  min-height: 42px;
  max-height: 120px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid #34495e;
}

.sidebar-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.sidebar-content {
  flex: 1;
  padding: 10px 0;
  /* 优化滚动性能 */
  contain: layout style;
}

.layout-vertical .sidebar-content {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 8px;
  overflow-x: auto;
  overflow-y: hidden;
}

.category-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  white-space: nowrap;
  /* 优化悬停效果 */
  will-change: background-color;
}

.category-item:hover {
  background: var(--sidebar-hover);
}

.category-item.active {
  background: var(--accent-color);
}

.category-item span {
  flex: 1;
  min-width: 0;
  font-size: 13px;
}

.layout-vertical .category-item {
  flex: 0 0 auto;
  min-height: 30px;
  border-radius: 4px;
}

.layout-vertical .category-item span {
  flex: 0 1 auto;
}

/* 拖拽分隔线 */
.resizer {
  width: 4px;
  background: transparent;
  cursor: col-resize;
  transition: background-color var(--transition-speed) ease;
}

.resizer:hover {
  background: var(--accent-color);
}

.layout-vertical .resizer {
  display: none;
}

/* 主内容区域 */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
  /* Allow flex child (.app-grid) to shrink and produce its own scroll */
  min-height: 0;
}

.content-header {
  background: var(--surface-bg);
  padding: 20px 30px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 60px;
  transition: all var(--transition-speed) ease;
  animation: slideDownFromTop 0.3s ease-out;
}

.content-header h1 {
  margin: 0;
  font-size: 24px;
  color: var(--text-color);
}

.search-box {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  transition: all 0.3s ease;
}

.search-input {
  padding: 8px 15px;
  border: 1px solid var(--border-color);
  border-radius: 20px;
  outline: none;
  font-size: 14px;
  width: 250px;
  transition: all var(--transition-speed) ease;
  animation: 0.3s ease-out;
}

.search-input:focus {
  border-color: var(--accent-color);
}

/* 搜索信息样式 */
.search-info {
  margin-top: 8px;
  font-size: 12px;
  color: var(--muted-text);
  text-align: center;
}

.search-hint {
  margin-left: 8px;
  font-size: 11px;
  opacity: 0.8;
}

/* 应用网格 */
.app-grid {
  flex: 1;
  padding: 15px;
  overflow-y: auto;
  overflow-x: hidden;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(var(--app-cell-size, 88px), 1fr));
  gap: 8px;
  align-content: start;
  justify-content: start;
  min-width: 0;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.app-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: var(--app-cell-height, var(--app-cell-size, 88px));
  min-width: 0;
  overflow: hidden;
  padding: 6px;
  background: var(--surface-bg);
  border-radius: 6px;
  cursor: pointer;
  transition: all var(--transition-speed) ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
}

.app-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 3px 12px var(--shadow-color);
}

.app-item.selected {
  outline: 2px solid var(--accent-color);
  outline-offset: 2px;
  background: var(--surface-hover);
}

/* 拖拽中的应用图标样式 */
.app-item.dragging {
  opacity: 0.4;
  cursor: grabbing;
}

.app-item.drag-target {
  outline: 2px solid var(--accent-color);
  outline-offset: 2px;
}

.app-item {
  cursor: grab;
}

.app-item:active {
  cursor: grabbing;
}

:global(body.app-sort-dragging) {
  cursor: grabbing;
  user-select: none;
}

.app-icon {
  width: var(--app-icon-size, 51px);
  height: var(--app-icon-size, 51px);
  flex: 0 0 auto;
  margin-bottom: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.name-position-top .app-icon {
  margin-top: 4px;
  margin-bottom: 0;
}

.name-position-top .app-name {
  order: -1;
}

.name-position-left .app-item,
.name-position-right .app-item {
  flex-direction: row;
  gap: 8px;
}

.name-position-left .app-icon,
.name-position-right .app-icon {
  margin-bottom: 0;
}

.name-position-left .app-name,
.name-position-right .app-name {
  flex: 1 1 auto;
  width: auto;
  text-align: left;
}

.name-position-left .app-name {
  order: -1;
  text-align: right;
}

.app-icon img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  border-radius: 4px;
}

.default-icon {
  width: 100%;
  height: 100%;
  background: var(--accent-color);
  color: white;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: calc(var(--app-icon-size, 51px) * 0.42);
  font-weight: bold;
}

.file-type-icon {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: calc(var(--app-icon-size, 51px) * 0.45);
  border-radius: 4px;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.app-name {
  width: 100%;
  min-width: 0;
  text-align: center;
  font-size: 12px;
  color: var(--text-color);
  line-height: 1.25;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 滚动条样式 */
.app-grid::-webkit-scrollbar {
  width: 0;
  height: 0;
  display: none;
}

.app-grid::-webkit-scrollbar-track {
  background: #f1f1f1;
}

.app-grid::-webkit-scrollbar-thumb {
  background: #bdc3c7;
  border-radius: 3px;
}

.app-grid::-webkit-scrollbar-thumb:hover {
  background: #95a5a6;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .app-grid {
    gap: 6px;
    padding: 12px;
  }

  .content-header {
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
  }

  .search-input {
    width: 100%;
  }
}

/* 搜索框动画 */
@keyframes slideInFromRight {
  from {
    opacity: 0;
    transform: translateX(20px);
  }

  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes slideDownFromTop {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 对话框样式 */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.dialog {
  background: white;
  border-radius: 8px;
  min-width: 320px;
  max-width: 90vw;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
}

.large-dialog {
  min-width: 480px;
  max-width: 600px;
}

.dialog-header {
  padding: 20px 20px 0;
}

.dialog-header h3 {
  margin: 0;
  color: #2c3e50;
  font-size: 18px;
  font-weight: 600;
}

.dialog-content {
  padding: 20px;
}

.dialog-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  outline: none;
  font-size: 14px;
  transition: border-color 0.2s ease;
}

.dialog-input:focus {
  border-color: #3498db;
}

.dialog-actions {
  padding: 0 20px 20px;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.dialog-button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.dialog-button-secondary {
  background: #ecf0f1;
  color: #2c3e50;
}

.dialog-button-secondary:hover {
  background: #d5dbdb;
}

.dialog-button-primary {
  background: #3498db;
  color: white;
}

.dialog-button-primary:hover {
  background: #2980b9;
}

/* 表单组样式 */
.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-size: 14px;
  font-weight: 500;
  color: #2c3e50;
}

.dialog-select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  outline: none;
  font-size: 14px;
  background: white;
  transition: border-color 0.2s ease;
}

.dialog-select:focus {
  border-color: #3498db;
}

/* 输入框组合样式 */
.input-group {
  display: flex;
  gap: 8px;
}

.input-group .dialog-input {
  flex: 1;
}

.browse-button {
  padding: 10px 16px;
  border: 1px solid #3498db;
  border-radius: 4px;
  background: #3498db;
  color: white;
  cursor: pointer;
  font-size: 14px;
  white-space: nowrap;
  transition: background-color 0.2s ease;
}

.browse-button:hover {
  background: #2980b9;
}

.browse-button.danger {
  background: #e74c3c;
  border-color: #e74c3c;
}

.browse-button.danger:hover {
  background: #c0392b;
}

/* 图标选择样式 */
.icon-section {
  display: flex;
  gap: 16px;
  align-items: center;
}

.icon-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.icon-button {
  padding: 8px 12px;
  font-size: 12px;
  white-space: nowrap;
}

/* 图标预览样式 */
.icon-preview-group {
  display: flex;
  gap: 12px;
  align-items: center;
}

.icon-preview {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #ddd;
  border-radius: 4px;
  background: #f8f9fa;
}

.preview-icon {
  width: 24px;
  height: 24px;
}

.preview-icon.file-type-icon {
  font-size: 16px;
}

.preview-icon.default-icon {
  background: #3498db;
  color: white;
  border-radius: 2px;
  font-size: 10px;
  font-weight: bold;
}

.icon-input {
  flex: 1;
}
</style>

<!-- 全局样式，用于Teleport到body的元素 -->
<style>
/* 全局禁用右键菜单和文本选择 */
* {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  -webkit-touch-callout: none;
  -webkit-tap-highlight-color: transparent;
}

/* 允许输入框和可编辑元素的文本选择 */
input,
textarea,
[contenteditable="true"] {
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
  user-select: text;
}

/* 确保整个页面容器允许溢出 */
html,
body {
  width: 100%;
  height: 100%;
  overflow: hidden !important;
  position: relative;
}

body.lora-animations-disabled *,
body.lora-animations-disabled *::before,
body.lora-animations-disabled *::after {
  transition: none !important;
  animation: none !important;
}

body.lora-animation-slow {
  --lora-global-transition: 0.35s;
}

body.lora-animation-normal {
  --lora-global-transition: 0.2s;
}

body.lora-animation-fast {
  --lora-global-transition: 0.1s;
}

/* 拖拽相关样式 */
.main-content.drag-over {
  position: relative;
}

.drag-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(52, 152, 219, 0.1);
  border: 2px dashed #3498db;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  backdrop-filter: blur(2px);
  animation: fadeIn var(--lora-global-transition, 0.2s) ease-out;
}

.drag-message {
  text-align: center;
  color: #3498db;
  font-size: 18px;
  font-weight: 600;
  padding: 20px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(52, 152, 219, 0.2);
}

.drag-message i {
  font-size: 48px;
  margin-bottom: 16px;
  display: block;
}

.drag-message p {
  margin: 0;
  font-size: 16px;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}

/* 拖拽时的主内容区域样式 */
.main-content.drag-over .app-grid {
  opacity: 0.3;
  transition: opacity var(--lora-global-transition, 0.2s) ease;
}

/* Toast 提示样式 */
.toast {
  position: fixed !important;
  top: 20px;
  right: 20px;
  min-width: 250px;
  max-width: 400px;
  padding: 12px 16px;
  background: white;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 10000;
  animation: slideInFromRight var(--lora-global-transition, 0.3s) ease-out;
  border-left: 4px solid #3498db;
}

.toast-info {
  border-left-color: #3498db;
}

.toast-success {
  border-left-color: #27ae60;
}

.toast-warning {
  border-left-color: #f39c12;
}

.toast-error {
  border-left-color: #e74c3c;
}

.toast-content {
  display: flex;
  align-items: center;
}

.toast-message {
  font-size: 14px;
  color: #2c3e50;
  line-height: 1.4;
}

@keyframes slideInFromRight {
  from {
    opacity: 0;
    transform: translateX(100%);
  }

  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* 加载指示器样式 */
.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid #e0e0e0;
  border-top: 3px solid #3498db;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.loading-text {
  color: #666;
  font-size: 14px;
  font-weight: 500;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}
</style>
