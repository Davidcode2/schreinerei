import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Plus, Calendar } from "lucide-react"
import { Link } from "react-router-dom"
import { PageHeader } from "@/components/shared"
import { VehiclesList } from "./VehiclesList"
import { ToolsList } from "./ToolsList"
import { ReservationsList } from "./ReservationsList"
import { ReservationDialog } from "./ReservationDialog"

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
    name?: string
  } | null>(null)

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
            <Link to="/fleet/calendar">
              <Button variant="outline" size="icon">
                <Calendar className="h-4 w-4" />
              </Button>
            </Link>
            <Button className="gap-2">
              <Plus className="h-4 w-4" />
              <span className="hidden sm:inline">Neu</span>
            </Button>
          </div>
        }
      />

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

      <ReservationDialog
        open={showReservationDialog}
        onOpenChange={handleCloseDialog}
        resourceId={reserveResource?.id}
        resourceType={reserveResource?.type}
        resourceName={reserveResource?.name}
      />
    </div>
  )
}
