import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { MapPin, Clock, Users, Star } from "lucide-react"
import { Link } from "react-router-dom"
import { StatusBadge } from "@/components/shared"
import type { DashboardSite } from "@/types/sites"

interface SiteCardProps {
  site: DashboardSite
  isActive: boolean
  onToggleActive: (siteId: string, nextActive: boolean) => void
  isToggling: boolean
}

function formatDate(date: string | null): string {
  if (!date) return "-"
  return new Date(date).toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "2-digit",
  })
}

export function SiteCard({
  site,
  isActive,
  onToggleActive,
  isToggling,
}: SiteCardProps) {
  return (
    <Card className="transition-shadow hover:shadow-md">
      <CardContent className="p-4">
        <div className="flex items-start justify-between gap-3 mb-3">
          <Link to={`/sites/${site.id}`} className="min-w-0 flex-1 space-y-1">
            <h3 className="font-display text-base line-clamp-1">{site.name}</h3>
            <p className="text-sm text-muted-foreground line-clamp-1">
              {site.customer_name}
            </p>
          </Link>
          <StatusBadge status={site.status} />
        </div>

        {site.location && (
          <Link to={`/sites/${site.id}`}>
            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-3">
              <div className="h-5 w-5 rounded bg-accent flex items-center justify-center flex-shrink-0">
                <MapPin className="h-3 w-3" />
              </div>
              <span className="line-clamp-1">{site.location}</span>
            </div>
          </Link>
        )}

        <div className="flex items-center gap-3 text-sm text-muted-foreground mb-3">
          <Link to={`/sites/${site.id}`} className="contents">
            {(site.start_date || site.end_date) && (
              <div className="flex items-center gap-1.5">
                <div className="h-5 w-5 rounded bg-accent flex items-center justify-center flex-shrink-0">
                  <Clock className="h-3 w-3" />
                </div>
                <span>
                  {formatDate(site.start_date)} - {formatDate(site.end_date)}
                </span>
              </div>
            )}
            <div className="flex items-center gap-1.5">
              <div className="h-5 w-5 rounded bg-accent flex items-center justify-center flex-shrink-0">
                <Users className="h-3 w-3" />
              </div>
              <span>{site.assigned_users}</span>
            </div>
          </Link>
        </div>

        {site.total_hours > 0 && (
          <div className="flex items-center justify-between text-xs text-muted-foreground mb-3">
            <span>{site.total_hours.toFixed(1)}h gebucht</span>
            {site.estimated_days && (
              <Badge variant="outline" className="text-xs">
                {site.estimated_days} Tage
              </Badge>
            )}
          </div>
        )}

        <div className="pt-2 border-t">
          <Button
            variant={isActive ? "default" : "outline"}
            size="sm"
            className={isActive ? "gap-2 w-full h-10" : "gap-2 w-full h-10"}
            onClick={() => onToggleActive(site.id, !isActive)}
            disabled={isToggling}
          >
            <Star className={isActive ? "h-3.5 w-3.5 fill-current" : "h-3.5 w-3.5"} />
            {isActive ? "Aktive Baustelle" : "Als aktiv setzen"}
          </Button>
        </div>
      </CardContent>
    </Card>
  )
}
