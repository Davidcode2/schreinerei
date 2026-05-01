import type { ResourceType } from "@/types/fleet"

const RESOURCE_COLOR_PALETTE = [
  {
    token: "sky",
    labelClassName: "text-sky-700 dark:text-sky-300",
    markerClassName: "bg-sky-500",
    borderClassName: "border-sky-300 dark:border-sky-700",
    tintClassName: "bg-sky-50 dark:bg-sky-950/40",
  },
  {
    token: "emerald",
    labelClassName: "text-emerald-700 dark:text-emerald-300",
    markerClassName: "bg-emerald-500",
    borderClassName: "border-emerald-300 dark:border-emerald-700",
    tintClassName: "bg-emerald-50 dark:bg-emerald-950/40",
  },
  {
    token: "amber",
    labelClassName: "text-amber-700 dark:text-amber-300",
    markerClassName: "bg-amber-500",
    borderClassName: "border-amber-300 dark:border-amber-700",
    tintClassName: "bg-amber-50 dark:bg-amber-950/40",
  },
  {
    token: "rose",
    labelClassName: "text-rose-700 dark:text-rose-300",
    markerClassName: "bg-rose-500",
    borderClassName: "border-rose-300 dark:border-rose-700",
    tintClassName: "bg-rose-50 dark:bg-rose-950/40",
  },
  {
    token: "indigo",
    labelClassName: "text-indigo-700 dark:text-indigo-300",
    markerClassName: "bg-indigo-500",
    borderClassName: "border-indigo-300 dark:border-indigo-700",
    tintClassName: "bg-indigo-50 dark:bg-indigo-950/40",
  },
  {
    token: "cyan",
    labelClassName: "text-cyan-700 dark:text-cyan-300",
    markerClassName: "bg-cyan-500",
    borderClassName: "border-cyan-300 dark:border-cyan-700",
    tintClassName: "bg-cyan-50 dark:bg-cyan-950/40",
  },
] as const

export interface ResourceCalendarColor {
  token: (typeof RESOURCE_COLOR_PALETTE)[number]["token"]
  labelClassName: string
  markerClassName: string
  borderClassName: string
  tintClassName: string
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
