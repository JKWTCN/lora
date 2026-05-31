<template>
  <div class="language-switch" :class="[`language-switch-${variant}`, { 'language-switch-dark': dark }]">
    <button 
      class="language-button" 
      @click="toggleLanguage"
      :title="$t('language.switch')"
    >
      <span class="language-icon">🌐</span>
      <span class="language-text">{{ currentLanguageName }}</span>
      <span class="language-arrow">▼</span>
    </button>
    
    <div v-if="showDropdown" class="language-dropdown">
      <div 
        v-for="lang in languages" 
        :key="lang.code"
        class="language-option"
        :class="{ active: lang.code === currentLanguage }"
        @click="selectLanguage(lang.code)"
      >
        <span class="language-flag">{{ lang.flag }}</span>
        <span class="language-name">{{ lang.nativeName }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { setLanguage, getCurrentLanguage } from '../i18n'
import type { SupportedLocale } from '../i18n/types'

const { locale } = useI18n()

withDefaults(defineProps<{
  variant?: 'titlebar' | 'settings'
  dark?: boolean
}>(), {
  variant: 'titlebar',
  dark: false
})

const showDropdown = ref(false)

const languages = [
  { code: 'zh-CN' as SupportedLocale, name: 'Chinese', nativeName: '中文', flag: '🇨🇳' },
  { code: 'en-US' as SupportedLocale, name: 'English', nativeName: 'English', flag: '🇺🇸' }
]

const currentLanguage = computed(() => getCurrentLanguage())

const currentLanguageName = computed(() => {
  const lang = languages.find(l => l.code === currentLanguage.value)
  return lang ? lang.nativeName : ''
})

const toggleLanguage = () => {
  showDropdown.value = !showDropdown.value
}

const selectLanguage = (langCode: SupportedLocale) => {
  setLanguage(langCode)
  showDropdown.value = false
}

const closeDropdown = (event: MouseEvent) => {
  const target = event.target as Element | null
  if (!target || !target.closest('.language-switch')) {
    showDropdown.value = false
  }
}

const handleLanguageChange = (event: CustomEvent) => {
  // 当其他窗口更改语言时，同步当前窗口的语言
  if (event.detail.language !== locale.value) {
    locale.value = event.detail.language
  }
}

onMounted(() => {
  document.addEventListener('click', closeDropdown)
  window.addEventListener('languageChanged', handleLanguageChange as EventListener)
})

onUnmounted(() => {
  document.removeEventListener('click', closeDropdown)
  window.removeEventListener('languageChanged', handleLanguageChange as EventListener)
})
</script>

<style scoped>
.language-switch {
  position: relative;
  display: inline-block;
}

.language-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 13px;
}

.language-button:hover {
  background: rgba(255, 255, 255, 0.05);
}

.language-switch-settings .language-button {
  width: 100%;
  max-width: 220px;
  height: 34px;
  justify-content: space-between;
  background: #ffffff;
  border: 1px solid #cbd5e1;
  color: #172033;
}

.language-switch-settings .language-button:hover {
  background: #eef6ff;
  border-color: #2f9ae0;
}

.language-switch-settings.language-switch-dark .language-button {
  background: #111827;
  border-color: #334155;
  color: #e5edf7;
}

.language-switch-settings.language-switch-dark .language-button:hover {
  background: #22364d;
  border-color: #38bdf8;
}

.language-icon {
  font-size: 16px;
}

.language-text {
  font-weight: 500;
}

.language-arrow {
  font-size: 10px;
  transition: transform 0.2s ease;
}

.language-dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 4px;
  background: white;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border: 1px solid #e0e0e0;
  overflow: hidden;
  z-index: 1000;
  min-width: 140px;
}

.language-switch-settings .language-dropdown {
  left: 0;
  right: auto;
  width: min(220px, 100%);
}

.language-option {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  cursor: pointer;
  transition: background-color 0.2s ease;
  color: #2c3e50;
}

.language-option:hover {
  background-color: #f5f5f5;
}

.language-option.active {
  background-color: #e3f2fd;
  color: #1976d2;
}

.language-switch-dark .language-dropdown {
  background: #111827;
  border-color: #334155;
}

.language-switch-dark .language-option {
  color: #e5edf7;
}

.language-switch-dark .language-option:hover {
  background-color: #1f2937;
}

.language-switch-dark .language-option.active {
  background-color: #22364d;
  color: #e0f2fe;
}

.language-flag {
  font-size: 16px;
}

.language-name {
  font-size: 13px;
  font-weight: 500;
}
</style>
