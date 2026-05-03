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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { Wrench } from "lucide-react"
import { toast } from "sonner"
import { useCreateTool, useUpdateTool } from "@/lib/api/hooks"
import type { ResourceStatus, Tool } from "@/types/fleet"

interface AddToolDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  mode?: "create" | "edit"
  initialData?: Tool
}

const RESOURCE_STATUS_OPTIONS: { value: ResourceStatus; label: string }[] = [
  { value: "available", label: "Verfügbar" },
  { value: "in_use", label: "In Benutzung" },
  { value: "maintenance", label: "Wartung" },
  { value: "reserved", label: "Reserviert" },
]

export function AddToolDialog({
  open,
  onOpenChange,
  mode = "create",
  initialData,
}: AddToolDialogProps) {
  const [name, setName] = useState(initialData?.name ?? "")
  const [category, setCategory] = useState(initialData?.category ?? "")
  const [location, setLocation] = useState(initialData?.location ?? "")
  const [description, setDescription] = useState(initialData?.description ?? "")
  const [status, setStatus] = useState<ResourceStatus>(initialData?.status ?? "available")
  const [qrCode, setQrCode] = useState(initialData?.qr_code ?? "")

  const createTool = useCreateTool()
  const updateTool = useUpdateTool()

  const resetForm = () => {
    if (mode === "edit" && initialData) {
      setName(initialData.name)
      setCategory(initialData.category ?? "")
      setLocation(initialData.location ?? "")
      setDescription(initialData.description ?? "")
      setStatus(initialData.status)
      setQrCode(initialData.qr_code ?? "")
      return
    }

    setName("")
    setCategory("")
    setLocation("")
    setDescription("")
    setStatus("available")
    setQrCode("")
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
      qr_code?: string
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
    if (qrCode) {
      payload.qr_code = qrCode
    }

    if (mode === "edit" && initialData) {
      updateTool.mutate(
        {
          id: initialData.id,
          ...payload,
          status,
        },
        {
          onSuccess: () => {
            toast.success("Werkzeug aktualisiert")
            handleOpenChange(false)
          },
          onError: (error) => {
            toast.error("Werkzeug konnte nicht aktualisiert werden")
            console.error("Update tool error:", error)
          },
        }
      )
      return
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

  const isPending = createTool.isPending || updateTool.isPending

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent">
              <Wrench className="h-4 w-4" />
            </div>
            {mode === "edit" ? "Werkzeug bearbeiten" : "Werkzeug hinzufügen"}
          </DialogTitle>
          <DialogDescription>
            {mode === "edit"
              ? "Werkzeugdaten anpassen"
              : "Neues Werkzeug zum Inventar hinzufügen"}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="space-y-2">
            <Label htmlFor="name">Name *</Label>
            <Input
              id="name"
              placeholder="z.B. Bohrhammer"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="h-10"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="category">Kategorie</Label>
            <Input
              id="category"
              placeholder="z.B. Elektrowerkzeug"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              className="h-10"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="location">Standort</Label>
            <Input
              id="location"
              placeholder="z.B. Werkstatt"
              value={location}
              onChange={(e) => setLocation(e.target.value)}
              className="h-10"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="qrCode">QR-Code</Label>
            <Input
              id="qrCode"
              placeholder="z.B. tool-drill-01"
              value={qrCode}
              onChange={(e) => setQrCode(e.target.value)}
              className="h-10"
            />
          </div>

          {mode === "edit" && (
            <div className="space-y-2">
              <Label htmlFor="toolStatus">Status</Label>
              <Select
                value={status}
                onValueChange={(value) => setStatus(value as ResourceStatus)}
              >
                <SelectTrigger id="toolStatus" className="h-10">
                  <SelectValue placeholder="Status wählen" />
                </SelectTrigger>
                <SelectContent>
                  {RESOURCE_STATUS_OPTIONS.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          )}

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
          <Button variant="outline" onClick={() => handleOpenChange(false)} className="shadow-sm">
            Abbrechen
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={!isFormValid || isPending}
            className="shadow-sm"
          >
            {isPending
              ? mode === "edit"
                ? "Wird gespeichert..."
                : "Wird erstellt..."
              : mode === "edit"
                ? "Speichern"
                : "Erstellen"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
