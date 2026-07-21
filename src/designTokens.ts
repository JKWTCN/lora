/** Runtime geometry shared by launcher interactions and CSS layout. */
export const LAUNCHER_GRID_SIZE = {
  default: 88,
  min: 56,
  max: 144,
  step: 4,
} as const

export const LAUNCHER_ICON_SIZE = {
  minWithLabel: 30,
  minWithoutLabel: 32,
  maxWithLabel: 82,
  labelRatio: 0.54,
  noLabelInset: 12,
  verticalLabelAllowance: 34,
} as const

export const SIDEBAR_SIZE = {
  min: 96,
  max: 240,
} as const
