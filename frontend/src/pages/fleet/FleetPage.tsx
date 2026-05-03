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
import { cn } from "@/lib/utils"

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
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button className="gap-2 h-10 shadow-sm">
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
        }
      />

      <section className="space-y-4">
        <div className="flex items-center gap-3">
          <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
            <Car className="h-4 w-4 text-muted-foreground" />
          </div>
          <div>
            <h2 className="font-display text-lg">Kalender</h2>
            <p className="text-sm text-muted-foreground">
              Aktuelle Reservierungen direkt im Fuhrpark sehen.
            </p>
          </div>
        </div>
        <CalendarView embedded />
      </section>

      <div className="flex gap-2 overflow-x-auto pb-1">
        {tabs.map((tab) => (
          <button
            key={tab.value}
            onClick={() => setActiveTab(tab.value)}
            className={cn(
              "rounded-full px-4 py-1.5 text-sm font-medium whitespace-nowrap transition-colors",
              activeTab === tab.value
                ? "bg-primary text-primary-foreground shadow-sm"
                : "bg-muted text-muted-foreground hover:bg-accent hover:text-foreground"
            )}
          >
            {tab.label}
          </button>
        ))}
      </div>

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
