import { useEffect, useState } from "react"
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
import { useAdjustMaterialStock, useUpdateMaterial } from "@/lib/api/hooks"
import type { Material } from "@/types/inventory"

const STOCK_CORRECTION_REASON = "Bestandskorrektur über Materialdialog"

interface MaterialEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  material: Material
}

export function MaterialEditDialog({
  open,
  onOpenChange,
  material,
}: MaterialEditDialogProps) {
  const [location, setLocation] = useState("")
  const [minQuantity, setMinQuantity] = useState("")
  const [targetQuantity, setTargetQuantity] = useState("")

  const updateMaterial = useUpdateMaterial()
  const adjustMaterialStock = useAdjustMaterialStock()

  useEffect(() => {
    if (!open) {
      return
    }

    setLocation(material.location ?? "")
    setMinQuantity(String(material.min_quantity))
    setTargetQuantity(String(material.quantity))
  }, [material, open])

  const isSaving = updateMaterial.isPending || adjustMaterialStock.isPending

  const handleSubmit = async () => {
    const trimmedLocation = location.trim()
    const nextMinQuantity = Number(minQuantity)
    const nextTargetQuantity = Number(targetQuantity)

    try {
      await updateMaterial.mutateAsync({
        id: material.id,
        data: {
          min_quantity: nextMinQuantity,
          ...(trimmedLocation
            ? { location: trimmedLocation }
            : { clear_location: true }),
        },
      })

      if (nextTargetQuantity !== material.quantity) {
        await adjustMaterialStock.mutateAsync({
          id: material.id,
          quantity: nextTargetQuantity - material.quantity,
          reason: STOCK_CORRECTION_REASON,
        })
      }

      toast.success("Material aktualisiert")
      onOpenChange(false)
    } catch {
      toast.error("Material konnte nicht aktualisiert werden")
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Material bearbeiten</DialogTitle>
          <DialogDescription>
            Lagerort, Mindestbestand und verfügbaren Bestand aktualisieren.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="space-y-2">
            <Label htmlFor="material-location">Lagerort</Label>
            <Input
              id="material-location"
              value={location}
              onChange={(event) => setLocation(event.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="material-min-quantity">Mindestbestand</Label>
            <Input
              id="material-min-quantity"
              type="number"
              min={0}
              step="1"
              value={minQuantity}
              onChange={(event) => setMinQuantity(event.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="material-target-quantity">Bestand korrigieren</Label>
            <Input
              id="material-target-quantity"
              type="number"
              min={0}
              step="1"
              value={targetQuantity}
              onChange={(event) => setTargetQuantity(event.target.value)}
            />
            <p className="text-sm text-muted-foreground">
              Setzt den verfügbaren Bestand direkt auf diesen Wert.
            </p>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button onClick={handleSubmit} disabled={isSaving} className="shadow-sm active:scale-[0.97] transition-transform">
            {isSaving ? "Speichert..." : "Änderungen speichern"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
