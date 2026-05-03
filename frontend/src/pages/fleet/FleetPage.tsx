import { Car } from "lucide-react"
import { ResourceOverviewPage } from "./ResourceOverviewPage"
import { VehiclesList } from "./VehiclesList"

export default function FleetPage() {
  return (
    <ResourceOverviewPage
      title="Fuhrpark"
      description="Fahrzeuge verwalten"
      resourceType="vehicle"
      resourceLabel="Fahrzeug"
      icon={Car}
      calendarDescription="Aktuelle Fahrzeugreservierungen direkt im Fuhrpark sehen."
      renderList={(onReserve) => <VehiclesList onReserve={onReserve} />}
    />
  )
}
