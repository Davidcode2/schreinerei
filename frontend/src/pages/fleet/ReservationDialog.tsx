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
import { Badge } from "@/components/ui/badge"
import { Calendar, AlertCircle } from "lucide-react"
import { useCreateReservation, useUpdateReservation, useAvailability, useVehicles, useTools } from "@/lib/api/hooks"
import { StatusTransitionButtons, statusLabels } from "@/components/fleet/StatusTransitionButtons"
import { toast } from "sonner"
import type { ResourceType, Vehicle, Tool, Reservation, ReservationStatus } from "@/types/fleet"

const formatDateToRfc3339 = (datetimeLocal: string): string => {
  const date = new Date(datetimeLocal)
  return date.toISOString()
}

const formatDateTimeLocal = (rfc3339?: string): string => {
  if (!rfc3339) return ""
  const date = new Date(rfc3339)
  return date.toISOString().slice(0, 16)
}

const getStatusVariant = (status: ReservationStatus): "default" | "secondary" | "destructive" | "outline" => {
  switch (status) {
    case "pending":
      return "outline"
    case "confirmed":
      return "default"
    case "in_use":
      return "default"
    case "completed":
      return "secondary"
    case "cancelled":
      return "destructive"
  }
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
  const [selectedResourceId, setSelectedResourceId] = useState(
    initialData?.resource_id ?? resourceId ?? ""
  )
  const [selectedResourceType, setSelectedResourceType] = useState<ResourceType>(
    initialData?.resource_type ?? resourceType ?? "vehicle"
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

  const createMutation = useCreateReservation()
  const updateMutation = useUpdateReservation()

  // Reset state when dialog opens with new data
  useEffect(() => {
    if (open) {
      if (mode === "edit" && initialData) {
        setSelectedResourceId(initialData.resource_id)
        setSelectedResourceType(initialData.resource_type)
        setSiteId(initialData.site_id ?? "")
        setStartTime(formatDateTimeLocal(initialData.start_time))
        setEndTime(formatDateTimeLocal(initialData.end_time))
        setNotes(initialData.notes ?? "")
      } else if (mode === "create") {
        setSelectedResourceId(resourceId ?? "")
        setSelectedResourceType(resourceType ?? "vehicle")
        setSiteId("")
        setStartTime(initialStartTime ?? "")
        setEndTime(initialEndTime ?? "")
        setNotes("")
      }
    }
  }, [open, mode, initialData, resourceId, resourceType, initialStartTime, initialEndTime])

  // Check availability when both times are set
  const { data: availability } = useAvailability(
    startTime && endTime && selectedResourceId
      ? {
          resource_type: selectedResourceType,
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
          ...(siteId ? { site_id: siteId } : {}),
          start_time: formatDateToRfc3339(startTime),
          end_time: formatDateToRfc3339(endTime),
          ...(notes ? { notes } : {}),
        })
        toast.success("Reservierung aktualisiert")
      } else {
        await createMutation.mutateAsync({
          resource_type: selectedResourceType,
          resource_id: selectedResourceId,
          ...(siteId ? { site_id: siteId } : {}),
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

  const resources = selectedResourceType === "vehicle" ? vehicles : tools
  const isEditing = mode === "edit" && initialData
  const canTransition = isEditing &&
    initialData.status !== "cancelled" &&
    initialData.status !== "completed"

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Calendar className="h-5 w-5" />
            {mode === "edit" ? "Reservierung bearbeiten" : "Reservierung erstellen"}
          </DialogTitle>
          <DialogDescription>
            {mode === "edit" ? "Reservierungsdetails ändern" : "Neue Reservierung anlegen"}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Status Display (edit mode only) */}
          {isEditing && (
            <div className="space-y-2">
              <Label>Status</Label>
              <div className="flex items-center gap-2">
                <Badge variant={getStatusVariant(initialData.status)}>
                  {statusLabels[initialData.status]}
                </Badge>
              </div>
            </div>
          )}

          {/* Status Transition Buttons (edit mode only, for active statuses) */}
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

          {/* Resource Type (create mode only) */}
          {mode === "create" && (
            <div className="space-y-2">
              <Label>Ressourcentyp</Label>
              <div className="flex gap-2">
                <Button
                  variant={selectedResourceType === "vehicle" ? "default" : "outline"}
                  size="sm"
                  onClick={() => {
                    setSelectedResourceType("vehicle")
                    setSelectedResourceId("")
                  }}
                >
                  Fahrzeug
                </Button>
                <Button
                  variant={selectedResourceType === "tool" ? "default" : "outline"}
                  size="sm"
                  onClick={() => {
                    setSelectedResourceType("tool")
                    setSelectedResourceId("")
                  }}
                >
                  Werkzeug
                </Button>
              </div>
            </div>
          )}

          {/* Resource Selection (create mode only) / Display (edit mode) */}
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
                  className="w-full h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
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

          {/* Time Range */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>Von</Label>
              <Input
                type="datetime-local"
                value={startTime}
                onChange={(e) => setStartTime(e.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label>Bis</Label>
              <Input
                type="datetime-local"
                value={endTime}
                onChange={(e) => setEndTime(e.target.value)}
              />
            </div>
          </div>

          {/* Availability Warning (create mode only) */}
          {mode === "create" && startTime && endTime && !isAvailable && (
            <div className="flex items-center gap-2 p-3 bg-yellow-50 dark:bg-yellow-950 rounded-lg text-sm">
              <AlertCircle className="h-4 w-4 text-yellow-600" />
              <span className="text-yellow-800 dark:text-yellow-200">
                Nicht verfügbar zu diesem Zeitpunkt
              </span>
            </div>
          )}

          {/* Notes */}
          <div className="space-y-2">
            <Label>Notiz (optional)</Label>
            <Input
              placeholder="z.B. Für Baustelle Müller"
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={
              mode === "create"
                ? createMutation.isPending || !isAvailable
                : updateMutation.isPending
            }
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
