import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { useNavigate } from 'react-router-dom'
import { apiClient } from '@/lib/api/client'
import { useQuery } from '@tanstack/react-query'
import { Skeleton } from '@/components/ui/skeleton'
import { Package, Truck, Wrench } from 'lucide-react'

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
          <DialogTitle>QR-Code erkannt</DialogTitle>
        </DialogHeader>

        {inventoryLookup.isLoading && (
          <div className="space-y-2">
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
              <Package className="h-8 w-8 text-primary" />
              <div>
                <p className="font-medium">{inventoryLookup.data.name}</p>
                <p className="text-sm text-muted-foreground">
                  {inventoryLookup.data.quantity} {inventoryLookup.data.unit}
                  {inventoryLookup.data.location && ` • ${inventoryLookup.data.location}`}
                </p>
              </div>
            </div>
            <Button onClick={() => handleNavigate('material', inventoryLookup.data.id)}>
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
              {fleetLookup.data.resource_type === 'vehicle' ? (
                <Truck className="h-8 w-8 text-primary" />
              ) : (
                <Wrench className="h-8 w-8 text-primary" />
              )}
              <div>
                <p className="font-medium">{fleetLookup.data.resource_name}</p>
                <p className="text-sm text-muted-foreground">
                  Status: {fleetLookup.data.status}
                  {fleetLookup.data.location && ` • ${fleetLookup.data.location}`}
                </p>
              </div>
            </div>
            <Button onClick={() => handleNavigate(fleetLookup.data.resource_type, fleetLookup.data.resource_id)}>
              Details anzeigen
            </Button>
          </div>
        )}

        {inventoryLookup.isError && fleetLookup.isError && (
          <div className="text-sm text-muted-foreground">
            QR-Code nicht erkannt oder nicht im System.
          </div>
        )}
      </DialogContent>
    </Dialog>
  )
}
