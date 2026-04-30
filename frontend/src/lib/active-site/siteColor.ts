const SITE_COLORS = [
  "bg-sky-500",
  "bg-emerald-500",
  "bg-amber-500",
  "bg-rose-500",
  "bg-indigo-500",
  "bg-cyan-500",
  "bg-lime-500",
  "bg-orange-500",
] as const

function hashValue(input: string): number {
  let hash = 0
  for (let index = 0; index < input.length; index += 1) {
    hash = (hash << 5) - hash + input.charCodeAt(index)
    hash |= 0
  }
  return Math.abs(hash)
}

export function getSiteColorClass(siteId: string | null | undefined): string {
  if (!siteId) {
    return "bg-muted-foreground"
  }

  return SITE_COLORS[hashValue(siteId) % SITE_COLORS.length] ?? "bg-sky-500"
}
