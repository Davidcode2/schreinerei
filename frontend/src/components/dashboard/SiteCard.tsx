import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { MapPin, Clock, Users } from "lucide-react"
import { Link } from "react-router-dom"
import { StatusBadge } from "@/components/shared"
import type { DashboardSite } from "@/types/sites"

interface SiteCardProps {
  site: DashboardSite
}

function formatDate(date: string | null): string {
  if (!date) return "-"
  return new Date(date).toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "2-digit",
  })
}

export function SiteCard({ site }: SiteCardProps) {
  return (
    <Link to={`/sites/${site.id}`}>
      <Card className="hover:border-primary/50 transition-colors cursor-pointer">
        <CardContent className="p-4">
          <div className="flex items-start justify-between mb-3">
            <div className="space-y-1">
              <h3 className="font-semibold line-clamp-1">{site.name}</h3>
              <p className="text-sm text-muted-foreground line-clamp-1">
                {site.customer_name}
              </p>
            </div>
            <StatusBadge status={site.status} />
          </div>

          {site.location && (
            <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
              <MapPin className="h-3 w-3" />
              <span className="line-clamp-1">{site.location}</span>
            </div>
          )}

          <div className="flex items-center gap-4 text-sm text-muted-foreground">
            {(site.start_date || site.end_date) && (
              <div className="flex items-center gap-1">
                <Clock className="h-3 w-3" />
                <span>
                  {formatDate(site.start_date)} - {formatDate(site.end_date)}
                </span>
              </div>
            )}
            <div className="flex items-center gap-1">
              <Users className="h-3 w-3" />
              <span>{site.assigned_users}</span>
            </div>
          </div>

          {site.total_hours > 0 && (
            <div className="mt-3 flex items-center justify-between">
              <span className="text-xs text-muted-foreground">
                {site.total_hours.toFixed(1)}h gebucht
              </span>
              <Badge variant="outline" className="text-xs">
                {site.estimated_days ? `${site.estimated_days} Tage` : ""}
              </Badge>
            </div>
          )}
        </CardContent>
      </Card>
    </Link>
  )
}
