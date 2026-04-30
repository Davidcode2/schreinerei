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

interface WithdrawDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  material: Material
  onConfirm: (quantity: number, notes?: string, siteId?: string | null) => void
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

  const handleQuantityChange = (value: number) => {
    const newQuantity = Math.max(1, Math.min(value, material.quantity))
    setQuantity(newQuantity)
  }

  const handleSubmit = () => {
    onConfirm(quantity, notes || undefined, siteId || null)
    setQuantity(1)
    setNotes("")
  }

  const quickAmounts = [1, 2, 5, 10].filter((n) => n <= material.quantity)

  useEffect(() => {
    if (open) {
      setSiteId(initialSiteId ?? "")
    }
  }, [open, initialSiteId])

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Material entnehmen</DialogTitle>
          <DialogDescription>
            Entnahme von {material.name}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Current Stock */}
          <div className="flex items-center justify-between p-3 bg-muted rounded-lg">
            <div className="flex items-center gap-2">
              <Package className="h-4 w-4 text-muted-foreground" />
              <span className="text-sm">Verfügbar</span>
            </div>
            <Badge variant="outline" className="font-mono">
              {material.quantity} {material.unit}
            </Badge>
          </div>

          {/* Quantity Selector */}
          <div className="space-y-2">
            <Label>Menge</Label>
            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="icon"
                onClick={() => handleQuantityChange(quantity - 1)}
                disabled={quantity <= 1}
              >
                <Minus className="h-4 w-4" />
              </Button>
              <Input
                type="number"
                value={quantity}
                onChange={(e) => handleQuantityChange(parseInt(e.target.value) || 1)}
                className="text-center font-mono"
                min={1}
                max={material.quantity}
              />
              <Button
                variant="outline"
                size="icon"
                onClick={() => handleQuantityChange(quantity + 1)}
                disabled={quantity >= material.quantity}
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
                    className="min-w-[48px]"
                  >
                    {amount}
                  </Button>
                ))}
              </div>
            )}
          </div>

          <div className="space-y-2">
            <Label htmlFor="site">Baustelle (optional)</Label>
            <select
              id="site"
              className="w-full h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
              value={siteId}
              onChange={(event) => setSiteId(event.target.value)}
            >
              <option value="">Keine Zuordnung</option>
              {sites.map((site) => (
                <option key={site.id} value={site.id}>
                  {site.name}
                </option>
              ))}
            </select>
          </div>

          {/* Notes */}
          <div className="space-y-2">
            <Label htmlFor="notes">Notiz (optional)</Label>
            <Input
              id="notes"
              placeholder="z.B. Baustelle Müller, Auftrag #123"
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Abbrechen
          </Button>
          <Button onClick={handleSubmit} disabled={isLoading}>
            {isLoading ? "Wird entnommen..." : `${quantity} ${material.unit} entnehmen`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
