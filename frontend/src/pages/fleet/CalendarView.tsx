import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Calendar, ChevronLeft, ChevronRight } from "lucide-react"
import { useState } from "react"
import { Link } from "react-router-dom"
import { LoadingSpinner, EmptyState } from "@/components/shared"
import { useCalendar } from "@/lib/api/hooks"
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
  const start = new Date(date)
  start.setDate(start.getDate() - start.getDay() + 1) // Monday
  const end = new Date(start)
  end.setDate(end.getDate() + 6) // Sunday

  return {
    start: start.toISOString().split("T")[0] ?? "",
    end: end.toISOString().split("T")[0] ?? "",
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

  const weekStart = new Date(currentWeek)
  weekStart.setDate(weekStart.getDate() - weekStart.getDay() + 1)

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

  const today = new Date().toISOString().split("T")[0] ?? ""

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

      {/* Week Grid Header */}
      <div className="grid grid-cols-8 gap-2">
        <div className="p-2 text-sm font-medium text-muted-foreground">
          Ressource
        </div>
        {Array.from({ length: 7 }).map((_, i) => {
          const date = new Date(weekStart)
          date.setDate(date.getDate() + i)
          const isToday = (date.toISOString().split("T")[0] ?? "") === today

          return (
            <div
              key={i}
              className={`p-2 text-center text-sm ${
                isToday
                  ? "bg-primary text-primary-foreground rounded-md"
                  : "text-muted-foreground"
              }`}
            >
              <div className="font-medium">{dayNames[i]}</div>
              <div className={isToday ? "" : "text-foreground"}>
                {date.getDate()}
              </div>
            </div>
          )
        })}
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
        <div className="space-y-2">
          {calendarData.resources.map((entry: CalendarEntry) => {
            const resourceColor = getResourceCalendarColor(
              entry.resource_type,
              entry.resource_id
            )

            return (
              <Card key={`${entry.resource_type}-${entry.resource_id}`}>
                <CardContent className="p-0">
                  <div className="grid grid-cols-8 gap-2">
                    <div
                      className={`border-r p-3 ${resourceColor.tintClassName}`}
                      data-resource-color={resourceColor.token}
                    >
                      <div className="flex items-center gap-2">
                        <span
                          aria-hidden="true"
                          className={`h-2.5 w-2.5 rounded-full ${resourceColor.markerClassName}`}
                        />
                        <p className={`line-clamp-1 text-sm font-medium ${resourceColor.labelClassName}`}>
                          {entry.resource_name}
                        </p>
                      </div>
                    </div>
                    {Array.from({ length: 7 }).map((_, i) => {
                      const date = new Date(weekStart)
                      date.setDate(date.getDate() + i)
                      const dateStr = date.toISOString().split("T")[0] ?? ""

                      const dayReservations = entry.reservations.filter(
                        (r: ReservationSummary) => {
                          const startDate = r.start_time.split("T")[0] ?? ""
                          const endDate = r.end_time.split("T")[0] ?? ""
                          return dateStr >= startDate && dateStr <= endDate
                        }
                      )

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
                        ? "bg-primary/10 ring-2 ring-primary/40"
                        : isCompletedSelection
                          ? "bg-primary/5 ring-1 ring-primary/30"
                          : ""

                      return (
                        <div key={i} className="min-h-[60px] border-l last:border-r">
                          {isEmpty ? (
                            <button
                              type="button"
                              className={`h-full min-h-[60px] w-full p-2 text-left transition hover:bg-muted/50 ${selectionClassName}`}
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
                            <div className={`p-2 ${selectionClassName}`}>
                              {dayReservations.map((r: ReservationSummary) => (
                                <div
                                  key={r.id}
                                  className={`mb-1 border-l-4 p-1 text-xs ${resourceColor.borderClassName} ${
                                    r.status === "confirmed"
                                      ? "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200"
                                      : r.status === "in_use"
                                        ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
                                        : r.status === "pending"
                                          ? "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200"
                                          : "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200"
                                  }`}
                                  data-resource-color={resourceColor.token}
                                >
                                  <p className="truncate">{r.user_name}</p>
                                </div>
                              ))}
                            </div>
                          )}
                        </div>
                      )
                    })}
                  </div>
                </CardContent>
              </Card>
            )
          })}
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
