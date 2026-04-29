import { useState } from "react"
import { useParams } from "react-router-dom"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import {
  Package,
  MapPin,
  QrCode,
  Minus,
  AlertTriangle,
  ShoppingCart,
} from "lucide-react"
import {
  PageHeader,
  LoadingSpinner,
  ErrorState,
  StatusBadge,
} from "@/components/shared"
import { useMaterial, useWithdrawMaterial, useCreateOrderRequest } from "@/lib/api/hooks"
import { WithdrawDialog } from "./WithdrawDialog"
import { toast } from "sonner"

export default function InventoryDetailPage() {
  const { id } = useParams<{ id: string }>()
  const [showWithdrawDialog, setShowWithdrawDialog] = useState(false)

  const { data: material, isLoading, error, refetch } = useMaterial(id!)
  const withdrawMutation = useWithdrawMaterial()
  const orderMutation = useCreateOrderRequest()

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

  const handleWithdraw = async (quantity: number, notes?: string) => {
    try {
      await withdrawMutation.mutateAsync({
        id: material.id,
        quantity,
        ...(notes ? { notes } : {}),
      })
      toast.success(`${quantity} ${material.unit} entnommen`)
      setShowWithdrawDialog(false)
    } catch {
      toast.error("Entnahme fehlgeschlagen")
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
            onClick={() => setShowWithdrawDialog(true)}
            disabled={material.quantity <= 0}
            className="gap-2"
          >
            <Minus className="h-4 w-4" />
            Entnahme
          </Button>
        }
      />

      {/* Status Card */}
      <Card>
        <CardContent className="p-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="rounded-xl bg-muted p-4">
                <Package className="h-8 w-8 text-muted-foreground" />
              </div>
              <div>
                <div className="flex items-center gap-2">
                  <span className="text-3xl font-bold">
                    {material.quantity}
                  </span>
                  <span className="text-xl text-muted-foreground">
                    {material.unit}
                  </span>
                </div>
                <p className="text-sm text-muted-foreground">
                  Mindestbestand: {material.min_quantity} {material.unit}
                </p>
              </div>
            </div>
            {material.is_low_stock && (
              <StatusBadge status="low_stock" />
            )}
          </div>
        </CardContent>
      </Card>

      {/* Details Grid */}
      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Details
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {material.location && (
              <div className="flex items-center gap-3">
                <MapPin className="h-4 w-4 text-muted-foreground" />
                <span>{material.location}</span>
              </div>
            )}
            {material.qr_code && (
              <div className="flex items-center gap-3">
                <QrCode className="h-4 w-4 text-muted-foreground" />
                <span className="font-mono text-sm">{material.qr_code}</span>
              </div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Aktionen
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            <Button
              variant="outline"
              className="w-full justify-start gap-2"
              onClick={() => setShowWithdrawDialog(true)}
              disabled={material.quantity <= 0}
            >
              <Minus className="h-4 w-4" />
              Material entnehmen
            </Button>
            {material.is_low_stock && (
              <Button
                variant="outline"
                className="w-full justify-start gap-2"
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

      {/* Low Stock Warning */}
      {material.is_low_stock && (
        <Card className="border-yellow-200 bg-yellow-50 dark:border-yellow-900 dark:bg-yellow-950">
          <CardContent className="p-4">
            <div className="flex items-start gap-3">
              <AlertTriangle className="h-5 w-5 text-yellow-600 mt-0.5" />
              <div>
                <h3 className="font-semibold">Niedriger Bestand</h3>
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

      <WithdrawDialog
        open={showWithdrawDialog}
        onOpenChange={setShowWithdrawDialog}
        material={material}
        onConfirm={handleWithdraw}
        isLoading={withdrawMutation.isPending}
      />
    </div>
  )
}
