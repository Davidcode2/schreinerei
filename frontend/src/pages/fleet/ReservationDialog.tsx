import { useState } from "react"
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
import { useCreateReservation, useAvailability, useVehicles, useTools } from "@/lib/api/hooks"
import { toast } from "sonner"
import type { ResourceType, Vehicle, Tool } from "@/types/fleet"

const formatDateToRfc3339 = (datetimeLocal: string): string => {
  const date = new Date(datetimeLocal)
  return date.toISOString()
}

interface ReservationDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  resourceId: string
  resourceType: ResourceType
}

export function ReservationDialog({
  open,
  onOpenChange,
  resourceId,
  resourceType,
}: ReservationDialogProps) {
  const [selectedResourceId, setSelectedResourceId] = useState(resourceId || "")
  const [selectedResourceType, setSelectedResourceType] = useState<ResourceType>(resourceType || "vehicle")
  const [siteId, setSiteId] = useState("")
  const [startTime, setStartTime] = useState("")
  const [endTime, setEndTime] = useState("")
  const [notes, setNotes] = useState("")

  const { data: vehicles } = useVehicles()
  const { data: tools } = useTools()

  const createMutation = useCreateReservation()

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

    if (!isAvailable) {
      toast.error("Das Resource ist zu diesem Zeitpunkt nicht verfügbar")
      return
    }

    try {
      await createMutation.mutateAsync({
        resource_type: selectedResourceType,
        resource_id: selectedResourceId,
        ...(siteId ? { site_id: siteId } : {}),
        start_time: formatDateToRfc3339(startTime),
        end_time: formatDateToRfc3339(endTime),
        ...(notes ? { notes } : {}),
      })
      toast.success("Reservierung erstellt")
      onOpenChange(false)
      // Reset form
      setSelectedResourceId("")
      setSiteId("")
      setStartTime("")
      setEndTime("")
      setNotes("")
    } catch {
      toast.error("Reservierung fehlgeschlagen")
    }
  }

  const resources = selectedResourceType === "vehicle" ? vehicles : tools

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Calendar className="h-5 w-5" />
            Reservierung erstellen
          </DialogTitle>
          <DialogDescription>
            Neue Reservierung anlegen
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Resource Type */}
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

          {/* Resource Selection */}
          {!resourceId && (
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

          {/* Availability Warning */}
          {startTime && endTime && !isAvailable && (
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
            disabled={createMutation.isPending || !isAvailable}
          >
            {createMutation.isPending ? "Wird erstellt..." : "Reservieren"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
