import { useEffect, useState } from "react"
import { Package, Plus } from "lucide-react"
import { Badge } from "@/components/ui/badge"
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
import type { Material } from "@/types/inventory"

interface StockInDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  material: Material
  onConfirm: (quantity: number, notes?: string) => void
  isLoading: boolean
}

export function StockInDialog({
  open,
  onOpenChange,
  material,
  onConfirm,
  isLoading,
}: StockInDialogProps) {
  const [quantity, setQuantity] = useState("1")
  const [notes, setNotes] = useState("")

  useEffect(() => {
    if (open) {
      setQuantity("1")
      setNotes("")
    }
  }, [open])

  const parsedQuantity = Math.max(1, Number(quantity) || 1)

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Material einlagern</DialogTitle>
          <DialogDescription>Neue Menge für {material.name} erfassen.</DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="flex items-center justify-between rounded-lg bg-muted p-3">
            <div className="flex items-center gap-2 text-sm">
              <Package className="h-4 w-4 text-muted-foreground" />
              <span>Aktueller Bestand</span>
            </div>
            <Badge variant="outline" className="font-mono">
              {material.quantity} {material.unit}
            </Badge>
          </div>

          <div className="space-y-2">
            <Label htmlFor="stock-in-quantity">Menge</Label>
            <Input
              id="stock-in-quantity"
              type="number"
              min={1}
              step="1"
              value={quantity}
              onChange={(event) => setQuantity(event.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="stock-in-notes">Notizen</Label>
            <Input
              id="stock-in-notes"
              placeholder="z. B. Lieferung von HolzLand, Lieferschein 1234"
              value={notes}
              onChange={(event) => setNotes(event.target.value)}
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button onClick={() => onConfirm(parsedQuantity, notes || undefined)} disabled={isLoading}>
            <Plus className="h-4 w-4" />
            {isLoading ? "Wird eingelagert..." : `${parsedQuantity} ${material.unit} einlagern`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
