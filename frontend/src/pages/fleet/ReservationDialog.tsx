import { useState, useEffect } from "react"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Calendar, AlertCircle } from "lucide-react"
import {
  useAvailability,
  useCreateReservation,
  usePreferences,
  useSites,
  useTools,
  useUpdateReservation,
  useVehicles,
} from "@/lib/api/hooks"
import { StatusTransitionButtons, statusLabels } from "@/components/fleet/StatusTransitionButtons"
import { formatDateTimeLocalInput } from "@/lib/utils"
import { toast } from "sonner"
import { cn } from "@/lib/utils"
import type { ResourceType, Vehicle, Tool, Reservation, ReservationStatus, ConflictDetail } from "@/types/fleet"

const formatDateToRfc3339 = (datetimeLocal: string): string => {
  const date = new Date(datetimeLocal)
  return date.toISOString()
}

const formatDateTimeLocal = (rfc3339?: string): string => {
  if (!rfc3339) return ""
  return formatDateTimeLocalInput(rfc3339)
}

const formatTimeRange = (start: string, end: string): string => {
  const startDate = new Date(start)
  const endDate = new Date(end)
  return `${startDate.toLocaleDateString("de-DE", { day: "2-digit", month: "2-digit" })} ${startDate.toLocaleTimeString("de-DE", { hour: "2-digit", minute: "2-digit" })} - ${endDate.toLocaleTimeString("de-DE", { hour: "2-digit", minute: "2-digit" })}`
}

const formatTimestamp = (value: string): string => {
  return new Date(value).toLocaleString("de-DE", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  })
}

interface ReservationDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  resourceId?: string
  resourceType?: ResourceType
  mode?: "create" | "edit"
  initialData?: Reservation
  initialStartTime?: string
  initialEndTime?: string
}

export function ReservationDialog({
  open,
  onOpenChange,
  resourceId,
  resourceType,
  mode = "create",
  initialData,
  initialStartTime,
  initialEndTime,
}: ReservationDialogProps) {
  const fixedResourceType = initialData?.resource_type ?? resourceType ?? "vehicle"
  const [selectedResourceId, setSelectedResourceId] = useState(
    initialData?.resource_id ?? resourceId ?? ""
  )
  const [siteId, setSiteId] = useState(initialData?.site_id ?? "")
  const [startTime, setStartTime] = useState(
    initialStartTime ?? formatDateTimeLocal(initialData?.start_time) ?? ""
  )
  const [endTime, setEndTime] = useState(
    initialEndTime ?? formatDateTimeLocal(initialData?.end_time) ?? ""
  )
  const [notes, setNotes] = useState(initialData?.notes ?? "")

  const { data: vehicles } = useVehicles()
  const { data: tools } = useTools()
  const { data: preferences } = usePreferences()
  const { data: sites } = useSites()

  const createMutation = useCreateReservation()
  const updateMutation = useUpdateReservation()

  useEffect(() => {
    if (open) {
        if (mode === "edit" && initialData) {
          setSelectedResourceId(initialData.resource_id)
          setSiteId(initialData.site_id ?? "")
          setStartTime(formatDateTimeLocal(initialData.start_time))
          setEndTime(formatDateTimeLocal(initialData.end_time))
          setNotes(initialData.notes ?? "")
        } else if (mode === "create") {
          setSelectedResourceId(resourceId ?? "")
          setSiteId(preferences?.active_site_id ?? "")
          setStartTime(initialStartTime ?? "")
          setEndTime(initialEndTime ?? "")
        setNotes("")
      }
    }
  }, [
    open,
    mode,
    initialData,
    resourceId,
    initialStartTime,
    initialEndTime,
    preferences?.active_site_id,
  ])

  const { data: availability } = useAvailability(
    startTime && endTime && selectedResourceId
      ? {
          resource_type: fixedResourceType,
          resource_id: selectedResourceId,
          start_time: formatDateToRfc3339(startTime),
          end_time: formatDateToRfc3339(endTime),
        }
      : { resource_type: "vehicle", resource_id: "", start_time: "", end_time: "" }
  )

  const isAvailable = availability?.available ?? true

  const handleSubmit = async () => {
    if (!selectedResourceId || !startTime || !endTime) {
      toast.error("Bitte füllen Sie alle Pflichtfelder aus")
      return
    }

    if (mode === "create" && !isAvailable) {
      toast.error("Die Resource ist zu diesem Zeitpunkt nicht verfügbar")
      return
    }

    try {
      if (mode === "edit" && initialData) {
        await updateMutation.mutateAsync({
          id: initialData.id,
          site_id: siteId || null,
          start_time: formatDateToRfc3339(startTime),
          end_time: formatDateToRfc3339(endTime),
          ...(notes ? { notes } : {}),
        })
        toast.success("Reservierung aktualisiert")
      } else {
        await createMutation.mutateAsync({
          resource_type: fixedResourceType,
          resource_id: selectedResourceId,
          site_id: siteId || null,
          start_time: formatDateToRfc3339(startTime),
          end_time: formatDateToRfc3339(endTime),
          ...(notes ? { notes } : {}),
        })
        toast.success("Reservierung erstellt")
      }
      onOpenChange(false)
    } catch {
      toast.error(
        mode === "edit" ? "Aktualisierung fehlgeschlagen" : "Reservierung fehlgeschlagen"
      )
    }
  }

  const resources = fixedResourceType === "vehicle" ? vehicles : tools
  const isEditing = mode === "edit" && initialData
  const canTransition = isEditing &&
    initialData.status !== "cancelled" &&
    initialData.status !== "completed"

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent">
              <Calendar className="h-4 w-4" />
            </div>
            {mode === "edit" ? "Reservierung bearbeiten" : "Reservierung erstellen"}
          </DialogTitle>
          <DialogDescription>
            {mode === "edit" ? "Reservierungsdetails ändern" : "Neue Reservierung anlegen"}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {isEditing && (
            <div className="space-y-4 rounded-xl border bg-muted/30 p-4">
              <div className="flex items-center justify-between gap-3">
                <div>
                  <p className="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
                    Reservierungsdetails
                  </p>
                  <p className="mt-1 text-sm font-semibold">{initialData.resource_name}</p>
                </div>
                <span className={cn(
                  "inline-flex rounded-full px-3 py-1 text-xs font-medium border",
                  initialData.status === "confirmed" ? "bg-success/15 text-success border-success/20" :
                  initialData.status === "in_use" ? "bg-destructive/10 text-destructive border-destructive/20" :
                  initialData.status === "pending" ? "bg-warning/15 text-warning-foreground border-warning/25" :
                  initialData.status === "completed" ? "bg-success/15 text-success border-success/20" :
                  initialData.status === "cancelled" ? "bg-destructive/10 text-destructive border-destructive/20" :
                  "bg-secondary text-secondary-foreground"
                )}>
                  {statusLabels[initialData.status]}
                </span>
              </div>
              <div className="grid gap-3 text-sm sm:grid-cols-2">
                <div>
                  <p className="text-muted-foreground">Gebucht von</p>
                  <p className="font-medium">{initialData.user_name || "Unbekannt"}</p>
                </div>
                <div>
                  <p className="text-muted-foreground">Baustelle</p>
                  <p className="font-medium">{initialData.site_name || "Keine Zuordnung"}</p>
                </div>
                <div>
                  <p className="text-muted-foreground">Erstellt</p>
                  <p className="font-medium">{formatTimestamp(initialData.created_at)}</p>
                </div>
                <div>
                  <p className="text-muted-foreground">Aktualisiert</p>
                  <p className="font-medium">{formatTimestamp(initialData.updated_at)}</p>
                </div>
              </div>
            </div>
          )}

          <div className="space-y-2">
            <Label>Baustelle (optional)</Label>
            <select
              className="w-full h-10 rounded-lg border border-input bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:ring-offset-2"
              value={siteId}
              onChange={(event) => setSiteId(event.target.value)}
            >
              <option value="">Keine Zuordnung</option>
              {sites?.map((site) => (
                <option key={site.id} value={site.id}>
                  {site.name}
                </option>
              ))}
            </select>
          </div>

          {canTransition && (
            <div className="space-y-2">
              <Label>Aktionen</Label>
              <StatusTransitionButtons
                reservationId={initialData.id}
                currentStatus={initialData.status}
                onTransition={() => {
                  toast.success("Status aktualisiert")
                  onOpenChange(false)
                }}
              />
            </div>
          )}

          {isEditing ? (
            <div className="space-y-2">
              <Label>Ressource</Label>
              <p className="text-sm font-medium">{initialData.resource_name}</p>
            </div>
          ) : (
            !resourceId && (
              <div className="space-y-2">
                <Label>Ressource</Label>
                <select
                  className="w-full h-10 rounded-lg border border-input bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:ring-offset-2"
                  value={selectedResourceId}
                  onChange={(e) => setSelectedResourceId(e.target.value)}
                >
                  <option value="">Bitte wählen...</option>
                  {resources?.map((r: Vehicle | Tool) => (
                    <option key={r.id} value={r.id}>
                      {r.name}
                    </option>
                  ))}
                </select>
              </div>
            )
          )}

          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>Von</Label>
              <Input
                type="datetime-local"
                value={startTime}
                onChange={(e) => setStartTime(e.target.value)}
                className="h-10"
              />
            </div>
            <div className="space-y-2">
              <Label>Bis</Label>
              <Input
                type="datetime-local"
                value={endTime}
                onChange={(e) => setEndTime(e.target.value)}
                className="h-10"
              />
            </div>
          </div>

          {mode === "create" && startTime && endTime && !isAvailable && (
            <div className="space-y-2">
              <div className="flex items-center gap-2 p-3 bg-warning/10 border border-warning/20 rounded-lg text-sm">
                <AlertCircle className="h-4 w-4 text-warning" />
                <span className="text-warning-foreground">
                  Nicht verfügbar zu diesem Zeitpunkt
                </span>
              </div>
              {availability?.conflicts && availability.conflicts.length > 0 && (
                <div className="space-y-1 text-sm">
                  <p className="font-medium text-muted-foreground">Bestehende Reservierungen:</p>
                  {availability.conflicts.map((conflict: ConflictDetail) => (
                    <div key={conflict.id} className="flex items-center gap-2 p-2 bg-muted rounded-lg">
                      <span className="font-medium">{conflict.user_name || "Unbekannt"}</span>
                      <span className="text-muted-foreground">
                        {formatTimeRange(conflict.start_time, conflict.end_time)}
                      </span>
                      <span className={cn(
                        "inline-flex rounded-full px-2 py-0.5 text-xs font-medium border",
                        conflict.status === "confirmed" ? "bg-success/15 text-success border-success/20" :
                        conflict.status === "pending" ? "bg-warning/15 text-warning-foreground border-warning/25" :
                        conflict.status === "cancelled" ? "bg-destructive/10 text-destructive border-destructive/20" :
                        "bg-secondary text-secondary-foreground"
                      )}>
                        {statusLabels[conflict.status as ReservationStatus] || conflict.status}
                      </span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}

          <div className="space-y-2">
            <Label>Notiz (optional)</Label>
            <Input
              placeholder="z.B. Für Baustelle Müller"
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              className="h-10"
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)} className="shadow-sm">
            Abbrechen
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={
              mode === "create"
                ? createMutation.isPending || !isAvailable
                : updateMutation.isPending
            }
            className="shadow-sm"
          >
            {mode === "edit"
              ? updateMutation.isPending
                ? "Wird gespeichert..."
                : "Speichern"
              : createMutation.isPending
                ? "Wird erstellt..."
                : "Reservieren"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
