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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { toast } from "sonner"
import { useCreateVehicle } from "@/lib/api/hooks"
import type { VehicleType } from "@/types/fleet"

interface AddVehicleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

const VEHICLE_TYPE_OPTIONS: { value: VehicleType; label: string }[] = [
  { value: "car", label: "PKW" },
  { value: "van", label: "Transporter" },
  { value: "truck", label: "LKW" },
  { value: "trailer", label: "Anhänger" },
  { value: "other", label: "Sonstige" },
]

export function AddVehicleDialog({
  open,
  onOpenChange,
}: AddVehicleDialogProps) {
  const [name, setName] = useState("")
  const [licensePlate, setLicensePlate] = useState("")
  const [vehicleType, setVehicleType] = useState<VehicleType | "">("")
  const [location, setLocation] = useState("")
  const [description, setDescription] = useState("")

  const createVehicle = useCreateVehicle()

  const resetForm = () => {
    setName("")
    setLicensePlate("")
    setVehicleType("")
    setLocation("")
    setDescription("")
  }

  const handleOpenChange = (open: boolean) => {
    if (!open) {
      resetForm()
    }
    onOpenChange(open)
  }

  const isFormValid = name && vehicleType

  const handleSubmit = () => {
    if (!isFormValid) return

    const payload: {
      name: string
      vehicle_type: VehicleType
      license_plate?: string
      location?: string
      description?: string
    } = {
      name,
      vehicle_type: vehicleType as VehicleType,
    }

    if (licensePlate) {
      payload.license_plate = licensePlate
    }
    if (location) {
      payload.location = location
    }
    if (description) {
      payload.description = description
    }

    createVehicle.mutate(payload, {
      onSuccess: () => {
        toast.success("Fahrzeug erstellt")
        handleOpenChange(false)
      },
      onError: (error) => {
        toast.error("Fahrzeug konnte nicht erstellt werden")
        console.error("Create vehicle error:", error)
      },
    })
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Fahrzeug hinzufügen</DialogTitle>
          <DialogDescription>
            Neues Fahrzeug zum Fuhrpark hinzufügen
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Name */}
          <div className="space-y-2">
            <Label htmlFor="name">Name *</Label>
            <Input
              id="name"
              placeholder="z.B. VW Transporter"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

          {/* License Plate */}
          <div className="space-y-2">
            <Label htmlFor="licensePlate">Kennzeichen</Label>
            <Input
              id="licensePlate"
              placeholder="z.B. B-AB 1234"
              value={licensePlate}
              onChange={(e) => setLicensePlate(e.target.value)}
            />
          </div>

          {/* Vehicle Type */}
          <div className="space-y-2">
            <Label htmlFor="vehicleType">Fahrzeugtyp *</Label>
            <Select
              value={vehicleType}
              onValueChange={(value) => setVehicleType(value as VehicleType)}
            >
              <SelectTrigger id="vehicleType">
                <SelectValue placeholder="Fahrzeugtyp wählen" />
              </SelectTrigger>
              <SelectContent>
                {VEHICLE_TYPE_OPTIONS.map((option) => (
                  <SelectItem key={option.value} value={option.value}>
                    {option.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* Location */}
          <div className="space-y-2">
            <Label htmlFor="location">Standort</Label>
            <Input
              id="location"
              placeholder="z.B. Hof 1"
              value={location}
              onChange={(e) => setLocation(e.target.value)}
            />
          </div>

          {/* Description */}
          <div className="space-y-2">
            <Label htmlFor="description">Beschreibung</Label>
            <Textarea
              id="description"
              placeholder="z.B. Baujah 2020, 7 Sitze"
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
            disabled={!isFormValid || createVehicle.isPending}
          >
            {createVehicle.isPending ? "Wird erstellt..." : "Erstellen"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
