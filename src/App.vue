<template>
  <div class="app-container">
    <!-- 自定义标题栏 -->
    <div class="titlebar">
      <div class="titlebar-left" data-tauri-drag-region>
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
          <div class="context-menu-item" @click="showInExplorer">
            <span>资源管理器菜单</span>
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

      <!-- 拖拽分隔线 -->
      <div class="resizer" @mousedown="startResize"></div>

      <!-- 主内容区域 -->
      <div class="main-content" :class="{ 'drag-over': isDragOver }">
        <div class="content-header" v-show="showSearchBox">
          <!-- <h1>{{ getCurrentCategoryName() }}</h1> -->
          <div class="search-box">
            <input v-model="searchQuery" type="text" placeholder="搜索应用..." class="search-input" ref="searchInputRef"
              @keyup.escape="hideSearchBox">
          </div>
        </div>

        <div class="app-grid" @contextmenu.prevent="showGridContextMenu($event)">
          <div v-for="app in filteredApps" :key="app.id" class="app-item" @click="launchApp(app)"
            @dblclick="launchApp(app)" @contextmenu.prevent="showAppContextMenu($event, app)">
            <div class="app-icon">
              <img :src="app.icon" :alt="app.name" v-if="app.icon && app.icon.startsWith('http')" />
              <div v-else-if="app.icon && !app.icon.startsWith('http')" class="file-type-icon"
                :class="'file-type-' + app.icon">
                {{ getFileTypeIcon(app.icon) }}
              </div>
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

// 重命名对话框相关
const renameDialog = ref({
  visible: false,
  newName: '',
  categoryId: ''
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

const launchApp = async (app: any) => {
  console.log(`启动应用: ${app.name}`)

  if (!app.path) {
    console.error('应用路径不存在')
    alert('应用路径不存在，无法启动')
    return
  }

  try {
    const result = await invoke('launch_app', { appPath: app.path })
    console.log('启动结果:', result)
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

const createNewProject = async () => {
  console.log('新建项目')
  // 这里可以添加新建项目的逻辑，比如打开文件选择对话框
  // 或者添加一个默认的新项目到当前分类
  const newApp: AppData = {
    id: Date.now(),
    name: '新项目',
    category: selectedCategory.value === 'all' ? 'utilities' : selectedCategory.value,
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
const runAsAdmin = () => {
  if (appContextMenu.value.app) {
    console.log(`以管理员权限运行: ${appContextMenu.value.app.name}`)
    // 这里可以添加 Tauri API 调用以管理员权限启动应用
  }
  hideAppContextMenu()
}

const openFileLocation = () => {
  if (appContextMenu.value.app) {
    console.log(`打开文件位置: ${appContextMenu.value.app.path}`)
    // 这里可以添加 Tauri API 调用打开文件夹
  }
  hideAppContextMenu()
}

const showInExplorer = () => {
  if (appContextMenu.value.app) {
    console.log(`在资源管理器中显示: ${appContextMenu.value.app.path}`)
    // 这里可以添加 Tauri API 调用显示资源管理器菜单
  }
  hideAppContextMenu()
}

const copyFullPath = () => {
  if (appContextMenu.value.app) {
    navigator.clipboard.writeText(appContextMenu.value.app.path || '')
    console.log(`已复制路径: ${appContextMenu.value.app.path}`)
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

const editApp = () => {
  if (appContextMenu.value.app) {
    console.log(`编辑应用: ${appContextMenu.value.app.name}`)
    // 这里可以打开编辑对话框
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

const deleteCategory = async () => {
  if (contextMenu.value.category && !contextMenu.value.category.isDefault) {
    const categoryId = contextMenu.value.category.id

    // 删除分类
    categories.value = categories.value.filter(cat => cat.id !== categoryId)

    // 如果当前选中的分类被删除，切换到"全部应用"
    if (selectedCategory.value === categoryId) {
      await selectCategory('all')
    }

    // 将该分类下的应用移动到"实用工具"分类
    apps.value.forEach(app => {
      if (app.category === categoryId) {
        app.category = 'utilities'
      }
    })

    // 保存数据
    await saveAppData()
  }
  hideContextMenu()
}

const deleteAllCategories = async () => {
  if (confirm('确定要删除所有自定义分组吗？这将把所有应用移动到"实用工具"分类中。')) {
    const defaultCategories = categories.value.filter(cat => cat.isDefault)
    const deletedCategoryIds = categories.value.filter(cat => !cat.isDefault).map(cat => cat.id)

    categories.value = defaultCategories

    // 将被删除分类下的应用移动到"实用工具"分类
    apps.value.forEach(app => {
      if (deletedCategoryIds.includes(app.category)) {
        app.category = 'utilities'
      }
    })

    // 切换到"全部应用"
    await selectCategory('all')

    // 保存数据
    await saveAppData()
  }
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

  // 全局禁用右键菜单
  document.addEventListener('contextmenu', disableContextMenu)

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

onUnmounted(() => {
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
  // 清理全局右键菜单禁用监听器
  document.removeEventListener('contextmenu', disableContextMenu)

  // 清理拖拽功能
  cleanupDragAndDrop()
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

const toggleMenu = () => {
  // 显示应用菜单或设置
  console.log('显示菜单')
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
    const newApp: AppData = {
      id: Date.now() + Math.floor(Math.random() * 1000), // 避免ID冲突，使用整数
      name: fileInfo.name,
      category: selectedCategory.value === 'all' ? 'utilities' : selectedCategory.value,
      icon: fileInfo.icon || '', // 使用后端返回的图标标识符
      path: fileInfo.path,
      target_path: fileInfo.target_path,
      is_shortcut: fileInfo.is_shortcut
    }

    console.log('创建新应用项:', newApp)
    apps.value.push(newApp)
    console.log('应用已添加到数组，当前应用数量:', apps.value.length)

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
</style>