import { useEffect, useMemo, useState } from "react"
import { Euro, FileText } from "lucide-react"
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { useBillingSettings } from "@/lib/api/hooks"
import type {
  CreateProjectInvoiceRequest,
  InvoicePricingMode,
  ProjectMaterialUsageLine,
  Site,
} from "@/types/sites"

type PricingModeValue = InvoicePricingMode | "none"

interface CreateInvoiceDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  site: Site
  totalHours: number
  materialLines: ProjectMaterialUsageLine[]
  isPending: boolean
  onSubmit: (data: CreateProjectInvoiceRequest) => Promise<void>
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

function resolvedMaterialUnitPriceCents(
  basePriceCents: number | null,
  priceMarkupPercentage: number | null
): number | null {
  if (basePriceCents == null) return null
  const markup = priceMarkupPercentage ?? 0
  return Math.round((basePriceCents * (100 + markup)) / 100)
}

function getInitialPricingMode(site: Site): PricingModeValue {
  if (site.invoice_pricing_mode) return site.invoice_pricing_mode
  if (site.fixed_price_cents != null) return "fixed_price"
  if (site.hourly_rate_cents != null) return "hourly_rate"
  return "none"
}

export function CreateInvoiceDialog({
  open,
  onOpenChange,
  site,
  totalHours,
  materialLines,
  isPending,
  onSubmit,
}: CreateInvoiceDialogProps) {
  const { data: billingSettings } = useBillingSettings()
  const [senderName, setSenderName] = useState("")
  const [senderAddress, setSenderAddress] = useState("")
  const [pricingMode, setPricingMode] = useState<PricingModeValue>(getInitialPricingMode(site))
  const [hourlyRate, setHourlyRate] = useState(formatMoney(site.hourly_rate_cents))
  const [fixedPrice, setFixedPrice] = useState(formatMoney(site.fixed_price_cents))
  const [materialMarkups, setMaterialMarkups] = useState<Record<string, string>>({})

  useEffect(() => {
    if (!open) return
    setSenderName(billingSettings?.sender_name ?? "")
    setSenderAddress(billingSettings?.sender_address ?? "")
    setPricingMode(getInitialPricingMode(site))
    setHourlyRate(formatMoney(site.hourly_rate_cents))
    setFixedPrice(formatMoney(site.fixed_price_cents))
    setMaterialMarkups(
      Object.fromEntries(
        materialLines.map((line) => [
          line.material_id,
          String(line.price_markup_percentage ?? 0),
        ])
      )
    )
  }, [open, site, materialLines, billingSettings?.sender_name, billingSettings?.sender_address])

  const parsedHourlyRate = parseMoney(hourlyRate)
  const parsedFixedPrice = parseMoney(fixedPrice)
  const pricingIsValid =
    pricingMode === "none" ||
    (pricingMode === "hourly_rate" && parsedHourlyRate != null) ||
    (pricingMode === "fixed_price" && parsedFixedPrice != null)
  const materialPricingIsValid = useMemo(
    () =>
      materialLines.every((line) => {
        if (line.base_price_cents == null) {
          return true
        }
        return parseNonNegativeInteger(materialMarkups[line.material_id] ?? "0") != null
      }),
    [materialLines, materialMarkups]
  )

  async function handleSubmit() {
    if (!pricingIsValid) return

    const payload: CreateProjectInvoiceRequest = {
      sender_name: senderName.trim() || null,
      sender_address: senderAddress.trim() || null,
      invoice_pricing_mode: pricingMode === "none" ? null : pricingMode,
      hourly_rate_cents: pricingMode === "hourly_rate" ? parsedHourlyRate : null,
      fixed_price_cents: pricingMode === "fixed_price" ? parsedFixedPrice : null,
      material_overrides: materialLines
        .filter((line) => line.base_price_cents != null)
        .map((line) => ({
          material_id: line.material_id,
          price_markup_percentage:
            parseNonNegativeInteger(materialMarkups[line.material_id] ?? "0") ?? 0,
        })),
    }

    await onSubmit(payload)
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[90vh] flex-col overflow-hidden sm:max-w-lg">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2.5 font-display">
            <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
              <FileText className="h-4 w-4 text-muted-foreground" />
            </div>
            Rechnung erstellen
          </DialogTitle>
          <DialogDescription>
            Abrechnungsdaten fuer diese Rechnung pruefen oder einmalig ueberschreiben.
          </DialogDescription>
        </DialogHeader>

        <div className="min-h-0 space-y-5 overflow-y-auto py-4 pr-1">
          <div className="rounded-lg border border-border/70 bg-card/70 p-4 text-sm">
            <p className="font-medium">Projektbasis</p>
            <p className="mt-1 text-muted-foreground">Gebuchte Stunden: {totalHours.toFixed(1)}h</p>
            <p className="text-muted-foreground">Projekt: {site.name}</p>
            <p className="text-muted-foreground">
              Umsatzsteuer: {billingSettings?.billing_tax_mode === "kleinunternehmer" ? "Kleinunternehmer (§ 19 UStG)" : "19% MwSt"}
            </p>
          </div>

          <div className="space-y-2">
            <Label htmlFor="invoice-sender-name">Absender</Label>
            <Input id="invoice-sender-name" value={senderName} onChange={(event) => setSenderName(event.target.value)} placeholder="Optional, z.B. Schreinerei Mustermann" className="h-10" />
          </div>

          <div className="space-y-2">
            <Label htmlFor="invoice-sender-address">Absenderadresse</Label>
            <Input id="invoice-sender-address" value={senderAddress} onChange={(event) => setSenderAddress(event.target.value)} placeholder="Optional, z.B. Werkstrasse 1" className="h-10" />
          </div>

          <div className="space-y-4 rounded-lg border border-border/70 bg-card/70 p-4 shadow-sm">
            <div className="flex items-center gap-2 text-sm font-medium">
              <Euro className="h-4 w-4 text-muted-foreground" />
              Einmalige Abrechnungslogik
            </div>

            <div className="space-y-2">
              <Label htmlFor="invoice-pricing-mode">Rechnungslogik</Label>
              <Select value={pricingMode} onValueChange={(value) => setPricingMode(value as PricingModeValue)}>
                <SelectTrigger id="invoice-pricing-mode" className="h-10">
                  <SelectValue placeholder="Rechnungslogik wählen" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="none">Projektvorgabe verwenden</SelectItem>
                  <SelectItem value="hourly_rate">Stundensatz</SelectItem>
                  <SelectItem value="fixed_price">Pauschalpreis</SelectItem>
                </SelectContent>
              </Select>
            </div>

            {pricingMode === "hourly_rate" && (
              <div className="space-y-2">
                <Label htmlFor="invoice-hourly-rate">Stundensatz (EUR)</Label>
                <Input id="invoice-hourly-rate" type="number" min="0" step="0.01" inputMode="decimal" value={hourlyRate} onChange={(event) => setHourlyRate(event.target.value)} placeholder="z.B. 85,00" className="h-10" />
              </div>
            )}

            {pricingMode === "fixed_price" && (
              <div className="space-y-2">
                <Label htmlFor="invoice-fixed-price">Pauschalpreis (EUR)</Label>
                <Input id="invoice-fixed-price" type="number" min="0" step="0.01" inputMode="decimal" value={fixedPrice} onChange={(event) => setFixedPrice(event.target.value)} placeholder="z.B. 2500,00" className="h-10" />
              </div>
            )}
          </div>

          {materialLines.length > 0 && (
            <div className="space-y-4 rounded-lg border border-border/70 bg-card/70 p-4 shadow-sm">
              <div className="space-y-1">
                <p className="text-sm font-medium">Materialabrechnung</p>
                <p className="text-sm text-muted-foreground">
                  Basispreise kommen aus dem Inventar. Der Aufschlag kann fuer diese Rechnung je Material angepasst werden.
                </p>
              </div>

              <div className="space-y-3">
                {materialLines.map((line) => {
                  const markup = parseNonNegativeInteger(materialMarkups[line.material_id] ?? "0")
                  const unitPriceCents = resolvedMaterialUnitPriceCents(
                    line.base_price_cents,
                    markup
                  )
                  const lineTotalCents =
                    unitPriceCents == null
                      ? null
                      : Math.round(unitPriceCents * line.total_withdrawn)

                  return (
                    <div key={line.material_id} className="rounded-lg border border-border/60 bg-background/70 p-3">
                      <div className="flex items-start justify-between gap-3">
                        <div>
                          <p className="text-sm font-medium">{line.material_name}</p>
                          <p className="text-sm text-muted-foreground">
                            {line.total_withdrawn} {line.unit} aus {line.withdrawal_count} Entnahmen
                          </p>
                        </div>
                        <div className="text-right text-sm text-muted-foreground">
                          <p>Basispreis: {formatMoney(line.base_price_cents)}</p>
                          <p>Standard: {line.price_markup_percentage ?? 0}%</p>
                        </div>
                      </div>

                      {line.base_price_cents != null ? (
                        <div className="mt-3 grid gap-3 sm:grid-cols-[180px_1fr] sm:items-end">
                          <div className="space-y-2">
                            <Label htmlFor={`material-markup-${line.material_id}`}>
                              Aufschlag {line.material_name} (%)
                            </Label>
                            <Input
                              id={`material-markup-${line.material_id}`}
                              type="number"
                              min="0"
                              step="1"
                              inputMode="numeric"
                              value={materialMarkups[line.material_id] ?? "0"}
                              onChange={(event) =>
                                setMaterialMarkups((current) => ({
                                  ...current,
                                  [line.material_id]: event.target.value,
                                }))
                              }
                              className="h-10"
                            />
                          </div>

                          <div className="rounded-md bg-accent/40 p-3 text-sm">
                            <p>Verkaufspreis pro Einheit: {formatMoney(unitPriceCents)}</p>
                            <p className="text-muted-foreground">
                              Materialsumme: {formatMoney(lineTotalCents)}
                            </p>
                          </div>
                        </div>
                      ) : (
                        <p className="mt-3 text-sm text-muted-foreground">
                          Kein Basispreis im Inventar hinterlegt. Dieses Material bleibt auf der Rechnung unbepreist.
                        </p>
                      )}
                    </div>
                  )
                })}
              </div>
            </div>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)} className="h-10">
            Abbrechen
          </Button>
          <Button onClick={handleSubmit} disabled={isPending || !pricingIsValid || !materialPricingIsValid} className="h-10 gap-2">
            {isPending ? "Wird erstellt..." : "Rechnung erstellen"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
