import { useState } from "react"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus } from "lucide-react"
import { EmptyState, ErrorState } from "@/components/shared"
import { ResourceCard, ResourceCardSkeleton } from "@/components/fleet/ResourceCard"
import { useTools } from "@/lib/api/hooks"
import type { Tool, ResourceStatus } from "@/types/fleet"

const statusFilters: { value: ResourceStatus | undefined; label: string }[] = [
  { value: undefined, label: "Alle" },
  { value: "available", label: "Verfügbar" },
  { value: "in_use", label: "In Benutzung" },
  { value: "maintenance", label: "Wartung" },
]

interface ToolsListProps {
  onReserve?: (id: string, type: "vehicle" | "tool") => void
}

export function ToolsList({ onReserve }: ToolsListProps) {
  const [selectedStatus, setSelectedStatus] = useState<ResourceStatus | undefined>()
  const [searchQuery, setSearchQuery] = useState("")

  const {
    data: tools,
    isLoading,
    error,
    refetch,
  } = useTools(selectedStatus ? { status: selectedStatus } : undefined)

  const filteredTools = tools?.filter((t: Tool) =>
    t.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    t.category?.toLowerCase().includes(searchQuery.toLowerCase()) ||
    t.location?.toLowerCase().includes(searchQuery.toLowerCase())
  )

  return (
    <div className="space-y-4">
      {/* Filters */}
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Werkzeug suchen..."
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

      {/* Tools Grid */}
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
              <Button className="gap-2">
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
            />
          ))}
        </div>
      )}
    </div>
  )
}
