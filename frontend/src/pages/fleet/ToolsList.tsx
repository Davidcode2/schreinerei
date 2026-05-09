import { useMemo, useState } from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus } from "lucide-react"
import { EmptyState, ErrorState } from "@/components/shared"
import { ResourceCard, ResourceCardSkeleton } from "@/components/fleet/ResourceCard"
import { useCalendar, useMaintenanceDue, useTools } from "@/lib/api/hooks"
import { AddToolDialog } from "./AddToolDialog"
import { buildEffectiveStatusMap, getEffectiveResourceStatus } from "./effectiveResourceStatus"
import { cn } from "@/lib/utils"
import type { Tool, ResourceStatus } from "@/types/fleet"

const statusFilters: { value: ResourceStatus | undefined; label: string }[] = [
  { value: undefined, label: "Alle" },
  { value: "available", label: "Verfügbar" },
  { value: "in_use", label: "In Benutzung" },
  { value: "maintenance", label: "Wartung" },
]

function isMaintenanceReminderVisible(dueDate: string) {
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  return new Date(`${dueDate}T00:00:00`) <= today
}

interface ToolsListProps {
  onReserve: (id: string, type: "vehicle" | "tool") => void
}

export function ToolsList({ onReserve }: ToolsListProps) {
  const [selectedStatus, setSelectedStatus] = useState<ResourceStatus | undefined>()
  const [searchQuery, setSearchQuery] = useState("")
  const [addToolOpen, setAddToolOpen] = useState(false)

  const {
    data: tools,
    isLoading,
    error,
    refetch,
  } = useTools()

  const todayStart = new Date()
  todayStart.setHours(0, 0, 0, 0)
  const todayEnd = new Date(todayStart)
  todayEnd.setHours(23, 59, 59, 999)

  const { data: todayCalendar } = useCalendar({
    start_date: todayStart.toISOString(),
    end_date: todayEnd.toISOString(),
    resource_type: "tool",
  })

  const { data: openMaintenance = [] } = useMaintenanceDue({ status: "open" })
  const maintenanceByAsset = useMemo(() => {
    const records = new Map<string, { count: number; hasOverdue: boolean }>()

    openMaintenance
      .filter((due) => due.resource_type === "tool" && isMaintenanceReminderVisible(due.due_date))
      .forEach((due) => {
        const current = records.get(due.asset_id) ?? { count: 0, hasOverdue: false }
        records.set(due.asset_id, {
          count: current.count + 1,
          hasOverdue: current.hasOverdue || due.severity === "overdue",
        })
      })

    return records
  }, [openMaintenance])

  const effectiveStatusMap = buildEffectiveStatusMap(todayCalendar?.resources, new Date())

  const toolsWithEffectiveStatus = tools?.map((tool: Tool) => ({
    ...tool,
    status: getEffectiveResourceStatus(tool.status, effectiveStatusMap, "tool", tool.id),
  }))

  const filteredTools = toolsWithEffectiveStatus?.filter((t: Tool) => {
    const matchesStatus = !selectedStatus || t.status === selectedStatus

    return matchesStatus && (
      t.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      t.category?.toLowerCase().includes(searchQuery.toLowerCase()) ||
      t.location?.toLowerCase().includes(searchQuery.toLowerCase())
    )
  })

  return (
    <div className="space-y-4">
      <div className="space-y-4">
        <div className="relative flex-1">
          <Search className="absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Werkzeug suchen..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-10 h-10 bg-card border-border"
          />
        </div>
        <div className="flex gap-2 overflow-x-auto pb-1">
          {statusFilters.map((filter) => (
            <button
              key={filter.label}
              onClick={() => setSelectedStatus(filter.value)}
              className={cn(
                "rounded-full px-4 py-1.5 text-sm font-medium whitespace-nowrap transition-colors",
                selectedStatus === filter.value
                  ? "bg-primary text-primary-foreground shadow-sm"
                  : "bg-muted text-muted-foreground hover:bg-accent hover:text-foreground"
              )}
            >
              {filter.label}
            </button>
          ))}
        </div>
      </div>

      {isLoading ? (
        <div className="grid gap-4 grid-cols-1 md:grid-cols-2">
          <ResourceCardSkeleton count={4} />
        </div>
      ) : error ? (
        <ErrorState
          message="Werkzeuge konnten nicht geladen werden"
          onRetry={() => refetch()}
        />
      ) : !filteredTools || filteredTools.length === 0 ? (
        <EmptyState
          icon={Search}
          title={searchQuery ? "Keine Ergebnisse" : "Keine Werkzeuge"}
          description={
            searchQuery
              ? "Keine Werkzeuge entsprechen Ihrer Suche."
              : "Fügen Sie Ihr erstes Werkzeug hinzu."
          }
          action={
            !searchQuery && (
              <Button className="gap-2 h-10 shadow-sm" onClick={() => setAddToolOpen(true)}>
                <Plus className="h-4 w-4" />
                Werkzeug hinzufügen
              </Button>
            )
          }
        />
      ) : (
        <div className="grid gap-4 grid-cols-1 md:grid-cols-2">
          {filteredTools.map((tool: Tool) => (
            <ResourceCard
              key={tool.id}
              resource={tool}
              type="tool"
              onReserve={onReserve}
              maintenanceDueCount={maintenanceByAsset.get(tool.id)?.count ?? 0}
              hasOverdueMaintenance={maintenanceByAsset.get(tool.id)?.hasOverdue ?? false}
            />
          ))}
        </div>
      )}
      <AddToolDialog open={addToolOpen} onOpenChange={setAddToolOpen} />
    </div>
  )
}
