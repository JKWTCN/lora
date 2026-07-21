<template>
  <div class="tray-menu-page" @click="hideMenu" @contextmenu.prevent>
    <div class="context-menu tray-context-menu" @click.stop>
      <button class="context-menu-item tray-menu-item" type="button" @click="togglePreventAutoHide">
        <span class="menu-mark" :class="{ active: preventAutoHide }">{{ preventAutoHide ? '✓' : '' }}</span>
        <span class="menu-label">{{ $t('settings.ui.window.preventAutoHide') }}</span>
      </button>
      <div class="context-menu-divider"></div>
      <button class="context-menu-item tray-menu-item" type="button" @click="openSettings">
        <span class="menu-mark"></span>
        <span class="menu-label">{{ $t('common.settings') }}</span>
      </button>
      <div class="context-menu-divider"></div>
      <button class="context-menu-item context-menu-item-danger tray-menu-item" type="button" @click="quitApp">
        <span class="menu-mark"></span>
        <span class="menu-label">{{ $t('main.confirm.exit') }}</span>
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
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    const theme = settings.theme || 'auto'
    const useDarkTheme = theme === 'dark' || (theme === 'auto' && prefersDark)
    document.body.classList.toggle('lora-theme-dark', useDarkTheme)
    document.body.classList.toggle('lora-theme-light', !useDarkTheme)
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
  padding: 5px;
  border-radius: 10px;
  background: rgba(248, 248, 248, 0.98);
  border: 0;
  box-shadow: inset 0 0 0 1px rgba(60, 60, 67, 0.16);
  animation: tray-menu-materialize 160ms cubic-bezier(.2,.8,.2,1);
}

.tray-context-menu .tray-menu-item {
  width: 100%;
  min-height: 28px;
  display: grid;
  grid-template-columns: 16px minmax(0, 1fr);
  align-items: center;
  gap: 6px;
  padding: 0 8px;
  border-radius: 6px;
  background: transparent;
  border: 0;
  appearance: none;
  font: inherit;
  font-size: 12px;
  font-weight: 500;
  line-height: 1.2;
  text-align: left;
}

.menu-mark {
  width: 16px;
  height: 16px;
  display: grid;
  place-items: center;
  text-align: center;
  color: transparent;
  font-size: 11px;
  font-weight: 700;
}

.menu-mark.active {
  color: #007aff;
}

.menu-label {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  text-align: left;
}

.tray-context-menu .context-menu-divider {
  margin: 2px 8px;
}

body.lora-theme-dark .tray-context-menu {
  background: rgba(38, 38, 40, 0.98);
  border: 0;
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.12);
}

body.lora-theme-dark .menu-mark.active {
  color: #0a84ff;
}

@keyframes tray-menu-materialize {
  from { opacity: 0; }
  to { opacity: 1; }
}

@media (prefers-reduced-motion: reduce) {
  .tray-context-menu { animation: none; }
}

@media (prefers-reduced-transparency: reduce) {
  .tray-context-menu { backdrop-filter: none; background: #f2f2f7; }
  body.lora-theme-dark .tray-context-menu { background: #2c2c2e; }
}
</style>
