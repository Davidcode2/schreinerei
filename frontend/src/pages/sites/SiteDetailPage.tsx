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
} from "lucide-react"
import { useNavigate, useParams } from "react-router-dom"
import {
  PageHeader,
  LoadingSpinner,
  ErrorState,
  StatusBadge,
} from "@/components/shared"
import { useSite, useActivities, useTimeEntries, useSiteAssignments } from "@/lib/api/hooks"
import { TimeEntryDialog } from "./TimeEntryDialog"
import { ActivityFeed } from "./ActivityFeed"
import { StatusChangeModal } from "./StatusChangeModal"
import { CreateNoteModal } from "./CreateNoteModal"
import { CameraUploadFlow } from "./CameraUploadFlow"
import { MediaViewer } from "./MediaViewer"
import { buildSiteDetailPath, resolveMediaViewerTarget } from "./mediaViewerRoute"

function formatDate(date: string | null): string {
  if (!date) return "-"
  return new Date(date).toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  })
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

  const totalHours = timeEntries?.reduce((sum, e) => sum + e.hours, 0) || 0

  const openCameraFlow = () => {
    setShowCameraFlow(true)
  }

  const openNoteModal = () => {
    setShowNoteModal(true)
  }

  return (
    <div className="space-y-6">
      <PageHeader
        title={site.name}
        description={site.customer_name}
        backTo="/sites"
        action={
          <Button
            onClick={() => setShowTimeDialog(true)}
            className="gap-2"
          >
            <Clock className="h-4 w-4" />
            Zeit buchen
          </Button>
        }
      />

      {/* Status Banner */}
      <Card>
        <CardContent className="p-4">
          <div className="flex flex-wrap items-center justify-between gap-4">
            <div className="flex items-center gap-4">
              <div 
                className="cursor-pointer hover:opacity-80 transition-opacity"
                onClick={() => setShowStatusModal(true)}
              >
                <StatusBadge status={site.status} />
              </div>
              {site.location && (
                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                  <MapPin className="h-4 w-4" />
                  <span>{site.location}</span>
                </div>
              )}
            </div>
            {(site.start_date || site.end_date) && (
              <div className="flex items-center gap-2 text-sm">
                <Calendar className="h-4 w-4 text-muted-foreground" />
                <span>
                  {formatDate(site.start_date)} – {formatDate(site.end_date)}
                </span>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Responsive Grid */}
      <div className="grid gap-6 md:grid-cols-2">
        {/* Details */}
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Details</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {site.description && (
              <div>
                <p className="text-sm text-muted-foreground mb-1">Beschreibung</p>
                <p className="text-sm">{site.description}</p>
              </div>
            )}

            <Separator />

            <div className="grid grid-cols-2 gap-4">
              <div>
                <p className="text-sm text-muted-foreground">Kunde</p>
                <p className="font-medium">{site.customer_name}</p>
              </div>
              <div>
                <p className="text-sm text-muted-foreground">Geplante Tage</p>
                <p className="font-medium">{site.estimated_days || "-"}</p>
              </div>
              <div>
                <p className="text-sm text-muted-foreground">Gebuchte Stunden</p>
                <p className="font-medium">{totalHours.toFixed(1)}h</p>
              </div>
              <div>
                <p className="text-sm text-muted-foreground">Zugewiesen</p>
                <p className="font-medium">{assignments?.length || 0} Mitarbeiter</p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Time Entries */}
        <Card>
          <CardHeader className="flex flex-row items-center justify-between">
            <CardTitle className="text-lg">Zeiterfassung</CardTitle>
            <Badge variant="outline">{totalHours.toFixed(1)}h gesamt</Badge>
          </CardHeader>
          <CardContent>
            {!timeEntries || timeEntries.length === 0 ? (
              <p className="text-sm text-muted-foreground text-center py-4">
                Noch keine Zeiteinträge
              </p>
            ) : (
              <div className="space-y-3">
                {timeEntries.slice(0, 5).map((entry) => (
                  <div
                    key={entry.id}
                    className="flex items-center justify-between py-2 border-b last:border-0"
                  >
                    <div>
                      <p className="text-sm font-medium capitalize">
                        {entry.work_type}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        {new Date(entry.work_date).toLocaleDateString("de-DE")}
                      </p>
                    </div>
                    <Badge variant="outline">{entry.hours}h</Badge>
                  </div>
                ))}
                {timeEntries.length > 5 && (
                  <p className="text-xs text-muted-foreground text-center">
                    +{timeEntries.length - 5} weitere Einträge
                  </p>
                )}
              </div>
            )}
          </CardContent>
        </Card>

        {/* Activities */}
        <Card className="md:col-span-2">
          <CardHeader className="flex flex-row items-center justify-between">
            <CardTitle className="text-lg">Aktivitäten</CardTitle>
            <div className="flex gap-2">
              <Button
                variant="outline"
                size="sm"
                title="Foto hinzufügen"
                onClick={openCameraFlow}
              >
                <Camera className="h-4 w-4" />
              </Button>
              <Button 
                variant="outline" 
                size="sm" 
                title="Notiz hinzufügen"
                onClick={openNoteModal}
              >
                <FileText className="h-4 w-4" />
              </Button>
            </div>
          </CardHeader>
          <CardContent>
            <ActivityFeed activities={activities || []} siteId={site.id} />
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

      <MediaViewer
        open={Boolean(activityId && attachmentId)}
        target={viewerTarget}
        onClose={() => navigate(buildSiteDetailPath(site.id))}
      />
    </div>
  )
}
