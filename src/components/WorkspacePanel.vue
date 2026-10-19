<template>
  <div class="dialog-overlay workspace-overlay" @click.self="close" @keydown.esc.stop.prevent="close" @keydown.tab="trapFocus">
    <section ref="panel" class="workspace-panel" role="dialog" aria-modal="true" aria-labelledby="workspace-title" tabindex="-1">
      <header>
        <div><h2 id="workspace-title">{{ t('workspaces.title') }}</h2><p>{{ t('workspaces.subtitle') }}</p></div>
        <button :disabled="busy" :aria-label="t('common.close')" @click="close"><i class="icon-close"></i></button>
      </header>
      <p v-if="error" class="error" role="alert">{{ error }}</p>
      <div v-if="loading" class="empty">{{ t('common.loading') }}</div>
      <template v-else-if="!editing">
        <div class="actions"><button class="primary" :disabled="busy" @click="create">{{ t('workspaces.create') }}</button></div>
        <p v-if="!workspaces.length" class="empty">{{ t('workspaces.empty') }}</p>
        <article v-for="workspace in workspaces" :key="workspace.id" class="workspace-card">
          <div class="card-heading"><div><h3>{{ workspace.name }}</h3><p>{{ t('workspaces.windowCount', { count: workspace.windows.length }) }}</p></div>
            <div class="actions">
              <button class="primary" :disabled="busy" @click="launch(workspace)">{{ launchingId === workspace.id ? t('workspaces.launching') : t('workspaces.launch') }}</button>
              <button :disabled="busy" @click="edit(workspace)">{{ t('common.edit') }}</button>
              <button :disabled="busy" @click="remove(workspace)">{{ t('common.delete') }}</button>
            </div>
          </div>
          <div class="monitor-previews"><div v-for="monitor in monitorIds(workspace.windows)" :key="monitor" class="monitor-preview" :aria-label="monitor">
            <span v-for="(window, index) in workspace.windows.filter(w => w.monitor === monitor)" :key="index" class="preview-window" :style="previewStyle(window)" :title="window.name">{{ window.name }}</span>
          </div></div>
        </article>
        <div v-if="results.length" class="launch-results" aria-live="polite">
          <h3>{{ t('workspaces.results') }}</h3>
          <p v-for="(result, index) in results" :key="index" :class="{ error: result.status === 'failed' }">
            {{ result.name }} · {{ result.error_code === 'permission_denied' ? t('workspaces.permissionStatus') : t(`workspaces.status.${result.status}`) }}<span v-if="result.error"> — {{ result.error_code === 'permission_denied' ? t('workspaces.permissionHint') : result.error }}</span>
          </p>
        </div>
      </template>
      <form v-else @submit.prevent="save">
        <label>{{ t('workspaces.name') }}<input ref="nameInput" v-model="draftName" maxlength="100" required :disabled="busy" /></label>
        <label class="checkbox"><input v-model="moveExisting" type="checkbox" :disabled="busy" />{{ t('workspaces.moveExisting') }}</label>
        <p class="hint">{{ t('workspaces.captureHint') }}</p>
        <div class="actions">
          <button type="button" :disabled="busy" @click="capture">{{ t('workspaces.capture') }}</button>
          <label class="layout-select">{{ t('workspaces.layout') }}
            <select v-model="layout" :disabled="busy" @change="changeLayout">
              <option value="captured">{{ t('workspaces.layouts.captured') }}</option>
              <option value="columns">{{ t('workspaces.layouts.columns') }}</option>
              <option value="rows">{{ t('workspaces.layouts.rows') }}</option>
              <option value="grid">{{ t('workspaces.layouts.grid') }}</option>
            </select>
          </label>
        </div>
        <div class="monitor-previews"><div v-for="monitor in monitorIds(selectedWindows)" :key="monitor" class="monitor-preview" :aria-label="monitor">
          <span v-for="(window, index) in selectedWindows.filter(w => w.monitor === monitor)" :key="index" class="preview-window" :style="previewStyle(window)" :title="window.name">{{ window.name }}</span>
        </div></div>
        <p v-if="!entries.length" class="empty">{{ t('workspaces.noWindows') }}</p>
        <article v-for="entry in entries" :key="entry.key" class="window-entry">
          <label class="checkbox"><input v-model="entry.selected" type="checkbox" :disabled="busy" @change="applyLayout" />
            <strong>{{ entry.window.name }}</strong><span class="window-title" :title="entry.window.title">{{ entry.window.title }}</span>
          </label>
          <p v-if="entry.selected && entry.window.elevated" class="hint">{{ t('workspaces.elevatedHint') }}</p>
          <details v-if="entry.selected">
            <summary>{{ t('workspaces.configure') }}</summary>
            <label>{{ t('workspaces.path') }}<input v-model="entry.window.path" required :disabled="busy" /></label>
            <label>{{ t('workspaces.arguments') }}<input v-model="entry.window.launch_args" @input="entry.window.folder_path = null" :placeholder="t('workspaces.argumentsHint')" :disabled="busy" /></label>
            <label class="checkbox"><input v-model="entry.window.run_as_admin" type="checkbox" :disabled="busy" />{{ t('workspaces.admin') }}</label>
            <p class="hint">{{ entry.window.monitor }}</p>
            <div class="geometry">
              <label v-for="field in geometryFields" :key="field">{{ t(`workspaces.${field}`) }} (%)
                <input type="number" :value="Math.round(entry.window[field] * 100)" :min="field === 'width' || field === 'height' ? 1 : -100" max="400" :disabled="busy" @input="setGeometry(entry.window, field, $event)" />
              </label>
            </div>
            <label class="checkbox"><input v-model="entry.window.maximized" type="checkbox" :disabled="busy" />{{ t('workspaces.maximized') }}</label>
            <label class="checkbox"><input v-model="entry.window.minimized" type="checkbox" :disabled="busy" />{{ t('workspaces.minimized') }}</label>
          </details>
        </article>
        <p class="hint">{{ t('workspaces.limitations') }}</p>
        <footer><button type="button" :disabled="busy" @click="editing = false">{{ t('common.cancel') }}</button><button class="primary" :disabled="busy || !selectedWindows.length || !draftName.trim()">{{ t('common.save') }}</button></footer>
      </form>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useI18n } from 'vue-i18n'
import { confirmDialog } from '../utils/customDialog'

interface WorkspaceWindow {
  name: string; title: string; path: string; launch_args: string; run_as_admin: boolean; elevated: boolean
  monitor: string; monitor_device: string; x: number; y: number; width: number; height: number
  maximized: boolean; minimized: boolean; visible_bounds?: boolean; folder_path?: string | null
}
interface Workspace { id: string; name: string; move_existing_windows: boolean; windows: WorkspaceWindow[] }
interface Entry { key: string; window: WorkspaceWindow; captured: WorkspaceWindow; selected: boolean }
interface LaunchResult { name: string; status: string; error: string | null; error_code?: string | null }
const props = defineProps<{ initialWorkspaceId?: string }>()
const emit = defineEmits<{ close: []; changed: [] }>()
const { t } = useI18n()
const panel = ref<HTMLElement>()
const nameInput = ref<HTMLInputElement>()
const workspaces = ref<Workspace[]>([])
const entries = ref<Entry[]>([])
const editing = ref(false)
const loading = ref(true)
const busy = ref(false)
const error = ref('')
const draftName = ref('')
const draftId = ref('')
const moveExisting = ref(true)
const layout = ref('captured')
const launchingId = ref('')
const results = ref<LaunchResult[]>([])
const geometryFields = ['x', 'y', 'width', 'height'] as const
type GeometryField = typeof geometryFields[number]
const selectedWindows = computed(() => entries.value.filter(e => e.selected).map(e => e.window))
let unlisten: UnlistenFn | undefined
let disposed = false
const previousFocus = document.activeElement as HTMLElement | null

const close = () => { if (!busy.value) emit('close') }
const monitorIds = (windows: WorkspaceWindow[]) => [...new Set(windows.map(w => w.monitor))]
const previewStyle = (window: WorkspaceWindow) => ({
  left: `${(window.maximized ? 0 : window.x) * 100}%`, top: `${(window.maximized ? 0 : window.y) * 100}%`,
  width: `${(window.maximized ? 1 : window.width) * 100}%`, height: `${(window.maximized ? 1 : window.height) * 100}%`,
  opacity: window.minimized ? 0.4 : 1
})
const trapFocus = (event: KeyboardEvent) => {
  const controls = panel.value?.querySelectorAll<HTMLElement>('button:not(:disabled), input:not(:disabled), select:not(:disabled), summary')
  if (!controls?.length) { event.preventDefault(); return }
  const first = controls[0]!, last = controls[controls.length - 1]!
  if (event.shiftKey && (document.activeElement === first || document.activeElement === panel.value)) { event.preventDefault(); last.focus() }
  else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus() }
}
const reload = async () => { workspaces.value = await invoke<Workspace[]>('list_workspaces') }
const capture = async () => {
  busy.value = true; error.value = ''
  try {
    const captured = await invoke<{ key: string; window: WorkspaceWindow }[]>('capture_workspace_windows')
    const previous = entries.value
    entries.value = captured.map((entry, index) => {
      const existing = previous.find(e => e.window.path === entry.window.path && e.window.title === entry.window.title)
      if (existing) {
        entry.window.launch_args = existing.window.launch_args
        entry.window.run_as_admin = existing.window.run_as_admin
      }
      return { ...entry, captured: { ...entry.window }, selected: existing?.selected ?? index < 50 }
    })
    layout.value = 'captured'
  } catch (e) { error.value = String(e) }
  finally { busy.value = false }
}
const create = async () => {
  draftId.value = crypto.randomUUID(); draftName.value = ''; moveExisting.value = true
  entries.value = []; editing.value = true; results.value = []
  await capture(); await nextTick(); nameInput.value?.focus()
}
const edit = async (workspace: Workspace) => {
  draftId.value = workspace.id; draftName.value = workspace.name; moveExisting.value = workspace.move_existing_windows
  entries.value = workspace.windows.map((window, index) => ({ key: `saved-${index}`, window: { ...window }, captured: { ...window }, selected: true }))
  layout.value = 'captured'; error.value = ''; editing.value = true
  await nextTick(); nameInput.value?.focus()
}
const applyLayout = () => {
  if (layout.value === 'captured') return
  for (const monitor of monitorIds(selectedWindows.value)) {
    const windows = selectedWindows.value.filter(w => w.monitor === monitor)
    const columns = layout.value === 'rows' ? 1 : layout.value === 'grid' ? Math.ceil(Math.sqrt(windows.length)) : windows.length
    const rows = Math.ceil(windows.length / columns)
    windows.forEach((window, index) => {
      window.x = (index % columns) / columns; window.y = Math.floor(index / columns) / rows
      window.width = 1 / columns; window.height = 1 / rows; window.visible_bounds = true; window.maximized = false; window.minimized = false
    })
  }
}
const changeLayout = () => {
  if (layout.value !== 'captured') { applyLayout(); return }
  for (const entry of entries.value) {
    for (const field of geometryFields) entry.window[field] = entry.captured[field]
    entry.window.visible_bounds = entry.captured.visible_bounds
    entry.window.maximized = entry.captured.maximized
    entry.window.minimized = entry.captured.minimized
  }
}
const setGeometry = (window: WorkspaceWindow, field: GeometryField, event: Event) => {
  const value = (event.target as HTMLInputElement).valueAsNumber
  if (Number.isFinite(value)) { window[field] = value / 100; window.visible_bounds = true }
  layout.value = 'captured'
}
const save = async () => {
  busy.value = true; error.value = ''
  try {
    await invoke('save_workspace', { workspace: { id: draftId.value, name: draftName.value.trim(), move_existing_windows: moveExisting.value, windows: selectedWindows.value } })
    emit('changed')
    await reload(); editing.value = false
  } catch (e) { error.value = String(e) }
  finally { busy.value = false }
}
const remove = async (workspace: Workspace) => {
  busy.value = true; error.value = ''
  try {
    if (!await confirmDialog(t('workspaces.deleteConfirm', { name: workspace.name }))) return
    await invoke('delete_workspace', { workspaceId: workspace.id }); emit('changed'); await reload()
  } catch (e) { error.value = String(e) }
  finally { busy.value = false }
}
const launch = async (workspace: Workspace) => {
  busy.value = true; error.value = ''; launchingId.value = workspace.id
  results.value = workspace.windows.map(w => ({ name: w.name, status: 'waiting', error: null }))
  try { results.value = await invoke<LaunchResult[]>('launch_workspace', { workspaceId: workspace.id }) }
  catch (e) { error.value = String(e); results.value = [] }
  finally { busy.value = false; launchingId.value = '' }
}
onMounted(async () => {
  panel.value?.focus()
  try {
    const stop = await listen<{ workspace_id: string; index: number; result: LaunchResult }>('workspace-launch-progress', event => {
      if (event.payload.workspace_id === launchingId.value) results.value[event.payload.index] = event.payload.result
    })
    if (disposed) { stop(); return }
    unlisten = stop
    await reload()
    const initialWorkspace = workspaces.value.find(workspace => workspace.id === props.initialWorkspaceId)
    if (initialWorkspace && !disposed) await edit(initialWorkspace)
  } catch (e) { error.value = String(e) }
  finally { loading.value = false }
})
onUnmounted(() => { disposed = true; unlisten?.(); previousFocus?.focus() })
</script>

<style scoped>
.workspace-overlay { position: fixed; inset: 0; z-index: 2000; display: flex; align-items: center; justify-content: center; padding: 16px; background: rgba(0, 0, 0, .4); }
.workspace-panel { width: min(760px, 100%); max-height: calc(100vh - 32px); overflow-y: auto; padding: 20px; border: 1px solid var(--border-color); border-radius: var(--ui-radius-xl); color: var(--text-color); background: var(--app-bg); box-shadow: 0 16px 64px rgba(0,0,0,.25); font-size: 13px; }
header, .card-heading, .actions, footer { display: flex; align-items: center; justify-content: space-between; gap: 8px; }
header { margin-bottom: 16px; }
h2 { margin: 0; font-size: 20px; } h3 { margin: 0; font-size: 15px; }
p { margin: 8px 0; line-height: 1.5; } header p, .hint, .window-title { opacity: .7; }
button, input, select { font: inherit; color: inherit; border: 1px solid var(--border-color); border-radius: var(--ui-radius-sm); background: var(--surface-bg); }
button { min-height: 32px; padding: 6px 12px; cursor: pointer; white-space: nowrap; }
button.primary { background: var(--accent-color); border-color: var(--accent-color); color: white; }
button:disabled { opacity: .5; cursor: default; }
button:focus-visible, input:focus-visible, select:focus-visible, summary:focus-visible { outline: 2px solid var(--accent-color); outline-offset: 2px; }
input:not([type=checkbox]), select { width: 100%; padding: 8px; box-sizing: border-box; }
label { display: flex; flex-direction: column; gap: 6px; margin: 12px 0; }
.checkbox { flex-direction: row; align-items: center; gap: 8px; } input[type=checkbox] { accent-color: var(--accent-color); }
.actions { justify-content: flex-start; flex-wrap: wrap; } .layout-select { flex-direction: row; align-items: center; }
.layout-select select { width: auto; }
.workspace-card, .window-entry { margin-top: 12px; padding: 12px; border: 1px solid var(--border-color); border-radius: var(--ui-radius-md); background: var(--surface-bg); }
.window-entry .checkbox { margin: 0; flex-wrap: wrap; }
.window-title { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0; flex: 1; }
details { margin-top: 10px; } summary { cursor: pointer; }
.geometry { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 8px; }
.monitor-previews { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 12px; }
.monitor-preview { position: relative; width: 168px; height: 96px; overflow: hidden; border: 1px solid var(--border-color); border-radius: 5px; background: var(--app-bg); }
.preview-window { position: absolute; box-sizing: border-box; padding: 3px; border: 1px solid var(--accent-color); border-radius: 3px; background: color-mix(in srgb, var(--accent-color) 18%, var(--surface-bg)); font-size: 10px; overflow: hidden; }
.empty { padding: 24px 0; text-align: center; opacity: .7; } .error { color: #d64545; overflow-wrap: anywhere; }
.launch-results { margin-top: 16px; } footer { justify-content: flex-end; margin-top: 16px; }
@media(max-width: 520px) { .workspace-panel { padding: 12px; } .card-heading { align-items: flex-start; flex-direction: column; } .geometry { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
</style>
