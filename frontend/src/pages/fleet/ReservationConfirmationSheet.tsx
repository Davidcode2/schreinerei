import { useEffect, useMemo, useState } from "react"
import { toast } from "sonner"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Calendar } from "lucide-react"
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet"
import { useCreateReservation, usePreferences, useSites } from "@/lib/api/hooks"
import { formatDateTimeLocalInput, formatLocalDateKey } from "@/lib/utils"
import type { ResourceType } from "@/types/fleet"

interface ReservationConfirmationSheetProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  resourceId: string
  resourceType: ResourceType
  resourceName: string
  startDate: string
  endDate: string
  projectId?: string | undefined
}

function formatDateRange(startDate: string, endDate: string): string {
  const formattedStart = new Date(`${startDate}T12:00:00`).toLocaleDateString("de-DE")
  const formattedEnd = new Date(`${endDate}T12:00:00`).toLocaleDateString("de-DE")

  return startDate === endDate
    ? formattedStart
    : `${formattedStart} bis ${formattedEnd}`
}

function toRfc3339(datetimeLocal: string): string {
  return new Date(datetimeLocal).toISOString()
}

function roundUpToNextHalfHour(date: Date): Date {
  const rounded = new Date(date)

  if (rounded.getSeconds() > 0 || rounded.getMilliseconds() > 0) {
    rounded.setMinutes(rounded.getMinutes() + 1)
  }

  rounded.setSeconds(0, 0)

  const minutes = rounded.getMinutes()
  const remainder = minutes % 30

  if (remainder !== 0) {
    rounded.setMinutes(minutes + (30 - remainder))
  }

  return rounded
}

export function buildDefaultTimes(startDate: string, endDate: string, now: Date): {
  start: string
  end: string
} {
  const defaultStart = new Date(`${startDate}T08:00`)
  const defaultEnd = new Date(`${endDate}T17:00`)
  const start = formatLocalDateKey(now) === startDate
    ? new Date(Math.max(defaultStart.getTime(), roundUpToNextHalfHour(now).getTime()))
    : defaultStart

  if (defaultEnd > start) {
    return {
      start: formatDateTimeLocalInput(start),
      end: formatDateTimeLocalInput(defaultEnd),
    }
  }

  const fallbackEnd = new Date(start)
  fallbackEnd.setHours(fallbackEnd.getHours() + 1)

  return {
    start: formatDateTimeLocalInput(start),
    end: formatDateTimeLocalInput(fallbackEnd),
  }
}

const formatSiteOption = (site: { name: string; project_type: "external_site" | "internal_workshop" }) =>
  site.project_type === "internal_workshop" ? `${site.name} (Werkstatt)` : `${site.name} (Extern)`

export function ReservationConfirmationSheet({
  open,
  onOpenChange,
  resourceId,
  resourceType,
  resourceName,
  startDate,
  endDate,
  projectId,
}: ReservationConfirmationSheetProps) {
  const { data: preferences } = usePreferences()
  const { data: sites } = useSites()
  const createReservation = useCreateReservation()
  const defaultTimes = useMemo(
    () => buildDefaultTimes(startDate, endDate, new Date()),
    [startDate, endDate]
  )

  const [siteId, setSiteId] = useState(preferences?.active_site_id ?? "")
  const [purpose, setPurpose] = useState("")
  const [useCustomTimes, setUseCustomTimes] = useState(false)
  const [startTime, setStartTime] = useState(defaultTimes.start)
  const [endTime, setEndTime] = useState(defaultTimes.end)

  useEffect(() => {
    if (!open) {
      return
    }

    const defaultSiteId = projectId ?? preferences?.active_site_id ?? ""
    const selectedSite = sites?.find((site) => site.id === defaultSiteId)

    setSiteId(defaultSiteId)
    setPurpose(selectedSite ? `Reservierung fuer ${selectedSite.name}` : "")
    setUseCustomTimes(false)
    setStartTime(defaultTimes.start)
    setEndTime(defaultTimes.end)
  }, [open, projectId, preferences?.active_site_id, sites, defaultTimes])

  const handleConfirm = async () => {
    const resolvedStartTime = useCustomTimes ? startTime : defaultTimes.start
    const resolvedEndTime = useCustomTimes ? endTime : defaultTimes.end

    if (!resolvedStartTime || !resolvedEndTime) {
      toast.error("Bitte Start- und Endzeit angeben")
      return
    }

    try {
      await createReservation.mutateAsync({
        resource_id: resourceId,
        resource_type: resourceType,
        site_id: siteId || null,
        project_id: siteId || null,
        start_time: toRfc3339(resolvedStartTime),
        end_time: toRfc3339(resolvedEndTime),
        purpose: purpose.trim() || null,
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
          <SheetTitle className="flex items-center gap-2">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent">
              <Calendar className="h-4 w-4" />
            </div>
            Reservierung bestaetigen
          </SheetTitle>
          <SheetDescription>
            {resourceName} fuer {formatDateRange(startDate, endDate)} reservieren.
          </SheetDescription>
        </SheetHeader>

        <div className="space-y-4 py-4">
          <div className="rounded-xl border bg-accent/40 p-4 text-sm">
            <p className="font-display font-normal">{resourceName}</p>
            <p className="text-muted-foreground">{formatDateRange(startDate, endDate)}</p>
          </div>

          <div className="space-y-2">
            <Label htmlFor="reservation-site">Projekt (optional)</Label>
            <select
              id="reservation-site"
              className="w-full h-10 rounded-lg border border-input bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-ring focus:ring-offset-2"
              value={siteId}
              onChange={(event) => setSiteId(event.target.value)}
            >
              <option value="">Keine Zuordnung</option>
              {sites?.map((site) => (
                <option key={site.id} value={site.id}>
                  {formatSiteOption(site)}
                </option>
              ))}
            </select>
          </div>

          <div className="space-y-2">
            <Label htmlFor="reservation-purpose">Zweck (optional)</Label>
            <Input
              id="reservation-purpose"
              maxLength={160}
              value={purpose}
              onChange={(event) => setPurpose(event.target.value)}
              placeholder="z.B. Montage vor Ort"
              className="h-10"
            />
          </div>

          <div className="space-y-3 rounded-xl border p-4">
            <label className="flex items-center gap-3 cursor-pointer">
              <input
                id="custom-times"
                type="checkbox"
                checked={useCustomTimes}
                onChange={(event) => setUseCustomTimes(event.target.checked)}
                className="h-4 w-4 rounded border-input"
              />
              <Label htmlFor="custom-times" className="cursor-pointer">Zeitangaben anpassen</Label>
            </label>

            {useCustomTimes && (
              <div className="grid gap-4 sm:grid-cols-2">
                <div className="space-y-2">
                  <Label htmlFor="start-time">Von</Label>
                  <Input
                    id="start-time"
                    type="datetime-local"
                    value={startTime}
                    onChange={(event) => setStartTime(event.target.value)}
                    className="h-10"
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="end-time">Bis</Label>
                  <Input
                    id="end-time"
                    type="datetime-local"
                    value={endTime}
                    onChange={(event) => setEndTime(event.target.value)}
                    className="h-10"
                  />
                </div>
              </div>
            )}
          </div>
        </div>

        <SheetFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)} className="shadow-sm">
            Abbrechen
          </Button>
          <Button onClick={handleConfirm} disabled={createReservation.isPending} className="shadow-sm">
            Reservierung bestaetigen
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
