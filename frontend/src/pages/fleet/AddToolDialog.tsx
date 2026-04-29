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
import { Textarea } from "@/components/ui/textarea"
import { toast } from "sonner"
import { useCreateTool } from "@/lib/api/hooks"

interface AddToolDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function AddToolDialog({
  open,
  onOpenChange,
}: AddToolDialogProps) {
  const [name, setName] = useState("")
  const [category, setCategory] = useState("")
  const [location, setLocation] = useState("")
  const [description, setDescription] = useState("")

  const createTool = useCreateTool()

  const resetForm = () => {
    setName("")
    setCategory("")
    setLocation("")
    setDescription("")
  }

  const handleOpenChange = (open: boolean) => {
    if (!open) {
      resetForm()
    }
    onOpenChange(open)
  }

  const isFormValid = name

  const handleSubmit = () => {
    if (!isFormValid) return

    const payload: {
      name: string
      category?: string
      location?: string
      description?: string
    } = {
      name,
    }

    if (category) {
      payload.category = category
    }
    if (location) {
      payload.location = location
    }
    if (description) {
      payload.description = description
    }

    createTool.mutate(payload, {
      onSuccess: () => {
        toast.success("Werkzeug erstellt")
        handleOpenChange(false)
      },
      onError: (error) => {
        toast.error("Werkzeug konnte nicht erstellt werden")
        console.error("Create tool error:", error)
      },
    })
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Werkzeug hinzufügen</DialogTitle>
          <DialogDescription>
            Neues Werkzeug zum Inventar hinzufügen
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Name */}
          <div className="space-y-2">
            <Label htmlFor="name">Name *</Label>
            <Input
              id="name"
              placeholder="z.B. Bohrhammer"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

          {/* Category */}
          <div className="space-y-2">
            <Label htmlFor="category">Kategorie</Label>
            <Input
              id="category"
              placeholder="z.B. Elektrowerkzeug"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
            />
          </div>

          {/* Location */}
          <div className="space-y-2">
            <Label htmlFor="location">Standort</Label>
            <Input
              id="location"
              placeholder="z.B. Werkstatt"
              value={location}
              onChange={(e) => setLocation(e.target.value)}
            />
          </div>

          {/* Description */}
          <div className="space-y-2">
            <Label htmlFor="description">Beschreibung</Label>
            <Textarea
              id="description"
              placeholder="z.B. Bosch GBH 2-21, inkl. Meißel"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={3}
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => handleOpenChange(false)}>
            Abbrechen
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={!isFormValid || createTool.isPending}
          >
            {createTool.isPending ? "Wird erstellt..." : "Erstellen"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
