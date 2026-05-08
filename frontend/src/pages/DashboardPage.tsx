import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { AlertTriangle, ArrowRight, Building2, Clock, Calendar } from "lucide-react"
import { useState } from "react"
import { Link } from "react-router-dom"
import { LoadingSpinner, ErrorState, EmptyState, PageHeader } from "@/components/shared"
import {
  useDashboardSites,
  useInventoryAlerts,
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

const defaultStatuses = ["active", "planned", "completed"]
const statusOptions = [
  { value: "active", label: "Aktiv" },
  { value: "planned", label: "Geplant" },
  { value: "completed", label: "Abgeschlossen" },
  { value: "archived", label: "Archiviert" },
] as const

export default function DashboardPage() {
  const { data: sites, isLoading: sitesLoading, error: sitesError, refetch: refetchSites } = useDashboardSites()
  const { data: lowStock = [] } = useLowStockMaterials()
  const { data: inventoryAlerts = [] } = useInventoryAlerts()
  const { data: timeEntries } = useMyTimeEntries()
  const { data: preferences } = usePreferences()
  const updatePreferences = useUpdatePreferences()
  const [visibleStatuses, setVisibleStatuses] = useState<string[]>(defaultStatuses)

  const activeSiteId = preferences?.active_site_id ?? null

  const activeSites = sites?.filter((s: DashboardSite) => s.status === "active") || []
  const visibleSites = sites?.filter((site: DashboardSite) => visibleStatuses.includes(site.status)) || []
  const todayEntries = timeEntries?.filter((e: TimeEntry) => 
    e.work_date === new Date().toISOString().split("T")[0]
  ) || []
  const totalHoursToday = todayEntries.reduce((sum: number, e: TimeEntry) => sum + e.hours, 0)
  const warningMaterials = Array.from(
    new Map([...lowStock, ...inventoryAlerts].map((material) => [material.id, material])).values()
  )

  const toggleStatus = (status: string) => {
    setVisibleStatuses((current) =>
      current.includes(status)
        ? current.filter((value) => value !== status)
        : [...current, status]
    )
  }

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
          title="Materialwarnungen"
          value={warningMaterials.length}
          icon={AlertTriangle}
          description={warningMaterials.length > 0 ? "Ablauf oder Nachschub" : "Alles im grünen Bereich"}
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

      {warningMaterials.length > 0 && (
        <Card className="border-warning/30 bg-warning/5">
          <CardHeader className="pb-3">
            <CardTitle className="font-display text-lg flex items-center gap-3">
              <div className="h-8 w-8 rounded-lg bg-warning/15 flex items-center justify-center">
                <AlertTriangle className="h-4 w-4 text-warning" />
              </div>
              Materialwarnungen
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-1">
              {warningMaterials.slice(0, 6).map((material: Material) => {
                const warningCopy = material.expired_quantity > 0
                  ? `${material.expired_quantity} ${material.unit} abgelaufen`
                  : material.expiring_soon_quantity > 0
                    ? `${material.expiring_soon_quantity} ${material.unit} bald ablaufend`
                    : `Bestand unter Minimum (${material.quantity} / ${material.min_quantity} ${material.unit})`

                return (
                  <Link
                    key={material.id}
                    to={`/inventory/${material.id}`}
                    className="flex items-center justify-between p-2.5 rounded-lg hover:bg-warning/10 transition-colors"
                  >
                    <div className="min-w-0">
                      <span className="font-medium text-sm truncate block">{material.name}</span>
                      <span className="text-xs text-muted-foreground">{warningCopy}</span>
                    </div>
                    <ArrowRight className="h-3.5 w-3.5 text-muted-foreground flex-shrink-0" />
                  </Link>
                )
              })}
              {warningMaterials.length > 6 && (
                <Link to="/inventory" className="block">
                  <Button variant="ghost" size="sm" className="w-full h-9 gap-2 mt-1">
                    {warningMaterials.length - 6} weitere anzeigen
                    <ArrowRight className="h-3.5 w-3.5" />
                  </Button>
                </Link>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      <Card>
        <CardHeader className="flex flex-row items-center justify-between pb-3">
          <div className="space-y-3">
            <CardTitle className="font-display text-lg">Projekte</CardTitle>
            <div className="flex flex-wrap gap-2">
              {statusOptions.map(({ value, label }) => (
                <Button
                  key={value}
                  variant={visibleStatuses.includes(value) ? "default" : "outline"}
                  size="sm"
                  className="h-8"
                  onClick={() => toggleStatus(value)}
                >
                  {label}
                </Button>
              ))}
            </div>
          </div>
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
          ) : visibleSites.length === 0 ? (
            <EmptyState
              icon={Building2}
              title="Keine Projekte im aktuellen Filter"
              description="Passe die Statusfilter an, um weitere Projekte einzublenden."
              action={
                <Link to="/sites">
                  <Button size="sm" className="h-10">Projekte anzeigen</Button>
                </Link>
              }
            />
          ) : (
            <div className="grid gap-4 grid-cols-1 md:grid-cols-2">
              {visibleSites.map((site: DashboardSite) => (
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

    </div>
  )
}
