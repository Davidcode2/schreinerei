import type { ResourceType } from "@/types/fleet"
import type { CSSProperties } from "react"

const RESOURCE_COLOR_PALETTE = [
  {
    token: "sky",
    labelClassName: "text-sky-700",
    markerClassName: "bg-sky-500",
    borderClassName: "border-sky-200",
    tintClassName: "bg-sky-100/80",
    softTintClassName: "bg-sky-100",
  },
  {
    token: "emerald",
    labelClassName: "text-emerald-700",
    markerClassName: "bg-emerald-500",
    borderClassName: "border-emerald-200",
    tintClassName: "bg-emerald-100/80",
    softTintClassName: "bg-emerald-100",
  },
  {
    token: "amber",
    labelClassName: "text-amber-700",
    markerClassName: "bg-amber-500",
    borderClassName: "border-amber-200",
    tintClassName: "bg-amber-100/80",
    softTintClassName: "bg-amber-100",
  },
  {
    token: "rose",
    labelClassName: "text-rose-700",
    markerClassName: "bg-rose-500",
    borderClassName: "border-rose-200",
    tintClassName: "bg-rose-100/80",
    softTintClassName: "bg-rose-100",
  },
  {
    token: "indigo",
    labelClassName: "text-indigo-700",
    markerClassName: "bg-indigo-500",
    borderClassName: "border-indigo-200",
    tintClassName: "bg-indigo-100/80",
    softTintClassName: "bg-indigo-100",
  },
  {
    token: "cyan",
    labelClassName: "text-cyan-700",
    markerClassName: "bg-cyan-500",
    borderClassName: "border-cyan-200",
    tintClassName: "bg-cyan-100/80",
    softTintClassName: "bg-cyan-100",
  },
] as const

export interface ResourceCalendarColor {
  token: (typeof RESOURCE_COLOR_PALETTE)[number]["token"] | string
  labelClassName: string
  markerClassName: string
  borderClassName: string
  tintClassName: string
  softTintClassName: string
  labelStyle?: CSSProperties
  markerStyle?: CSSProperties
  borderStyle?: CSSProperties
  tintStyle?: CSSProperties
  softTintStyle?: CSSProperties
}

function hashValue(input: string): number {
  let hash = 0

  for (let index = 0; index < input.length; index += 1) {
    hash = (hash << 5) - hash + input.charCodeAt(index)
    hash |= 0
  }

  return Math.abs(hash)
}

function isHexDisplayColor(value: string | null | undefined): value is string {
  return /^#[0-9a-f]{6}$/i.test(value ?? "")
}

function hexToRgba(hexColor: string, alpha: number): string {
  const red = Number.parseInt(hexColor.slice(1, 3), 16)
  const green = Number.parseInt(hexColor.slice(3, 5), 16)
  const blue = Number.parseInt(hexColor.slice(5, 7), 16)

  return `rgb(${red} ${green} ${blue} / ${alpha})`
}

export function getResourceCalendarColor(
  resourceType: ResourceType,
  resourceId: string,
  displayColor?: string | null
): ResourceCalendarColor {
  const identity = `${resourceType}:${resourceId}`
  const paletteIndex = hashValue(identity) % RESOURCE_COLOR_PALETTE.length
  const fallbackColor = RESOURCE_COLOR_PALETTE[paletteIndex] ?? RESOURCE_COLOR_PALETTE[0]

  if (!isHexDisplayColor(displayColor)) {
    return fallbackColor
  }

  const normalizedColor = displayColor.toLowerCase()

  return {
    ...fallbackColor,
    token: normalizedColor,
    labelStyle: { color: normalizedColor },
    markerStyle: { backgroundColor: normalizedColor },
    borderStyle: { borderColor: hexToRgba(normalizedColor, 0.35) },
    tintStyle: { backgroundColor: hexToRgba(normalizedColor, 0.1) },
    softTintStyle: { backgroundColor: hexToRgba(normalizedColor, 0.16) },
  }
}

export const resourceCalendarColorPalette = RESOURCE_COLOR_PALETTE
