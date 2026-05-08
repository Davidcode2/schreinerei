import { useMemo, useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Separator } from "@/components/ui/separator"
import {
  MapPin,
  Calendar,
  Clock,
  Camera,
  FileText,
  Download,
  Building2,
  Users,
  Timer,
  PencilRuler,
} from "lucide-react"
import { useNavigate, useParams } from "react-router-dom"
import {
  PageHeader,
  LoadingSpinner,
  ErrorState,
  StatusBadge,
} from "@/components/shared"
import { useSite, useSiteSummary, useSiteInvoiceSummary, useActivities, useTimeEntries, useSiteAssignments } from "@/lib/api/hooks"
import { useAuthStore } from "@/lib/auth/authStore"
import { toast } from 'sonner'
import type { WorkType } from "@/types/sites"
import { TimeEntryDialog } from "./TimeEntryDialog"
import { ActivityFeed } from "./ActivityFeed"
import { StatusChangeModal } from "./StatusChangeModal"
import { CreateNoteModal } from "./CreateNoteModal"
import { CameraUploadFlow } from "./CameraUploadFlow"
import { MediaViewer } from "./MediaViewer"
import { ProjectAssignmentsSection } from "./ProjectAssignmentsSection"
import { ProjectPlanningSheet } from "./ProjectPlanningSheet"
import {
  buildMediaViewerPath,
  buildSiteDetailPath,
  resolveMediaViewerTarget,
} from "./mediaViewerRoute"

function formatDate(date: string | null): string {
  if (!date) return "-"
  return new Date(date).toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  })
}

function formatCurrency(cents: number | null): string {
  if (cents == null) return "-"
  return new Intl.NumberFormat("de-DE", {
    style: "currency",
    currency: "EUR",
  }).format(cents / 100)
}

function buildInvoiceSummaryFilename(siteName: string): string {
  const slug = siteName.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '')
  return `${slug || 'projekt'}-projektuebersicht.json`
}

function getWorkTypeLabel(workType: WorkType): string {
  const labels: Record<WorkType, string> = {
    site: "Baustelle",
    workshop: "Werkstatt",
    travel: "Fahrt",
    other: "Sonstiges",
  }

  return labels[workType]
}

export default function SiteDetailPage() {
  const { id, activityId, attachmentId } = useParams<{
    id: string
    activityId?: string
    attachmentId?: string
    slug?: string
  }>()
  const navigate = useNavigate()
  const user = useAuthStore((state) => state.user)
  const [showTimeDialog, setShowTimeDialog] = useState(false)
  const [showStatusModal, setShowStatusModal] = useState(false)
  const [showNoteModal, setShowNoteModal] = useState(false)
  const [showCameraFlow, setShowCameraFlow] = useState(false)
  const [showPlanningSheet, setShowPlanningSheet] = useState(false)

  const { data: site, isLoading, error, refetch } = useSite(id!)
  const { data: siteSummary } = useSiteSummary(id!)
  const { refetch: refetchInvoiceSummary } = useSiteInvoiceSummary(id!, false)
  const { data: activities, refetch: refetchActivities } = useActivities(id!)
  const { data: timeEntries } = useTimeEntries(id!)
  const { data: assignments } = useSiteAssignments(id!)

  const viewerTarget = useMemo(
    () => resolveMediaViewerTarget(activities || [], activityId, attachmentId),
    [activities, activityId, attachmentId]
  )
  const isAdmin = user?.role === 'admin'

  if (isLoading) {
    return <LoadingSpinner className="min-h-[400px]" size="lg" />
  }

  if (error || !site) {
    return (
      <ErrorState
        message="Baustelle konnte nicht geladen werden"
        onRetry={() => refetch()}
      />
    )
  }

  const viewerPath = viewerTarget
    ? buildMediaViewerPath(site.id, viewerTarget.activity.id, viewerTarget.attachment.attachment_id, viewerTarget.title)
    : buildSiteDetailPath(id || "")

  const totalHours = siteSummary?.labor.total_hours ?? 0
  const materialSummary = siteSummary?.materials

  async function handleExportInvoiceSummary() {
    try {
      const response = await refetchInvoiceSummary()
      if (!response.data) {
        throw new Error('missing summary')
      }

      const blob = new Blob([JSON.stringify(response.data, null, 2)], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = buildInvoiceSummaryFilename(site.name)
      document.body.appendChild(link)
      link.click()
      link.remove()
      URL.revokeObjectURL(url)
      toast.success('Projektübersicht exportiert')
    } catch {
      toast.error('Projektübersicht konnte nicht exportiert werden')
    }
  }

  return (
    <div className="space-y-6">
      <PageHeader
        title={site.name}
        description={site.customer_name}
        backTo="/sites"
        action={
          <div className="flex gap-2">
            {isAdmin && (
              <Button
                variant="outline"
                onClick={handleExportInvoiceSummary}
                className="gap-2 h-10"
              >
                <Download className="h-4 w-4" />
                Projektübersicht exportieren
              </Button>
            )}
            <Button
              variant="outline"
              onClick={() => setShowPlanningSheet(true)}
              className="gap-2 h-10"
            >
              <PencilRuler className="h-4 w-4" />
              Planen
            </Button>
            <Button
              onClick={() => setShowTimeDialog(true)}
              className="gap-2 h-10 shadow-sm"
            >
              <Clock className="h-4 w-4" />
              Zeit buchen
            </Button>
          </div>
        }
      />

      <Card className="overflow-hidden">
        <CardContent className="p-5">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div className="flex items-center gap-3 flex-wrap">
              <div
                className="cursor-pointer hover:opacity-80 transition-opacity"
                onClick={() => setShowStatusModal(true)}
              >
                <StatusBadge status={site.status} />
              </div>
              <Badge variant="outline" className="text-xs font-normal">
                {site.project_type === "internal_workshop" ? "Werkstattprojekt" : "Baustelle"}
              </Badge>
              {site.location && (
                <div className="flex items-center gap-1.5 text-sm text-muted-foreground">
                  <MapPin className="h-4 w-4 flex-shrink-0" />
                  <span>{site.location}</span>
                </div>
              )}
            </div>
            {(site.start_date || site.end_date) && (
              <div className="flex items-center gap-1.5 text-sm text-muted-foreground">
                <Calendar className="h-4 w-4 flex-shrink-0" />
                <span>
                  {formatDate(site.start_date)} – {formatDate(site.end_date)}
                </span>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="space-y-3 pb-3">
          <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
            <div className="space-y-1.5">
              <CardTitle className="text-base font-semibold">Projekt-Timeline</CardTitle>
              <p className="text-sm text-muted-foreground">
                Der zentrale Ort für Notizen, Fotos und Dokumente rund um dieses Projekt.
              </p>
            </div>
            <div className="flex gap-2">
              <Button
                variant="outline"
                size="sm"
                className="gap-2 h-9"
                onClick={() => setShowCameraFlow(true)}
              >
                <Camera className="h-4 w-4" />
                <span>Kamera</span>
              </Button>
              <Button
                variant="outline"
                size="sm"
                className="gap-2 h-9"
                onClick={() => setShowNoteModal(true)}
              >
                <FileText className="h-4 w-4" />
                <span>Eintrag</span>
              </Button>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <ActivityFeed activities={activities || []} siteId={site.id} />
        </CardContent>
      </Card>

      <div className="grid gap-4 sm:gap-6 md:grid-cols-2">
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-base font-semibold">Projektdetails</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {site.description && (
              <div>
                <p className="text-xs font-medium uppercase tracking-widest text-muted-foreground mb-1">Beschreibung</p>
                <p className="text-sm leading-relaxed">{site.description}</p>
              </div>
            )}

            {(site.budget_amount_cents != null || site.quote_reference || site.billing_reference || site.billing_notes) && (
              <>
                <Separator />
                <div>
                  <p className="text-xs font-medium uppercase tracking-widest text-muted-foreground mb-3">Budget & Abrechnung</p>
                  <div className="space-y-3 rounded-lg bg-accent/25 p-4">
                    <div className="grid grid-cols-2 gap-4">
                      <div>
                        <p className="text-xs text-muted-foreground">Budget</p>
                        <p className="text-sm font-medium">{formatCurrency(site.budget_amount_cents)}</p>
                      </div>
                      <div>
                        <p className="text-xs text-muted-foreground">Gebuchte Stunden</p>
                        <p className="text-sm font-medium">{siteSummary ? `${siteSummary.labor.total_hours.toFixed(1)}h` : '-'}</p>
                      </div>
                      <div>
                        <p className="text-xs text-muted-foreground">Angebotsreferenz</p>
                        <p className="text-sm font-medium">{site.quote_reference || '-'}</p>
                      </div>
                      <div>
                        <p className="text-xs text-muted-foreground">Materialentnahmen</p>
                        <p className="text-sm font-medium">{siteSummary ? siteSummary.materials.withdrawal_count : 0}</p>
                      </div>
                      <div>
                        <p className="text-xs text-muted-foreground">Abrechnungsreferenz</p>
                        <p className="text-sm font-medium">{site.billing_reference || '-'}</p>
                      </div>
                      <div>
                        <p className="text-xs text-muted-foreground">Verbrauchte Materialien</p>
                        <p className="text-sm font-medium">{siteSummary ? siteSummary.materials.distinct_material_count : 0}</p>
                      </div>
                    </div>
                    {site.billing_notes && (
                      <div>
                        <p className="text-xs text-muted-foreground mb-1">Abrechnungshinweise</p>
                        <p className="text-sm leading-relaxed">{site.billing_notes}</p>
                      </div>
                    )}
                  </div>
                </div>
              </>
            )}

            <Separator />

            {siteSummary && (
              <>
                <div>
                  <p className="text-xs font-medium uppercase tracking-widest text-muted-foreground mb-3">Projektkennzahlen</p>
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <p className="text-xs text-muted-foreground">Materialien</p>
                      <p className="text-sm font-medium">{materialSummary?.distinct_material_count || 0}</p>
                    </div>
                    <div>
                      <p className="text-xs text-muted-foreground">Entnahmen</p>
                      <p className="text-sm font-medium">{materialSummary?.withdrawal_count || 0}</p>
                    </div>
                    <div>
                      <p className="text-xs text-muted-foreground">Baustelle-Stunden</p>
                      <p className="text-sm font-medium">{siteSummary.labor.site_hours.toFixed(1)}h</p>
                    </div>
                    <div>
                      <p className="text-xs text-muted-foreground">Werkstatt-Stunden</p>
                      <p className="text-sm font-medium">{siteSummary.labor.workshop_hours.toFixed(1)}h</p>
                    </div>
                  </div>
                </div>

                {materialSummary && materialSummary.lines.length > 0 && (
                  <>
                    <Separator />
                    <div>
                      <p className="text-xs font-medium uppercase tracking-widest text-muted-foreground mb-3">Materialverbrauch</p>
                      <div className="space-y-3">
                        {materialSummary.lines.slice(0, 4).map((line) => (
                          <div key={line.material_id} className="flex items-start justify-between gap-3 rounded-lg bg-accent/25 p-3">
                            <div>
                              <p className="text-sm font-medium">{line.material_name}</p>
                              <p className="text-xs text-muted-foreground">{line.category_name}</p>
                            </div>
                            <div className="text-right">
                              <p className="text-sm font-medium">{line.total_withdrawn} {line.unit}</p>
                              <p className="text-xs text-muted-foreground">{line.withdrawal_count} Entnahmen</p>
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  </>
                )}
              </>
            )}

            <div className="grid grid-cols-2 gap-4">
              <div className="flex items-start gap-2.5">
                <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                  <Building2 className="h-3.5 w-3.5 text-muted-foreground" />
                </div>
                <div>
                  <p className="text-xs text-muted-foreground">{site.project_type === "internal_workshop" ? "Bezug" : "Kunde"}</p>
                  <p className="text-sm font-medium">{site.customer_name || "-"}</p>
                </div>
              </div>
              <div className="flex items-start gap-2.5">
                <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                  <Calendar className="h-3.5 w-3.5 text-muted-foreground" />
                </div>
                <div>
                  <p className="text-xs text-muted-foreground">Geplante Tage</p>
                  <p className="text-sm font-medium">{site.estimated_days || "-"}</p>
                </div>
              </div>
              <div className="flex items-start gap-2.5">
                <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                  <Timer className="h-3.5 w-3.5 text-muted-foreground" />
                </div>
                <div>
                  <p className="text-xs text-muted-foreground">Gebuchte Stunden</p>
                  <p className="text-sm font-medium">{totalHours.toFixed(1)}h</p>
                </div>
              </div>
              <div className="flex items-start gap-2.5">
                <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                  <Users className="h-3.5 w-3.5 text-muted-foreground" />
                </div>
                <div>
                  <p className="text-xs text-muted-foreground">Zugewiesen</p>
                  <p className="text-sm font-medium">{assignments?.length || 0} Mitarbeiter</p>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-3">
            <CardTitle className="text-base font-semibold">Zeiterfassung</CardTitle>
            <Badge variant="outline" className="text-xs font-normal">{totalHours.toFixed(1)}h gesamt</Badge>
          </CardHeader>
          <CardContent>
            {!timeEntries || timeEntries.length === 0 ? (
              <div className="flex flex-col items-center py-8 text-center">
                <div className="rounded-2xl bg-accent/60 p-3 mb-3">
                  <Clock className="h-6 w-6 text-muted-foreground" />
                </div>
                <p className="text-sm text-muted-foreground">
                  Noch keine Zeiteinträge
                </p>
              </div>
            ) : (
              <div className="space-y-2">
                {timeEntries.slice(0, 5).map((entry) => (
                  <div
                    key={entry.id}
                    className="flex items-center justify-between py-2.5 px-3 rounded-lg hover:bg-accent/40 transition-colors"
                  >
                    <div>
                      <p className="text-sm font-medium capitalize">
                        {getWorkTypeLabel(entry.work_type)}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        {new Date(entry.work_date).toLocaleDateString("de-DE")}
                      </p>
                    </div>
                    <Badge variant="outline" className="text-xs font-normal">{entry.hours}h</Badge>
                  </div>
                ))}
                {timeEntries.length > 5 && (
                  <p className="text-xs text-muted-foreground text-center pt-2">
                    +{timeEntries.length - 5} weitere Einträge
                  </p>
                )}
              </div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-base font-semibold">Projektplanung</CardTitle>
          </CardHeader>
          <CardContent>
            <ProjectAssignmentsSection siteId={site.id} assignments={assignments || []} />
          </CardContent>
        </Card>

      </div>

      <TimeEntryDialog
        open={showTimeDialog}
        onOpenChange={setShowTimeDialog}
        siteId={site.id}
        siteName={site.name}
      />

      <StatusChangeModal
        open={showStatusModal}
        onOpenChange={setShowStatusModal}
        siteId={site.id}
        siteName={site.name}
        currentStatus={site.status}
        onSuccess={() => refetch()}
      />

      <CreateNoteModal
        open={showNoteModal}
        onOpenChange={setShowNoteModal}
        siteId={site.id}
        onSuccess={() => refetchActivities()}
      />

      <CameraUploadFlow
        open={showCameraFlow}
        onOpenChange={setShowCameraFlow}
        siteId={site.id}
        onSuccess={() => refetchActivities()}
      />

      <ProjectPlanningSheet
        open={showPlanningSheet}
        onOpenChange={setShowPlanningSheet}
        site={site}
      />

      <MediaViewer
        open={Boolean(activityId && attachmentId)}
        sharePath={viewerPath}
        target={viewerTarget}
        onClose={() => navigate(buildSiteDetailPath(site.id))}
      />
    </div>
  )
}
