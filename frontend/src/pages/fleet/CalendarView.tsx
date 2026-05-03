import { Button } from "@/components/ui/button"
import { Calendar, ChevronLeft, ChevronRight } from "lucide-react"
import { useState } from "react"
import { Link } from "react-router-dom"
import { LoadingSpinner, EmptyState } from "@/components/shared"
import { useCalendar } from "@/lib/api/hooks"
import { cn, formatLocalDateKey, startOfLocalWeek } from "@/lib/utils"
import type { CalendarEntry, ReservationSummary, ResourceType } from "@/types/fleet"
import { ReservationConfirmationSheet } from "./ReservationConfirmationSheet"
import {
  advanceRangeSelection,
  type CompletedRangeSelection,
  type PendingRangeSelection,
} from "./calendarRangeSelection"
import { getResourceCalendarColor } from "./resourceCalendarColor"

interface CalendarViewProps {
  embedded?: boolean
}

function getWeekDates(date: Date): { start: string; end: string } {
  const start = startOfLocalWeek(date)
  const end = new Date(start)
  end.setDate(end.getDate() + 6) // Sunday
  end.setHours(23, 59, 59, 999)

  return {
    start: start.toISOString(),
    end: end.toISOString(),
  }
}

function formatWeekHeader(startDate: Date): string {
  const endDate = new Date(startDate)
  endDate.setDate(endDate.getDate() + 6)

  const startMonth = startDate.toLocaleDateString("de-DE", { month: "long" })
  const endMonth = endDate.toLocaleDateString("de-DE", { month: "long" })
  const year = startDate.getFullYear()

  if (startMonth === endMonth) {
    return `${startMonth} ${year}`
  }
  return `${startMonth} - ${endMonth} ${year}`
}

const dayNames = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"]
const calendarGridColumns = "minmax(220px, 1.25fr) repeat(7, minmax(96px, 1fr))"

const reservationStatusStyles = {
  confirmed: {
    badgeClassName: "bg-emerald-100 text-emerald-800 dark:bg-emerald-500/15 dark:text-emerald-200",
    label: "Bestätigt",
  },
  in_use: {
    badgeClassName: "bg-blue-100 text-blue-800 dark:bg-blue-500/15 dark:text-blue-200",
    label: "Im Einsatz",
  },
  pending: {
    badgeClassName: "bg-amber-100 text-amber-800 dark:bg-amber-500/15 dark:text-amber-200",
    label: "Anfrage",
  },
  completed: {
    badgeClassName: "bg-slate-200 text-slate-700 dark:bg-slate-800 dark:text-slate-200",
    label: "Erledigt",
  },
  cancelled: {
    badgeClassName: "bg-rose-100 text-rose-800 dark:bg-rose-500/15 dark:text-rose-200",
    label: "Storniert",
  },
} as const

function isDateInRange(date: string, startDate: string, endDate: string): boolean {
  return date >= startDate && date <= endDate
}

function formatDateLabel(date: string): string {
  return new Date(`${date}T12:00:00`).toLocaleDateString("de-DE", {
    weekday: "long",
    day: "2-digit",
    month: "2-digit",
  })
}

export default function CalendarView({ embedded = false }: CalendarViewProps) {
  const [currentWeek, setCurrentWeek] = useState(() => {
    const now = new Date()
    now.setHours(0, 0, 0, 0)
    return now
  })

  const [pendingSelection, setPendingSelection] = useState<PendingRangeSelection | null>(null)
  const [completedSelection, setCompletedSelection] = useState<
    (CompletedRangeSelection & { resourceName: string }) | null
  >(null)

  const { start, end } = getWeekDates(currentWeek)
  const { data: calendarData, isLoading, error } = useCalendar({ start_date: start, end_date: end })

  const weekStart = startOfLocalWeek(currentWeek)

  const prevWeek = () => {
    const newWeek = new Date(currentWeek)
    newWeek.setDate(newWeek.getDate() - 7)
    setCurrentWeek(newWeek)
  }

  const nextWeek = () => {
    const newWeek = new Date(currentWeek)
    newWeek.setDate(newWeek.getDate() + 7)
    setCurrentWeek(newWeek)
  }

  const today = formatLocalDateKey(new Date())

  const clearSelection = () => {
    setPendingSelection(null)
    setCompletedSelection(null)
  }

  const handleSlotClick = (entry: CalendarEntry, dateStr: string) => {
    const selectionResult = advanceRangeSelection(pendingSelection, {
      resourceId: entry.resource_id,
      resourceType: entry.resource_type as ResourceType,
      date: dateStr,
    })

    if ("pendingSelection" in selectionResult) {
      setPendingSelection(selectionResult.pendingSelection)
      setCompletedSelection(null)
      return
    }

    setPendingSelection(null)
    setCompletedSelection({
      ...selectionResult.completedSelection,
      resourceName: entry.resource_name,
    })
  }

  return (
    <div className="space-y-6">
      {!embedded && (
        <div>
          <h1 className="text-2xl font-bold">Kalender</h1>
          <p className="text-muted-foreground">Ressourcenbelegung</p>
        </div>
      )}

      <div className="flex items-center justify-between gap-4">
        <div className="flex items-center gap-2">
          <Button variant="outline" size="icon" onClick={prevWeek}>
            <ChevronLeft className="h-4 w-4" />
          </Button>
          <Button variant="outline" size="icon" onClick={nextWeek}>
            <ChevronRight className="h-4 w-4" />
          </Button>
        </div>
        <span className="min-w-[180px] text-right font-medium sm:text-center">
          {formatWeekHeader(weekStart)}
        </span>
      </div>

      {/* Resources Grid */}
      {isLoading ? (
        <LoadingSpinner className="py-8" />
      ) : error ? (
        <EmptyState
          icon={Calendar}
          title="Fehler beim Laden"
          description="Der Kalender konnte nicht geladen werden."
        />
      ) : !calendarData?.resources || calendarData.resources.length === 0 ? (
        <EmptyState
          icon={Calendar}
          title="Keine Ressourcen"
          description="Es sind keine Ressourcen für die Kalenderansicht verfügbar."
        />
      ) : (
        <div className="overflow-x-auto rounded-2xl border bg-card/70 shadow-sm">
          <div className="min-w-[980px]">
            <div
              className="grid gap-px border-b bg-border/60"
              style={{ gridTemplateColumns: calendarGridColumns }}
            >
              <div className="bg-muted/40 px-4 py-3 text-sm font-semibold text-muted-foreground">
                Ressource
              </div>
              {Array.from({ length: 7 }).map((_, i) => {
                const date = new Date(weekStart)
                date.setDate(date.getDate() + i)
                const isToday = formatLocalDateKey(date) === today

                return (
                  <div
                    key={i}
                    className={cn(
                      "px-3 py-3 text-center text-sm",
                      isToday
                        ? "bg-primary text-primary-foreground"
                        : "bg-muted/30 text-muted-foreground"
                    )}
                  >
                    <div className="font-medium">{dayNames[i]}</div>
                    <div className={isToday ? "text-primary-foreground" : "text-foreground"}>
                      {date.getDate()}
                    </div>
                  </div>
                )
              })}
            </div>

            {calendarData.resources.map((entry: CalendarEntry) => {
              const resourceColor = getResourceCalendarColor(
                entry.resource_type,
                entry.resource_id
              )

              return (
                <div
                  key={`${entry.resource_type}-${entry.resource_id}`}
                  className="grid gap-px border-b bg-border/60 last:border-b-0"
                  style={{ gridTemplateColumns: calendarGridColumns }}
                >
                  <div
                    className={cn(
                      "flex min-h-[88px] items-center px-4 py-3",
                      resourceColor.tintClassName
                    )}
                    data-resource-color={resourceColor.token}
                  >
                    <div className="min-w-0 space-y-2">
                      <div className="flex items-center gap-2">
                        <span
                          aria-hidden="true"
                          className={cn(
                            "h-3 w-3 rounded-full ring-4 ring-background/80",
                            resourceColor.markerClassName
                          )}
                        />
                        <p className={cn("line-clamp-1 text-sm font-semibold", resourceColor.labelClassName)}>
                          {entry.resource_name}
                        </p>
                      </div>
                      <div className="flex items-center gap-2 text-[11px] uppercase tracking-[0.18em] text-muted-foreground">
                        <span>{entry.resource_type === "vehicle" ? "Fahrzeug" : "Werkzeug"}</span>
                      </div>
                    </div>
                  </div>

                  {Array.from({ length: 7 }).map((_, i) => {
                    const date = new Date(weekStart)
                    date.setDate(date.getDate() + i)
                    const dateStr = formatLocalDateKey(date)

                    const dayReservations = entry.reservations.filter((r: ReservationSummary) => {
                      const startDate = formatLocalDateKey(new Date(r.start_time))
                      const endDate = formatLocalDateKey(new Date(r.end_time))
                      return dateStr >= startDate && dateStr <= endDate
                    })

                    const isEmpty = dayReservations.length === 0
                    const isPendingStart =
                      pendingSelection?.resourceId === entry.resource_id &&
                      pendingSelection.resourceType === entry.resource_type &&
                      pendingSelection.firstDate === dateStr
                    const isCompletedSelection =
                      completedSelection?.resourceId === entry.resource_id &&
                      completedSelection.resourceType === entry.resource_type &&
                      isDateInRange(
                        dateStr,
                        completedSelection.startDate,
                        completedSelection.endDate
                      )
                    const selectionClassName = isPendingStart
                      ? "bg-primary/10 ring-2 ring-inset ring-primary/40"
                      : isCompletedSelection
                        ? "bg-primary/5 ring-1 ring-inset ring-primary/30"
                        : ""

                    return (
                      <div key={i} className="bg-background">
                        {isEmpty ? (
                          <button
                            type="button"
                            className={cn(
                              "h-full min-h-[88px] w-full p-2 text-left transition hover:bg-muted/40",
                              selectionClassName
                            )}
                            aria-label={`${entry.resource_name} am ${formatDateLabel(dateStr)} auswaehlen`}
                            aria-pressed={isPendingStart || isCompletedSelection}
                            data-selection-state={
                              isPendingStart
                                ? "pending"
                                : isCompletedSelection
                                  ? "completed"
                                  : "idle"
                            }
                            onClick={() => handleSlotClick(entry, dateStr)}
                          />
                        ) : (
                          <div className={cn("min-h-[88px] p-2", selectionClassName)}>
                            <div className="space-y-2">
                              {dayReservations.map((r: ReservationSummary) => {
                                const reservationStatus = reservationStatusStyles[r.status]

                                return (
                                  <div
                                    key={r.id}
                                    className={cn(
                                      "rounded-xl border px-2.5 py-2 shadow-sm",
                                      resourceColor.borderClassName,
                                      resourceColor.softTintClassName
                                    )}
                                    data-resource-color={resourceColor.token}
                                  >
                                    <div className="flex items-center gap-2">
                                      <span
                                        aria-hidden="true"
                                        className={cn("h-2 w-2 rounded-full", resourceColor.markerClassName)}
                                      />
                                      <p className="truncate text-xs font-semibold text-foreground">
                                        {r.user_name}
                                      </p>
                                    </div>
                                    <div className="mt-2 flex items-center gap-2 text-[11px]">
                                      <span
                                        className={cn(
                                          "inline-flex rounded-full px-2 py-0.5 font-medium",
                                          reservationStatus.badgeClassName
                                        )}
                                      >
                                        {reservationStatus.label}
                                      </span>
                                      {r.site_name ? (
                                        <span className="truncate text-muted-foreground">
                                          {r.site_name}
                                        </span>
                                      ) : null}
                                    </div>
                                  </div>
                                )
                              })}
                            </div>
                          </div>
                        )}
                      </div>
                    )
                  })}
                </div>
              )
            })}
          </div>
        </div>
      )}

      {!embedded && (
        <div className="flex justify-center">
          <Link to="/fleet">
            <Button variant="outline">Zurück zum Fuhrpark</Button>
          </Link>
        </div>
      )}

      {completedSelection && (
        <ReservationConfirmationSheet
          open
          onOpenChange={(open) => {
            if (!open) {
              clearSelection()
            }
          }}
          resourceId={completedSelection.resourceId}
          resourceType={completedSelection.resourceType}
          resourceName={completedSelection.resourceName}
          startDate={completedSelection.startDate}
          endDate={completedSelection.endDate}
        />
      )}
    </div>
  )
}
