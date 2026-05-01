import { useState } from "react"
import { toast } from "sonner"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet"
import { useCreateReservation, usePreferences, useSites } from "@/lib/api/hooks"
import type { ResourceType } from "@/types/fleet"

interface ReservationConfirmationSheetProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  resourceId: string
  resourceType: ResourceType
  resourceName: string
  startDate: string
  endDate: string
}

function formatDateRange(startDate: string, endDate: string): string {
  const formattedStart = new Date(`${startDate}T12:00:00`).toLocaleDateString("de-DE")
  const formattedEnd = new Date(`${endDate}T12:00:00`).toLocaleDateString("de-DE")

  return startDate === endDate
    ? formattedStart
    : `${formattedStart} bis ${formattedEnd}`
}

function toDefaultStartTime(date: string): string {
  return `${date}T08:00`
}

function toDefaultEndTime(date: string): string {
  return `${date}T17:00`
}

function toRfc3339(datetimeLocal: string): string {
  return new Date(datetimeLocal).toISOString()
}

export function ReservationConfirmationSheet({
  open,
  onOpenChange,
  resourceId,
  resourceType,
  resourceName,
  startDate,
  endDate,
}: ReservationConfirmationSheetProps) {
  const { data: preferences } = usePreferences()
  const { data: sites } = useSites()
  const createReservation = useCreateReservation()

  const [siteId, setSiteId] = useState(preferences?.active_site_id ?? "")
  const [useCustomTimes, setUseCustomTimes] = useState(false)
  const [startTime, setStartTime] = useState(() => toDefaultStartTime(startDate))
  const [endTime, setEndTime] = useState(() => toDefaultEndTime(endDate))

  const handleConfirm = async () => {
    const resolvedStartTime = useCustomTimes ? startTime : toDefaultStartTime(startDate)
    const resolvedEndTime = useCustomTimes ? endTime : toDefaultEndTime(endDate)

    if (!resolvedStartTime || !resolvedEndTime) {
      toast.error("Bitte Start- und Endzeit angeben")
      return
    }

    try {
      await createReservation.mutateAsync({
        resource_id: resourceId,
        resource_type: resourceType,
        site_id: siteId || null,
        start_time: toRfc3339(resolvedStartTime),
        end_time: toRfc3339(resolvedEndTime),
      })
      toast.success("Reservierung erstellt")
      onOpenChange(false)
    } catch {
      toast.error("Reservierung fehlgeschlagen")
    }
  }

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent
        side="bottom"
        className="mx-auto max-w-2xl rounded-t-2xl"
        data-testid="reservation-confirmation-sheet"
      >
        <SheetHeader>
          <SheetTitle>Reservierung bestaetigen</SheetTitle>
          <SheetDescription>
            {resourceName} fuer {formatDateRange(startDate, endDate)} reservieren.
          </SheetDescription>
        </SheetHeader>

        <div className="space-y-4 py-4">
          <div className="rounded-lg border bg-muted/30 p-4 text-sm">
            <p className="font-medium">Ausgewaehlter Zeitraum</p>
            <p className="text-muted-foreground">{formatDateRange(startDate, endDate)}</p>
          </div>

          <div className="space-y-2">
            <Label htmlFor="reservation-site">Baustelle (optional)</Label>
            <select
              id="reservation-site"
              className="w-full h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
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

          <div className="space-y-3 rounded-lg border p-4">
            <div className="flex items-center gap-3">
              <input
                id="custom-times"
                type="checkbox"
                checked={useCustomTimes}
                onChange={(event) => setUseCustomTimes(event.target.checked)}
              />
              <Label htmlFor="custom-times">Zeitangaben anpassen</Label>
            </div>

            {useCustomTimes && (
              <div className="grid gap-4 sm:grid-cols-2">
                <div className="space-y-2">
                  <Label htmlFor="start-time">Von</Label>
                  <Input
                    id="start-time"
                    type="datetime-local"
                    value={startTime}
                    onChange={(event) => setStartTime(event.target.value)}
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="end-time">Bis</Label>
                  <Input
                    id="end-time"
                    type="datetime-local"
                    value={endTime}
                    onChange={(event) => setEndTime(event.target.value)}
                  />
                </div>
              </div>
            )}
          </div>
        </div>

        <SheetFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button onClick={handleConfirm} disabled={createReservation.isPending}>
            Reservierung bestaetigen
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
