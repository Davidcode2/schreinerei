import type { LucideIcon } from "lucide-react"
import { Plus } from "lucide-react"
import { useState, type ReactNode } from "react"
import { PageHeader } from "@/components/shared"
import { Button } from "@/components/ui/button"
import { AddToolDialog } from "./AddToolDialog"
import { AddVehicleDialog } from "./AddVehicleDialog"
import CalendarView from "./CalendarView"
import { ReservationDialog } from "./ReservationDialog"

interface ResourceOverviewPageProps {
  title: string
  description: string
  resourceType: "vehicle" | "tool"
  resourceLabel: string
  icon: LucideIcon
  calendarDescription: string
  renderList: (onReserve: (id: string, type: "vehicle" | "tool") => void) => ReactNode
}

export function ResourceOverviewPage({
  title,
  description,
  resourceType,
  resourceLabel,
  icon: Icon,
  calendarDescription,
  renderList,
}: ResourceOverviewPageProps) {
  const [showReservationDialog, setShowReservationDialog] = useState(false)
  const [reserveResource, setReserveResource] = useState<{
    id: string
    type: "vehicle" | "tool"
  } | null>(null)
  const [showAddDialog, setShowAddDialog] = useState(false)

  const handleReserve = (id: string, type: "vehicle" | "tool") => {
    setReserveResource({ id, type })
    setShowReservationDialog(true)
  }

  const handleCloseReservationDialog = () => {
    setShowReservationDialog(false)
    setReserveResource(null)
  }

  return (
    <div className="space-y-6">
      <PageHeader
        title={title}
        description={description}
        action={
          <Button className="gap-2 h-10 shadow-sm" onClick={() => setShowAddDialog(true)}>
            <Plus className="h-4 w-4" />
            <span className="hidden sm:inline">{resourceLabel} hinzufügen</span>
            <span className="sm:hidden">Neu</span>
          </Button>
        }
      />

      <section className="space-y-4">
        <div className="flex items-center gap-3">
          <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
            <Icon className="h-4 w-4 text-muted-foreground" />
          </div>
          <div>
            <h2 className="font-display text-lg">Kalender</h2>
            <p className="text-sm text-muted-foreground">{calendarDescription}</p>
          </div>
        </div>
        <CalendarView embedded resourceType={resourceType} />
      </section>

      {renderList(handleReserve)}

      {reserveResource && (
        <ReservationDialog
          open={showReservationDialog}
          onOpenChange={handleCloseReservationDialog}
          resourceId={reserveResource.id}
          resourceType={reserveResource.type}
        />
      )}

      <AddVehicleDialog
        open={resourceType === "vehicle" && showAddDialog}
        onOpenChange={setShowAddDialog}
      />
      <AddToolDialog
        open={resourceType === "tool" && showAddDialog}
        onOpenChange={setShowAddDialog}
      />
    </div>
  )
}
