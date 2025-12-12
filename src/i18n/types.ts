export type SupportedLocale = 'zh-CN' | 'en-US'

export interface LanguageOption {
  code: SupportedLocale
  name: string
  nativeName: string
}