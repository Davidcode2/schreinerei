import type { LucideIcon } from "lucide-react"
import type { ReactNode } from "react"
import { Calendar, Pencil } from "lucide-react"
import { EmptyState, PageHeader, StatusBadge } from "@/components/shared"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import type { Reservation, ResourceStatus } from "@/types/fleet"
import { formatReservationDateTime } from "./resourceLabels"

interface DetailItem {
  key: string
  icon: LucideIcon
  label: string
  value: ReactNode
}

interface ResourceDetailPageProps {
  title: string
  description?: string
  backTo: string
  status: ResourceStatus
  summaryIcon: LucideIcon
  summaryContent: ReactNode
  detailItems: DetailItem[]
  detailDescription?: string | null
  reserveLabel: string
  editLabel: string
  emptyReservationsDescription: string
  reservations: Reservation[]
  onReserve: () => void
  onEdit: () => void
  reserveDisabled?: boolean
  editDialog: ReactNode
  reservationDialog: ReactNode
}

export function ResourceDetailPage({
  title,
  description,
  backTo,
  status,
  summaryIcon: SummaryIcon,
  summaryContent,
  detailItems,
  detailDescription,
  reserveLabel,
  editLabel,
  emptyReservationsDescription,
  reservations,
  onReserve,
  onEdit,
  reserveDisabled = false,
  editDialog,
  reservationDialog,
}: ResourceDetailPageProps) {
  return (
    <div className="space-y-6">
      <PageHeader
        title={title}
        description={description}
        backTo={backTo}
        action={
          <Button
            onClick={onReserve}
            className="gap-2 h-10 shadow-sm"
            disabled={reserveDisabled}
          >
            <Calendar className="h-4 w-4" />
            Reservieren
          </Button>
        }
      />

      <Card className="overflow-hidden">
        <CardContent className="p-5">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div className="flex items-center gap-3 flex-wrap">
              <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
                <SummaryIcon className="h-4 w-4" />
              </div>
              {summaryContent}
              <StatusBadge status={status} />
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-3">
            <CardTitle className="text-base font-semibold">Details</CardTitle>
            <Button
              variant="ghost"
              size="icon"
              className="h-9 w-9"
              onClick={onEdit}
              aria-label={editLabel}
            >
              <Pencil className="h-4 w-4" />
            </Button>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              {detailItems.map(({ key, icon: Icon, label, value }) => (
                <div key={key} className="flex items-start gap-2.5">
                  <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                    <Icon className="h-3.5 w-3.5 text-muted-foreground" />
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground">{label}</p>
                    <div className="text-sm font-medium">{value}</div>
                  </div>
                </div>
              ))}
            </div>

            {detailDescription ? (
              <div>
                <p className="text-xs font-medium uppercase tracking-widest text-muted-foreground mb-1">
                  Beschreibung
                </p>
                <p className="text-sm leading-relaxed">{detailDescription}</p>
              </div>
            ) : null}
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-base font-semibold">Aktionen</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            <Button
              onClick={onReserve}
              disabled={reserveDisabled}
              className="w-full justify-start gap-2 h-10 shadow-sm active:scale-[0.97] transition-transform"
            >
              <Calendar className="h-4 w-4" />
              {reserveDisabled ? "Nicht verfügbar" : reserveLabel}
            </Button>
            <Button
              variant="outline"
              onClick={onEdit}
              className="w-full justify-start gap-2 h-10 active:scale-[0.97] transition-transform"
            >
              <Pencil className="h-4 w-4" />
              {editLabel}
            </Button>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base font-semibold">Reservierungen</CardTitle>
        </CardHeader>
        <CardContent>
          {reservations.length === 0 ? (
            <EmptyState
              icon={Calendar}
              title="Keine Reservierungen"
              description={emptyReservationsDescription}
            />
          ) : (
            <div className="space-y-3">
              {reservations.map((reservation) => (
                <Card
                  key={reservation.id}
                  className="overflow-hidden transition-colors hover:border-primary/30 hover:shadow-sm"
                >
                  <CardContent className="p-4">
                    <div className="flex items-start justify-between gap-3">
                      <div className="min-w-0 flex-1">
                        <p className="text-sm font-medium truncate">{reservation.user_name}</p>
                        <p className="text-xs text-muted-foreground mt-1">
                          {formatReservationDateTime(reservation.start_time)} - {formatReservationDateTime(reservation.end_time)}
                        </p>
                        {reservation.site_name ? (
                          <p className="text-xs text-muted-foreground mt-1">{reservation.site_name}</p>
                        ) : null}
                      </div>
                      <StatusBadge status={reservation.status} />
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {editDialog}
      {reservationDialog}
    </div>
  )
}
