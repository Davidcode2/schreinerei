import { useEffect, useState } from "react"
import { Euro, Save } from "lucide-react"
import { toast } from "sonner"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
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

  useEffect(() => {
    setHourlyRate(formatRate(data?.default_hourly_rate_cents ?? null))
  }, [data?.default_hourly_rate_cents])

  if (!isAdmin) {
    return null
  }

  const parsedRate = parseRate(hourlyRate)
  const isInvalid = parsedRate === "invalid"
  const currentRate = data?.default_hourly_rate_cents ?? null
  const nextRate = parsedRate === "invalid" ? currentRate : parsedRate
  const isUnchanged = nextRate === currentRate

  function handleSave() {
    if (parsedRate === "invalid") {
      toast.error("Bitte geben Sie einen gueltigen Stundensatz ein")
      return
    }

    updateBillingSettings.mutate(
      { default_hourly_rate_cents: parsedRate },
      {
        onSuccess: () => {
          toast.success("Standard-Stundensatz gespeichert")
        },
        onError: () => {
          toast.error("Standard-Stundensatz konnte nicht gespeichert werden")
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
          Standard-Stundensatz fuer neue Projekte. Bereits bestehende Projekte behalten ihren eigenen Wert.
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
