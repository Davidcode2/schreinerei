import { useState } from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus } from "lucide-react"
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
    <div className="space-y-6">
      <PageHeader
        title="Baustellen"
        description="Baustellenverwaltung"
        action={
          <Button className="gap-2" onClick={() => setAddSiteOpen(true)}>
            <Plus className="h-4 w-4" />
            <span className="hidden sm:inline">Baustelle anlegen</span>
          </Button>
        }
      />

      {/* Status Tabs */}
      <div className="flex gap-2 overflow-x-auto pb-2 -mb-2">
        {statusTabs.map((tab) => (
          <Button
            key={tab.label}
            variant={selectedStatus === tab.value ? "default" : "outline"}
            size="sm"
            onClick={() => setSelectedStatus(tab.value)}
            className="rounded-full whitespace-nowrap"
          >
            {tab.label}
          </Button>
        ))}
      </div>

      {/* Search */}
      <div className="relative">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder="Baustelle suchen..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="pl-10"
        />
      </div>

      {/* Sites Grid */}
      {isLoading ? (
        <div className="grid gap-4 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
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
          title={searchQuery ? "Keine Ergebnisse" : "Keine Baustellen"}
          description={
            searchQuery
              ? "Keine Baustellen entsprechen Ihrer Suche."
              : "Legen Sie Ihre erste Baustelle an."
          }
          action={
            !searchQuery && (
              <Button className="gap-2" onClick={() => setAddSiteOpen(true)}>
                <Plus className="h-4 w-4" />
                Baustelle anlegen
              </Button>
            )
          }
        />
      ) : (
        <>
          <p className="text-sm text-muted-foreground">
            {filteredSites.length} Baustell{filteredSites.length !== 1 ? "en" : "e"} gefunden
          </p>
          <div className="grid gap-4 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
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

      {/* Add Site Dialog */}
      <AddSiteDialog open={addSiteOpen} onOpenChange={setAddSiteOpen} />
    </div>
  )
}
