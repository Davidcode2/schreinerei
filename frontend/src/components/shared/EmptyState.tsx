import { Card, CardContent } from "@/components/ui/card"
import { Package, type LucideIcon } from "lucide-react"

interface EmptyStateProps {
  icon?: LucideIcon
  title: string
  description?: string
  action?: React.ReactNode
}

export function EmptyState({
  icon: Icon = Package,
  title,
  description,
  action,
}: EmptyStateProps) {
  return (
    <Card className="border-dashed border-2 bg-card/50">
      <CardContent className="flex flex-col items-center justify-center py-16 text-center">
        <div className="rounded-2xl bg-accent/60 p-4 mb-5">
          <Icon className="h-8 w-8 text-muted-foreground" />
        </div>
        <h3 className="font-display text-xl mb-2">{title}</h3>
        {description && (
          <p className="text-sm text-muted-foreground mb-5 max-w-sm leading-relaxed">
            {description}
          </p>
        )}
        {action && <div>{action}</div>}
      </CardContent>
    </Card>
  )
}
