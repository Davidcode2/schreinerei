import { useEffect, useState } from "react"
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
import { Badge } from "@/components/ui/badge"
import { Package, Minus, Plus } from "lucide-react"
import type { Material } from "@/types/inventory"

function formatExpiryDate(date: string) {
  const [year, month, day] = date.split("-")
  return `${day}.${month}.${year}`
}

interface WithdrawDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  material: Material
  onConfirm: (
    quantity: number,
    notes?: string,
    siteId?: string | null,
    disposal?: boolean
  ) => void
  isLoading: boolean
  sites: Array<{ id: string; name: string }>
  initialSiteId?: string | null
}

export function WithdrawDialog({
  open,
  onOpenChange,
  material,
  onConfirm,
  isLoading,
  sites,
  initialSiteId,
}: WithdrawDialogProps) {
  const [quantity, setQuantity] = useState(1)
  const [notes, setNotes] = useState("")
  const [siteId, setSiteId] = useState(initialSiteId ?? "")
  const [disposal, setDisposal] = useState(false)

  const maxQuantity = disposal ? Math.max(1, material.expired_quantity) : material.quantity
  const requiresProjectLink = !disposal
  const canSubmit = quantity > 0 && (!requiresProjectLink || Boolean(siteId))

  const handleQuantityChange = (value: number) => {
    const newQuantity = Math.max(1, Math.min(value, maxQuantity))
    setQuantity(newQuantity)
  }

  const handleSubmit = () => {
    if (!canSubmit) {
      return
    }

    onConfirm(quantity, notes || undefined, disposal ? null : siteId || null, disposal)
    setQuantity(1)
    setNotes("")
    setDisposal(false)
  }

  const quickAmounts = [1, 2, 5, 10].filter((n) => n <= maxQuantity)

  useEffect(() => {
    if (open) {
      setSiteId(initialSiteId ?? "")
      setDisposal(false)
    }
  }, [open, initialSiteId])

  useEffect(() => {
    setQuantity((current) => Math.max(1, Math.min(current, maxQuantity)))
  }, [maxQuantity])

  const canDispose = material.expired_quantity > 0
  const oldestExpiryLabel = material.next_expiry_on ? formatExpiryDate(material.next_expiry_on) : null
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[90vh] flex-col overflow-hidden sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Material entnehmen</DialogTitle>
          <DialogDescription>
            Entnahme von {material.name}
          </DialogDescription>
        </DialogHeader>

        <div className="min-h-0 space-y-4 overflow-y-auto py-4 pr-1">
          {/* Current Stock */}
          <div className="flex items-center justify-between rounded-xl border border-border/70 bg-card/70 p-4 shadow-sm">
            <div className="flex items-center gap-2.5">
              <div className="flex h-7 w-7 items-center justify-center rounded-md bg-accent">
                <Package className="h-3.5 w-3.5 text-muted-foreground" />
              </div>
              <span className="text-sm">Verfügbar</span>
            </div>
            <Badge variant="outline" className="font-mono text-xs">
              {material.quantity} {material.unit}
            </Badge>
          </div>

          {material.can_expire && (
            <div className="space-y-3 rounded-xl border border-border/70 bg-card/70 p-4 shadow-sm text-sm">
              {oldestExpiryLabel && (
                <p>
                  Ältestes MHD: <span className="font-medium">{oldestExpiryLabel}</span>
                </p>
              )}
              {material.expired_quantity > 0 && (
                <p className="text-destructive">{material.expired_quantity} Stück abgelaufen</p>
              )}
              {material.expiring_soon_quantity > 0 && (
                <p className="text-warning-foreground">
                  {material.expiring_soon_quantity} Stück laufen in den nächsten 10 Tagen ab
                </p>
              )}
              {material.legacy_quantity > 0 && (
                <p className="text-muted-foreground">
                  Ohne MHD erfasster Bestand: {material.legacy_quantity} {material.unit}
                </p>
              )}
              {material.expiry_batches.length > 0 && (
                <div className="space-y-2">
                  <p className="font-medium">Erfasste MHD-Chargen</p>
                  <div className="max-h-40 space-y-1 overflow-y-auto pr-1">
                    {material.expiry_batches.map((batch) => (
                      <div key={`${batch.expires_on}-${batch.quantity}`} className="flex justify-between">
                        <span>{formatExpiryDate(batch.expires_on)}</span>
                        <span className="text-muted-foreground">{batch.quantity} {material.unit}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}

          {canDispose && (
          <label className="flex items-center gap-3 rounded-xl border border-destructive/30 bg-destructive/5 p-4 text-sm">
            <input
              type="checkbox"
              checked={disposal}
              onChange={(event) => setDisposal(event.target.checked)}
              className="h-4 w-4 rounded border-input"
            />
            <span>Entsorgung</span>
          </label>
          )}

          {/* Quantity Selector */}
          <div className="space-y-3 rounded-xl border border-border/70 bg-card/70 p-4 shadow-sm">
            <Label>Menge</Label>
            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="icon"
                onClick={() => handleQuantityChange(quantity - 1)}
                disabled={quantity <= 1}
                className="h-11 w-11 shrink-0"
              >
                <Minus className="h-4 w-4" />
              </Button>
              <Input
                type="number"
                value={quantity}
                onChange={(e) => handleQuantityChange(parseInt(e.target.value) || 1)}
                className="h-11 text-center font-mono tabular-nums"
                min={1}
                max={maxQuantity}
              />
              <Button
                variant="outline"
                size="icon"
                onClick={() => handleQuantityChange(quantity + 1)}
                disabled={quantity >= maxQuantity}
                className="h-11 w-11 shrink-0"
              >
                <Plus className="h-4 w-4" />
              </Button>
            </div>

            {/* Quick Amount Buttons */}
            {quickAmounts.length > 0 && (
              <div className="flex flex-wrap gap-2">
                {quickAmounts.map((amount) => (
                  <Button
                    key={amount}
                    variant={quantity === amount ? "default" : "outline"}
                    size="sm"
                    onClick={() => setQuantity(amount)}
                    className="min-w-[48px] shadow-sm"
                  >
                    {amount}
                  </Button>
                ))}
              </div>
            )}
          </div>

          {!disposal && (
            <div className="space-y-2 rounded-xl border border-border/70 bg-card/70 p-4 shadow-sm">
                <Label htmlFor="site">Projekt</Label>
                <select
                  id="site"
                  className="h-11 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                  value={siteId}
                  onChange={(event) => setSiteId(event.target.value)}
                >
                <option value="">Projekt auswählen</option>
                {sites.map((site) => (
                  <option key={site.id} value={site.id}>
                    {site.name}
                  </option>
                ))}
              </select>
              <p className="text-xs text-muted-foreground">
                Reale Materialentnahmen werden immer einem Projekt zugeordnet.
              </p>
            </div>
          )}

          {/* Notes */}
          <div className="space-y-2 rounded-xl border border-border/70 bg-card/70 p-4 shadow-sm">
            <Label htmlFor="notes">Notiz (optional)</Label>
            <Input
              id="notes"
              placeholder="z.B. Baustelle Müller, Auftrag #123"
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              className="h-11"
            />
          </div>
        </div>

        <DialogFooter className="border-t border-border/70 pt-4">
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button onClick={handleSubmit} disabled={isLoading || !canSubmit} className="gap-2 shadow-sm active:scale-[0.97] transition-transform">
            {isLoading
              ? disposal
                ? "Wird entsorgt..."
                : "Wird entnommen..."
              : disposal
                ? `${quantity} ${material.unit} entsorgen`
                : `${quantity} ${material.unit} entnehmen`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
