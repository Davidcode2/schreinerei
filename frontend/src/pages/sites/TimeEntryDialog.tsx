import { useState, useEffect } from "react"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Clock } from "lucide-react"
import { useCreateTimeEntry, useUpdateTimeEntry, useDeleteTimeEntry } from "@/lib/api/hooks"
import { toast } from "sonner"
import type { WorkType, TimeEntry } from "@/types/sites"

interface TimeEntryDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  siteId?: string
  siteName?: string
  mode?: "create" | "edit"
  initialData?: TimeEntry
}

const workTypes: { value: WorkType; label: string }[] = [
  { value: "site", label: "Baustelle" },
  { value: "workshop", label: "Werkstatt" },
  { value: "travel", label: "Fahrt" },
  { value: "other", label: "Sonstiges" },
]

const quickHours = [0.5, 1, 2, 4, 8]

export function TimeEntryDialog({
  open,
  onOpenChange,
  siteId,
  siteName,
  mode = "create",
  initialData,
}: TimeEntryDialogProps) {
  const [workType, setWorkType] = useState<WorkType>(initialData?.work_type ?? "site")
  const [hours, setHours] = useState(initialData?.hours ?? 0.5)
  const [workDate, setWorkDate] = useState(
    initialData?.work_date ?? new Date().toISOString().split("T")[0] ?? ""
  )
  const [notes, setNotes] = useState(initialData?.notes ?? "")
  const [errors, setErrors] = useState<Record<string, string>>({})
  const [touched, setTouched] = useState<Record<string, boolean>>({})
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)

  const createMutation = useCreateTimeEntry()
  const updateMutation = useUpdateTimeEntry()
  const deleteMutation = useDeleteTimeEntry()

  // Reset state when dialog opens with new data
  useEffect(() => {
    if (open) {
      if (mode === "edit" && initialData) {
        setWorkType(initialData.work_type)
        setHours(initialData.hours)
        setWorkDate(initialData.work_date)
        setNotes(initialData.notes ?? "")
      } else {
        setWorkType("site")
        setHours(0.5)
        setWorkDate(new Date().toISOString().split("T")[0] ?? "")
        setNotes("")
      }
      setErrors({})
      setTouched({})
      setShowDeleteConfirm(false)
    }
  }, [open, mode, initialData])

  const validateHours = (value: number): string | null => {
    if (value <= 0) return "Stunden müssen größer als 0 sein"
    if (value > 24) return "Stunden dürfen 24 nicht überschreiten"
    return null
  }

  const setHoursError = (error: string | null) => {
    if (error) {
      setErrors(prev => ({ ...prev, hours: error }))
    } else {
      setErrors(prev => {
        const { hours: _, ...rest } = prev
        return rest
      })
    }
  }

  const isFormValid = hours > 0 && hours <= 24 && workDate

  const handleSubmit = async () => {
    try {
      if (mode === "edit" && initialData) {
        await updateMutation.mutateAsync({
          id: initialData.id,
          work_type: workType,
          hours,
          work_date: workDate,
          ...(notes ? { notes } : {}),
        })
        toast.success("Zeiteintrag aktualisiert")
      } else {
        await createMutation.mutateAsync({
          ...(workType === "site" && siteId ? { site_id: siteId } : {}),
          work_type: workType,
          hours,
          work_date: workDate,
          ...(notes ? { notes } : {}),
        })
        toast.success("Zeit erfasst")
      }
      onOpenChange(false)
    } catch {
      toast.error(
        mode === "edit" ? "Aktualisierung fehlgeschlagen" : "Zeiterfassung fehlgeschlagen"
      )
    }
  }

  const handleDelete = async () => {
    if (!initialData) return
    try {
      await deleteMutation.mutateAsync(initialData.id)
      toast.success("Zeiteintrag gelöscht")
      onOpenChange(false)
      setShowDeleteConfirm(false)
    } catch {
      toast.error("Löschen fehlgeschlagen")
    }
  }

  const isPending = createMutation.isPending || updateMutation.isPending

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              <Clock className="h-5 w-5" />
              {mode === "edit" ? "Zeit bearbeiten" : "Zeit buchen"}
            </DialogTitle>
            <DialogDescription>
              {mode === "edit"
                ? "Zeiteintrag anpassen"
                : siteName || "Neuen Zeiteintrag erstellen"}
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            {/* Work Type */}
            <div className="space-y-2">
              <Label>Art der Arbeit</Label>
              <div className="flex flex-wrap gap-2">
                {workTypes.map((type) => (
                  <Button
                    key={type.value}
                    variant={workType === type.value ? "default" : "outline"}
                    size="sm"
                    onClick={() => setWorkType(type.value)}
                  >
                    {type.label}
                  </Button>
                ))}
              </div>
            </div>

            {/* Date */}
            <div className="space-y-2">
              <Label>Datum</Label>
              <Input
                type="date"
                value={workDate}
                onChange={(e) => setWorkDate(e.target.value)}
              />
            </div>

            {/* Hours */}
            <div className="space-y-2">
              <Label>Stunden</Label>
              <Input
                type="number"
                value={hours}
                onChange={(e) => {
                  const value = parseFloat(e.target.value) || 0
                  setHours(value)
                  if (touched.hours) {
                    setHoursError(validateHours(value))
                  }
                }}
                onBlur={() => {
                  setTouched(prev => ({ ...prev, hours: true }))
                  setHoursError(validateHours(hours))
                }}
                min={0.5}
                max={24}
                step={0.5}
                className={errors.hours ? "border-destructive" : ""}
              />
              {errors.hours && (
                <p className="text-sm text-destructive">{errors.hours}</p>
              )}
              <div className="flex flex-wrap gap-2">
                {quickHours.map((h) => (
                  <Button
                    key={h}
                    variant={hours === h ? "default" : "outline"}
                    size="sm"
                    onClick={() => {
                      setHours(h)
                      setHoursError(null)
                    }}
                    className="min-w-[48px]"
                  >
                    {h}h
                  </Button>
                ))}
              </div>
            </div>

            {/* Notes */}
            <div className="space-y-2">
              <Label>Notiz (optional)</Label>
              <Input
                placeholder="z.B. Montage Schränke"
                value={notes}
                onChange={(e) => setNotes(e.target.value)}
              />
            </div>
          </div>

          <DialogFooter className="flex-col gap-2 sm:flex-row">
            {mode === "edit" && (
              <Button
                variant="destructive"
                onClick={() => setShowDeleteConfirm(true)}
                disabled={deleteMutation.isPending}
                className="w-full sm:w-auto"
              >
                Löschen
              </Button>
            )}
            <div className="flex gap-2 w-full sm:w-auto sm:ml-auto">
              <Button variant="outline" onClick={() => onOpenChange(false)} className="flex-1">
                Abbrechen
              </Button>
              <Button
                onClick={handleSubmit}
                disabled={!isFormValid || isPending}
                className="flex-1"
              >
                {isPending
                  ? "Wird gespeichert..."
                  : mode === "edit"
                    ? "Speichern"
                    : "Buchen"}
              </Button>
            </div>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation Dialog */}
      <AlertDialog open={showDeleteConfirm} onOpenChange={setShowDeleteConfirm}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Zeiteintrag löschen</AlertDialogTitle>
            <AlertDialogDescription>
              Möchten Sie diesen Zeiteintrag wirklich löschen? Diese Aktion kann nicht
              rückgängig gemacht werden.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Abbrechen</AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDelete}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              Löschen
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  )
}
