import { useState } from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus } from "lucide-react"
import { EmptyState, ErrorState } from "@/components/shared"
import { ResourceCard, ResourceCardSkeleton } from "@/components/fleet/ResourceCard"
import { useVehicles } from "@/lib/api/hooks"
import { AddVehicleDialog } from "./AddVehicleDialog"
import type { Vehicle, ResourceStatus } from "@/types/fleet"

const statusFilters: { value: ResourceStatus | undefined; label: string }[] = [
  { value: undefined, label: "Alle" },
  { value: "available", label: "Verfügbar" },
  { value: "in_use", label: "In Benutzung" },
  { value: "maintenance", label: "Wartung" },
]

interface VehiclesListProps {
  onReserve: (id: string, type: "vehicle" | "tool") => void
}

export function VehiclesList({ onReserve }: VehiclesListProps) {
  const [selectedStatus, setSelectedStatus] = useState<ResourceStatus | undefined>()
  const [searchQuery, setSearchQuery] = useState("")
  const [addVehicleOpen, setAddVehicleOpen] = useState(false)

  const {
    data: vehicles,
    isLoading,
    error,
    refetch,
  } = useVehicles(selectedStatus ? { status: selectedStatus } : undefined)

  const filteredVehicles = vehicles?.filter((v: Vehicle) =>
    v.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    v.license_plate?.toLowerCase().includes(searchQuery.toLowerCase()) ||
    v.location?.toLowerCase().includes(searchQuery.toLowerCase())
  )

  return (
    <div className="space-y-4">
      {/* Filters */}
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Fahrzeug suchen..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-10"
          />
        </div>
        <div className="flex gap-2 overflow-x-auto pb-2 sm:pb-0">
          {statusFilters.map((filter) => (
            <Button
              key={filter.label}
              variant={selectedStatus === filter.value ? "default" : "outline"}
              size="sm"
              onClick={() => setSelectedStatus(filter.value)}
              className="rounded-full whitespace-nowrap"
            >
              {filter.label}
            </Button>
          ))}
        </div>
      </div>

      {/* Vehicles Grid */}
      {isLoading ? (
        <div className="grid gap-4 grid-cols-1 md:grid-cols-2">
          <ResourceCardSkeleton count={4} />
        </div>
      ) : error ? (
        <ErrorState
          message="Fahrzeuge konnten nicht geladen werden"
          onRetry={() => refetch()}
        />
      ) : !filteredVehicles || filteredVehicles.length === 0 ? (
        <EmptyState
          icon={Search}
          title={searchQuery ? "Keine Ergebnisse" : "Keine Fahrzeuge"}
          description={
            searchQuery
              ? "Keine Fahrzeuge entsprechen Ihrer Suche."
              : "Fügen Sie Ihr erstes Fahrzeug hinzu."
          }
          action={
            !searchQuery && (
              <Button className="gap-2" onClick={() => setAddVehicleOpen(true)}>
                <Plus className="h-4 w-4" />
                Fahrzeug hinzufügen
              </Button>
            )
          }
        />
      ) : (
        <div className="grid gap-4 grid-cols-1 md:grid-cols-2">
          {filteredVehicles.map((vehicle: Vehicle) => (
            <ResourceCard
              key={vehicle.id}
              resource={vehicle}
              type="vehicle"
              onReserve={onReserve}
            />
          ))}
        </div>
      )}
      <AddVehicleDialog open={addVehicleOpen} onOpenChange={setAddVehicleOpen} />
    </div>
  )
}
