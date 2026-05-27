<template>
  <div class="tray-menu-page" @click="hideMenu" @contextmenu.prevent>
    <div class="context-menu tray-context-menu" @click.stop>
      <button class="context-menu-item tray-menu-item" type="button" @click="togglePreventAutoHide">
        <span class="menu-mark">{{ preventAutoHide ? '✓' : '○' }}</span>
        <span>{{ $t('settings.ui.window.preventAutoHide') }}</span>
      </button>
      <div class="context-menu-divider"></div>
      <button class="context-menu-item tray-menu-item" type="button" @click="openSettings">
        <span class="menu-mark"></span>
        <span>{{ $t('common.settings') }}</span>
      </button>
      <div class="context-menu-divider"></div>
      <button class="context-menu-item context-menu-item-danger tray-menu-item" type="button" @click="quitApp">
        <span class="menu-mark"></span>
        <span>{{ $t('main.confirm.exit') }}</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'

const preventAutoHide = ref(false)
let unlistenRefresh: UnlistenFn | null = null

const currentWindow = getCurrentWindow()

const loadSettings = async () => {
  try {
    const settings = await invoke('load_app_settings') as any
    preventAutoHide.value = settings.prevent_auto_hide || false
  } catch (error) {
    console.error('加载托盘菜单设置失败:', error)
  }
}

const hideMenu = async () => {
  await currentWindow.hide()
}

const togglePreventAutoHide = async () => {
  const newValue = !preventAutoHide.value

  try {
    await invoke('update_prevent_auto_hide', {
      preventAutoHide: newValue
    })
    preventAutoHide.value = newValue
    await emit('prevent-auto-hide-changed', newValue)
  } catch (error) {
    console.error('更新阻止自动隐藏设置失败:', error)
  } finally {
    await hideMenu()
  }
}

const openSettings = async () => {
  try {
    await hideMenu()
    await invoke('open_settings_window')
  } catch (error) {
    console.error('打开设置窗口失败:', error)
  }
}

const quitApp = async () => {
  await invoke('quit_app')
}

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    hideMenu()
  }
}

onMounted(async () => {
  await loadSettings()
  unlistenRefresh = await listen('tray-menu-refresh', () => {
    loadSettings()
  })
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  unlistenRefresh?.()
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style>
html,
body,
#app {
  width: 100%;
  height: 100%;
  margin: 0;
  overflow: hidden;
  background: transparent;
}

* {
  box-sizing: border-box;
}

.tray-menu-page {
  width: 100%;
  height: 100%;
  padding: 0;
  background: transparent;
}

.tray-context-menu {
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  min-width: 0;
  border-radius: 6px;
  box-shadow: none;
}

.tray-menu-item {
  width: 100%;
  justify-content: flex-start;
  gap: 6px;
  background: transparent;
  border: 0;
  appearance: none;
  font: inherit;
  text-align: left;
}

.menu-mark {
  flex: 0 0 18px;
  text-align: center;
  color: #f5f7f8;
}
</style>
