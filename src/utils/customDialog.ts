import { App as VueApp, createApp, defineComponent, h, nextTick, reactive } from 'vue'

type DialogType = 'info' | 'warning' | 'error' | 'success'
type DialogMode = 'alert' | 'confirm'

interface DialogOptions {
  title?: string
  message: string
  type?: DialogType
  mode: DialogMode
  confirmText?: string
  cancelText?: string
}

interface PendingDialog extends DialogOptions {
  resolve: (value: boolean) => void
}

const state = reactive({
  visible: false,
  title: '',
  message: '',
  type: 'info' as DialogType,
  mode: 'alert' as DialogMode,
  confirmText: 'OK',
  cancelText: 'Cancel'
})

const queue: PendingDialog[] = []
let activeDialog: PendingDialog | null = null
let mounted = false
let translate: ((key: string) => string) | null = null

const defaultTitle: Record<DialogType, string> = {
  info: 'Info',
  warning: 'Warning',
  error: 'Error',
  success: 'Success'
}

const defaultIcon: Record<DialogType, string> = {
  info: 'i',
  warning: '!',
  error: '!',
  success: '✓'
}

const label = (key: string, fallback: string) => {
  const translated = translate?.(key)
  return translated && translated !== key ? translated : fallback
}

const closeDialog = (value: boolean) => {
  if (!activeDialog) return

  const current = activeDialog
  activeDialog = null
  state.visible = false
  current.resolve(value)

  window.setTimeout(showNextDialog, 120)
}

const showNextDialog = () => {
  if (activeDialog || queue.length === 0) return

  activeDialog = queue.shift() || null
  if (!activeDialog) return

  state.title = activeDialog.title || label(`common.${activeDialog.type || 'info'}`, defaultTitle[activeDialog.type || 'info'])
  state.message = activeDialog.message
  state.type = activeDialog.type || 'info'
  state.mode = activeDialog.mode
  state.confirmText = activeDialog.confirmText || label('common.ok', 'OK')
  state.cancelText = activeDialog.cancelText || label('common.cancel', 'Cancel')
  state.visible = true

  void nextTick(() => {
    const button = document.querySelector<HTMLButtonElement>('.custom-dialog-primary')
    button?.focus()
  })
}

const CustomDialogHost = defineComponent({
  name: 'CustomDialogHost',
  setup() {
    const onKeydown = (event: KeyboardEvent) => {
      if (!state.visible) return

      if (event.key === 'Escape') {
        event.preventDefault()
        closeDialog(false)
      }

      if (event.key === 'Enter') {
        event.preventDefault()
        closeDialog(true)
      }
    }

    return () => state.visible
      ? h('div', {
        class: 'custom-dialog-overlay',
        tabindex: -1,
        onKeydown
      }, [
        h('div', {
          class: ['custom-dialog', `custom-dialog-${state.type}`],
          role: 'dialog',
          'aria-modal': 'true',
          'aria-labelledby': 'custom-dialog-title',
          onClick: (event: MouseEvent) => event.stopPropagation()
        }, [
          h('div', { class: 'custom-dialog-icon' }, defaultIcon[state.type]),
          h('div', { class: 'custom-dialog-body' }, [
            h('h3', { id: 'custom-dialog-title' }, state.title),
            h('p', { class: 'custom-dialog-message' }, state.message),
            h('div', { class: 'custom-dialog-actions' }, [
              state.mode === 'confirm'
                ? h('button', {
                  class: 'custom-dialog-button custom-dialog-secondary',
                  type: 'button',
                  onClick: () => closeDialog(false)
                }, state.cancelText)
                : null,
              h('button', {
                class: 'custom-dialog-button custom-dialog-primary',
                type: 'button',
                onClick: () => closeDialog(true)
              }, state.confirmText)
            ])
          ])
        ])
      ])
      : null
  }
})

const ensureMounted = () => {
  if (mounted) return

  const host = document.createElement('div')
  host.id = 'custom-dialog-host'
  document.body.appendChild(host)
  createApp(CustomDialogHost).mount(host)
  mounted = true
}

const openDialog = (options: DialogOptions) => {
  ensureMounted()

  return new Promise<boolean>((resolve) => {
    queue.push({ ...options, resolve })
    showNextDialog()
  })
}

export const installCustomDialogs = (app: VueApp, i18n: { global?: { t?: (key: string) => string } }) => {
  translate = i18n.global?.t || null
  app.config.globalProperties.$alert = alertDialog
  app.config.globalProperties.$confirm = confirmDialog
}

export const alertDialog = async (message: string, options: Omit<Partial<DialogOptions>, 'message' | 'mode'> = {}) => {
  await openDialog({
    mode: 'alert',
    message,
    type: options.type || 'info',
    title: options.title,
    confirmText: options.confirmText || label('common.ok', 'OK')
  })
}

export const confirmDialog = (message: string, options: Omit<Partial<DialogOptions>, 'message' | 'mode'> = {}) => {
  return openDialog({
    mode: 'confirm',
    message,
    type: options.type || 'warning',
    title: options.title,
    confirmText: options.confirmText || label('common.confirm', 'Confirm'),
    cancelText: options.cancelText || label('common.cancel', 'Cancel')
  })
}
