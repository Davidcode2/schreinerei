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
import { StepContainer } from "@/components/ui/step-container"
import { StepIndicator } from "@/components/ui/step-indicator"
import { Car } from "lucide-react"
import { toast } from "sonner"
import { useCreateVehicle, useUpdateVehicle } from "@/lib/api/hooks"
import type { ResourceStatus, Vehicle, VehicleType } from "@/types/fleet"

interface AddVehicleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  mode?: "create" | "edit"
  initialData?: Vehicle
}

const VEHICLE_TYPE_OPTIONS: { value: VehicleType; label: string }[] = [
  { value: "car", label: "PKW" },
  { value: "van", label: "Transporter" },
  { value: "truck", label: "LKW" },
  { value: "trailer", label: "Anhänger" },
  { value: "other", label: "Sonstige" },
]

const RESOURCE_STATUS_OPTIONS: { value: ResourceStatus; label: string }[] = [
  { value: "available", label: "Verfügbar" },
  { value: "in_use", label: "In Benutzung" },
  { value: "maintenance", label: "Wartung" },
  { value: "reserved", label: "Reserviert" },
]

export function AddVehicleDialog({
  open,
  onOpenChange,
  mode = "create",
  initialData,
}: AddVehicleDialogProps) {
  const [name, setName] = useState(initialData?.name ?? "")
  const [licensePlate, setLicensePlate] = useState(initialData?.license_plate ?? "")
  const [vehicleType, setVehicleType] = useState<VehicleType | "">(initialData?.vehicle_type ?? "")
  const [location, setLocation] = useState(initialData?.location ?? "")
  const [description, setDescription] = useState(initialData?.description ?? "")
  const [status, setStatus] = useState<ResourceStatus>(initialData?.status ?? "available")
  const [qrCode, setQrCode] = useState(initialData?.qr_code ?? "")
  const [displayColor, setDisplayColor] = useState(initialData?.display_color ?? "#2563eb")
  const [currentStep, setCurrentStep] = useState(1)

  const createVehicle = useCreateVehicle()
  const updateVehicle = useUpdateVehicle()

  const resetForm = () => {
    if (mode === "edit" && initialData) {
      setName(initialData.name)
      setLicensePlate(initialData.license_plate ?? "")
      setVehicleType(initialData.vehicle_type)
      setLocation(initialData.location ?? "")
      setDescription(initialData.description ?? "")
      setStatus(initialData.status)
      setQrCode(initialData.qr_code ?? "")
      setDisplayColor(initialData.display_color)
      setCurrentStep(1)
      return
    }

    setName("")
    setLicensePlate("")
    setVehicleType("")
    setLocation("")
    setDescription("")
    setStatus("available")
    setQrCode("")
    setDisplayColor("#2563eb")
    setCurrentStep(1)
  }

  const handleOpenChange = (open: boolean) => {
    if (!open) {
      resetForm()
    }
    onOpenChange(open)
  }

  const isFormValid = name && vehicleType
  const isStep1Valid = Boolean(name && vehicleType)

  const handleSubmit = () => {
    if (!isFormValid) return

    const payload: {
      name: string
      vehicle_type: VehicleType
      license_plate?: string
      location?: string
      description?: string
      qr_code?: string
      display_color?: string
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
    if (qrCode) {
      payload.qr_code = qrCode
    }
    if (mode === "edit") {
      payload.display_color = displayColor.toLowerCase()
    }

    if (mode === "edit" && initialData) {
      updateVehicle.mutate(
        {
          id: initialData.id,
          ...payload,
          status,
        },
        {
          onSuccess: () => {
            toast.success("Fahrzeug aktualisiert")
            handleOpenChange(false)
          },
          onError: (error) => {
            toast.error("Fahrzeug konnte nicht aktualisiert werden")
            console.error("Update vehicle error:", error)
          },
        }
      )
      return
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

  const isPending = createVehicle.isPending || updateVehicle.isPending

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="flex max-h-[90vh] flex-col overflow-hidden sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent">
              <Car className="h-4 w-4" />
            </div>
            {mode === "edit" ? "Fahrzeug bearbeiten" : "Fahrzeug hinzufügen"}
          </DialogTitle>
          <DialogDescription>
            {mode === "edit"
              ? "Fahrzeugdaten anpassen"
              : "Neues Fahrzeug zum Fuhrpark hinzufügen"}
          </DialogDescription>
        </DialogHeader>

        <StepIndicator
          currentStep={currentStep}
          totalSteps={2}
          onStepClick={(step) => {
            if (step === 1 || isStep1Valid) {
              setCurrentStep(step)
            }
          }}
        />

        <StepContainer
          currentStep={currentStep}
          onStepChange={(step) => {
            if (step === 1 || isStep1Valid) {
              setCurrentStep(step)
            }
          }}
          totalSteps={2}
          className="min-h-0 flex-1"
        >
          <div className="flex h-full flex-col overflow-y-auto py-4 pr-1">
            {currentStep === 1 ? (
              <div className="space-y-4">
                <div className="space-y-1">
                  <p className="text-sm font-medium">Basisdaten</p>
                  <p className="text-sm text-muted-foreground">
                    Geben Sie die wichtigsten Fahrzeugdaten an.
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="name">Name *</Label>
                  <Input
                    id="name"
                    placeholder="z.B. VW Transporter"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    className="h-10"
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="licensePlate">Kennzeichen</Label>
                  <Input
                    id="licensePlate"
                    placeholder="z.B. B-AB 1234"
                    value={licensePlate}
                    onChange={(e) => setLicensePlate(e.target.value)}
                    className="h-10"
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="vehicleType">Fahrzeugtyp *</Label>
                  <Select
                    value={vehicleType}
                    onValueChange={(value) => setVehicleType(value as VehicleType)}
                  >
                    <SelectTrigger id="vehicleType" className="h-10">
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
              </div>
            ) : (
              <div className="space-y-4">
                <div className="space-y-1">
                  <p className="text-sm font-medium">Details</p>
                  <p className="text-sm text-muted-foreground">
                    Zusätzliche Angaben helfen bei Verwaltung und Zuordnung.
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="location">Standort</Label>
                  <Input
                    id="location"
                    placeholder="z.B. Hof 1"
                    value={location}
                    onChange={(e) => setLocation(e.target.value)}
                    className="h-10"
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="qrCode">QR-Code</Label>
                  <Input
                    id="qrCode"
                    placeholder="z.B. fleet-van-01"
                    value={qrCode}
                    onChange={(e) => setQrCode(e.target.value)}
                    className="h-10"
                  />
                </div>

                {mode === "edit" && (
                  <div className="grid gap-4 sm:grid-cols-[1fr_auto]">
                    <div className="space-y-2">
                      <Label htmlFor="vehicleStatus">Status</Label>
                      <Select
                        value={status}
                        onValueChange={(value) => setStatus(value as ResourceStatus)}
                      >
                        <SelectTrigger id="vehicleStatus" className="h-10">
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

                    <div className="space-y-2">
                      <Label htmlFor="displayColor">Kalenderfarbe</Label>
                      <Input
                        id="displayColor"
                        type="color"
                        value={displayColor}
                        onChange={(event) => setDisplayColor(event.target.value)}
                        className="h-10 w-full min-w-16 cursor-pointer p-1 sm:w-16"
                      />
                    </div>
                  </div>
                )}

                <div className="space-y-2">
                  <Label htmlFor="description">Beschreibung</Label>
                  <Textarea
                    id="description"
                    placeholder="z.B. Baujahr 2020, 7 Sitze"
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    rows={3}
                  />
                </div>
              </div>
            )}
          </div>
        </StepContainer>

        <DialogFooter>
          {currentStep === 1 ? (
            <>
              <Button variant="outline" onClick={() => handleOpenChange(false)} className="shadow-sm">
                Abbrechen
              </Button>
              <Button onClick={() => setCurrentStep(2)} disabled={!isStep1Valid} className="shadow-sm">
                Weiter
              </Button>
            </>
          ) : (
            <>
              <Button variant="outline" onClick={() => setCurrentStep(1)} className="shadow-sm">
                Zurück
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
            </>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
