import { Wrench } from "lucide-react"
import { ResourceOverviewPage } from "./ResourceOverviewPage"
import { ToolsList } from "./ToolsList"

export default function ToolsPage() {
  return (
    <ResourceOverviewPage
      title="Werkzeuge"
      description="Werkzeuge verwalten"
      resourceType="tool"
      resourceLabel="Werkzeug"
      icon={Wrench}
      calendarDescription="Aktuelle Werkzeugreservierungen direkt in der Werkzeugansicht sehen."
      renderList={(onReserve) => <ToolsList onReserve={onReserve} />}
    />
  )
}
