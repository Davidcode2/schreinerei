import { Button } from "@/components/ui/button"
import { useUpdateReservation } from "@/lib/api/hooks"
import { toast } from "sonner"
import type { ReservationStatus } from "@/types/fleet"

interface StatusTransitionButtonsProps {
  reservationId: string
  currentStatus: ReservationStatus
  onTransition?: () => void
}

const validTransitions: Record<ReservationStatus, ReservationStatus[]> = {
  pending: ["confirmed", "cancelled"],
  confirmed: ["in_use", "cancelled"],
  in_use: ["completed"],
  completed: [],
  cancelled: [],
}

const statusLabels: Record<ReservationStatus, string> = {
  pending: "Ausstehend",
  confirmed: "Bestätigt",
  in_use: "In Nutzung",
  completed: "Abgeschlossen",
  cancelled: "Storniert",
}

const transitionLabels: Record<ReservationStatus, string> = {
  pending: "Bestätigen",
  confirmed: "Starten",
  in_use: "Abschließen",
  completed: "",
  cancelled: "",
}

export function StatusTransitionButtons({
  reservationId,
  currentStatus,
  onTransition,
}: StatusTransitionButtonsProps) {
  const updateMutation = useUpdateReservation()

  const handleTransition = async (newStatus: ReservationStatus) => {
    try {
      await updateMutation.mutateAsync({
        id: reservationId,
        status: newStatus,
      })
      toast.success(`Status geändert zu: ${statusLabels[newStatus]}`)
      onTransition?.()
    } catch {
      toast.error("Statusänderung fehlgeschlagen")
    }
  }

  const transitions = validTransitions[currentStatus] || []

  if (transitions.length === 0) {
    return null
  }

  return (
    <div className="flex flex-wrap gap-2">
      {transitions.map((targetStatus) => {
        const isDestructive = targetStatus === "cancelled"
        const isSuccess = targetStatus === "completed"
        const isPrimary = targetStatus === "confirmed" || targetStatus === "in_use"

        return (
          <Button
            key={targetStatus}
            variant={isDestructive ? "destructive" : isSuccess ? "default" : isPrimary ? "default" : "outline"}
            size="sm"
            onClick={() => handleTransition(targetStatus)}
            disabled={updateMutation.isPending}
            className={isSuccess ? "bg-green-600 hover:bg-green-700" : ""}
          >
            {transitionLabels[targetStatus]}
          </Button>
        )
      })}
    </div>
  )
}

export { statusLabels }
