import type { ResourceType } from "@/types/fleet"

const RESOURCE_COLOR_PALETTE = [
  {
    token: "sky",
    labelClassName: "text-sky-950 dark:text-sky-100",
    markerClassName: "bg-sky-500",
    borderClassName: "border-sky-200 dark:border-sky-800",
    tintClassName: "bg-sky-50/90 dark:bg-sky-950/35",
    softTintClassName: "bg-sky-100/80 dark:bg-sky-950/45",
  },
  {
    token: "emerald",
    labelClassName: "text-emerald-950 dark:text-emerald-100",
    markerClassName: "bg-emerald-500",
    borderClassName: "border-emerald-200 dark:border-emerald-800",
    tintClassName: "bg-emerald-50/90 dark:bg-emerald-950/35",
    softTintClassName: "bg-emerald-100/80 dark:bg-emerald-950/45",
  },
  {
    token: "amber",
    labelClassName: "text-amber-950 dark:text-amber-100",
    markerClassName: "bg-amber-500",
    borderClassName: "border-amber-200 dark:border-amber-800",
    tintClassName: "bg-amber-50/90 dark:bg-amber-950/35",
    softTintClassName: "bg-amber-100/80 dark:bg-amber-950/45",
  },
  {
    token: "rose",
    labelClassName: "text-rose-950 dark:text-rose-100",
    markerClassName: "bg-rose-500",
    borderClassName: "border-rose-200 dark:border-rose-800",
    tintClassName: "bg-rose-50/90 dark:bg-rose-950/35",
    softTintClassName: "bg-rose-100/80 dark:bg-rose-950/45",
  },
  {
    token: "indigo",
    labelClassName: "text-indigo-950 dark:text-indigo-100",
    markerClassName: "bg-indigo-500",
    borderClassName: "border-indigo-200 dark:border-indigo-800",
    tintClassName: "bg-indigo-50/90 dark:bg-indigo-950/35",
    softTintClassName: "bg-indigo-100/80 dark:bg-indigo-950/45",
  },
  {
    token: "cyan",
    labelClassName: "text-cyan-950 dark:text-cyan-100",
    markerClassName: "bg-cyan-500",
    borderClassName: "border-cyan-200 dark:border-cyan-800",
    tintClassName: "bg-cyan-50/90 dark:bg-cyan-950/35",
    softTintClassName: "bg-cyan-100/80 dark:bg-cyan-950/45",
  },
] as const

export interface ResourceCalendarColor {
  token: (typeof RESOURCE_COLOR_PALETTE)[number]["token"]
  labelClassName: string
  markerClassName: string
  borderClassName: string
  tintClassName: string
  softTintClassName: string
}

function hashValue(input: string): number {
  let hash = 0

  for (let index = 0; index < input.length; index += 1) {
    hash = (hash << 5) - hash + input.charCodeAt(index)
    hash |= 0
  }

  return Math.abs(hash)
}

export function getResourceCalendarColor(
  resourceType: ResourceType,
  resourceId: string
): ResourceCalendarColor {
  const identity = `${resourceType}:${resourceId}`
  const paletteIndex = hashValue(identity) % RESOURCE_COLOR_PALETTE.length

  return RESOURCE_COLOR_PALETTE[paletteIndex] ?? RESOURCE_COLOR_PALETTE[0]
}

export const resourceCalendarColorPalette = RESOURCE_COLOR_PALETTE
