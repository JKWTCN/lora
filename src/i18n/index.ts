import { createI18n } from 'vue-i18n'
import zhCN from './locales/zh-CN'
import enUS from './locales/en-US'
import type { SupportedLocale } from './types'

// 获取本地存储的语言设置，如果没有则使用浏览器语言或默认中文
const getSavedLanguage = (): SupportedLocale => {
  const savedLanguage = localStorage.getItem('language') as SupportedLocale
  if (savedLanguage && ['zh-CN', 'en-US'].includes(savedLanguage)) {
    return savedLanguage
  }
  
  // 检查浏览器语言
  const browserLanguage = navigator.language
  if (browserLanguage.startsWith('zh')) {
    return 'zh-CN'
  } else if (browserLanguage.startsWith('en')) {
    return 'en-US'
  }
  
  // 默认使用中文
  return 'zh-CN'
}

const i18n = createI18n({
  legacy: false,
  locale: getSavedLanguage(),
  fallbackLocale: 'en-US',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS
  },
  // 全局注入$t函数
  globalInjection: true
})

export default i18n

// 导出切换语言的函数
export const setLanguage = (language: SupportedLocale) => {
  if (['zh-CN', 'en-US'].includes(language)) {
    i18n.global.locale.value = language
    localStorage.setItem('language', language)
    
    // 更新HTML lang属性
    document.documentElement.lang = language
    
    // 触发自定义事件，通知其他窗口语言已更改
    window.dispatchEvent(new CustomEvent('languageChanged', { detail: { language } }))
  }
}

// 导出当前语言
export const getCurrentLanguage = () => {
  return i18n.global.locale.value
}