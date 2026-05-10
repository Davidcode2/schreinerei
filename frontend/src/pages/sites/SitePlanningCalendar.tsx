import { useMemo, useState } from "react"
import { CalendarClock, ChevronLeft, ChevronRight, Plus, Trash2 } from "lucide-react"
import { toast } from "sonner"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { LoadingSpinner } from "@/components/shared"
import { cn, formatDateTimeLocalInput, formatLocalDateKey, startOfLocalWeek } from "@/lib/utils"
import {
  useCreateSiteAppointment,
  useDeleteSiteAppointment,
  useSiteAppointments,
  useUpdateSiteAppointment,
} from "@/lib/api/hooks"
import { useUsers } from "@/lib/api/hooks/useIam"
import type { SiteAppointment, SiteAssignment, SiteAppointmentKind } from "@/types/sites"

interface SitePlanningCalendarProps {
  siteId: string
  assignments: SiteAssignment[]
  canEdit: boolean
}

interface PlannerDraft {
  appointmentId?: string
  title: string
  appointment_kind: SiteAppointmentKind
  starts_at: string
  ends_at: string
  notes: string
  assigned_user_ids: string[]
}

const dayNames = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"]
const startHour = 6
const endHour = 20
const hourSlotHeight = 56
const totalVisibleHours = endHour - startHour

const appointmentKindMeta: Record<
  SiteAppointmentKind,
  { label: string; chipClassName: string; cardClassName: string }
> = {
  customer_appointment: {
    label: "Kundentermin",
    chipClassName: "bg-amber-100 text-amber-900 border-amber-300",
    cardClassName: "border-amber-300 bg-amber-50 text-amber-950",
  },
  worker_deployment: {
    label: "Einsatz",
    chipClassName: "bg-sky-100 text-sky-900 border-sky-300",
    cardClassName: "border-sky-300 bg-sky-50 text-sky-950",
  },
  milestone: {
    label: "Meilenstein",
    chipClassName: "bg-emerald-100 text-emerald-900 border-emerald-300",
    cardClassName: "border-emerald-300 bg-emerald-50 text-emerald-950",
  },
}

function getAppointmentKindMeta(
  kind: string
): { label: string; chipClassName: string; cardClassName: string } {
  return (
    appointmentKindMeta[kind as SiteAppointmentKind] ??
    appointmentKindMeta.customer_appointment!
  )
}

function formatWeekHeader(startDate: Date): string {
  const endDate = new Date(startDate)
  endDate.setDate(endDate.getDate() + 6)

  return `${startDate.toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "long",
  })} - ${endDate.toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "long",
    year: "numeric",
  })}`
}

function buildWeekBounds(date: Date) {
  const start = startOfLocalWeek(date)
  const end = new Date(start)
  end.setDate(end.getDate() + 7)

  return {
    start,
    end,
  }
}

function toIso(datetimeLocal: string): string {
  return new Date(datetimeLocal).toISOString()
}

function defaultDraft(day?: Date, hour: number = 8): PlannerDraft {
  const base = day ? new Date(day) : new Date()
  base.setHours(hour, 0, 0, 0)
  const end = new Date(base)
  end.setHours(Math.min(base.getHours() + 2, endHour), 0, 0, 0)

  return {
    title: "",
    appointment_kind: "customer_appointment",
    starts_at: formatDateTimeLocalInput(base),
    ends_at: formatDateTimeLocalInput(end),
    notes: "",
    assigned_user_ids: [],
  }
}

function appointmentToDraft(appointment: SiteAppointment): PlannerDraft {
  return {
    appointmentId: appointment.id,
    title: appointment.title,
    appointment_kind: appointment.appointment_kind,
    starts_at: formatDateTimeLocalInput(appointment.starts_at),
    ends_at: formatDateTimeLocalInput(appointment.ends_at),
    notes: appointment.notes ?? "",
    assigned_user_ids: appointment.assigned_user_ids,
  }
}

function getSegmentStyle(appointment: SiteAppointment, day: Date) {
  const dayStart = new Date(day)
  dayStart.setHours(startHour, 0, 0, 0)
  const dayEnd = new Date(day)
  dayEnd.setHours(endHour, 0, 0, 0)

  const appointmentStart = new Date(appointment.starts_at)
  const appointmentEnd = new Date(appointment.ends_at)

  const segmentStart = Math.max(appointmentStart.getTime(), dayStart.getTime())
  const segmentEnd = Math.min(appointmentEnd.getTime(), dayEnd.getTime())

  if (segmentEnd <= segmentStart) {
    return null
  }

  const top = ((segmentStart - dayStart.getTime()) / 3_600_000) * hourSlotHeight
  const height = Math.max(
    ((segmentEnd - segmentStart) / 3_600_000) * hourSlotHeight,
    36
  )

  return { top, height }
}

function formatTimeRange(start: string, end: string) {
  return `${new Date(start).toLocaleTimeString("de-DE", {
    hour: "2-digit",
    minute: "2-digit",
  })} - ${new Date(end).toLocaleTimeString("de-DE", {
    hour: "2-digit",
    minute: "2-digit",
  })}`
}

function formatSlotLabel(day: Date, hour: number) {
  return `${day.toLocaleDateString("de-DE", {
    weekday: "long",
    day: "2-digit",
    month: "2-digit",
  })} um ${String(hour).padStart(2, "0")}:00`
}

export function SitePlanningCalendar({
  siteId,
  assignments,
  canEdit,
}: SitePlanningCalendarProps) {
  const [currentWeek, setCurrentWeek] = useState(() => {
    const now = new Date()
    now.setHours(0, 0, 0, 0)
    return now
  })
  const [draft, setDraft] = useState<PlannerDraft>(defaultDraft())
  const [dialogOpen, setDialogOpen] = useState(false)

  const { start, end } = buildWeekBounds(currentWeek)
  const { data: appointments, isLoading, error } = useSiteAppointments(siteId, {
    start_date: start.toISOString(),
    end_date: end.toISOString(),
  })
  const { data: users } = useUsers()
  const createAppointment = useCreateSiteAppointment()
  const updateAppointment = useUpdateSiteAppointment()
  const deleteAppointment = useDeleteSiteAppointment()

  const userLabels = useMemo(() => {
    const map = new Map<string, string>()
    for (const user of users ?? []) {
      map.set(user.id, user.name || user.email)
    }
    return map
  }, [users])

  const assignmentLabels = useMemo(
    () =>
      assignments.map((assignment) => ({
        id: assignment.user_id,
        label: userLabels.get(assignment.user_id) ?? assignment.user_id,
      })),
    [assignments, userLabels]
  )

  const days = Array.from({ length: 7 }, (_, index) => {
    const date = new Date(start)
    date.setDate(date.getDate() + index)
    return date
  })

  const isSaving = createAppointment.isPending || updateAppointment.isPending

  function openCreateDialog(day?: Date, hour?: number) {
    setDraft(defaultDraft(day, hour))
    setDialogOpen(true)
  }

  function openEditDialog(appointment: SiteAppointment) {
    setDraft(appointmentToDraft(appointment))
    setDialogOpen(true)
  }

  function toggleAssignedUser(userId: string) {
    setDraft((current) => ({
      ...current,
      assigned_user_ids: current.assigned_user_ids.includes(userId)
        ? current.assigned_user_ids.filter((entry) => entry !== userId)
        : [...current.assigned_user_ids, userId],
    }))
  }

  async function handleSubmit() {
    if (!draft.title.trim()) {
      toast.error("Titel fehlt")
      return
    }
    if (!draft.starts_at || !draft.ends_at) {
      toast.error("Start und Ende werden benötigt")
      return
    }

    const trimmedNotes = draft.notes.trim()
    const payload = {
      title: draft.title.trim(),
      appointment_kind: draft.appointment_kind,
      starts_at: toIso(draft.starts_at),
      ends_at: toIso(draft.ends_at),
      assigned_user_ids: draft.assigned_user_ids,
      notes: trimmedNotes || null,
    }

    if (draft.appointmentId) {
      updateAppointment.mutate(
        {
          siteId,
          appointmentId: draft.appointmentId,
          ...payload,
          ...(trimmedNotes ? {} : { clear_notes: true }),
        },
        {
          onSuccess: () => {
            toast.success("Planung aktualisiert")
            setDialogOpen(false)
          },
          onError: () => toast.error("Planung konnte nicht aktualisiert werden"),
        }
      )
      return
    }

    createAppointment.mutate(
      {
        siteId,
        ...payload,
      },
      {
        onSuccess: () => {
          toast.success("Termin eingeplant")
          setDialogOpen(false)
        },
        onError: () => toast.error("Termin konnte nicht angelegt werden"),
      }
    )
  }

  function handleDelete() {
    if (!draft.appointmentId) return

    deleteAppointment.mutate(
      {
        siteId,
        appointmentId: draft.appointmentId,
      },
      {
        onSuccess: () => {
          toast.success("Termin entfernt")
          setDialogOpen(false)
        },
        onError: () => toast.error("Termin konnte nicht gelöscht werden"),
      }
    )
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="space-y-1">
          <p className="text-sm font-medium">Termine und abgestimmte Einsätze</p>
          <p className="text-xs text-muted-foreground">
            Geplante Kundentermine und Vor-Ort-Einsätze. Zeiterfassung bleibt separat.
          </p>
        </div>
        <div className="flex min-w-0 flex-wrap items-center gap-2">
          <Button
            variant="outline"
            size="icon"
            className="h-9 w-9"
            onClick={() => {
              const previous = new Date(currentWeek)
              previous.setDate(previous.getDate() - 7)
              setCurrentWeek(previous)
            }}
          >
            <ChevronLeft className="h-4 w-4" />
          </Button>
          <div className="min-w-0 flex-1 basis-[160px] text-center text-sm font-medium">
            {formatWeekHeader(start)}
          </div>
          <Button
            variant="outline"
            size="icon"
            className="h-9 w-9"
            onClick={() => {
              const next = new Date(currentWeek)
              next.setDate(next.getDate() + 7)
              setCurrentWeek(next)
            }}
          >
            <ChevronRight className="h-4 w-4" />
          </Button>
          {canEdit && (
            <Button className="h-9 gap-2" onClick={() => openCreateDialog()}>
              <Plus className="h-4 w-4" />
              Termin
            </Button>
          )}
        </div>
      </div>

      <div className="flex flex-wrap gap-2">
        {(Object.entries(appointmentKindMeta) as Array<
          [SiteAppointmentKind, (typeof appointmentKindMeta)[SiteAppointmentKind]]
        >).map(([kind, meta]) => (
          <Badge key={kind} variant="outline" className={meta.chipClassName}>
            {meta.label}
          </Badge>
        ))}
      </div>

      {isLoading ? (
        <LoadingSpinner className="py-10" />
      ) : error ? (
        <div className="rounded-2xl border border-destructive/30 bg-destructive/5 p-4 text-sm text-destructive">
          Projektplanung konnte nicht geladen werden.
        </div>
      ) : (
        <div className="overflow-x-auto rounded-2xl border bg-card/70 shadow-sm">
          <div className="min-w-[880px]">
            <div className="grid grid-cols-[72px_repeat(7,minmax(112px,1fr))] border-b bg-muted/30">
              <div className="px-3 py-3 text-xs font-semibold uppercase tracking-wide text-muted-foreground">
                Zeit
              </div>
              {days.map((day, index) => {
                const isToday = formatLocalDateKey(day) === formatLocalDateKey(new Date())
                return (
                  <button
                    key={day.toISOString()}
                    type="button"
                    className={cn(
                      "border-l px-3 py-3 text-left transition-colors",
                      canEdit && "hover:bg-accent/40",
                      isToday && "bg-primary/10"
                    )}
                    onClick={() => canEdit && openCreateDialog(day)}
                  >
                    <div className="text-xs text-muted-foreground">{dayNames[index]}</div>
                    <div className="text-sm font-semibold">
                      {day.toLocaleDateString("de-DE", { day: "2-digit", month: "2-digit" })}
                    </div>
                  </button>
                )
              })}
            </div>

            <div className="grid grid-cols-[72px_repeat(7,minmax(112px,1fr))]">
              <div className="border-r">
                {Array.from({ length: totalVisibleHours }).map((_, index) => (
                  <div
                    key={index}
                    className="relative border-b px-3 text-xs text-muted-foreground"
                    style={{ height: hourSlotHeight }}
                  >
                    <span className="-translate-y-2 inline-block bg-background pr-1">
                      {String(startHour + index).padStart(2, "0")}:00
                    </span>
                  </div>
                ))}
              </div>

              {days.map((day) => {
                const dateKey = formatLocalDateKey(day)
                const dayAppointments =
                  appointments?.filter((appointment) => {
                    const style = getSegmentStyle(appointment, day)
                    return style !== null
                  }) ?? []

                return (
                  <div
                    key={dateKey}
                    className="relative border-l"
                    style={{ height: totalVisibleHours * hourSlotHeight }}
                  >
                    {Array.from({ length: totalVisibleHours }).map((_, index) => (
                      canEdit ? (
                        <button
                          key={index}
                          type="button"
                          className="block w-full border-b border-dashed border-border/70 text-left transition-colors hover:bg-accent/30 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-ring"
                          style={{ height: hourSlotHeight }}
                          aria-label={`Termin am ${formatSlotLabel(day, startHour + index)} erstellen`}
                          onClick={() => openCreateDialog(day, startHour + index)}
                        />
                      ) : (
                        <div
                          key={index}
                          className="border-b border-dashed border-border/70"
                          style={{ height: hourSlotHeight }}
                        />
                      )
                    ))}

                    {dayAppointments.map((appointment) => {
                      const style = getSegmentStyle(appointment, day)
                      if (!style) return null

                      const meta = getAppointmentKindMeta(appointment.appointment_kind)
                      const assignedLabels = appointment.assigned_user_ids
                        .map((userId) => userLabels.get(userId) ?? userId)
                        .slice(0, 3)

                      return (
                        <button
                          key={`${appointment.id}-${dateKey}`}
                          type="button"
                          className={cn(
                            "absolute left-2 right-2 rounded-xl border px-3 py-2 text-left shadow-sm",
                            meta.cardClassName
                          )}
                          style={{
                            top: style.top + 4,
                            height: style.height - 8,
                          }}
                          onClick={() => canEdit && openEditDialog(appointment)}
                        >
                          <div className="line-clamp-1 text-xs font-semibold uppercase tracking-wide opacity-80">
                            {meta.label}
                          </div>
                          <div className="line-clamp-2 text-sm font-semibold">
                            {appointment.title}
                          </div>
                          <div className="mt-1 text-xs opacity-80">
                            {formatTimeRange(appointment.starts_at, appointment.ends_at)}
                          </div>
                          {assignedLabels.length > 0 && (
                            <div className="mt-2 line-clamp-2 text-xs opacity-80">
                              {assignedLabels.join(", ")}
                            </div>
                          )}
                        </button>
                      )
                    })}
                  </div>
                )
              })}
            </div>
          </div>
        </div>
      )}

      {!isLoading && !error && (appointments?.length ?? 0) === 0 && (
        <div className="rounded-2xl border border-dashed bg-muted/20 px-4 py-8 text-center">
          <div className="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-2xl bg-accent/70">
            <CalendarClock className="h-6 w-6 text-muted-foreground" />
          </div>
          <p className="text-sm font-medium">Noch keine geplanten Termine</p>
          <p className="mt-1 text-sm text-muted-foreground">
            Kundentermine und abgestimmte Einsätze erscheinen hier auf Stundenbasis.
          </p>
        </div>
      )}

      <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
        <DialogContent className="max-w-xl">
          <DialogHeader>
            <DialogTitle>
              {draft.appointmentId ? "Planung bearbeiten" : "Termin planen"}
            </DialogTitle>
            <DialogDescription>
              Geplante Kundentermine und Vor-Ort-Einsätze mit Uhrzeit pflegen.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="appointment-title">Titel</Label>
              <Input
                id="appointment-title"
                value={draft.title}
                onChange={(event) =>
                  setDraft((current) => ({ ...current, title: event.target.value }))
                }
                placeholder="z. B. Abnahme mit Kunde"
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="appointment-kind">Typ</Label>
              <Select
                value={draft.appointment_kind}
                onValueChange={(value) =>
                  setDraft((current) => ({
                    ...current,
                    appointment_kind: value as SiteAppointmentKind,
                  }))
                }
              >
                <SelectTrigger id="appointment-kind">
                  <SelectValue placeholder="Typ wählen" />
                </SelectTrigger>
                <SelectContent>
                  {(Object.entries(appointmentKindMeta) as Array<
                    [SiteAppointmentKind, (typeof appointmentKindMeta)[SiteAppointmentKind]]
                  >).map(([kind, meta]) => (
                    <SelectItem key={kind} value={kind}>
                      {meta.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="grid gap-4 sm:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="appointment-start">Start</Label>
                <Input
                  id="appointment-start"
                  type="datetime-local"
                  value={draft.starts_at}
                  onChange={(event) =>
                    setDraft((current) => ({ ...current, starts_at: event.target.value }))
                  }
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="appointment-end">Ende</Label>
                <Input
                  id="appointment-end"
                  type="datetime-local"
                  value={draft.ends_at}
                  onChange={(event) =>
                    setDraft((current) => ({ ...current, ends_at: event.target.value }))
                  }
                />
              </div>
            </div>

            <div className="space-y-2">
              <Label>Eingeplante Mitarbeiter</Label>
              {assignmentLabels.length === 0 ? (
                <p className="text-sm text-muted-foreground">
                  Noch kein Projektteam zugewiesen.
                </p>
              ) : (
                <div className="flex flex-wrap gap-2">
                  {assignmentLabels.map((assignment) => {
                    const selected = draft.assigned_user_ids.includes(assignment.id)

                    return (
                      <Button
                        key={assignment.id}
                        type="button"
                        variant={selected ? "default" : "outline"}
                        className="h-9"
                        onClick={() => toggleAssignedUser(assignment.id)}
                      >
                        {assignment.label}
                      </Button>
                    )
                  })}
                </div>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="appointment-notes">Notiz</Label>
              <Textarea
                id="appointment-notes"
                rows={4}
                value={draft.notes}
                onChange={(event) =>
                  setDraft((current) => ({ ...current, notes: event.target.value }))
                }
                placeholder="Absprachen, Anfahrt, Vorbereitung, Kontaktperson"
              />
            </div>

            <div className="flex flex-col-reverse gap-2 sm:flex-row sm:justify-between">
              <div>
                {draft.appointmentId && (
                  <Button
                    type="button"
                    variant="outline"
                    className="gap-2 text-destructive hover:text-destructive"
                    disabled={deleteAppointment.isPending}
                    onClick={handleDelete}
                  >
                    <Trash2 className="h-4 w-4" />
                    Löschen
                  </Button>
                )}
              </div>
              <div className="flex gap-2">
                <Button type="button" variant="outline" onClick={() => setDialogOpen(false)}>
                  Abbrechen
                </Button>
                <Button type="button" disabled={isSaving} onClick={handleSubmit}>
                  {draft.appointmentId ? "Speichern" : "Anlegen"}
                </Button>
              </div>
            </div>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  )
}
