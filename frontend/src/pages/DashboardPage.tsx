import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { AlertTriangle, Package, ArrowRight } from "lucide-react"
import { Link } from "react-router-dom"
import { StatusBadge, LoadingSpinner, ErrorState, EmptyState } from "@/components/shared"
import {
  useDashboardSites,
  useLowStockMaterials,
  useMyTimeEntries,
} from "@/lib/api/hooks"
import { StatsCard } from "@/components/dashboard/StatsCard"
import { SiteCard } from "@/components/dashboard/SiteCard"
import type { DashboardSite } from "@/types/sites"
import type { Material } from "@/types/inventory"
import type { TimeEntry } from "@/types/sites"

export default function DashboardPage() {
  const { data: sites, isLoading: sitesLoading, error: sitesError, refetch: refetchSites } = useDashboardSites()
  const { data: lowStock, isLoading: lowStockLoading } = useLowStockMaterials()
  const { data: timeEntries, isLoading: timeLoading } = useMyTimeEntries()

  const activeSites = sites?.filter((s: DashboardSite) => s.status === "active") || []
  const todayEntries = timeEntries?.filter((e: TimeEntry) => 
    e.work_date === new Date().toISOString().split("T")[0]
  ) || []
  const totalHoursToday = todayEntries.reduce((sum: number, e: TimeEntry) => sum + e.hours, 0)

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground">Übersicht über alle Aktivitäten</p>
      </div>

      {/* Stats Grid */}
      <div className="grid gap-4 grid-cols-2 md:grid-cols-4">
        <StatsCard
          title="Aktive Baustellen"
          value={activeSites.length}
          icon={({ className }: { className?: string }) => (
            <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M3 21h18M9 8h1M9 12h1M9 16h1M14 8h1M14 12h1M14 16h1M5 21V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16" />
            </svg>
          )}
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
          icon={({ className }: { className?: string }) => (
            <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <circle cx="12" cy="12" r="10" />
              <polyline points="12 6 12 12 16 14" />
            </svg>
          )}
        />
        <StatsCard
          title="Diese Woche"
          value={`${(timeEntries || []).reduce((sum: number, e: TimeEntry) => sum + e.hours, 0).toFixed(1)}h`}
          icon={({ className }: { className?: string }) => (
            <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
              <line x1="16" y1="2" x2="16" y2="6" />
              <line x1="8" y1="2" x2="8" y2="6" />
              <line x1="3" y1="10" x2="21" y2="10" />
            </svg>
          )}
        />
      </div>

      {/* Active Sites */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle className="text-lg">Aktive Baustellen</CardTitle>
          <Link to="/sites">
            <Button variant="ghost" size="sm" className="gap-2">
              Alle anzeigen
              <ArrowRight className="h-4 w-4" />
            </Button>
          </Link>
        </CardHeader>
        <CardContent>
          {sitesLoading ? (
            <LoadingSpinner className="py-8" />
          ) : sitesError ? (
            <ErrorState message="Baustellen konnten nicht geladen werden" onRetry={() => refetchSites()} />
          ) : activeSites.length === 0 ? (
            <EmptyState
              icon={({ className }: { className?: string }) => (
                <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <path d="M3 21h18M9 8h1M9 12h1M9 16h1M14 8h1M14 12h1M14 16h1M5 21V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16" />
                </svg>
              )}
              title="Keine aktiven Baustellen"
              description="Es gibt derzeit keine aktiven Baustellen."
              action={
                <Link to="/sites">
                  <Button size="sm">Baustellen anzeigen</Button>
                </Link>
              }
            />
          ) : (
            <div className="grid gap-4 grid-cols-1 md:grid-cols-2">
              {activeSites.map((site: DashboardSite) => (
                <SiteCard key={site.id} site={site} />
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Low Stock Alert */}
      {lowStock && lowStock.length > 0 && (
        <Card className="border-yellow-200 bg-yellow-50 dark:border-yellow-900 dark:bg-yellow-950">
          <CardHeader>
            <CardTitle className="text-lg flex items-center gap-2">
              <AlertTriangle className="h-5 w-5 text-yellow-600" />
              Niedrige Bestände
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {lowStock.slice(0, 5).map((material: Material) => (
                <Link
                  key={material.id}
                  to={`/inventory/${material.id}`}
                  className="flex items-center justify-between p-2 rounded-lg hover:bg-yellow-100 dark:hover:bg-yellow-900 transition-colors"
                >
                  <div className="flex items-center gap-3">
                    <Package className="h-4 w-4 text-muted-foreground" />
                    <span className="font-medium">{material.name}</span>
                  </div>
                  <StatusBadge status="low_stock" />
                </Link>
              ))}
              {lowStock.length > 5 && (
                <Link to="/inventory">
                  <Button variant="link" className="w-full">
                    {lowStock.length - 5} weitere anzeigen
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
