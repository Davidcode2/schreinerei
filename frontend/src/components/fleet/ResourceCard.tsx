import { useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { MapPin, QrCode, Trash2 } from "lucide-react"
import { StatusBadge } from "@/components/shared"
import { DeleteConfirmDialog } from "@/components/shared/DeleteConfirmDialog"
import { useDeleteVehicle, useDeleteTool } from "@/lib/api/hooks"
import { cn } from "@/lib/utils"
import { toast } from "sonner"
import type { Vehicle, Tool } from "@/types/fleet"
import { getResourceCalendarColor } from "@/pages/fleet/resourceCalendarColor"

interface ResourceCardProps {
  resource: Vehicle | Tool
  type: "vehicle" | "tool"
  onReserve: (id: string, type: "vehicle" | "tool") => void
}

function isVehicle(resource: Vehicle | Tool): resource is Vehicle {
  return "vehicle_type" in resource
}

export function ResourceCard({ resource, type, onReserve }: ResourceCardProps) {
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false)
  const deleteVehicleMutation = useDeleteVehicle()
  const deleteToolMutation = useDeleteTool()

  const isAvailable = resource.status === "available"
  const deleteMutation = type === "vehicle" ? deleteVehicleMutation : deleteToolMutation
  const resourceColor = getResourceCalendarColor(type, resource.id)

  const handleDelete = () => {
    deleteMutation.mutate(resource.id, {
      onSuccess: () => {
        toast.success(type === "vehicle" ? "Fahrzeug gelöscht" : "Werkzeug gelöscht")
        setDeleteDialogOpen(false)
      },
      onError: (error: Error) => {
        toast.error(error.message || "Löschen fehlgeschlagen")
      },
    })
  }

  return (
    <>
      <Card className={cn("overflow-hidden transition-colors hover:border-primary/50 hover:shadow-sm", resourceColor.borderClassName)}>
        <div className={cn("h-1.5 w-full", resourceColor.markerClassName)} />
        <CardContent className="p-4">
          <div className="mb-3 flex items-start justify-between gap-3">
            <div className="flex min-w-0 items-start gap-3">
              <span
                aria-hidden="true"
                className={cn("mt-1 h-3 w-3 flex-shrink-0 rounded-full ring-4 ring-background", resourceColor.markerClassName)}
              />
              <div className="min-w-0 space-y-1">
                <h3 className="truncate font-semibold">{resource.name}</h3>
                {isVehicle(resource) ? (
                  <p className="truncate text-sm text-muted-foreground capitalize">
                    {resource.vehicle_type}
                    {resource.license_plate && ` • ${resource.license_plate}`}
                  </p>
                ) : (
                  <p className="truncate text-sm text-muted-foreground">
                    {resource.category || "Werkzeug"}
                  </p>
                )}
              </div>
            </div>
            <StatusBadge status={resource.status} />
          </div>

          {resource.location && (
            <div className="mb-3 flex items-center gap-2 text-sm text-muted-foreground">
              <MapPin className="h-3 w-3" />
              <span>{resource.location}</span>
            </div>
          )}

          <div className={cn("flex items-center justify-between border-t pt-3", resourceColor.borderClassName)}>
            <div className="flex items-center gap-2">
              {resource.qr_code && (
                <div className={cn("flex items-center gap-1 rounded-full border px-2 py-1 text-xs", resourceColor.borderClassName, resourceColor.tintClassName)}>
                  <QrCode className="h-3 w-3" />
                  <span className="font-mono">{resource.qr_code}</span>
                </div>
              )}
              <Button
                variant="ghost"
                size="icon"
                className="h-8 w-8 text-muted-foreground hover:text-destructive"
                onClick={(e) => {
                  e.preventDefault()
                  e.stopPropagation()
                  setDeleteDialogOpen(true)
                }}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
            <Button
              size="sm"
              variant={isAvailable ? "default" : "outline"}
              disabled={!isAvailable}
              onClick={() => onReserve(resource.id, type)}
            >
              {isAvailable ? "Reservieren" : "Nicht verfügbar"}
            </Button>
          </div>
        </CardContent>
      </Card>

      <DeleteConfirmDialog
        open={deleteDialogOpen}
        onOpenChange={setDeleteDialogOpen}
        onConfirm={handleDelete}
        itemName={resource.name}
        isPending={deleteMutation.isPending}
      />
    </>
  )
}

interface ResourceCardSkeletonProps {
  count?: number
}

export function ResourceCardSkeleton({ count = 1 }: ResourceCardSkeletonProps) {
  return (
    <>
      {Array.from({ length: count }).map((_, i) => (
        <Card key={i}>
          <CardContent className="p-4">
            <div className="flex items-start justify-between mb-3">
              <div className="space-y-2 flex-1">
                <div className="h-5 bg-muted rounded w-3/4 animate-pulse" />
                <div className="h-4 bg-muted rounded w-1/2 animate-pulse" />
              </div>
              <div className="h-5 bg-muted rounded w-16 animate-pulse" />
            </div>
            <div className="h-3 bg-muted rounded w-1/3 animate-pulse mb-3" />
            <div className="flex items-center justify-between pt-3 border-t">
              <div className="h-3 bg-muted rounded w-20 animate-pulse" />
              <div className="h-8 bg-muted rounded w-24 animate-pulse" />
            </div>
          </CardContent>
        </Card>
      ))}
    </>
  )
}
