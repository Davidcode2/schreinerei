import { useState } from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus, ArrowRight } from "lucide-react"
import { Link } from 'react-router-dom'
import {
  PageHeader,
  EmptyState,
  ErrorState,
} from "@/components/shared"
import { SiteCard, SiteCardSkeleton } from "@/components/sites/SiteCard"
import { usePreferences, useSites, useUpdatePreferences } from "@/lib/api/hooks"
import { AddSiteDialog } from "./AddSiteDialog"
import type { Site, SiteStatus } from "@/types/sites"
import { toast } from "sonner"
import { cn } from "@/lib/utils"

const statusTabs: { value: SiteStatus | undefined; label: string }[] = [
  { value: undefined, label: "Alle" },
  { value: "planned", label: "Geplant" },
  { value: "active", label: "Aktiv" },
  { value: "completed", label: "Abgeschlossen" },
]

export default function SitesListPage() {
  const [selectedStatus, setSelectedStatus] = useState<SiteStatus | undefined>()
  const [searchQuery, setSearchQuery] = useState("")
  const [addSiteOpen, setAddSiteOpen] = useState(false)

  const {
    data: sites,
    isLoading,
    error,
    refetch,
  } = useSites(selectedStatus ? { status: selectedStatus } : undefined)
  const { data: preferences } = usePreferences()
  const updatePreferences = useUpdatePreferences()

  const activeSiteId = preferences?.active_site_id ?? null

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

  const filteredSites = sites?.filter((site: Site) =>
    site.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    site.customer_name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    site.location?.toLowerCase().includes(searchQuery.toLowerCase())
  )

  return (
    <div>
      <PageHeader
        title="Projekte"
        description="Externe Baustellen und interne Werkstattprojekte im Überblick"
        action={
          <div className="flex gap-2">
            <Link to="/sites/history">
              <Button variant="outline" className="gap-2 h-10">
                <ArrowRight className="h-4 w-4" />
                Historische Auswertung
              </Button>
            </Link>
            <Button className="gap-2 h-10 shadow-sm" onClick={() => setAddSiteOpen(true)}>
              <Plus className="h-4 w-4" />
              <span className="hidden sm:inline">Projekt anlegen</span>
              <span className="sm:hidden">Anlegen</span>
            </Button>
          </div>
        }
      />

      <div className="space-y-4 mb-6">
        <div className="flex gap-2 overflow-x-auto pb-1 -mb-1 scrollbar-none">
          {statusTabs.map((tab) => (
            <button
              key={tab.label}
              onClick={() => setSelectedStatus(tab.value)}
              className={cn(
                "flex-shrink-0 rounded-full px-4 py-1.5 text-sm font-medium transition-all",
                selectedStatus === tab.value
                  ? "bg-primary text-primary-foreground shadow-sm"
                  : "bg-card border border-border text-muted-foreground hover:bg-accent hover:text-foreground"
              )}
            >
              {tab.label}
            </button>
          ))}
        </div>

        <div className="relative">
          <Search className="absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder="Projekt suchen..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-10 h-10 bg-card border-border"
          />
        </div>
      </div>

      {isLoading ? (
        <div className="grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
          <SiteCardSkeleton count={6} />
        </div>
      ) : error ? (
        <ErrorState
          message="Baustellen konnten nicht geladen werden"
          onRetry={() => refetch()}
        />
      ) : !filteredSites || filteredSites.length === 0 ? (
        <EmptyState
          icon={Search}
          title={searchQuery ? "Keine Ergebnisse" : "Keine Projekte"}
          description={
            searchQuery
              ? "Keine Projekte entsprechen Ihrer Suche."
              : "Legen Sie Ihr erstes Projekt an."
          }
          action={
            !searchQuery && (
              <Button className="gap-2 h-10" onClick={() => setAddSiteOpen(true)}>
                <Plus className="h-4 w-4" />
                Projekt anlegen
              </Button>
            )
          }
        />
      ) : (
        <>
          <p className="text-sm text-muted-foreground mb-4">
            {filteredSites.length} Projekt{filteredSites.length !== 1 ? "e" : ""} gefunden
          </p>
          <div className="grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3">
            {filteredSites.map((site: Site) => (
              <SiteCard
                key={site.id}
                site={site}
                isActive={activeSiteId === site.id}
                onToggleActive={handleToggleActive}
                isToggling={updatePreferences.isPending}
              />
            ))}
          </div>
        </>
      )}

      <AddSiteDialog open={addSiteOpen} onOpenChange={setAddSiteOpen} />
    </div>
  )
}
