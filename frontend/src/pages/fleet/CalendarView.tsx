import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Calendar, ChevronLeft, ChevronRight } from "lucide-react"
import { useState } from "react"
import { Link } from "react-router-dom"
import { LoadingSpinner, EmptyState } from "@/components/shared"
import { useCalendar } from "@/lib/api/hooks"
import { ReservationDialog } from "./ReservationDialog"
import type { CalendarEntry, ReservationSummary, ResourceType } from "@/types/fleet"

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

export default function CalendarView() {
  const [currentWeek, setCurrentWeek] = useState(() => {
    const now = new Date()
    now.setHours(0, 0, 0, 0)
    return now
  })

  const [dialogOpen, setDialogOpen] = useState(false)
  const [selectedSlot, setSelectedSlot] = useState<{
    resourceId: string
    resourceType: ResourceType
    startTime: string
    endTime: string
  } | null>(null)

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

  const handleSlotClick = (
    entry: CalendarEntry,
    dateStr: string
  ) => {
    // Create datetime-local values for the clicked day
    const startTime = `${dateStr}T08:00`  // Default 8am start
    const endTime = `${dateStr}T17:00`    // Default 5pm end

    setSelectedSlot({
      resourceId: entry.resource_id,
      resourceType: entry.resource_type as ResourceType,
      startTime,
      endTime,
    })
    setDialogOpen(true)
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Kalender</h1>
          <p className="text-muted-foreground">Ressourcenbelegung</p>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" size="icon" onClick={prevWeek}>
            <ChevronLeft className="h-4 w-4" />
          </Button>
          <span className="min-w-[180px] text-center font-medium">
            {formatWeekHeader(weekStart)}
          </span>
          <Button variant="outline" size="icon" onClick={nextWeek}>
            <ChevronRight className="h-4 w-4" />
          </Button>
        </div>
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
          {calendarData.resources.map((entry: CalendarEntry) => (
            <Card key={`${entry.resource_type}-${entry.resource_id}`}>
              <CardContent className="p-0">
                <div className="grid grid-cols-8 gap-2">
                  <div className="p-3 border-r">
                    <p className="font-medium text-sm line-clamp-1">
                      {entry.resource_name}
                    </p>
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

                    return (
                      <div
                        key={i}
                        className={`p-2 min-h-[60px] border-l last:border-r ${
                          isEmpty ? "cursor-pointer hover:bg-muted/50" : ""
                        }`}
                        onClick={() => {
                          if (isEmpty) {
                            handleSlotClick(entry, dateStr)
                          }
                        }}
                      >
                        {dayReservations.map((r: ReservationSummary) => (
                          <div
                            key={r.id}
                            className={`text-xs p-1 rounded mb-1 ${
                              r.status === "confirmed"
                                ? "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200"
                                : r.status === "in_use"
                                ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
                                : r.status === "pending"
                                ? "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200"
                                : "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200"
                            }`}
                          >
                            <p className="truncate">{r.user_name}</p>
                          </div>
                        ))}
                      </div>
                    )
                  })}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}

      <div className="flex justify-center">
        <Link to="/fleet">
          <Button variant="outline">Zurück zum Fuhrpark</Button>
        </Link>
      </div>

      {/* Reservation Dialog for click-to-create */}
      {selectedSlot && (
        <ReservationDialog
          open={dialogOpen}
          onOpenChange={(open) => {
            setDialogOpen(open)
            if (!open) setSelectedSlot(null)
          }}
          resourceId={selectedSlot.resourceId}
          resourceType={selectedSlot.resourceType}
          initialStartTime={selectedSlot.startTime}
          initialEndTime={selectedSlot.endTime}
        />
      )}
    </div>
  )
}
