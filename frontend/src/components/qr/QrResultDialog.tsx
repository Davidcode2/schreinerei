import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { useNavigate } from 'react-router-dom'
import { apiClient } from '@/lib/api/client'
import { useQuery } from '@tanstack/react-query'
import { Skeleton } from '@/components/ui/skeleton'
import { Package, Truck, Wrench, QrCode } from 'lucide-react'

interface QrResultDialogProps {
  qrCode: string | null
  onClose: () => void
}

interface InventoryQrResponse {
  id: string
  name: string
  quantity: number
  unit: string
  location: string | null
}

interface FleetQrResponse {
  resource_type: 'vehicle' | 'tool'
  resource_id: string
  resource_name: string
  status: string
  location: string | null
}

export default function QrResultDialog({ qrCode, onClose }: QrResultDialogProps) {
  const navigate = useNavigate()

  // Try to lookup in inventory
  const inventoryLookup = useQuery({
    queryKey: ['qr-inventory', qrCode],
    queryFn: () => apiClient.get<InventoryQrResponse>(`/api/v1/inventory/qr/${qrCode}`),
    enabled: !!qrCode
  })

  // Try to lookup in fleet if inventory not found
  const fleetLookup = useQuery({
    queryKey: ['qr-fleet', qrCode],
    queryFn: () => apiClient.get<FleetQrResponse>(`/api/v1/fleet/qr/${qrCode}`),
    enabled: !!qrCode && inventoryLookup.isError
  })

  const handleNavigate = (type: 'material' | 'vehicle' | 'tool', id: string) => {
    onClose()
    if (type === 'material') {
      navigate(`/inventory/${id}`)
    } else {
      navigate(`/fleet`)
    }
  }

  return (
    <Dialog open={!!qrCode} onOpenChange={() => onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="font-display">QR-Code erkannt</DialogTitle>
        </DialogHeader>

        {inventoryLookup.isLoading && (
          <div className="space-y-3 py-2">
            <Skeleton className="h-4 w-full" />
            <Skeleton className="h-4 w-3/4" />
          </div>
        )}

        {inventoryLookup.data && (
          <div className="space-y-4">
            <p className="text-sm text-muted-foreground">
              Material gefunden:
            </p>
            <div className="flex items-center gap-3">
              <div className="rounded-2xl bg-accent p-2.5 flex-shrink-0">
                <Package className="h-5 w-5 text-primary" />
              </div>
              <div className="min-w-0">
                <p className="font-medium">{inventoryLookup.data.name}</p>
                <p className="text-sm text-muted-foreground">
                  {inventoryLookup.data.quantity} {inventoryLookup.data.unit}
                  {inventoryLookup.data.location && ` · ${inventoryLookup.data.location}`}
                </p>
              </div>
            </div>
            <Button
              onClick={() => handleNavigate('material', inventoryLookup.data.id)}
              className="w-full h-10 rounded-lg active:scale-[0.97] transition-transform"
            >
              Zum Material
            </Button>
          </div>
        )}

        {fleetLookup.data && (
          <div className="space-y-4">
            <p className="text-sm text-muted-foreground">
              {fleetLookup.data.resource_type === 'vehicle' ? 'Fahrzeug' : 'Werkzeug'} gefunden:
            </p>
            <div className="flex items-center gap-3">
              <div className="rounded-2xl bg-accent p-2.5 flex-shrink-0">
                {fleetLookup.data.resource_type === 'vehicle' ? (
                  <Truck className="h-5 w-5 text-primary" />
                ) : (
                  <Wrench className="h-5 w-5 text-primary" />
                )}
              </div>
              <div className="min-w-0">
                <p className="font-medium">{fleetLookup.data.resource_name}</p>
                <p className="text-sm text-muted-foreground">
                  Status: {fleetLookup.data.status}
                  {fleetLookup.data.location && ` · ${fleetLookup.data.location}`}
                </p>
              </div>
            </div>
            <Button
              onClick={() => handleNavigate(fleetLookup.data.resource_type, fleetLookup.data.resource_id)}
              className="w-full h-10 rounded-lg active:scale-[0.97] transition-transform"
            >
              Details anzeigen
            </Button>
          </div>
        )}

        {inventoryLookup.isError && fleetLookup.isError && (
          <div className="flex flex-col items-center justify-center py-6 text-center space-y-3">
            <div className="rounded-2xl bg-accent/60 p-4">
              <QrCode className="h-8 w-8 text-muted-foreground" />
            </div>
            <div>
              <p className="font-display text-lg">Nicht gefunden</p>
              <p className="text-sm text-muted-foreground mt-1">
                QR-Code nicht erkannt oder nicht im System.
              </p>
            </div>
          </div>
        )}
      </DialogContent>
    </Dialog>
  )
}
