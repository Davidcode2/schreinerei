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
    <div className={cn("mb-6 md:mb-8", className)}>
      {breadcrumbs && breadcrumbs.length > 0 && (
        <nav className="flex items-center gap-2 text-sm text-muted-foreground mb-3">
          {breadcrumbs.map((crumb, index) => (
            <span key={crumb.href} className="flex items-center gap-2">
              {index > 0 && (
                <span className="text-border">/</span>
              )}
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
          <Button variant="ghost" size="sm" className="gap-2 -ml-2 mb-3 h-9 text-muted-foreground hover:text-foreground">
            <ChevronLeft className="h-4 w-4" />
            Zurück
          </Button>
        </Link>
      )}

      <div className="flex min-w-0 flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="min-w-0">
          <h1 className="break-words font-display text-2xl font-normal tracking-tight [overflow-wrap:anywhere] md:text-3xl">{title}</h1>
          {description && (
            <p className="mt-1 break-words text-sm text-muted-foreground [overflow-wrap:anywhere]">{description}</p>
          )}
        </div>
        {action && <div className="flex-shrink-0 mt-2 sm:mt-0">{action}</div>}
      </div>
    </div>
  )
}
