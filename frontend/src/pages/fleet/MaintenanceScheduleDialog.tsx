import { useState, type FormEvent } from "react"
import { toast } from "sonner"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { useCreateMaintenanceSchedule } from "@/lib/api/hooks"

interface MaintenanceScheduleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  assetId: string
}

function defaultDueDate() {
  const date = new Date()
  date.setDate(date.getDate() + 30)
  return date.toISOString().slice(0, 10)
}

export function MaintenanceScheduleDialog({
  open,
  onOpenChange,
  assetId,
}: MaintenanceScheduleDialogProps) {
  const [taskDescription, setTaskDescription] = useState("")
  const [intervalDays, setIntervalDays] = useState("90")
  const [nextDueDate, setNextDueDate] = useState(defaultDueDate)
  const createMutation = useCreateMaintenanceSchedule()

  const resetForm = () => {
    setTaskDescription("")
    setIntervalDays("90")
    setNextDueDate(defaultDueDate())
  }

  const handleOpenChange = (nextOpen: boolean) => {
    if (!nextOpen) {
      resetForm()
    }

    onOpenChange(nextOpen)
  }

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()

    createMutation.mutate(
      {
        asset_id: assetId,
        task_description: taskDescription.trim(),
        interval_days: Number(intervalDays),
        next_due_date: nextDueDate,
      },
      {
        onSuccess: () => {
          toast.success("Wartung geplant")
          resetForm()
          onOpenChange(false)
        },
        onError: (error: Error) => {
          toast.error(error.message || "Wartung konnte nicht geplant werden")
        },
      }
    )
  }

  const isValid = taskDescription.trim() && Number(intervalDays) > 0 && nextDueDate

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Wartung planen</DialogTitle>
          <DialogDescription>
            Offene Erinnerungen bleiben sichtbar, bis sie erledigt werden.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="maintenance-task">Aufgabe</Label>
            <Textarea
              id="maintenance-task"
              value={taskDescription}
              onChange={(event) => setTaskDescription(event.target.value)}
              placeholder="z.B. Ölwechsel, Sicherheitsprüfung"
              rows={3}
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div className="space-y-2">
              <Label htmlFor="maintenance-interval">Intervall</Label>
              <Input
                id="maintenance-interval"
                type="number"
                min={1}
                value={intervalDays}
                onChange={(event) => setIntervalDays(event.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="maintenance-due">Nächste Fälligkeit</Label>
              <Input
                id="maintenance-due"
                type="date"
                value={nextDueDate}
                onChange={(event) => setNextDueDate(event.target.value)}
              />
            </div>
          </div>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => handleOpenChange(false)}
              disabled={createMutation.isPending}
            >
              Abbrechen
            </Button>
            <Button type="submit" disabled={!isValid || createMutation.isPending}>
              Speichern
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
