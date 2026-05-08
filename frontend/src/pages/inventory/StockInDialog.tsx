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
  onConfirm: (
    quantity: number,
    notes?: string,
    expiresOn?: string,
    batchCode?: string,
    supplierName?: string,
    receiptReference?: string,
    receiptDate?: string
  ) => void
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
  const [expiresOn, setExpiresOn] = useState("")
  const [batchCode, setBatchCode] = useState("")
  const [supplierName, setSupplierName] = useState("")
  const [receiptReference, setReceiptReference] = useState("")
  const [receiptDate, setReceiptDate] = useState("")

  useEffect(() => {
    if (open) {
      setQuantity("1")
      setNotes("")
      setExpiresOn("")
      setBatchCode("")
      setSupplierName("")
      setReceiptReference("")
      setReceiptDate("")
    }
  }, [open])

  const parsedQuantity = Math.max(1, Number(quantity) || 1)
  const isSubmitDisabled = isLoading || (material.can_expire && !expiresOn)

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Material einlagern</DialogTitle>
          <DialogDescription>Neue Menge für {material.name} erfassen.</DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="flex items-center justify-between rounded-lg bg-accent/50 p-3">
            <div className="flex items-center gap-2.5 text-sm">
              <div className="flex h-7 w-7 items-center justify-center rounded-md bg-accent">
                <Package className="h-3.5 w-3.5 text-muted-foreground" />
              </div>
              <span>Aktueller Bestand</span>
            </div>
            <Badge variant="outline" className="font-mono text-xs">
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

          {material.can_expire && (
            <div className="space-y-4 rounded-xl border border-border/70 bg-card/70 p-4 shadow-sm">
              <div className="space-y-2">
                <Label htmlFor="stock-in-expires-on" title="Mindesthaltbarkeitsdatum">
                  MHD
                </Label>
                <Input
                  id="stock-in-expires-on"
                  type="date"
                  value={expiresOn}
                  onChange={(event) => setExpiresOn(event.target.value)}
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="stock-in-batch-code">Charge / Los</Label>
                <Input
                  id="stock-in-batch-code"
                  placeholder="optional, z. B. LOT-2026-05"
                  value={batchCode}
                  onChange={(event) => setBatchCode(event.target.value)}
                />
              </div>
            </div>
          )}

          <div className="space-y-4 rounded-xl border border-border/70 bg-card/70 p-4 shadow-sm">
            <p className="text-sm font-medium">Wareneingang</p>

            <div className="space-y-2">
              <Label htmlFor="stock-in-supplier">Lieferant</Label>
              <Input
                id="stock-in-supplier"
                placeholder="optional, z. B. HolzLand"
                value={supplierName}
                onChange={(event) => setSupplierName(event.target.value)}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="stock-in-reference">Belegnummer</Label>
              <Input
                id="stock-in-reference"
                placeholder="optional, z. B. LS-1234"
                value={receiptReference}
                onChange={(event) => setReceiptReference(event.target.value)}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="stock-in-receipt-date">Belegdatum</Label>
              <Input
                id="stock-in-receipt-date"
                type="date"
                value={receiptDate}
                onChange={(event) => setReceiptDate(event.target.value)}
              />
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button
            onClick={() =>
              onConfirm(
                parsedQuantity,
                notes || undefined,
                expiresOn || undefined,
                batchCode || undefined,
                supplierName || undefined,
                receiptReference || undefined,
                receiptDate || undefined
              )
            }
            disabled={isSubmitDisabled}
            className="gap-2 shadow-sm active:scale-[0.97] transition-transform"
          >
            <Plus className="h-4 w-4" />
            {isLoading ? "Wird eingelagert..." : `${parsedQuantity} ${material.unit} einlagern`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
