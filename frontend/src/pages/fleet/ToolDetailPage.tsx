import { useState } from "react"
import { useParams } from "react-router-dom"
import { MapPin, QrCode, Wrench } from "lucide-react"
import { LoadingSpinner, ErrorState } from "@/components/shared"
import { useReservations, useTool } from "@/lib/api/hooks"
import { AddToolDialog } from "./AddToolDialog"
import { ReservationDialog } from "./ReservationDialog"
import { ResourceDetailPage } from "./ResourceDetailPage"

export default function ToolDetailPage() {
  const { id } = useParams<{ id: string }>()
  const [showEditDialog, setShowEditDialog] = useState(false)
  const [showReservationDialog, setShowReservationDialog] = useState(false)

  const { data: tool, isLoading, error, refetch } = useTool(id!)
  const { data: reservations = [] } = useReservations({
    resource_type: "tool",
    ...(id ? { resource_id: id } : {}),
  })

  if (isLoading) {
    return <LoadingSpinner className="min-h-[400px]" size="lg" />
  }

  if (error || !tool) {
    return (
      <ErrorState
        message="Werkzeug konnte nicht geladen werden"
        onRetry={() => refetch()}
      />
    )
  }

  const isAvailable = tool.status === "available"

  return (
    <ResourceDetailPage
      title={tool.name}
      description={tool.description || tool.category || "Werkzeug"}
      backTo="/tools"
      status={tool.status}
      summaryIcon={Wrench}
      summaryContent={tool.category ? <span className="text-lg font-medium">{tool.category}</span> : null}
      detailItems={[
        ...(tool.category
          ? [{ key: "category", icon: Wrench, label: "Kategorie", value: tool.category }]
          : []),
        ...(tool.location
          ? [{ key: "location", icon: MapPin, label: "Standort", value: tool.location }]
          : []),
        ...(tool.qr_code
          ? [{ key: "qr", icon: QrCode, label: "QR-Code", value: <span className="font-mono">{tool.qr_code}</span> }]
          : []),
      ]}
      detailDescription={tool.description}
      reserveLabel="Werkzeug reservieren"
      editLabel="Werkzeug bearbeiten"
      emptyReservationsDescription="Für dieses Werkzeug gibt es aktuell keine Reservierungen."
      reservations={reservations}
      onReserve={() => setShowReservationDialog(true)}
      onEdit={() => setShowEditDialog(true)}
      reserveDisabled={!isAvailable}
      editDialog={
        <AddToolDialog
          open={showEditDialog}
          onOpenChange={setShowEditDialog}
          mode="edit"
          initialData={tool}
        />
      }
      reservationDialog={
        <ReservationDialog
          open={showReservationDialog}
          onOpenChange={setShowReservationDialog}
          resourceId={tool.id}
          resourceType="tool"
        />
      }
    />
  )
}
