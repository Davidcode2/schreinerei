import { useState } from "react"
import { useParams } from "react-router-dom"
import { Car, MapPin, QrCode, Wrench } from "lucide-react"
import { LoadingSpinner, ErrorState } from "@/components/shared"
import { useReservations, useVehicle } from "@/lib/api/hooks"
import { AddVehicleDialog } from "./AddVehicleDialog"
import { ReservationDialog } from "./ReservationDialog"
import { ResourceDetailPage } from "./ResourceDetailPage"
import { getVehicleTypeLabel } from "./resourceLabels"

export default function VehicleDetailPage() {
  const { id } = useParams<{ id: string }>()
  const [showEditDialog, setShowEditDialog] = useState(false)
  const [showReservationDialog, setShowReservationDialog] = useState(false)

  const { data: vehicle, isLoading, error, refetch } = useVehicle(id!)
  const { data: reservations = [] } = useReservations({
    resource_type: "vehicle",
    resource_id: id,
  })

  if (isLoading) {
    return <LoadingSpinner className="min-h-[400px]" size="lg" />
  }

  if (error || !vehicle) {
    return (
      <ErrorState
        message="Fahrzeug konnte nicht geladen werden"
        onRetry={() => refetch()}
      />
    )
  }

  const isAvailable = vehicle.status === "available"

  return (
    <ResourceDetailPage
      title={vehicle.name}
      description={vehicle.description || getVehicleTypeLabel(vehicle.vehicle_type)}
      backTo="/fleet"
      status={vehicle.status}
      summaryIcon={Car}
      summaryContent={
        <div className="flex items-center gap-2">
          <span className="text-lg font-medium">{getVehicleTypeLabel(vehicle.vehicle_type)}</span>
          {vehicle.license_plate ? (
            <span className="text-sm text-muted-foreground">· {vehicle.license_plate}</span>
          ) : null}
        </div>
      }
      detailItems={[
        {
          key: "type",
          icon: Wrench,
          label: "Fahrzeugtyp",
          value: getVehicleTypeLabel(vehicle.vehicle_type),
        },
        ...(vehicle.license_plate
          ? [{ key: "license", icon: Car, label: "Kennzeichen", value: vehicle.license_plate }]
          : []),
        ...(vehicle.location
          ? [{ key: "location", icon: MapPin, label: "Standort", value: vehicle.location }]
          : []),
        ...(vehicle.qr_code
          ? [{ key: "qr", icon: QrCode, label: "QR-Code", value: <span className="font-mono">{vehicle.qr_code}</span> }]
          : []),
      ]}
      detailDescription={vehicle.description}
      reserveLabel="Fahrzeug reservieren"
      editLabel="Fahrzeug bearbeiten"
      emptyReservationsDescription="Für dieses Fahrzeug gibt es aktuell keine Reservierungen."
      reservations={reservations}
      onReserve={() => setShowReservationDialog(true)}
      onEdit={() => setShowEditDialog(true)}
      reserveDisabled={!isAvailable}
      editDialog={
        <AddVehicleDialog
          open={showEditDialog}
          onOpenChange={setShowEditDialog}
          mode="edit"
          initialData={vehicle}
        />
      }
      reservationDialog={
        <ReservationDialog
          open={showReservationDialog}
          onOpenChange={setShowReservationDialog}
          resourceId={vehicle.id}
          resourceType="vehicle"
        />
      }
    />
  )
}
