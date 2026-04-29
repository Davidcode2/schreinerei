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
import { useCreateSite } from "@/lib/api/hooks"

interface AddSiteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function AddSiteDialog({
  open,
  onOpenChange,
}: AddSiteDialogProps) {
  const [name, setName] = useState("")
  const [customerName, setCustomerName] = useState("")
  const [location, setLocation] = useState("")
  const [description, setDescription] = useState("")

  const createSite = useCreateSite()

  const resetForm = () => {
    setName("")
    setCustomerName("")
    setLocation("")
    setDescription("")
  }

  const handleOpenChange = (open: boolean) => {
    if (!open) {
      resetForm()
    }
    onOpenChange(open)
  }

  const isFormValid = name && customerName

  const handleSubmit = () => {
    if (!isFormValid) return

    createSite.mutate(
      {
        name,
        customer_name: customerName,
        location: location || undefined,
        description: description || undefined,
      },
      {
        onSuccess: () => {
          toast.success("Baustelle erstellt")
          handleOpenChange(false)
        },
        onError: (error) => {
          toast.error("Baustelle konnte nicht erstellt werden")
          console.error("Create site error:", error)
        },
      }
    )
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Baustelle anlegen</DialogTitle>
          <DialogDescription>
            Neue Baustelle erstellen
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Name */}
          <div className="space-y-2">
            <Label htmlFor="name">Baustellenname *</Label>
            <Input
              id="name"
              placeholder="z.B. Villa Müller"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

          {/* Customer Name */}
          <div className="space-y-2">
            <Label htmlFor="customerName">Kunde *</Label>
            <Input
              id="customerName"
              placeholder="z.B. Familie Müller"
              value={customerName}
              onChange={(e) => setCustomerName(e.target.value)}
            />
          </div>

          {/* Location */}
          <div className="space-y-2">
            <Label htmlFor="location">Standort</Label>
            <Input
              id="location"
              placeholder="z.B. Musterstraße 1, 12345 Berlin"
              value={location}
              onChange={(e) => setLocation(e.target.value)}
            />
          </div>

          {/* Description */}
          <div className="space-y-2">
            <Label htmlFor="description">Beschreibung</Label>
            <Textarea
              id="description"
              placeholder="z.B. Küchenumbau, neues Treppenhaus"
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
            disabled={!isFormValid || createSite.isPending}
          >
            {createSite.isPending ? "Wird erstellt..." : "Erstellen"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
