import { useEffect, useState } from "react"
import { Euro, Save } from "lucide-react"
import { toast } from "sonner"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { useBillingSettings, useUpdateBillingSettings } from "@/lib/api/hooks"

interface BillingSettingsSectionProps {
  isAdmin: boolean
}

function formatRate(value: number | null): string {
  if (value == null) return ""
  return (value / 100).toFixed(2)
}

function parseRate(value: string): number | null | "invalid" {
  const normalized = value.replace(",", ".").trim()
  if (!normalized) return null
  const parsed = Number(normalized)
  if (!Number.isFinite(parsed) || parsed < 0) return "invalid"
  return Math.round(parsed * 100)
}

export function BillingSettingsSection({ isAdmin }: BillingSettingsSectionProps) {
  const { data, isLoading } = useBillingSettings()
  const updateBillingSettings = useUpdateBillingSettings()
  const [hourlyRate, setHourlyRate] = useState("")
  const [billingTaxMode, setBillingTaxMode] = useState<"standard" | "kleinunternehmer">("standard")
  const [senderName, setSenderName] = useState("")
  const [senderAddress, setSenderAddress] = useState("")

  useEffect(() => {
    setHourlyRate(formatRate(data?.default_hourly_rate_cents ?? null))
    setBillingTaxMode(data?.billing_tax_mode ?? "standard")
    setSenderName(data?.sender_name ?? "")
    setSenderAddress(data?.sender_address ?? "")
  }, [data])

  if (!isAdmin) {
    return null
  }

  const parsedRate = parseRate(hourlyRate)
  const isInvalid = parsedRate === "invalid"
  const currentRate = data?.default_hourly_rate_cents ?? null
  const nextRate = parsedRate === "invalid" ? currentRate : parsedRate
  const isUnchanged =
    nextRate === currentRate &&
    billingTaxMode === (data?.billing_tax_mode ?? "standard") &&
    senderName.trim() === (data?.sender_name ?? "") &&
    senderAddress.trim() === (data?.sender_address ?? "")

  function handleSave() {
    if (parsedRate === "invalid") {
      toast.error("Bitte geben Sie einen gueltigen Stundensatz ein")
      return
    }

    updateBillingSettings.mutate(
      {
        default_hourly_rate_cents: parsedRate,
        billing_tax_mode: billingTaxMode,
        sender_name: senderName.trim() || null,
        sender_address: senderAddress.trim() || null,
      },
      {
        onSuccess: () => {
          toast.success("Abrechnungseinstellungen gespeichert")
        },
        onError: () => {
          toast.error("Abrechnungseinstellungen konnten nicht gespeichert werden")
        },
      }
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-3 font-display text-lg">
          <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
            <Euro className="h-4 w-4" />
          </span>
          Abrechnung
        </CardTitle>
        <CardDescription>
          Standard-Stundensatz für neue Projekte. Bereits bestehende Projekte behalten ihren eigenen Wert.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="default-hourly-rate">Standard-Stundensatz (EUR)</Label>
          <Input
            id="default-hourly-rate"
            type="number"
            min="0"
            step="0.01"
            inputMode="decimal"
            placeholder="z.B. 85,00"
            value={hourlyRate}
            onChange={(event) => setHourlyRate(event.target.value)}
            disabled={isLoading || updateBillingSettings.isPending}
            aria-invalid={isInvalid}
          />
          <p className="text-sm text-muted-foreground">
            Leer lassen, wenn neue Projekte keinen vorausgefuellten Stundensatz erhalten sollen.
          </p>
        </div>

        <div className="space-y-2">
          <Label htmlFor="billing-tax-mode">Umsatzsteuer</Label>
          <Select value={billingTaxMode} onValueChange={(value) => setBillingTaxMode(value as "standard" | "kleinunternehmer") }>
            <SelectTrigger id="billing-tax-mode" className="h-10">
              <SelectValue placeholder="Steuermodus wählen" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="standard">Normales Unternehmen (19% MwSt)</SelectItem>
              <SelectItem value="kleinunternehmer">Kleinunternehmer (§ 19 UStG)</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <Label htmlFor="billing-sender-name">Absendername</Label>
          <Input
            id="billing-sender-name"
            placeholder="z.B. Schreinerei Mustermann"
            value={senderName}
            onChange={(event) => setSenderName(event.target.value)}
            disabled={isLoading || updateBillingSettings.isPending}
          />
          <p className="text-sm text-muted-foreground">
            Wird als Standard-Absender für neue Rechnungen verwendet.
          </p>
        </div>

        <div className="space-y-2">
          <Label htmlFor="billing-sender-address">Absenderadresse</Label>
          <Textarea
            id="billing-sender-address"
            rows={3}
            placeholder="z.B. Werkstrasse 1\n12345 Musterstadt"
            value={senderAddress}
            onChange={(event) => setSenderAddress(event.target.value)}
            disabled={isLoading || updateBillingSettings.isPending}
          />
          <p className="text-sm text-muted-foreground">
            Diese Adresse wird im Rechnungsdialog automatisch vorausgefüllt.
          </p>
        </div>

        <div className="flex justify-end">
          <Button
            onClick={handleSave}
            disabled={isLoading || updateBillingSettings.isPending || isInvalid || isUnchanged}
            className="gap-2"
          >
            <Save className="h-4 w-4" />
            {updateBillingSettings.isPending ? "Wird gespeichert..." : "Speichern"}
          </Button>
        </div>
      </CardContent>
    </Card>
  )
}
