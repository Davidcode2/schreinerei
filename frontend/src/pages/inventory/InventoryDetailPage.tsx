import { useEffect, useState } from "react"
import { useParams, useSearchParams } from "react-router-dom"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import {
  Package,
  MapPin,
  QrCode,
  Minus,
  Pencil,
  Plus,
  AlertTriangle,
  ShoppingCart,
  TrendingDown,
  Archive,
} from "lucide-react"
import {
  PageHeader,
  LoadingSpinner,
  ErrorState,
  StatusBadge,
} from "@/components/shared"
import {
  useCreateOrderRequest,
  useEnrichedMaterialHistory,
  useMaterial,
  usePreferences,
  useSites,
  useStockInMaterial,
  useWithdrawMaterial,
} from "@/lib/api/hooks"
import { MaterialEditDialog } from "./MaterialEditDialog"
import { MaterialHistoryFeed } from "./MaterialHistoryFeed"
import { StockInDialog } from "./StockInDialog"
import { WithdrawDialog } from "./WithdrawDialog"
import { toast } from "sonner"

export default function InventoryDetailPage() {
  const { id } = useParams<{ id: string }>()
  const [searchParams, setSearchParams] = useSearchParams()
  const [showEditDialog, setShowEditDialog] = useState(false)
  const [showStockInDialog, setShowStockInDialog] = useState(false)
  const [showWithdrawDialog, setShowWithdrawDialog] = useState(false)

  const { data: material, isLoading, error, refetch } = useMaterial(id!)
  const {
    data: history = [],
    error: historyError,
    isLoading: isHistoryLoading,
    refetch: refetchHistory,
  } = useEnrichedMaterialHistory(id!)
  const { data: preferences } = usePreferences()
  const { data: sites } = useSites()
  const stockInMutation = useStockInMaterial()
  const withdrawMutation = useWithdrawMaterial()
  const orderMutation = useCreateOrderRequest()

  const withdrawSiteIdFromQuery = searchParams.get("siteId")
  const shouldOpenWithdrawDialog = searchParams.get("action") === "withdraw"

  useEffect(() => {
    if (shouldOpenWithdrawDialog) {
      setShowWithdrawDialog(true)
    }
  }, [shouldOpenWithdrawDialog])

  const closeWithdrawDialog = () => {
    setShowWithdrawDialog(false)

    if (shouldOpenWithdrawDialog) {
      const nextParams = new URLSearchParams(searchParams)
      nextParams.delete("action")
      nextParams.delete("siteId")
      setSearchParams(nextParams, { replace: true })
    }
  }

  if (isLoading) {
    return <LoadingSpinner className="min-h-[400px]" size="lg" />
  }

  if (error || !material) {
    return (
      <ErrorState
        message="Material konnte nicht geladen werden"
        onRetry={() => refetch()}
      />
    )
  }

  const handleWithdraw = async (
    quantity: number,
    notes?: string,
    siteId?: string | null,
    disposal?: boolean
  ) => {
    try {
      await withdrawMutation.mutateAsync({
        id: material.id,
        quantity,
        notes: notes ?? null,
        site_id: siteId ?? null,
        disposal: disposal ?? false,
      })
      toast.success(
        disposal
          ? `${quantity} ${material.unit} entsorgt`
          : `${quantity} ${material.unit} entnommen`
      )
      closeWithdrawDialog()
    } catch {
      toast.error("Entnahme fehlgeschlagen")
    }
  }

  const handleStockIn = async (quantity: number, notes?: string, expiresOn?: string) => {
    try {
      await stockInMutation.mutateAsync({
        id: material.id,
        quantity,
        notes: notes ?? null,
        expires_on: expiresOn ?? null,
      })
      toast.success(`${quantity} ${material.unit} eingelagert`)
      setShowStockInDialog(false)
    } catch {
      toast.error("Einlagerung fehlgeschlagen")
    }
  }

  const handleOrderRequest = async () => {
    try {
      await orderMutation.mutateAsync({
        material_id: material.id,
        quantity: material.min_quantity * 2,
        reason: "Niedriger Bestand",
      })
      toast.success("Bestellanforderung erstellt")
    } catch {
      toast.error("Bestellanforderung fehlgeschlagen")
    }
  }

  return (
    <div className="space-y-6">
      <PageHeader
        title={material.name}
        description={material.description || undefined}
        backTo="/inventory"
        action={
          <Button
            onClick={() => setShowStockInDialog(true)}
            className="gap-2 h-10 shadow-sm"
            disabled={stockInMutation.isPending}
          >
            <Plus className="h-4 w-4" />
            Einlagern
          </Button>
        }
      />

      <Card className="overflow-hidden">
        <CardContent className="p-5">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
            <div className="flex items-center gap-3 flex-wrap">
              <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
                <Package className="h-4 w-4" />
              </div>
              <div className="flex items-center gap-2">
                <span className="text-2xl font-bold font-mono">
                  {material.quantity}
                </span>
                <span className="text-lg text-muted-foreground">
                  {material.unit}
                </span>
              </div>
              {material.is_low_stock && (
                <StatusBadge status="low_stock" />
              )}
            </div>
            <div className="flex items-center gap-1.5 text-sm text-muted-foreground">
              <TrendingDown className="h-4 w-4 flex-shrink-0" />
              <span>Min: {material.min_quantity} {material.unit}</span>
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-3">
            <CardTitle className="text-base font-semibold">Details</CardTitle>
            <Button
              variant="ghost"
              size="icon"
              className="h-9 w-9"
              onClick={() => setShowEditDialog(true)}
              aria-label="Material bearbeiten"
            >
              <Pencil className="h-4 w-4" />
            </Button>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              {material.location && (
                <div className="flex items-start gap-2.5">
                  <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                    <MapPin className="h-3.5 w-3.5 text-muted-foreground" />
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground">Lagerort</p>
                    <p className="text-sm font-medium">{material.location}</p>
                  </div>
                </div>
              )}
              {material.qr_code && (
                <div className="flex items-start gap-2.5">
                  <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                    <QrCode className="h-3.5 w-3.5 text-muted-foreground" />
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground">QR-Code</p>
                    <p className="text-sm font-mono">{material.qr_code}</p>
                  </div>
                </div>
              )}
              <div className="flex items-start gap-2.5">
                <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                  <Archive className="h-3.5 w-3.5 text-muted-foreground" />
                </div>
                <div>
                  <p className="text-xs text-muted-foreground">Mindestbestand</p>
                  <p className="text-sm font-medium">{material.min_quantity} {material.unit}</p>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-base font-semibold">Aktionen</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            <Button
              onClick={() => setShowWithdrawDialog(true)}
              disabled={material.quantity <= 0 || withdrawMutation.isPending}
              className="w-full justify-start gap-2 h-10 shadow-sm active:scale-[0.97] transition-transform"
            >
              <Minus className="h-4 w-4" />
              Material entnehmen
            </Button>
            <Button
              variant="outline"
              onClick={() => setShowStockInDialog(true)}
              disabled={stockInMutation.isPending}
              className="w-full justify-start gap-2 h-10 active:scale-[0.97] transition-transform"
            >
              <Plus className="h-4 w-4" />
              Material einlagern
            </Button>
            {material.is_low_stock && (
              <Button
                variant="outline"
                className="w-full justify-start gap-2 h-10 active:scale-[0.97] transition-transform"
                onClick={handleOrderRequest}
                disabled={orderMutation.isPending}
              >
                <ShoppingCart className="h-4 w-4" />
                Nachbestellen
              </Button>
            )}
          </CardContent>
        </Card>
      </div>

      {material.is_low_stock && (
        <Card className="border-warning/30 bg-warning/5">
          <CardContent className="p-4">
            <div className="flex items-start gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-warning/15 flex-shrink-0">
                <AlertTriangle className="h-4 w-4 text-warning" />
              </div>
              <div>
                <h3 className="text-sm font-semibold">Niedriger Bestand</h3>
                <p className="text-sm text-muted-foreground">
                  Der Bestand liegt unter dem Mindestbestand von{" "}
                  {material.min_quantity} {material.unit}. Erwägen Sie eine
                  Nachbestellung.
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {material.expired_quantity > 0 && (
        <Card className="border-destructive/30 bg-destructive/5">
          <CardContent className="p-4">
            <div className="flex items-start gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-destructive/15 flex-shrink-0">
                <AlertTriangle className="h-4 w-4 text-destructive" />
              </div>
              <div>
                <h3 className="text-sm font-semibold">Abgelaufener Bestand</h3>
                <p className="text-sm text-muted-foreground">
                  Es sind {material.expired_quantity} {material.unit} abgelaufen.
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {material.expiring_soon_quantity > 0 && (
        <Card className="border-warning/30 bg-warning/5">
          <CardContent className="p-4">
            <div className="flex items-start gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-warning/15 flex-shrink-0">
                <AlertTriangle className="h-4 w-4 text-warning" />
              </div>
              <div>
                <h3 className="text-sm font-semibold">Bald ablaufender Bestand</h3>
                <p className="text-sm text-muted-foreground">
                  {material.expiring_soon_quantity} {material.unit} laufen innerhalb der nächsten 10 Tage ab.
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {material.can_expire && material.legacy_quantity > 0 && (
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-accent flex-shrink-0">
                <Archive className="h-3.5 w-3.5 text-muted-foreground" />
              </div>
              <p className="text-sm text-muted-foreground">
                Ohne MHD erfasster Bestand: <span className="font-medium text-foreground">{material.legacy_quantity} {material.unit}</span>
              </p>
            </div>
          </CardContent>
        </Card>
      )}

      <Card>
        <CardHeader>
          <CardTitle className="text-base font-semibold">Historie</CardTitle>
        </CardHeader>
        <CardContent>
          {historyError ? (
            <ErrorState
              message="Historie konnte nicht geladen werden. Bitte Seite neu laden oder später erneut versuchen."
              onRetry={() => {
                void refetchHistory()
              }}
            />
          ) : isHistoryLoading ? (
            <div className="flex items-center justify-center py-8">
              <div className="rounded-2xl bg-accent/60 p-3">
                <Package className="h-6 w-6 text-muted-foreground animate-pulse" />
              </div>
            </div>
          ) : (
            <MaterialHistoryFeed entries={history} />
          )}
        </CardContent>
      </Card>

      <WithdrawDialog
        open={showWithdrawDialog}
        onOpenChange={(open) => {
          if (open) {
            setShowWithdrawDialog(true)
            return
          }

          closeWithdrawDialog()
        }}
        material={material}
        onConfirm={handleWithdraw}
        isLoading={withdrawMutation.isPending}
        sites={(sites ?? []).map((site) => ({ id: site.id, name: site.name }))}
        initialSiteId={withdrawSiteIdFromQuery ?? preferences?.active_site_id ?? null}
      />

      <StockInDialog
        open={showStockInDialog}
        onOpenChange={setShowStockInDialog}
        material={material}
        onConfirm={handleStockIn}
        isLoading={stockInMutation.isPending}
      />

      <MaterialEditDialog
        open={showEditDialog}
        onOpenChange={setShowEditDialog}
        material={material}
      />
    </div>
  )
}
