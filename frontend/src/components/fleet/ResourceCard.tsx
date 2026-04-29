import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { MapPin, Calendar, QrCode } from "lucide-react"
import { StatusBadge } from "@/components/shared"
import type { Vehicle, Tool, ResourceStatus } from "@/types/fleet"

interface ResourceCardProps {
  resource: Vehicle | Tool
  type: "vehicle" | "tool"
  onReserve?: (id: string, type: "vehicle" | "tool") => void
}

function isVehicle(resource: Vehicle | Tool): resource is Vehicle {
  return "vehicle_type" in resource
}

function getStatusLabel(status: ResourceStatus): string {
  switch (status) {
    case "available":
      return "Verfügbar"
    case "in_use":
      return "In Benutzung"
    case "maintenance":
      return "Wartung"
    case "reserved":
      return "Reserviert"
    default:
      return status
  }
}

export function ResourceCard({ resource, type, onReserve }: ResourceCardProps) {
  const isAvailable = resource.status === "available"

  return (
    <Card className="hover:border-primary/50 transition-colors">
      <CardContent className="p-4">
        <div className="flex items-start justify-between mb-3">
          <div className="space-y-1">
            <h3 className="font-semibold">{resource.name}</h3>
            {isVehicle(resource) ? (
              <p className="text-sm text-muted-foreground capitalize">
                {resource.vehicle_type}
                {resource.license_plate && ` • ${resource.license_plate}`}
              </p>
            ) : (
              <p className="text-sm text-muted-foreground">
                {resource.category || "Werkzeug"}
              </p>
            )}
          </div>
          <StatusBadge status={resource.status} />
        </div>

        {resource.location && (
          <div className="flex items-center gap-2 text-sm text-muted-foreground mb-3">
            <MapPin className="h-3 w-3" />
            <span>{resource.location}</span>
          </div>
        )}

        <div className="flex items-center justify-between pt-3 border-t">
          {resource.qr_code && (
            <div className="flex items-center gap-1 text-xs text-muted-foreground">
              <QrCode className="h-3 w-3" />
              <span className="font-mono">{resource.qr_code}</span>
            </div>
          )}
          <Button
            size="sm"
            variant={isAvailable ? "default" : "outline"}
            disabled={!isAvailable}
            onClick={() => onReserve?.(resource.id, type)}
            className="ml-auto"
          >
            {isAvailable ? "Reservieren" : "Nicht verfügbar"}
          </Button>
        </div>
      </CardContent>
    </Card>
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
