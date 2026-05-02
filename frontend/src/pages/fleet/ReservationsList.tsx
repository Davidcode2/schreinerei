import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Calendar, MapPin, X } from "lucide-react"
import { EmptyState, ErrorState, StatusBadge } from "@/components/shared"
import { useReservations, useCancelReservation } from "@/lib/api/hooks"
import { toast } from "sonner"
import type { Reservation } from "@/types/fleet"

interface ReservationsListProps {
  showOnlyMine?: boolean
}

function formatDateTime(dateString: string): string {
  return new Date(dateString).toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  })
}

export function ReservationsList({ showOnlyMine = false }: ReservationsListProps) {
  const {
    data: reservations,
    isLoading,
    error,
    refetch,
  } = useReservations(showOnlyMine ? { user_id: "me" } : undefined)

  const cancelMutation = useCancelReservation()

  const handleCancel = async (id: string) => {
    try {
      await cancelMutation.mutateAsync(id)
      toast.success("Reservierung storniert")
    } catch {
      toast.error("Stornierung fehlgeschlagen")
    }
  }

  if (isLoading) {
    return (
      <div className="space-y-4">
        {[1, 2, 3].map((i) => (
          <Card key={i}>
            <CardContent className="p-4">
              <div className="h-5 bg-muted rounded w-1/3 animate-pulse mb-2" />
              <div className="h-4 bg-muted rounded w-1/2 animate-pulse" />
            </CardContent>
          </Card>
        ))}
      </div>
    )
  }

  if (error) {
    return (
      <ErrorState
        message="Reservierungen konnten nicht geladen werden"
        onRetry={() => refetch()}
      />
    )
  }

  if (!reservations || reservations.length === 0) {
    return (
      <EmptyState
        icon={Calendar}
        title="Keine Reservierungen"
        description={
          showOnlyMine
            ? "Sie haben noch keine Reservierungen."
            : "Es gibt noch keine Reservierungen."
        }
      />
    )
  }

  return (
    <div className="space-y-4">
      {reservations.map((reservation: Reservation) => (
        <Card key={reservation.id}>
          <CardContent className="p-4">
            <div className="flex items-start justify-between mb-3">
              <div>
                <h3 className="font-semibold">{reservation.resource_name}</h3>
                <p className="text-sm text-muted-foreground">
                  {reservation.user_name}
                </p>
              </div>
              <StatusBadge status={reservation.status} />
            </div>

            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
              <Calendar className="h-3 w-3" />
              <span>
                {formatDateTime(reservation.start_time)} –{" "}
                {formatDateTime(reservation.end_time)}
              </span>
            </div>

            {reservation.site_name && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <MapPin className="h-3 w-3" />
                <span>{reservation.site_name}</span>
              </div>
            )}

            {(reservation.status === "confirmed" || reservation.status === "in_use") && (
              <div className="flex justify-end mt-3 pt-3 border-t">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handleCancel(reservation.id)}
                  disabled={cancelMutation.isPending}
                  className="gap-2 text-destructive hover:text-destructive"
                >
                  <X className="h-3 w-3" />
                  Stornieren
                </Button>
              </div>
            )}
          </CardContent>
        </Card>
      ))}
    </div>
  )
}
