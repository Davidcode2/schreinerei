import { useState } from "react"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Clock } from "lucide-react"
import { useCreateTimeEntry } from "@/lib/api/hooks"
import { toast } from "sonner"
import type { WorkType } from "@/types/sites"

interface TimeEntryDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  siteId?: string
  siteName?: string
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
}: TimeEntryDialogProps) {
  const [workType, setWorkType] = useState<WorkType>("site")
  const [hours, setHours] = useState(0.5)
  const [workDate, setWorkDate] = useState(new Date().toISOString().split("T")[0] ?? "")
  const [notes, setNotes] = useState("")
  const [errors, setErrors] = useState<Record<string, string>>({})
  const [touched, setTouched] = useState<Record<string, boolean>>({})

  const createMutation = useCreateTimeEntry()

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
      await createMutation.mutateAsync({
        ...(workType === "site" && siteId ? { site_id: siteId } : {}),
        work_type: workType,
        hours,
        work_date: workDate,
        ...(notes ? { notes } : {}),
      })
      toast.success("Zeit erfasst")
      onOpenChange(false)
      // Reset form
      setWorkType("site")
      setHours(0.5)
      setNotes("")
      setErrors({})
      setTouched({})
    } catch {
      toast.error("Zeiterfassung fehlgeschlagen")
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Clock className="h-5 w-5" />
            Zeit buchen
          </DialogTitle>
          <DialogDescription>
            {siteName || "Neuen Zeiteintrag erstellen"}
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

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button onClick={handleSubmit} disabled={!isFormValid || createMutation.isPending}>
            {createMutation.isPending ? "Wird gespeichert..." : "Speichern"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
