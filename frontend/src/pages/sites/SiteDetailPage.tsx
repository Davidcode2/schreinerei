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
import { useSite, useActivities, useTimeEntries, useSiteAssignments } from "@/lib/api/hooks"
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
  const [showTimeDialog, setShowTimeDialog] = useState(false)
  const [showStatusModal, setShowStatusModal] = useState(false)
  const [showNoteModal, setShowNoteModal] = useState(false)
  const [showCameraFlow, setShowCameraFlow] = useState(false)
  const [showPlanningSheet, setShowPlanningSheet] = useState(false)

  const { data: site, isLoading, error, refetch } = useSite(id!)
  const { data: activities, refetch: refetchActivities } = useActivities(id!)
  const { data: timeEntries } = useTimeEntries(id!)
  const { data: assignments } = useSiteAssignments(id!)

  const viewerTarget = useMemo(
    () => resolveMediaViewerTarget(activities || [], activityId, attachmentId),
    [activities, activityId, attachmentId]
  )

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

  const totalHours = timeEntries?.reduce((sum, e) => sum + e.hours, 0) || 0

  return (
    <div className="space-y-6">
      <PageHeader
        title={site.name}
        description={site.customer_name}
        backTo="/sites"
        action={
          <div className="flex gap-2">
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

            <Separator />

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
