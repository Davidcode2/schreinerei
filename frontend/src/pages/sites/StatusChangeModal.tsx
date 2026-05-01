import { useState } from "react"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { ArrowRight } from "lucide-react"
import { useUpdateSite } from "@/lib/api/hooks"
import { toast } from "sonner"
import type { SiteStatus } from "@/types/sites"

interface StatusChangeModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  siteId: string
  siteName: string
  currentStatus: SiteStatus
  onSuccess: () => void
}

const statusLabels: Record<SiteStatus, string> = {
  planned: "Geplant",
  active: "Aktiv",
  completed: "Abgeschlossen",
  archived: "Archiviert",
}

export function StatusChangeModal({
  open,
  onOpenChange,
  siteId,
  siteName,
  currentStatus,
  onSuccess,
}: StatusChangeModalProps) {
  const [isChanging, setIsChanging] = useState(false)
  const updateSite = useUpdateSite()

  const getValidTransitions = (status: SiteStatus): SiteStatus[] => {
    switch (status) {
      case "planned":
        return ["active"]
      case "active":
        return ["completed"]
      case "completed":
        return ["archived"]
      case "archived":
        return []
      default:
        return []
    }
  }

  const handleStatusChange = async (newStatus: SiteStatus) => {
    setIsChanging(true)
    try {
      await updateSite.mutateAsync({
        id: siteId,
        status: newStatus,
      })
      toast.success(`Status geändert zu "${statusLabels[newStatus]}"`)
      onSuccess()
      onOpenChange(false)
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Unbekannter Fehler"
      
      if (errorMessage.includes("Invalid status transition") || errorMessage.includes("bereits geändert")) {
        toast.error("Status wurde bereits geändert. Aktualisieren...", {
          duration: 2000,
        })
        setTimeout(() => {
          onSuccess()
        }, 1000)
      } else {
        toast.error("Fehler beim Ändern des Status")
      }
    } finally {
      setIsChanging(false)
    }
  }

  const validTransitions = getValidTransitions(currentStatus)

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Baustellen-Status ändern</DialogTitle>
          <DialogDescription>
            {siteName}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          <div className="flex items-center justify-center gap-3 py-4">
            <div className="text-sm font-medium text-muted-foreground">
              {statusLabels[currentStatus]}
            </div>
            {validTransitions.length > 0 && (
              <>
                <ArrowRight className="h-4 w-4 text-muted-foreground" />
                <div className="flex gap-2">
                  {validTransitions.map((targetStatus) => (
                    <Button
                      key={targetStatus}
                      onClick={() => handleStatusChange(targetStatus)}
                      disabled={isChanging}
                      className="gap-2"
                    >
                      {targetStatus === "active" && "Aktivieren"}
                      {targetStatus === "completed" && "Abschließen"}
                      {targetStatus === "archived" && "Archivieren"}
                    </Button>
                  ))}
                </div>
              </>
            )}
          </div>

          {validTransitions.length === 0 && (
            <p className="text-sm text-muted-foreground text-center">
              Keine weiteren Statusänderungen möglich
            </p>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
