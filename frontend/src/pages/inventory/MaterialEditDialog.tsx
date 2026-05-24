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

function formatMoney(value: number | null): string {
  if (value == null) return ""
  return (value / 100).toFixed(2)
}

function parseMoney(value: string): number | null {
  const normalized = value.replace(",", ".").trim()
  if (!normalized) return null
  const parsed = Number(normalized)
  if (!Number.isFinite(parsed) || parsed < 0) return null
  return Math.round(parsed * 100)
}

function parseNonNegativeInteger(value: string): number | null {
  const trimmed = value.trim()
  if (!trimmed) return null
  const parsed = Number(trimmed)
  if (!Number.isInteger(parsed) || parsed < 0) return null
  return parsed
}

export function MaterialEditDialog({
  open,
  onOpenChange,
  material,
}: MaterialEditDialogProps) {
  const [location, setLocation] = useState("")
  const [minQuantity, setMinQuantity] = useState("")
  const [basePrice, setBasePrice] = useState("")
  const [priceMarkupPercentage, setPriceMarkupPercentage] = useState("")
  const [targetQuantity, setTargetQuantity] = useState("")

  const updateMaterial = useUpdateMaterial()
  const adjustMaterialStock = useAdjustMaterialStock()
  const parsedBasePrice = parseMoney(basePrice)
  const parsedPriceMarkupPercentage = parseNonNegativeInteger(priceMarkupPercentage)

  useEffect(() => {
    if (!open) {
      return
    }

    setLocation(material.location ?? "")
    setMinQuantity(String(material.min_quantity))
    setBasePrice(formatMoney(material.base_price_cents))
    setPriceMarkupPercentage(
      material.price_markup_percentage != null
        ? String(material.price_markup_percentage)
        : ""
    )
    setTargetQuantity(String(material.quantity))
  }, [material, open])

  const isSaving = updateMaterial.isPending || adjustMaterialStock.isPending
  const isSaveValid =
    (basePrice.trim() === "" || parsedBasePrice != null) &&
    (priceMarkupPercentage.trim() === "" || parsedPriceMarkupPercentage != null)

  const handleSubmit = async () => {
    const trimmedLocation = location.trim()
    const nextMinQuantity = Number(minQuantity)
    const nextBasePrice = parsedBasePrice
    const nextPriceMarkupPercentage = parsedPriceMarkupPercentage
    const nextTargetQuantity = Number(targetQuantity)

    try {
      await updateMaterial.mutateAsync({
        id: material.id,
        data: {
          min_quantity: nextMinQuantity,
          ...(trimmedLocation
            ? { location: trimmedLocation }
            : { clear_location: true }),
          ...(basePrice.trim()
            ? { base_price_cents: nextBasePrice }
            : { clear_base_price_cents: true }),
          ...(priceMarkupPercentage.trim()
            ? { price_markup_percentage: nextPriceMarkupPercentage }
            : { clear_price_markup_percentage: true }),
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

          <div className="space-y-2">
            <Label htmlFor="material-base-price">Basispreis (EUR)</Label>
            <Input
              id="material-base-price"
              type="number"
              min={0}
              step="0.01"
              inputMode="decimal"
              value={basePrice}
              onChange={(event) => setBasePrice(event.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="material-price-markup">Aufschlag (%)</Label>
            <Input
              id="material-price-markup"
              type="number"
              min={0}
              step="1"
              inputMode="numeric"
              value={priceMarkupPercentage}
              onChange={(event) => setPriceMarkupPercentage(event.target.value)}
            />
            <p className="text-sm text-muted-foreground">
              Wird standardmaessig fuer Kundenrechnungen dieses Materials verwendet.
            </p>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button onClick={handleSubmit} disabled={isSaving || !isSaveValid} className="shadow-sm active:scale-[0.97] transition-transform">
            {isSaving ? "Speichert..." : "Änderungen speichern"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
