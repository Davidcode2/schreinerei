import { Badge } from "@/components/ui/badge"
import { cn } from "@/lib/utils"

interface StatusBadgeProps {
  status: string
  variant?: "default" | "success" | "warning" | "destructive" | "outline"
  className?: string
}

const statusVariantMap: Record<string, "default" | "success" | "warning" | "destructive" | "outline"> = {
  // Success (green)
  available: "success",
  active: "success",
  confirmed: "success",
  completed: "success",

  // Warning (yellow)
  maintenance: "warning",
  low_stock: "warning",
  pending: "warning",
  planned: "warning",

  // Destructive/Red
  in_use: "destructive",
  reserved: "destructive",
  cancelled: "destructive",

  // Default/Outline (gray)
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
    default: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
    success: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
    warning: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
    destructive: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
    outline: "border border-input bg-background",
  }

  return (
    <Badge
      className={cn(
        "font-normal",
        variantClasses[resolvedVariant],
        className
      )}
    >
      {label}
    </Badge>
  )
}
