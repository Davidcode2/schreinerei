import { useMemo, useState } from "react"
import { UserPlus, Users, X } from "lucide-react"
import { toast } from "sonner"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { useAssignUser, useRemoveAssignment } from "@/lib/api/hooks"
import { useUsers } from "@/lib/api/hooks/useIam"
import type { SiteAssignment } from "@/types/sites"

interface ProjectAssignmentsSectionProps {
  siteId: string
  assignments: SiteAssignment[]
}

export function ProjectAssignmentsSection({ siteId, assignments }: ProjectAssignmentsSectionProps) {
  const { data: users } = useUsers()
  const assignUser = useAssignUser()
  const removeAssignment = useRemoveAssignment()
  const [selectedUserId, setSelectedUserId] = useState("")

  const assignedIds = useMemo(() => new Set(assignments.map((assignment) => assignment.user_id)), [assignments])
  const availableUsers = (users ?? []).filter((user) => !assignedIds.has(user.id))

  function handleAssign() {
    if (!selectedUserId) return
    assignUser.mutate(
      { siteId, user_id: selectedUserId, role: "worker" },
      {
        onSuccess: () => {
          toast.success("Mitarbeiter zugewiesen")
          setSelectedUserId("")
        },
        onError: () => toast.error("Zuweisung fehlgeschlagen"),
      }
    )
  }

  function handleRemove(userId: string) {
    removeAssignment.mutate(
      { siteId, userId },
      {
        onSuccess: () => toast.success("Zuweisung entfernt"),
        onError: () => toast.error("Zuweisung konnte nicht entfernt werden"),
      }
    )
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <Users className="h-4 w-4 text-muted-foreground" />
        <p className="text-sm font-medium">Projektteam</p>
      </div>

      <div className="flex flex-col gap-2 sm:flex-row">
        <Select value={selectedUserId} onValueChange={setSelectedUserId}>
          <SelectTrigger className="h-10 flex-1">
            <SelectValue placeholder="Mitarbeiter auswählen" />
          </SelectTrigger>
          <SelectContent>
            {availableUsers.length === 0 ? (
              <SelectItem value="__none" disabled>
                Keine weiteren Mitarbeiter verfügbar
              </SelectItem>
            ) : (
              availableUsers.map((user) => (
                <SelectItem key={user.id} value={user.id}>
                  {user.name || user.email}
                </SelectItem>
              ))
            )}
          </SelectContent>
        </Select>
        <Button className="h-10 gap-2" disabled={!selectedUserId || assignUser.isPending} onClick={handleAssign}>
          <UserPlus className="h-4 w-4" />
          Hinzufügen
        </Button>
      </div>

      <div className="space-y-2">
        {assignments.length === 0 ? (
          <p className="text-sm text-muted-foreground">Noch keine Mitarbeiter zugewiesen.</p>
        ) : (
          assignments.map((assignment) => {
            const user = users?.find((entry) => entry.id === assignment.user_id)
            const label = user?.name || user?.email || assignment.user_id

            return (
              <div key={assignment.id} className="flex items-center justify-between rounded-xl border bg-card px-3 py-2">
                <div className="min-w-0">
                  <p className="truncate text-sm font-medium">{label}</p>
                  <Badge variant="outline" className="mt-1 text-[11px] font-normal">
                    {assignment.role === "lead" ? "Leitung" : "Mitarbeiter"}
                  </Badge>
                </div>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8 text-muted-foreground hover:text-destructive"
                  disabled={removeAssignment.isPending}
                  onClick={() => handleRemove(assignment.user_id)}
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            )
          })
        )}
      </div>
    </div>
  )
}
