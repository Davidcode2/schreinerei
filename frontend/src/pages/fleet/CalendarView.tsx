import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Calendar, ChevronLeft, ChevronRight } from "lucide-react"
import { useState } from "react"
import { Link } from "react-router-dom"
import { LoadingSpinner, EmptyState, PageHeader } from "@/components/shared"
import { useCalendar, useReservation } from "@/lib/api/hooks"
import { cn, formatLocalDateKey, startOfLocalWeek } from "@/lib/utils"
import type { CalendarEntry, ReservationSummary, ResourceType } from "@/types/fleet"
import { ReservationConfirmationSheet } from "./ReservationConfirmationSheet"
import { ReservationDialog } from "./ReservationDialog"
import {
  advanceRangeSelection,
  type CompletedRangeSelection,
  type PendingRangeSelection,
} from "./calendarRangeSelection"
import { getResourceCalendarColor } from "./resourceCalendarColor"

interface CalendarViewProps {
  embedded?: boolean
  resourceType?: ResourceType
  siteId?: string
}

function getWeekDates(date: Date): { start: string; end: string } {
  const start = startOfLocalWeek(date)
  const end = new Date(start)
  end.setDate(end.getDate() + 6)
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
const mobileCalendarGridColumns = "grid-cols-[88px_repeat(7,minmax(0,1fr))]"
const desktopCalendarGridColumns = "sm:grid-cols-[minmax(220px,1.25fr)_repeat(7,minmax(96px,1fr))]"

const reservationStatusStyles = {
  confirmed: {
    badgeClassName: "bg-success/15 text-success border-success/20",
    label: "Bestätigt",
  },
  in_use: {
    badgeClassName: "bg-primary/10 text-primary border-primary/20",
    label: "Im Einsatz",
  },
  pending: {
    badgeClassName: "bg-warning/15 text-warning-foreground border-warning/25",
    label: "Anfrage",
  },
  completed: {
    badgeClassName: "bg-muted text-muted-foreground border-border",
    label: "Erledigt",
  },
  cancelled: {
    badgeClassName: "bg-destructive/10 text-destructive border-destructive/20",
    label: "Storniert",
  },
} as const

const resourceTypeLabels: Record<ResourceType, string> = {
  vehicle: "Fahrzeug",
  tool: "Werkzeug",
  machine: "Maschine",
}

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

export default function CalendarView({ embedded = false, resourceType, siteId }: CalendarViewProps) {
  const [currentWeek, setCurrentWeek] = useState(() => {
    const now = new Date()
    now.setHours(0, 0, 0, 0)
    return now
  })

  const [pendingSelection, setPendingSelection] = useState<PendingRangeSelection | null>(null)
  const [completedSelection, setCompletedSelection] = useState<
    (CompletedRangeSelection & { resourceName: string }) | null
  >(null)
  const [selectedReservationId, setSelectedReservationId] = useState<string | null>(null)

  const { start, end } = getWeekDates(currentWeek)
  const { data: calendarData, isLoading, error } = useCalendar({
    start_date: start,
    end_date: end,
    ...(resourceType ? { resource_type: resourceType } : {}),
    ...(siteId ? { site_id: siteId } : {}),
  })
  const {
    data: selectedReservation,
    isLoading: isReservationLoading,
    error: reservationError,
  } = useReservation(selectedReservationId)

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

  const closeReservationDetails = () => {
    setSelectedReservationId(null)
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
        <PageHeader
          title="Kalender"
          description="Ressourcenbelegung"
        />
      )}

      <div className="flex items-center justify-between gap-4">
        <div className="flex items-center gap-1.5">
          <Button variant="outline" size="icon" onClick={prevWeek} className="h-9 w-9 shadow-sm">
            <ChevronLeft className="h-4 w-4" />
          </Button>
          <Button variant="outline" size="icon" onClick={nextWeek} className="h-9 w-9 shadow-sm">
            <ChevronRight className="h-4 w-4" />
          </Button>
        </div>
        <span className="min-w-[180px] text-right font-display font-normal tracking-tight sm:text-center">
          {formatWeekHeader(weekStart)}
        </span>
      </div>

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
        <div
          data-testid="calendar"
          className={cn(
            "overflow-hidden bg-card/70 shadow-sm",
            embedded
              ? "-mx-4 border-y sm:mx-0 sm:rounded-2xl sm:border"
              : "rounded-2xl border"
          )}
        >
          <div className="sm:min-w-[980px]">
            <div
              className={cn(
                "grid gap-px border-b bg-border/60",
                mobileCalendarGridColumns,
                desktopCalendarGridColumns
              )}
            >
              <div className="bg-muted/40 px-2 py-2 text-[11px] font-semibold text-muted-foreground sm:px-4 sm:py-3 sm:text-sm">
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
                      "px-1 py-2 text-center text-[11px] sm:px-3 sm:py-3 sm:text-sm",
                      isToday
                        ? "bg-primary text-primary-foreground"
                        : "bg-muted/30 text-muted-foreground"
                    )}
                  >
                    <div className="font-medium">{dayNames[i]}</div>
                    <div className={cn("text-xs sm:text-sm", isToday ? "text-primary-foreground" : "text-foreground")}>
                      {date.getDate()}
                    </div>
                  </div>
                )
              })}
            </div>

            {calendarData.resources.map((entry: CalendarEntry) => {
              const resourceColor = getResourceCalendarColor(
                entry.resource_type,
                entry.resource_id,
                entry.resource_display_color
              )

              return (
                <div
                  key={`${entry.resource_type}-${entry.resource_id}`}
                  className={cn(
                    "grid gap-px border-b bg-border/60 last:border-b-0",
                    mobileCalendarGridColumns,
                    desktopCalendarGridColumns
                  )}
                >
                  <div
                    className={cn(
                      "flex min-h-[72px] items-center px-2 py-2 sm:min-h-[88px] sm:px-4 sm:py-3",
                      resourceColor.tintClassName
                    )}
                    style={resourceColor.tintStyle}
                    data-resource-color={resourceColor.token}
                  >
                    <div className="min-w-0 space-y-2">
                      <div className="flex items-center gap-2">
                        <span
                          aria-hidden="true"
                          className={cn(
                            "h-2.5 w-2.5 rounded-full ring-2 ring-background/80 sm:h-3 sm:w-3 sm:ring-4",
                            resourceColor.markerClassName
                          )}
                          style={resourceColor.markerStyle}
                        />
                          <p
                            className={cn("line-clamp-2 text-xs font-semibold leading-tight sm:line-clamp-1 sm:text-sm", resourceColor.labelClassName)}
                            style={resourceColor.labelStyle}
                          >
                            {entry.resource_name}
                          </p>
                        </div>
                        <div className="hidden items-center gap-2 text-[11px] uppercase tracking-[0.18em] text-muted-foreground sm:flex">
                          <span>{resourceTypeLabels[entry.resource_type]}</span>
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
                              "h-full min-h-[72px] w-full p-1 transition hover:bg-muted/40 sm:min-h-[88px] sm:p-2",
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
                          <div className={cn("min-h-[72px] p-1 sm:min-h-[88px] sm:p-2", selectionClassName)}>
                            <div className="space-y-2">
                              {dayReservations.map((r: ReservationSummary) => {
                                const reservationStatus = reservationStatusStyles[r.status]

                                return (
                                  <div
                                    key={r.id}
                                    role="button"
                                    tabIndex={0}
                                    className={cn(
                                      "rounded-lg border px-1.5 py-1.5 text-left shadow-sm transition hover:-translate-y-0.5 hover:shadow-md focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 sm:rounded-xl sm:px-2.5 sm:py-2",
                                      resourceColor.borderClassName,
                                      resourceColor.softTintClassName
                                    )}
                                    style={{
                                      ...resourceColor.borderStyle,
                                      ...resourceColor.softTintStyle,
                                    }}
                                    data-resource-color={resourceColor.token}
                                    onClick={() => setSelectedReservationId(r.id)}
                                    onKeyDown={(event) => {
                                      if (event.key === "Enter" || event.key === " ") {
                                        event.preventDefault()
                                        setSelectedReservationId(r.id)
                                      }
                                    }}
                                  >
                                     <div className="flex items-center gap-1.5 sm:gap-2">
                                       <span
                                         aria-hidden="true"
                                         className={cn("h-2 w-2 rounded-full", resourceColor.markerClassName)}
                                         style={resourceColor.markerStyle}
                                       />
                                       <p className="truncate text-[11px] font-semibold text-foreground sm:text-xs">
                                         {r.user_name}
                                       </p>
                                     </div>
                                     <div className="mt-1 flex items-center gap-1 text-[10px] sm:mt-2 sm:gap-2 sm:text-[11px]">
                                       <span
                                         className={cn(
                                           "inline-flex rounded-full px-2 py-0.5 font-medium border",
                                           reservationStatus.badgeClassName
                                         )}
                                       >
                                         {reservationStatus.label}
                                       </span>
                                       {r.site_name ? (
                                         <span className="hidden truncate text-muted-foreground sm:inline">
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
            <Button variant="outline" className="shadow-sm">Zurück zum Fuhrpark</Button>
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
          projectId={siteId}
        />
      )}

      {selectedReservationId && !selectedReservation && (
        <Dialog open onOpenChange={(open) => !open && closeReservationDetails()}>
          <DialogContent className="sm:max-w-md">
            <DialogHeader>
              <DialogTitle>Reservierung laden</DialogTitle>
              <DialogDescription>
                {reservationError
                  ? "Die Reservierungsdetails konnten nicht geladen werden."
                  : "Reservierungsdetails werden geladen."}
              </DialogDescription>
            </DialogHeader>
            <div className="flex min-h-24 items-center justify-center">
              {isReservationLoading ? <LoadingSpinner /> : null}
            </div>
          </DialogContent>
        </Dialog>
      )}

      {selectedReservation && (
        <ReservationDialog
          open
          onOpenChange={(open) => !open && closeReservationDetails()}
          mode="edit"
          initialData={selectedReservation}
        />
      )}
    </div>
  )
}
