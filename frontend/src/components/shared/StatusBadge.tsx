import { Badge } from "@/components/ui/badge"
import { cn } from "@/lib/utils"

interface StatusBadgeProps {
  status: string
  variant?: "default" | "success" | "warning" | "destructive" | "outline"
  className?: string
}

const statusVariantMap: Record<string, "default" | "success" | "warning" | "destructive" | "outline"> = {
  available: "success",
  active: "success",
  confirmed: "success",
  completed: "success",

  maintenance: "warning",
  low_stock: "warning",
  pending: "warning",
  planned: "warning",

  in_use: "destructive",
  reserved: "destructive",
  cancelled: "destructive",

  archived: "outline",
}

const statusLabels: Record<string, string> = {
  available: "Verfügbar",
  active: "Aktiv",
  confirmed: "Bestätigt",
  completed: "Abgeschlossen",
  maintenance: "Wartung",
  low_stock: "Niedrig",
  pending: "Ausstehend",
  planned: "Geplant",
  in_use: "In Benutzung",
  reserved: "Reserviert",
  cancelled: "Storniert",
  archived: "Archiviert",
}

export function StatusBadge({
  status,
  variant,
  className,
}: StatusBadgeProps) {
  const resolvedVariant = variant || statusVariantMap[status] || "default"
  const label = statusLabels[status] || status

  const variantClasses = {
    default: "bg-secondary text-secondary-foreground",
    success: "bg-success/15 text-success border-success/20",
    warning: "bg-warning/15 text-warning-foreground border-warning/25",
    destructive: "bg-destructive/10 text-destructive border-destructive/20",
    outline: "border border-input bg-background",
  }

  return (
    <Badge
      className={cn(
        "font-medium border text-xs",
        variantClasses[resolvedVariant],
        className
      )}
    >
      {label}
    </Badge>
  )
}
