import { Card, CardContent } from "@/components/ui/card"
import { type LucideIcon } from "lucide-react"
import { cn } from "@/lib/utils"

interface StatsCardProps {
  title: string
  value: string | number
  icon: LucideIcon
  description?: string
  trend?: {
    value: number
    label: string
    positive?: boolean
  }
  className?: string
}

export function StatsCard({
  title,
  value,
  icon: Icon,
  description,
  trend,
  className,
}: StatsCardProps) {
  return (
    <Card className={cn("transition-shadow hover:shadow-md", className)}>
      <CardContent className="p-4">
        <div className="flex min-w-0 items-start justify-between gap-2 mb-3">
          <p className="min-w-0 flex-1 break-words pr-1 text-xs font-medium leading-tight text-muted-foreground uppercase tracking-wide [overflow-wrap:anywhere]">{title}</p>
          <div className="h-8 w-8 shrink-0 rounded-lg bg-accent flex items-center justify-center">
            <Icon className="h-4 w-4 text-muted-foreground" />
          </div>
        </div>
        <div className="font-display text-2xl">{value}</div>
        {description && (
          <p className="text-xs text-muted-foreground mt-1">{description}</p>
        )}
        {trend && (
          <p
            className={cn(
              "text-xs mt-1",
              trend.positive ? "text-success" : "text-destructive"
            )}
          >
            {trend.positive ? "+" : ""}
            {trend.value}% {trend.label}
          </p>
        )}
      </CardContent>
    </Card>
  )
}
