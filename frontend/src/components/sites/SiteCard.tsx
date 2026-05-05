import { useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { MapPin, Calendar, Trash2, Building2, Star } from "lucide-react"
import { Link } from "react-router-dom"
import { StatusBadge } from "@/components/shared"
import { DeleteConfirmDialog } from "@/components/shared/DeleteConfirmDialog"
import { useDeleteSite } from "@/lib/api/hooks"
import { toast } from "sonner"
import { cn } from "@/lib/utils"
import type { Site } from "@/types/sites"

interface SiteCardProps {
  site: Site
  isActive: boolean
  onToggleActive: (siteId: string, nextActive: boolean) => void
  isToggling: boolean
}

function formatDate(date: string | null): string {
  if (!date) return "-"
  return new Date(date).toLocaleDateString("de-DE", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  })
}

export function SiteCard({
  site,
  isActive,
  onToggleActive,
  isToggling,
}: SiteCardProps) {
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false)
  const deleteMutation = useDeleteSite()

  const handleDelete = () => {
    deleteMutation.mutate(site.id, {
      onSuccess: () => {
        toast.success("Baustelle gelöscht")
        setDeleteDialogOpen(false)
      },
      onError: (error: Error) => {
        toast.error(error.message || "Löschen fehlgeschlagen")
      },
    })
  }

  return (
    <>
      <Card
        className={cn(
          "group relative overflow-hidden transition-all duration-200 hover:shadow-md",
          isActive
            ? "border-primary/40 ring-2 ring-primary/20 shadow-sm"
            : "hover:border-primary/30"
        )}
      >
        {isActive && (
          <div className="absolute top-0 left-0 right-0 h-1 bg-primary" />
        )}
        <CardContent className="p-5">
          <div className="flex items-start justify-between gap-3 mb-3">
            <Link to={`/sites/${site.id}`} className="flex-1 min-w-0">
              <div className="flex items-center gap-2.5 mb-1">
                <div className={cn(
                  "flex h-9 w-9 flex-shrink-0 items-center justify-center rounded-lg",
                  isActive ? "bg-primary/10 text-primary" : "bg-accent text-muted-foreground"
                )}>
                  <Building2 className="h-4 w-4" />
                </div>
                <div className="min-w-0">
                  <h3 className="font-semibold text-[15px] leading-tight line-clamp-1">{site.name}</h3>
                  <p className="text-sm text-muted-foreground line-clamp-1">
                    {site.customer_name || (site.project_type === "internal_workshop" ? "Internes Werkstattprojekt" : "-")}
                  </p>
                </div>
              </div>
            </Link>
            <div className="flex flex-col items-end gap-2">
              <StatusBadge status={site.status} />
              <Badge variant="outline" className="text-[11px] font-normal">
                {site.project_type === "internal_workshop" ? "Werkstatt" : "Extern"}
              </Badge>
            </div>
          </div>

          <Link to={`/sites/${site.id}`} className="block">
            {site.location && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
                <MapPin className="h-3.5 w-3.5 flex-shrink-0" />
                <span className="line-clamp-1">{site.location}</span>
              </div>
            )}

            {site.description && (
              <p className="text-sm text-muted-foreground line-clamp-2 mb-3 leading-relaxed">
                {site.description}
              </p>
            )}

            <div className="flex items-center gap-3 flex-wrap">
              {(site.start_date || site.end_date) && (
                <div className="flex items-center gap-1.5 text-sm text-muted-foreground">
                  <Calendar className="h-3.5 w-3.5 flex-shrink-0" />
                  <span className="text-xs">
                    {formatDate(site.start_date)} – {formatDate(site.end_date)}
                  </span>
                </div>
              )}
              {site.estimated_days && (
                <Badge variant="outline" className="text-xs font-normal">
                  {site.estimated_days} Tage
                </Badge>
              )}
            </div>
          </Link>

          <div className="flex items-center gap-2 mt-4 pt-3 border-t border-border/60">
            <Button
              variant={isActive ? "default" : "outline"}
              size="sm"
              onClick={(e) => {
                e.preventDefault()
                e.stopPropagation()
                onToggleActive(site.id, !isActive)
              }}
              disabled={isToggling}
              className={cn(
                "gap-2 flex-1 h-9",
                isActive && "gap-2"
              )}
            >
              <Star className={cn("h-3.5 w-3.5", isActive && "fill-current")} />
              {isActive ? "Aktiv" : "Auswählen"}
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="h-9 w-9 text-muted-foreground hover:text-destructive"
              onClick={(e) => {
                e.preventDefault()
                e.stopPropagation()
                setDeleteDialogOpen(true)
              }}
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        </CardContent>
      </Card>

      <DeleteConfirmDialog
        open={deleteDialogOpen}
        onOpenChange={setDeleteDialogOpen}
        onConfirm={handleDelete}
        itemName={site.name}
        isPending={deleteMutation.isPending}
      />
    </>
  )
}

interface SiteCardSkeletonProps {
  count?: number
}

export function SiteCardSkeleton({ count = 1 }: SiteCardSkeletonProps) {
  return (
    <>
      {Array.from({ length: count }).map((_, i) => (
        <Card key={i}>
          <CardContent className="p-5">
            <div className="flex items-start gap-2.5 mb-3">
              <div className="h-9 w-9 rounded-lg bg-muted animate-pulse" />
              <div className="space-y-2 flex-1">
                <div className="h-5 bg-muted rounded w-3/4 animate-pulse" />
                <div className="h-4 bg-muted rounded w-1/2 animate-pulse" />
              </div>
              <div className="h-5 bg-muted rounded-full w-16 animate-pulse" />
            </div>
            <div className="h-3 bg-muted rounded w-1/3 animate-pulse mb-3" />
            <div className="flex items-center gap-4 mb-3">
              <div className="h-3 bg-muted rounded w-24 animate-pulse" />
              <div className="h-5 bg-muted rounded w-12 animate-pulse" />
            </div>
            <div className="h-9 bg-muted rounded animate-pulse mt-4 pt-3" />
          </CardContent>
        </Card>
      ))}
    </>
  )
}
