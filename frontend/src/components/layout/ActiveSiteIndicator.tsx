import { Building2 } from "lucide-react"
import { usePreferences, useSites } from "@/lib/api/hooks"
import { getSiteColorClass } from "@/lib/active-site/siteColor"

export function ActiveSiteIndicator() {
  const { data: preferences } = usePreferences()
  const { data: sites } = useSites()

  const activeSiteId = preferences?.active_site_id ?? null
  const activeSite = sites?.find((site) => site.id === activeSiteId)

  return (
    <div className="mx-2 mb-3 rounded-lg border bg-background/80 px-3 py-2">
      <div className="flex items-center gap-2">
        <span
          className={`h-2.5 w-2.5 rounded-full ${getSiteColorClass(activeSiteId)}`}
        />
        <span className="text-[11px] font-medium uppercase tracking-wide text-muted-foreground">
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
