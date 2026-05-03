import { useEffect, useState } from "react"
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

interface CategoryDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  title: string
  description: string
  submitLabel: string
  isSubmitting: boolean
  initialValues?: {
    name: string
    description: string
    canExpire: boolean
  }
  onSubmit: (values: { name: string; description: string; canExpire: boolean }) => void
}

const defaultValues = {
  name: "",
  description: "",
  canExpire: false,
}

export function CategoryDialog({
  open,
  onOpenChange,
  title,
  description,
  submitLabel,
  isSubmitting,
  initialValues,
  onSubmit,
}: CategoryDialogProps) {
  const [name, setName] = useState(defaultValues.name)
  const [details, setDetails] = useState(defaultValues.description)
  const [canExpire, setCanExpire] = useState(defaultValues.canExpire)

  useEffect(() => {
    if (!open) {
      return
    }

    const values = initialValues ?? defaultValues
    setName(values.name)
    setDetails(values.description)
    setCanExpire(values.canExpire)
  }, [initialValues, open])

  const isSubmitDisabled = name.trim().length === 0 || isSubmitting

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{description}</DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="space-y-2">
            <Label htmlFor="category-name">Name</Label>
            <Input
              id="category-name"
              value={name}
              onChange={(event) => setName(event.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="category-description">Beschreibung</Label>
            <Textarea
              id="category-description"
              rows={4}
              value={details}
              onChange={(event) => setDetails(event.target.value)}
            />
          </div>

          <label className="flex items-center gap-3 rounded-lg border border-border/60 p-3">
            <input
              id="category-can-expire"
              type="checkbox"
              checked={canExpire}
              onChange={(event) => setCanExpire(event.target.checked)}
              className="h-4 w-4 rounded border-input"
            />
            <span className="text-sm font-medium" title="Mindesthaltbarkeitsdatum">
              MHD
            </span>
          </label>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button
            onClick={() =>
              onSubmit({
                name: name.trim(),
                description: details.trim(),
                canExpire,
              })
            }
            disabled={isSubmitDisabled}
          >
            {isSubmitting ? "Speichert..." : submitLabel}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
