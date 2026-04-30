import { useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { MapPin, Calendar, Trash2 } from "lucide-react"
import { Link } from "react-router-dom"
import { StatusBadge } from "@/components/shared"
import { DeleteConfirmDialog } from "@/components/shared/DeleteConfirmDialog"
import { useDeleteSite } from "@/lib/api/hooks"
import { toast } from "sonner"
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
      <Card className="hover:border-primary/50 transition-colors h-full">
        <CardContent className="p-4">
          <div className="flex items-start justify-between mb-3">
            <Link to={`/sites/${site.id}`} className="space-y-1 flex-1">
              <h3 className="font-semibold line-clamp-1">{site.name}</h3>
              <p className="text-sm text-muted-foreground line-clamp-1">
                {site.customer_name}
              </p>
            </Link>
            <div className="flex items-center gap-2">
              <Button
                variant={isActive ? "default" : "outline"}
                size="sm"
                onClick={(e) => {
                  e.preventDefault()
                  e.stopPropagation()
                  onToggleActive(site.id, !isActive)
                }}
                disabled={isToggling}
              >
                {isActive ? "Aktiv" : "Aktiv setzen"}
              </Button>
              <StatusBadge status={site.status} />
              <Button
                variant="ghost"
                size="icon"
                className="h-8 w-8 text-muted-foreground hover:text-destructive"
                onClick={(e) => {
                  e.preventDefault()
                  e.stopPropagation()
                  setDeleteDialogOpen(true)
                }}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          </div>

          <Link to={`/sites/${site.id}`}>
            {site.location && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground mb-2">
                <MapPin className="h-3 w-3" />
                <span className="line-clamp-1">{site.location}</span>
              </div>
            )}

            {site.description && (
              <p className="text-sm text-muted-foreground line-clamp-2 mb-3">
                {site.description}
              </p>
            )}

            <div className="flex items-center gap-4 text-sm text-muted-foreground">
              {(site.start_date || site.end_date) && (
                <div className="flex items-center gap-1">
                  <Calendar className="h-3 w-3" />
                  <span className="text-xs">
                    {formatDate(site.start_date)} - {formatDate(site.end_date)}
                  </span>
                </div>
              )}
              {site.estimated_days && (
                <Badge variant="outline" className="text-xs">
                  {site.estimated_days} Tage
                </Badge>
              )}
              {isActive && <Badge className="text-xs">Aktiv</Badge>}
            </div>
          </Link>
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
          <CardContent className="p-4">
            <div className="flex items-start justify-between mb-3">
              <div className="space-y-2 flex-1">
                <div className="h-5 bg-muted rounded w-3/4 animate-pulse" />
                <div className="h-4 bg-muted rounded w-1/2 animate-pulse" />
              </div>
              <div className="h-5 bg-muted rounded w-16 animate-pulse" />
            </div>
            <div className="h-3 bg-muted rounded w-1/3 animate-pulse mb-3" />
            <div className="flex items-center gap-4">
              <div className="h-3 bg-muted rounded w-24 animate-pulse" />
              <div className="h-5 bg-muted rounded w-12 animate-pulse" />
            </div>
          </CardContent>
        </Card>
      ))}
    </>
  )
}
