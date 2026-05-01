import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Plus, Car, Wrench } from "lucide-react"
import { PageHeader } from "@/components/shared"
import { VehiclesList } from "./VehiclesList"
import { ToolsList } from "./ToolsList"
import { ReservationsList } from "./ReservationsList"
import { ReservationDialog } from "./ReservationDialog"
import { AddVehicleDialog } from "./AddVehicleDialog"
import { AddToolDialog } from "./AddToolDialog"
import CalendarView from "./CalendarView"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"

type TabType = "vehicles" | "tools" | "reservations"

const tabs: { value: TabType; label: string }[] = [
  { value: "vehicles", label: "Fahrzeuge" },
  { value: "tools", label: "Werkzeuge" },
  { value: "reservations", label: "Reservierungen" },
]

export default function FleetPage() {
  const [activeTab, setActiveTab] = useState<TabType>("vehicles")
  const [showReservationDialog, setShowReservationDialog] = useState(false)
  const [reserveResource, setReserveResource] = useState<{
    id: string
    type: "vehicle" | "tool"
  } | null>(null)
  const [dialogType, setDialogType] = useState<"vehicle" | "tool" | null>(null)

  const handleReserve = (id: string, type: "vehicle" | "tool") => {
    setReserveResource({ id, type })
    setShowReservationDialog(true)
  }

  const handleCloseDialog = () => {
    setShowReservationDialog(false)
    setReserveResource(null)
  }

  return (
    <div className="space-y-6">
      <PageHeader
        title="Fuhrpark & Werkzeuge"
        description="Fahrzeuge und Werkzeuge verwalten"
        action={
          <div className="flex gap-2">
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button className="gap-2">
                  <Plus className="h-4 w-4" />
                  <span className="hidden sm:inline">Neu</span>
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem onClick={() => setDialogType("vehicle")}>
                  <Car className="h-4 w-4 mr-2" />
                  Fahrzeug
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setDialogType("tool")}>
                  <Wrench className="h-4 w-4 mr-2" />
                  Werkzeug
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        }
      />

      <section className="space-y-4">
        <div className="space-y-1">
          <h2 className="text-lg font-semibold">Kalender</h2>
          <p className="text-sm text-muted-foreground">
            Aktuelle Reservierungen direkt im Fuhrpark sehen.
          </p>
        </div>
        <CalendarView embedded />
      </section>

      {/* Tabs */}
      <div className="flex gap-2 overflow-x-auto pb-2 -mb-2">
        {tabs.map((tab) => (
          <Button
            key={tab.value}
            variant={activeTab === tab.value ? "default" : "outline"}
            onClick={() => setActiveTab(tab.value)}
            className="rounded-full whitespace-nowrap"
          >
            {tab.label}
          </Button>
        ))}
      </div>

      {/* Tab Content */}
      {activeTab === "vehicles" && <VehiclesList onReserve={handleReserve} />}
      {activeTab === "tools" && <ToolsList onReserve={handleReserve} />}
      {activeTab === "reservations" && <ReservationsList showOnlyMine />}

      {reserveResource && (
        <ReservationDialog
          open={showReservationDialog}
          onOpenChange={handleCloseDialog}
          resourceId={reserveResource.id}
          resourceType={reserveResource.type}
        />
      )}

      <AddVehicleDialog
        open={dialogType === "vehicle"}
        onOpenChange={(open) => !open && setDialogType(null)}
      />
      <AddToolDialog
        open={dialogType === "tool"}
        onOpenChange={(open) => !open && setDialogType(null)}
      />
    </div>
  )
}
