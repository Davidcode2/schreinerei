import { Building2 } from "lucide-react"
import { usePreferences, useSites } from "@/lib/api/hooks"
import { getSiteColorClass } from "@/lib/active-site/siteColor"
import { cn } from "@/lib/utils"

interface ActiveSiteIndicatorProps {
  compact?: boolean
  className?: string
}

export function ActiveSiteIndicator({ compact = false, className }: ActiveSiteIndicatorProps) {
  const { data: preferences } = usePreferences()
  const { data: sites } = useSites()

  const activeSiteId = preferences?.active_site_id ?? null
  const activeSite = sites?.find((site) => site.id === activeSiteId)

  if (compact) {
    return (
      <div
        className={cn(
          "min-w-0 rounded-lg border border-border/60 bg-background px-3 py-1.5 shadow-sm",
          className
        )}
      >
        <div className="flex items-center gap-2.5">
          <span
            className={`h-2.5 w-2.5 flex-shrink-0 rounded-full ${getSiteColorClass(activeSiteId)}`}
          />
          <div className="min-w-0">
            <p className="text-[10px] font-semibold uppercase tracking-widest text-muted-foreground">
              Baustelle
            </p>
            <p className="truncate text-sm font-semibold leading-tight">
              {activeSite?.name ?? "Keine ausgewählt"}
            </p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className={cn("mx-3 mb-3 rounded-lg border border-border/60 bg-accent/40 px-3 py-2.5", className)}>
      <div className="flex items-center gap-2">
        <span
          className={`h-2.5 w-2.5 rounded-full ${getSiteColorClass(activeSiteId)}`}
        />
        <span className="text-[10px] font-semibold uppercase tracking-widest text-muted-foreground">
          Aktive Baustelle
        </span>
      </div>
      <p className="mt-1 flex items-center gap-2 text-sm font-medium">
        <Building2 className="h-3.5 w-3.5 text-muted-foreground" />
        {activeSite?.name ?? "Keine aktive Baustelle"}
      </p>
    </div>
  )
}
