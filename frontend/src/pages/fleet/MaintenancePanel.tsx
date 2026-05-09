import { AlertTriangle, Check, Plus, Wrench } from "lucide-react"
import { toast } from "sonner"
import { EmptyState } from "@/components/shared"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useResolveMaintenanceDue } from "@/lib/api/hooks"
import { cn } from "@/lib/utils"
import type { MaintenanceDue } from "@/types/fleet"

interface MaintenancePanelProps {
  dueRecords: MaintenanceDue[]
  onAddSchedule: () => void
  canManage: boolean
}

function formatDate(value: string) {
  return new Intl.DateTimeFormat("de-DE", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  }).format(new Date(`${value}T00:00:00`))
}

export function MaintenancePanel({ dueRecords, onAddSchedule, canManage }: MaintenancePanelProps) {
  const resolveMutation = useResolveMaintenanceDue()
  const today = new Date()
  today.setHours(0, 0, 0, 0)

  const handleResolve = (id: string) => {
    resolveMutation.mutate(
      { id },
      {
        onSuccess: () => toast.success("Wartung erledigt"),
        onError: (error: Error) => toast.error(error.message || "Wartung konnte nicht erledigt werden"),
      }
    )
  }

  const getState = (dueDate: string) => {
    const date = new Date(`${dueDate}T00:00:00`)

    if (date < today) return "overdue"
    if (date.getTime() === today.getTime()) return "due"
    return "planned"
  }

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between gap-3">
        <CardTitle className="text-base font-semibold">Wartung</CardTitle>
        {canManage ? (
          <Button variant="outline" size="sm" className="gap-2" onClick={onAddSchedule}>
            <Plus className="h-4 w-4" />
            Planen
          </Button>
        ) : null}
      </CardHeader>
      <CardContent>
        {dueRecords.length === 0 ? (
          <EmptyState
            icon={Wrench}
            title="Keine offene Wartung"
            description="Für diese Ressource sind keine Wartungen fällig."
          />
        ) : (
          <div className="space-y-3">
            {dueRecords.map((due) => {
              const state = getState(due.due_date)
              const isActionable = state !== "planned"

              return (
                <div
                  key={due.id}
                  className={cn(
                    "rounded-lg border p-4",
                    state === "overdue"
                      ? "border-destructive/40 bg-destructive/5"
                      : state === "due"
                        ? "border-amber-500/30 bg-amber-500/5"
                        : "border-border bg-muted/20"
                  )}
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="min-w-0">
                      <div className="flex items-center gap-2">
                        <AlertTriangle
                          className={cn(
                            "h-4 w-4",
                            state === "overdue"
                              ? "text-destructive"
                              : state === "due"
                                ? "text-amber-600"
                                : "text-muted-foreground"
                          )}
                        />
                        <p className="text-sm font-medium">{due.task_description}</p>
                      </div>
                      <p className="mt-1 text-xs text-muted-foreground">
                        {state === "planned" ? "Nächste Fälligkeit" : "Fällig"} am {formatDate(due.due_date)}
                      </p>
                    </div>
                    <Badge variant={state === "overdue" ? "destructive" : "secondary"}>
                      {state === "overdue" ? "Überfällig" : state === "due" ? "Fällig" : "Geplant"}
                    </Badge>
                  </div>
                  {canManage && isActionable ? (
                    <Button
                      variant="outline"
                      size="sm"
                      className="mt-3 gap-2"
                      onClick={() => handleResolve(due.id)}
                      disabled={resolveMutation.isPending}
                    >
                      <Check className="h-4 w-4" />
                      Erledigt
                    </Button>
                  ) : null}
                </div>
              )
            })}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
