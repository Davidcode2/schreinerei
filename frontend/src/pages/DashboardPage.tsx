import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { AlertTriangle, Package, ArrowRight, Building2, Clock, Calendar } from "lucide-react"
import { Link } from "react-router-dom"
import { StatusBadge, LoadingSpinner, ErrorState, EmptyState, PageHeader } from "@/components/shared"
import {
  useDashboardSites,
  useLowStockMaterials,
  useMyTimeEntries,
  usePreferences,
  useUpdatePreferences,
} from "@/lib/api/hooks"
import { StatsCard } from "@/components/dashboard/StatsCard"
import { SiteCard } from "@/components/dashboard/SiteCard"
import type { DashboardSite } from "@/types/sites"
import type { Material } from "@/types/inventory"
import type { TimeEntry } from "@/types/sites"
import { toast } from "sonner"

export default function DashboardPage() {
  const { data: sites, isLoading: sitesLoading, error: sitesError, refetch: refetchSites } = useDashboardSites()
  const { data: lowStock } = useLowStockMaterials()
  const { data: timeEntries } = useMyTimeEntries()
  const { data: preferences } = usePreferences()
  const updatePreferences = useUpdatePreferences()

  const activeSiteId = preferences?.active_site_id ?? null

  const activeSites = sites?.filter((s: DashboardSite) => s.status === "active") || []
  const todayEntries = timeEntries?.filter((e: TimeEntry) => 
    e.work_date === new Date().toISOString().split("T")[0]
  ) || []
  const totalHoursToday = todayEntries.reduce((sum: number, e: TimeEntry) => sum + e.hours, 0)

  const handleToggleActive = async (siteId: string, nextActive: boolean) => {
    try {
      await updatePreferences.mutateAsync({
        active_site_id: nextActive ? siteId : null,
      })
      toast.success(nextActive ? "Aktive Baustelle gesetzt" : "Aktive Baustelle entfernt")
    } catch {
      toast.error("Aktive Baustelle konnte nicht aktualisiert werden")
    }
  }

  return (
    <div className="space-y-6">
      <PageHeader
        title="Dashboard"
        description="Übersicht über alle Aktivitäten"
      />

      <div className="grid gap-4 grid-cols-2 md:grid-cols-4">
        <StatsCard
          title="Aktive Projekte"
          value={activeSites.length}
          icon={Building2}
        />
        <StatsCard
          title="Niedrige Bestände"
          value={lowStock?.length || 0}
          icon={AlertTriangle}
          description={lowStock && lowStock.length > 0 ? "Handlungsbedarf" : "Alles im grünen Bereich"}
        />
        <StatsCard
          title="Heute gebucht"
          value={`${totalHoursToday.toFixed(1)}h`}
          icon={Clock}
        />
        <StatsCard
          title="Diese Woche"
          value={`${(timeEntries || []).reduce((sum: number, e: TimeEntry) => sum + e.hours, 0).toFixed(1)}h`}
          icon={Calendar}
        />
      </div>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between pb-3">
          <CardTitle className="font-display text-lg">Aktive Projekte</CardTitle>
          <Link to="/sites">
            <Button variant="ghost" size="sm" className="gap-2 h-9">
              Alle anzeigen
              <ArrowRight className="h-4 w-4" />
            </Button>
          </Link>
        </CardHeader>
        <CardContent>
          {sitesLoading ? (
            <LoadingSpinner className="py-8" />
          ) : sitesError ? (
            <ErrorState message="Projekte konnten nicht geladen werden" onRetry={() => refetchSites()} />
          ) : activeSites.length === 0 ? (
            <EmptyState
              icon={Building2}
              title="Keine aktiven Projekte"
              description="Es gibt derzeit keine aktiven Projekte."
              action={
                <Link to="/sites">
                  <Button size="sm" className="h-10">Projekte anzeigen</Button>
                </Link>
              }
            />
          ) : (
            <div className="grid gap-4 grid-cols-1 md:grid-cols-2">
              {activeSites.map((site: DashboardSite) => (
                <SiteCard
                  key={site.id}
                  site={site}
                  isActive={activeSiteId === site.id}
                  onToggleActive={handleToggleActive}
                  isToggling={updatePreferences.isPending}
                />
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {lowStock && lowStock.length > 0 && (
        <Card className="border-amber-200 bg-amber-50/50 dark:border-amber-900/50 dark:bg-amber-950/30">
          <CardHeader className="pb-3">
            <CardTitle className="font-display text-lg flex items-center gap-3">
              <div className="h-8 w-8 rounded-lg bg-amber-100 dark:bg-amber-900/50 flex items-center justify-center">
                <AlertTriangle className="h-4 w-4 text-amber-600 dark:text-amber-400" />
              </div>
              Niedrige Bestände
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-1">
              {lowStock.slice(0, 5).map((material: Material) => (
                <Link
                  key={material.id}
                  to={`/inventory/${material.id}`}
                  className="flex items-center justify-between p-2.5 rounded-lg hover:bg-amber-100/60 dark:hover:bg-amber-900/30 transition-colors"
                >
                  <div className="flex items-center gap-3 min-w-0">
                    <div className="h-7 w-7 rounded bg-amber-100 dark:bg-amber-900/50 flex items-center justify-center flex-shrink-0">
                      <Package className="h-3.5 w-3.5 text-amber-600 dark:text-amber-400" />
                    </div>
                    <span className="font-medium text-sm truncate">{material.name}</span>
                  </div>
                  <StatusBadge status="low_stock" />
                </Link>
              ))}
              {lowStock.length > 5 && (
                <Link to="/inventory" className="block">
                  <Button variant="ghost" size="sm" className="w-full h-9 gap-2 mt-1">
                    {lowStock.length - 5} weitere anzeigen
                    <ArrowRight className="h-3.5 w-3.5" />
                  </Button>
                </Link>
              )}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
