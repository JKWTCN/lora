<template>
  <div class="app-container" @mouseenter="isMouseInWindow = true" @mouseleave="isMouseInWindow = false">
    <!-- 自定义标题栏 -->
    <div class="titlebar">
      <div class="titlebar-left" data-tauri-drag-region @mousedown="handleDragStart" @mouseup="handleDragEnd">
        <span class="app-title" data-tauri-drag-region>Lora</span>
      </div>
      <div class="titlebar-right">
        <button class="titlebar-button" @click="toggleSearch" title="搜索">
          <i class="icon-search"></i>
        </button>
        <button class="titlebar-button" @click="toggleMenu" title="菜单">
          <i class="icon-menu"></i>
        </button>
        <button class="titlebar-button titlebar-close" @click="closeApp" title="关闭">
          <i class="icon-close"></i>
        </button>
      </div>
    </div>

    <!-- 主启动器容器 -->
    <div class="launcher-container">
      <!-- 侧栏 -->
      <div class="sidebar" :style="sidebarWidth > 0 ? { width: sidebarWidth + 'px' } : {}">
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
            <span>新建分组</span>
          </div>
          <div v-if="contextMenu.category && !contextMenu.category.isDefault" class="context-menu-item"
            :class="{ 'context-menu-item-disabled': !contextMenu.category }" @click="renameCategory">
            <span>重命名</span>
          </div>
          <div v-if="contextMenu.category && !contextMenu.category.isDefault"
            class="context-menu-item context-menu-item-danger"
            :class="{ 'context-menu-item-disabled': !contextMenu.category }" @click="deleteCategory">
            <span>删除</span>
          </div>
          <template v-if="!contextMenu.category">
            <div class="context-menu-item context-menu-item-disabled">
              <span>重命名</span>
            </div>
            <div class="context-menu-item context-menu-item-disabled">
              <span>删除</span>
            </div>
          </template>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item context-menu-item-danger" @click="deleteAllCategories">
            <span>删除全部分组</span>
          </div>
        </div>
      </Teleport>

      <!-- 应用右键菜单 -->
      <Teleport to="body">
        <div v-if="appContextMenu.visible" class="context-menu"
          :style="{ left: appContextMenu.x + 'px', top: appContextMenu.y + 'px' }" @click.stop>
          <div class="context-menu-item" @click="runAsAdmin">
            <span>管理员权限运行</span>
          </div>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item" @click="openFileLocation">
            <span>打开文件位置</span>
          </div>
          <div class="context-menu-item" @click="copyFullPath">
            <span>复制完整路径</span>
          </div>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item" @click="showMoveToSubmenu">
            <span>移动到</span>
            <span class="arrow-right">▶</span>
          </div>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item" @click="editApp">
            <span>编辑</span>
          </div>
          <div class="context-menu-item context-menu-item-danger" @click="deleteApp">
            <span>删除</span>
          </div>
          <div class="context-menu-item context-menu-item-danger" @click="deleteAllApps">
            <span>删除全部</span>
          </div>
        </div>
      </Teleport>

      <!-- 网格空白处右键菜单 -->
      <Teleport to="body">
        <div v-if="gridContextMenu.visible" class="context-menu"
          :style="{ left: gridContextMenu.x + 'px', top: gridContextMenu.y + 'px' }" @click.stop>
          <div class="context-menu-item" @click="createNewProject">
            <span>新建项目</span>
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

      <!-- 主菜单 -->
      <Teleport to="body">
        <div v-if="mainMenu.visible" class="context-menu main-menu"
          :style="{ left: mainMenu.x + 'px', top: mainMenu.y + 'px' }" @click.stop>
          <div class="context-menu-item" @click="togglePreventAutoHide">
            <span>{{ appSettings.preventAutoHide ? '✓' : '○' }} 阻止自动隐藏</span>
          </div>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item" @click="openSettings">
            <span>设置</span>
          </div>
          <div class="context-menu-divider"></div>
          <div class="context-menu-item context-menu-item-danger" @click="confirmExit">
            <span>退出</span>
          </div>
        </div>
      </Teleport>

      <!-- 重命名对话框 -->
      <div v-if="renameDialog.visible" class="dialog-overlay" @click="cancelRename">
        <div class="dialog" @click.stop>
          <div class="dialog-header">
            <h3>重命名分组</h3>
          </div>
          <div class="dialog-content">
            <input v-model="renameDialog.newName" type="text" class="dialog-input" placeholder="请输入新名称"
              @keyup.enter="confirmRename" @keyup.escape="cancelRename" ref="renameInput">
          </div>
          <div class="dialog-actions">
            <button class="dialog-button dialog-button-secondary" @click="cancelRename">取消</button>
            <button class="dialog-button dialog-button-primary" @click="confirmRename">确认</button>
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
            <h3>编辑应用</h3>
          </div>
          <div class="dialog-content">
            <div class="form-group">
              <label>应用名称:</label>
              <input v-model="editAppDialog.editedName" type="text" class="dialog-input" placeholder="请输入应用名称"
                @keyup.enter="confirmEditApp" @keyup.escape="cancelEditApp">
            </div>
            <div class="form-group">
              <label>所属分组:</label>
              <select v-model="editAppDialog.editedCategory" class="dialog-select">
                <option v-for="category in categories.filter(cat => cat.id !== 'all')" :key="category.id"
                  :value="category.id">
                  {{ category.name }}
                </option>
              </select>
            </div>
            <div class="form-group">
              <label>目标路径:</label>
              <div class="input-group">
                <input v-model="editAppDialog.editedTargetPath" type="text" class="dialog-input"
                  placeholder="请输入文件、文件夹路径或网址" @blur="detectTargetType">
                <button class="browse-button" @click="browseTarget" type="button">
                  浏览
                </button>
              </div>
            </div>
            <div class="form-group">
              <label>启动参数 (可选):</label>
              <input v-model="editAppDialog.editedLaunchArgs" type="text" class="dialog-input"
                placeholder="请输入启动参数 (如: --fullscreen --debug)">
            </div>
            <div class="form-group">
              <label>图标 (可选):</label>
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
                    选择图标
                  </button>
                  <button v-if="editAppDialog.editedIcon" class="browse-button icon-button danger" @click="clearIcon"
                    type="button">
                    清除图标
                  </button>
                </div>
              </div>
            </div>
          </div>
          <div class="dialog-actions">
            <button class="dialog-button dialog-button-secondary" @click="cancelEditApp">取消</button>
            <button class="dialog-button dialog-button-primary" @click="confirmEditApp">确认</button>
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
            <input v-model="searchQuery" type="text" placeholder="搜索应用..." class="search-input" ref="searchInputRef"
              @keyup.escape="hideSearchBox" @keydown="handleSearchKeydown">
            <div v-if="searchQuery" class="search-info">
              找到 {{ filteredApps.length }} 个结果
              <span v-if="filteredApps.length > 0" class="search-hint">
                • 按 Enter 启动第一个 • 按 ESC 退出搜索
              </span>
            </div>
          </div>
        </div>

        <div class="app-grid" @contextmenu.prevent="showGridContextMenu($event)">
          <div v-for="app in filteredApps" :key="app.id" class="app-item" @click="launchApp(app)"
            @dblclick="launchApp(app)" @contextmenu.prevent="showAppContextMenu($event, app)">
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
            <p>拖拽程序文件到这里添加到启动器</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, Teleport } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// 定义数据接口
interface AppData {
  id: number
  name: string
  category: string
  icon: string
  path: string
  target_path?: string
  is_shortcut?: boolean
  launch_args?: string // 启动参数
  target_type?: 'file' | 'folder' | 'url' // 目标类型
}

interface CategoryData {
  id: string
  name: string
  icon: string
  isDefault: boolean
}

// 响应式数据
const sidebarWidth = ref(0) // 初始化为0，将通过自适应计算
const isResizing = ref(false)
const selectedCategory = ref('all')
const searchQuery = ref('')
const showSearchBox = ref(false) // 控制搜索框显示状态

// 拖拽相关状态
const isDragOver = ref(false)
const dragCounter = ref(0)

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

// 应用设置
const appSettings = ref({
  preventAutoHide: false, // 阻止自动隐藏
  windowWidth: undefined as number | undefined, // 窗口宽度
  windowHeight: undefined as number | undefined, // 窗口高度
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
  { id: 'all', name: '全部应用', icon: 'icon-apps', isDefault: true },
])

// 应用数据
const apps = ref<AppData[]>([])

// 确保"全部应用"分组始终存在
const ensureDefaultCategory = () => {
  const hasAllCategory = categories.value.some(cat => cat.id === 'all')
  if (!hasAllCategory) {
    categories.value.unshift({ id: 'all', name: '全部应用', icon: 'icon-apps', isDefault: true })
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
    apps.value = storage.apps || []

    // 转换后端的 is_default 为前端使用的 isDefault
    const categoriesFromBackend = storage.categories || []
    categories.value = categoriesFromBackend.map((category: any) => ({
      ...category,
      isDefault: category.is_default,
      is_default: undefined // 移除后端字段
    })).map(({ is_default, ...rest }: any) => rest) // 完全移除 is_default 字段

    // 确保"全部应用"分组始终存在
    ensureDefaultCategory()

    // 恢复选中的分组，如果没有则默认选择"全部应用"
    if (storage.selected_category) {
      // 检查选中的分组是否还存在
      const categoryExists = categories.value.some(cat => cat.id === storage.selected_category)
      if (categoryExists) {
        selectedCategory.value = storage.selected_category
      } else {
        selectedCategory.value = 'all'
      }
    } else {
      selectedCategory.value = 'all'
    }

    console.log('应用数据加载成功:', {
      apps: apps.value,
      categories: categories.value,
      selectedCategory: selectedCategory.value
    })
  } catch (error) {
    console.error('加载应用数据失败:', error)
    // 使用默认数据
    categories.value = [
      { id: 'all', name: '全部应用', icon: 'icon-apps', isDefault: true }
    ]
    apps.value = []
    selectedCategory.value = 'all'
    console.log('使用默认数据:', {
      apps: apps.value,
      categories: categories.value,
      selectedCategory: selectedCategory.value
    })
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

    await invoke('save_app_data', {
      apps: apps.value,
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
    // 转换后端的 snake_case 为前端的 camelCase
    appSettings.value = {
      preventAutoHide: settings.prevent_auto_hide || false,
      windowWidth: settings.window_width,
      windowHeight: settings.window_height,
    }
    console.log('应用设置加载成功:', appSettings.value)
  } catch (error) {
    console.error('加载应用设置失败:', error)
  }
}

// 计算属性
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
  if (searchQuery.value) {
    result = result.filter(app =>
      app.name.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
    console.log('按搜索词筛选后:', result)
  }

  console.log('最终结果:', result)
  return result
})

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

const launchApp = async (app: any) => {
  console.log(`启动应用: ${app.name}`)

  const targetPath = app.target_path || app.path
  if (!targetPath) {
    console.error('应用路径不存在')
    alert('应用路径不存在，无法启动')
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
      // 启动文件
      await invoke('launch_app', {
        appPath: targetPath,
        launchArgs: app.launch_args || ''
      })
    }
    console.log('启动成功')
  } catch (error) {
    console.error('启动应用失败:', error)
    alert(`启动应用失败: ${error}`)
  }
}

// 右键菜单相关方法
const showContextMenu = (e: MouseEvent, category: any) => {
  // 隐藏其他所有菜单
  hideAppContextMenu()
  hideMoveToSubmenu()

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
}

// 应用右键菜单相关方法
const showAppContextMenu = (e: MouseEvent, app: any) => {
  // 隐藏其他所有菜单
  hideContextMenu()
  hideMoveToSubmenu()

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
      ? '已启用阻止自动隐藏'
      : '已禁用阻止自动隐藏'
    showToast(message, 'success')
  } catch (error) {
    console.error('更新阻止自动隐藏设置失败:', error)
    showToast('设置更新失败', 'error')
  }

  hideMainMenu()
}

const openSettings = () => {
  console.log('打开设置')
  // TODO: 实现设置界面
  showToast('设置功能开发中...', 'info')
  hideMainMenu()
}

const confirmExit = () => {
  if (confirm('确定要退出应用吗？')) {
    closeApp()
  }
  hideMainMenu()
}

const createNewProject = async () => {
  console.log('新建项目')

  const defaultCategory = getDefaultCategoryForNewApp()
  if (!defaultCategory) {
    showToast('请先创建一个分组', 'warning')
    return
  }

  // 这里可以添加新建项目的逻辑，比如打开文件选择对话框
  // 或者添加一个默认的新项目到当前分类
  const newApp: AppData = {
    id: Date.now(),
    name: '新项目',
    category: defaultCategory,
    icon: '',
    path: ''
  }

  apps.value.push(newApp)
  console.log('已添加新项目:', newApp)

  // 保存数据
  await saveAppData()

  hideGridContextMenu()
}

// 应用右键菜单功能
const runAsAdmin = async () => {
  if (appContextMenu.value.app) {
    try {
      console.log(`以管理员权限运行: ${appContextMenu.value.app.name}`)
      const result = await invoke('run_as_admin', { appPath: appContextMenu.value.app.path })
      console.log('管理员权限运行结果:', result)
    } catch (error) {
      console.error('以管理员权限运行失败:', error)
      alert(`以管理员权限运行失败: ${error}`)
    }
  }
  hideAppContextMenu()
}

const openFileLocation = async () => {
  if (appContextMenu.value.app) {
    try {
      console.log(`打开文件位置: ${appContextMenu.value.app.path}`)
      const result = await invoke('open_file_location', { filePath: appContextMenu.value.app.path })
      console.log('打开文件位置结果:', result)
    } catch (error) {
      console.error('打开文件位置失败:', error)
      alert(`打开文件位置失败: ${error}`)
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
      showToast('路径已复制到剪贴板')
    } catch (error) {
      console.error('复制路径失败:', error)
      // 备用方法：创建临时文本区域
      const textArea = document.createElement('textarea')
      textArea.value = appContextMenu.value.app.path || ''
      document.body.appendChild(textArea)
      textArea.select()
      document.execCommand('copy')
      document.body.removeChild(textArea)
      showToast('路径已复制到剪贴板')
    }
  }
  hideAppContextMenu()
}

const showMoveToSubmenu = () => {
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

const editApp = async () => {
  if (appContextMenu.value.app) {
    console.log(`编辑应用: ${appContextMenu.value.app.name}`)
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
  hideAppContextMenu()
}

const deleteApp = async () => {
  if (appContextMenu.value.app) {
    if (confirm(`确定要删除应用 "${appContextMenu.value.app.name}" 吗？`)) {
      try {
        // 调用后端删除
        await invoke('delete_app', { appId: appContextMenu.value.app.id })

        // 从前端数组中移除
        apps.value = apps.value.filter(app => app.id !== appContextMenu.value.app.id)
        console.log(`已删除应用: ${appContextMenu.value.app.name}`)
      } catch (error) {
        console.error('删除应用失败:', error)
        alert('删除应用失败')
      }
    }
  }
  hideAppContextMenu()
}

const deleteAllApps = async () => {
  if (confirm('确定要删除当前分类下的所有应用吗？')) {
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
      alert('删除应用失败')
    }
  }
  hideAppContextMenu()
}

const createNewCategory = async () => {
  const newId = Date.now().toString()
  const newCategory: CategoryData = {
    id: newId,
    name: '新分组',
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
      showToast('应用信息已更新', 'success')
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
    const choice = confirm('选择文件请点击"确定"，选择文件夹请点击"取消"')

    let selectedPath = ''
    if (choice) {
      // 选择文件
      const filters = [
        ['所有文件', ['*']],
        ['可执行文件', ['exe', 'bat', 'cmd', 'msi']],
        ['脚本文件', ['ps1', 'vbs', 'js', 'py']],
        ['快捷方式', ['lnk', 'url']]
      ]
      selectedPath = await invoke('open_file_dialog', {
        title: '选择目标文件',
        filters: filters
      }) as string
    } else {
      // 选择文件夹
      selectedPath = await invoke('open_folder_dialog', {
        title: '选择目标文件夹'
      }) as string
    }

    if (selectedPath) {
      editAppDialog.value.editedTargetPath = selectedPath
      // 自动检测目标类型
      await detectTargetType()
    }
  } catch (error) {
    console.error('浏览文件失败:', error)
    if (error !== '用户取消了选择') {
      showToast('浏览文件失败: ' + error, 'error')
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
      ['图片文件', ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'ico', 'svg']],
      ['图标文件', ['ico', 'png']],
      ['所有文件', ['*']]
    ]
    const selectedPath = await invoke('open_file_dialog', {
      title: '选择图标文件',
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
    if (error !== '用户取消了选择') {
      showToast('选择图标失败: ' + error, 'error')
    }
  }
}

// 清除图标
const clearIcon = () => {
  editAppDialog.value.editedIcon = ''
}

const deleteCategory = async () => {
  if (contextMenu.value.category && !contextMenu.value.category.isDefault) {
    const categoryId = contextMenu.value.category.id

    // 确认删除操作
    const appsInCategory = apps.value.filter(app => app.category === categoryId)
    const confirmMessage = appsInCategory.length > 0
      ? `确定要删除分组 "${contextMenu.value.category.name}" 吗？这将同时删除该分组下的 ${appsInCategory.length} 个应用。`
      : `确定要删除分组 "${contextMenu.value.category.name}" 吗？`

    if (!confirm(confirmMessage)) {
      hideContextMenu()
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
  hideContextMenu()
}

const deleteAllCategories = async () => {
  const customCategories = categories.value.filter(cat => !cat.isDefault)
  if (customCategories.length === 0) {
    alert('没有自定义分组可以删除。')
    hideContextMenu()
    return
  }

  // 计算要删除的应用数量
  const deletedCategoryIds = customCategories.map(cat => cat.id)
  const appsToDelete = apps.value.filter(app => deletedCategoryIds.includes(app.category))

  const confirmMessage = appsToDelete.length > 0
    ? `确定要删除所有 ${customCategories.length} 个自定义分组吗？这将同时删除 ${appsToDelete.length} 个应用。`
    : `确定要删除所有 ${customCategories.length} 个自定义分组吗？`

  if (!confirm(confirmMessage)) {
    hideContextMenu()
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
  hideContextMenu()
}

// 拖拽调整侧栏宽度
const startResize = (e: MouseEvent) => {
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
  if (newWidth > 80 && newWidth < 200) {
    sidebarWidth.value = newWidth
  }
}

const stopResize = () => {
  isResizing.value = false
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
}

// 生命周期
onMounted(async () => {
  // 简单延迟等待 Tauri 完全初始化
  await new Promise(resolve => setTimeout(resolve, 500))
  console.log('开始加载应用数据...')

  // 加载应用数据
  await loadAppData()

  // 加载应用设置
  await loadAppSettings()

  // 在加载设置后，更新托盘菜单以反映当前设置
  try {
    await invoke('update_tray_menu', {
      preventAutoHide: appSettings.value.preventAutoHide
    })
    console.log('托盘菜单已更新以反映当前设置')
  } catch (error) {
    console.error('更新托盘菜单失败:', error)
  }

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

  // 监听窗口大小变化
  try {
    const window = getCurrentWindow()
    await window.listen('tauri://resize', async () => {
      try {
        const size = await window.innerSize()
        // 保存新的窗口大小
        await invoke('save_window_size', {
          width: size.width,
          height: size.height
        })
        console.log(`保存窗口大小: ${size.width}x${size.height}`)
      } catch (error) {
        console.error('保存窗口大小失败:', error)
      }
    })
    console.log('窗口大小监听器设置成功')
  } catch (error) {
    console.error('设置窗口大小监听器失败:', error)
  }

  // 计算侧栏的自然宽度
  const sidebar = document.querySelector('.sidebar') as HTMLElement
  if (sidebar) {
    // 先让侧栏自适应，然后获取其实际宽度
    sidebar.style.width = 'auto'
    const rect = sidebar.getBoundingClientRect()
    sidebarWidth.value = Math.max(rect.width, 80) // 确保最小宽度为80px
  }

  // 添加全局点击监听，点击搜索框外部时隐藏搜索框
  const handleClickOutside = (event: Event) => {
    const target = event.target as HTMLElement
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

  // 添加鼠标移动和离开事件监听器
  document.addEventListener('mousemove', handleMouseMove)
  document.addEventListener('mouseleave', handleMouseLeave)

  // 添加全局 mouseup 事件监听器，确保拖拽结束能被正确检测
  document.addEventListener('mouseup', handleDragEnd)

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
      ? '已启用阻止自动隐藏'
      : '已禁用阻止自动隐藏'
    showToast(message, 'success')
  })

  // 等待DOM完全渲染后设置拖拽功能
  nextTick(async () => {
    await setupDragAndDrop()
  })
})

// 禁用右键菜单的函数
const disableContextMenu = (e: Event) => {
  e.preventDefault()
  return false
}

// 全局键盘事件处理函数
const handleGlobalKeydown = (event: KeyboardEvent) => {
  // 检查是否正在编辑状态（对话框打开、输入框聚焦等）
  const isEditing = document.activeElement?.tagName === 'INPUT' ||
    document.activeElement?.tagName === 'TEXTAREA' ||
    document.activeElement?.tagName === 'SELECT' ||
    document.querySelector('.dialog-overlay') ||
    contextMenu.value.visible ||
    appContextMenu.value.visible ||
    moveToSubmenu.value.visible ||
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

// 追踪鼠标位置
const handleMouseMove = (event: MouseEvent) => {
  mousePosition.value = { x: event.clientX, y: event.clientY }
  isMouseInWindow.value = true
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

// 窗口失焦处理函数
const handleWindowBlur = async () => {
  // 只有在没有阻止自动隐藏的情况下才隐藏窗口
  if (!appSettings.value.preventAutoHide) {
    // 延迟检查，给鼠标事件时间更新状态
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
    }, 100) // 100ms 延迟
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
      launchApp(firstApp)
      hideSearchBox()
    }
    event.preventDefault()
  }
  // 下箭头键可以考虑添加应用选择功能
  // 这里暂时不实现，因为需要添加选中状态管理
}

const toggleMenu = (e: MouseEvent) => {
  // 显示主菜单
  showMainMenu(e)
}

const closeApp = async () => {
  console.log('关闭应用被调用')
  try {
    const appWindow = getCurrentWindow()
    console.log('获取到窗口对象:', appWindow)
    await appWindow.close()
    console.log('窗口关闭命令已发送')
  } catch (error) {
    console.error('关闭窗口时出错:', error)
  }
}

// 拖拽处理函数
const handleDragEnter = (e: Event) => {
  console.log('拖拽进入事件触发')
  e.preventDefault()
  dragCounter.value++
  isDragOver.value = true
}

const handleDragLeave = (e: Event) => {
  console.log('拖拽离开事件触发')
  e.preventDefault()
  dragCounter.value--
  if (dragCounter.value === 0) {
    isDragOver.value = false
  }
}

const handleDragOver = (e: Event) => {
  console.log('拖拽悬停事件触发')
  e.preventDefault()
}

const handleDrop = async (e: Event) => {

  e.preventDefault()
  isDragOver.value = false
  dragCounter.value = 0

  console.log('拖拽释放事件触发')

  const dragEvent = e as DragEvent
  const files = dragEvent.dataTransfer?.files
  if (!files || files.length === 0) {
    console.log('没有检测到文件')
    return
  }

  console.log('拖拽文件数量:', files.length)

  for (let i = 0; i < files.length; i++) {
    const file = files[i]
    const filePath = (file as any).path || file.name
    await handleFileDrop(filePath)
  }

  console.log('拖拽处理完成，当前应用总数:', apps.value.length)
}

// 处理单个文件拖拽的函数
const handleFileDrop = async (filePath: string) => {
  console.log('处理文件:', filePath)
  console.log("调用自定义命令")
  await invoke('my_custom_command');

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
      const realIcon = await invoke('get_app_icon', { filePath: filePath }) as string
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

  } catch (error) {
    console.error('处理文件失败:', error)
    // 可以显示错误提示
    alert(`无法添加文件 "${filePath}": ${error}`)
  }
}

// 在 onMounted 中添加拖拽事件监听器
const setupDragAndDrop = async () => {
  console.log('设置拖拽功能...')

  // 监听 Tauri 拖拽事件
  try {
    console.log('设置 Tauri 拖拽事件监听...')
    await listen('tauri://drag', (event) => {
      console.log('Tauri 拖拽事件:', event)
    })

    await listen('tauri://drag-over', (event) => {
      console.log('Tauri 拖拽悬停事件:', event)
      isDragOver.value = true
    })

    await listen('tauri://drag-drop', async (event: any) => {
      console.log('Tauri 拖拽释放事件:', event)
      isDragOver.value = false

      if (event.payload && event.payload.paths) {
        const paths = event.payload.paths as string[]
        console.log('拖拽的文件路径:', paths)

        for (const filePath of paths) {
          await handleFileDrop(filePath)
        }
      }
    })

    await listen('tauri://drag-leave', (event) => {
      console.log('Tauri 拖拽离开事件:', event)
      isDragOver.value = false
    })

    console.log('Tauri 拖拽事件监听设置完成')
  } catch (error) {
    console.error('设置 Tauri 拖拽事件失败:', error)
  }

  // 同时保留传统的DOM拖拽事件作为备用
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
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

/* 自定义标题栏样式 */
.titlebar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 32px;
  background: #2c3e50;
  color: white;
  padding: 0 16px;
  user-select: none;
  -webkit-user-select: none;
  position: relative;
  z-index: 1000;
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
  transition: background-color 0.2s ease;
  font-size: 12px;
  position: relative;
  z-index: 1001;
  pointer-events: auto;
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
  background: #f5f5f5;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  position: relative;
  overflow: visible;
}

/* 侧栏样式 */
.sidebar {
  background: #2c3e50;
  color: white;
  width: auto;
  min-width: 100px;
  max-width: 200px;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
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
}

.category-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  white-space: nowrap;
}

.category-item:hover {
  background: #34495e;
}

.category-item.active {
  background: #3498db;
}

.category-item span {
  flex: 1;
  min-width: 0;
  font-size: 13px;
}

/* 拖拽分隔线 */
.resizer {
  width: 4px;
  background: #bdc3c7;
  cursor: col-resize;
  transition: background-color 0.2s ease;
}

.resizer:hover {
  background: #3498db;
}

/* 主内容区域 */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-header {
  background: white;
  padding: 20px 30px;
  border-bottom: 1px solid #e0e0e0;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 60px;
  transition: all 0.3s ease;
  animation: slideDownFromTop 0.3s ease-out;
}

.content-header h1 {
  margin: 0;
  font-size: 24px;
  color: #2c3e50;
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
  border: 1px solid #ddd;
  border-radius: 20px;
  outline: none;
  font-size: 14px;
  width: 250px;
  transition: all 0.3s ease;
  animation: 0.3s ease-out;
}

.search-input:focus {
  border-color: #3498db;
}

/* 搜索信息样式 */
.search-info {
  margin-top: 8px;
  font-size: 12px;
  color: #7f8c8d;
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
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  gap: 8px;
  align-content: start;
}

.app-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 6px;
  background: white;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
}

.app-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.12);
}

.app-icon {
  width: 28px;
  height: 28px;
  margin-bottom: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
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
  background: #3498db;
  color: white;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: bold;
}

.file-type-icon {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  border-radius: 4px;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.app-name {
  text-align: center;
  font-size: 10px;
  color: #2c3e50;
  line-height: 1.2;
  word-break: break-word;
}

/* 滚动条样式 */
.app-grid::-webkit-scrollbar {
  width: 6px;
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
    grid-template-columns: repeat(auto-fill, minmax(70px, 1fr));
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
  overflow: visible !important;
  position: relative;
}

/* 右键菜单全局样式 */
.context-menu {
  position: fixed !important;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  z-index: 999999;
  min-width: 120px;
  padding: 2px 0;
  pointer-events: auto;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  /* 确保菜单能够超出窗口边界 */
  overflow: visible;
  transform: translateZ(0);
  will-change: transform;
}

.context-menu.submenu {
  z-index: 1000000;
  min-width: 100px;
}

.context-menu-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 12px;
  cursor: pointer;
  font-size: 12px;
  color: #2c3e50;
  transition: background-color 0.2s ease;
}

.context-menu-item:hover {
  background: #f8f9fa;
}

.context-menu-item-disabled {
  color: #bdc3c7 !important;
  cursor: not-allowed !important;
  pointer-events: none;
}

.context-menu-item-disabled:hover {
  background: transparent !important;
}

.context-menu-item-danger {
  color: #e74c3c;
}

.context-menu-item-danger:hover {
  background: #fdf2f2;
}

.arrow-right {
  margin-left: auto;
  font-size: 10px;
  color: #95a5a6;
}

.context-menu-divider {
  height: 1px;
  background: #e0e0e0;
  margin: 2px 0;
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
  animation: fadeIn 0.2s ease-out;
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
  transition: opacity 0.2s ease;
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
  animation: slideInFromRight 0.3s ease-out;
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
</style>