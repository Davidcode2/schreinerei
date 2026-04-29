import { Button } from "@/components/ui/button"
import { ChevronLeft } from "lucide-react"
import { Link } from "react-router-dom"
import { cn } from "@/lib/utils"

interface Breadcrumb {
  label: string
  href: string
}

interface PageHeaderProps {
  title: string
  description: string | undefined
  action?: React.ReactNode
  breadcrumbs?: Breadcrumb[]
  backTo?: string
  className?: string
}

export function PageHeader({
  title,
  description,
  action,
  breadcrumbs,
  backTo,
  className,
}: PageHeaderProps) {
  return (
    <div className={cn("space-y-2", className)}>
      {breadcrumbs && breadcrumbs.length > 0 && (
        <nav className="flex items-center gap-2 text-sm text-muted-foreground">
          {breadcrumbs.map((crumb, index) => (
            <span key={crumb.href} className="flex items-center gap-2">
              {index > 0 && <span>/</span>}
              <Link
                to={crumb.href}
                className="hover:text-foreground transition-colors"
              >
                {crumb.label}
              </Link>
            </span>
          ))}
        </nav>
      )}

      {backTo && (
        <Link to={backTo}>
          <Button variant="ghost" size="sm" className="gap-2 -ml-2 mb-2">
            <ChevronLeft className="h-4 w-4" />
            Zurück
          </Button>
        </Link>
      )}

      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">{title}</h1>
          {description && (
            <p className="text-muted-foreground">{description}</p>
          )}
        </div>
        {action && <div className="flex-shrink-0">{action}</div>}
      </div>
    </div>
  )
}
