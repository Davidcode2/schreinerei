import { useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Package, QrCode, MapPin, Trash2 } from "lucide-react"
import { Link } from "react-router-dom"
import { StatusBadge } from "@/components/shared"
import { DeleteConfirmDialog } from "@/components/shared/DeleteConfirmDialog"
import { useDeleteMaterial } from "@/lib/api/hooks"
import { toast } from "sonner"
import { cn } from "@/lib/utils"
import type { Material } from "@/types/inventory"

interface MaterialCardProps {
  material: Material
  categoryName?: string
}

export function MaterialCard({ material, categoryName }: MaterialCardProps) {
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false)
  const deleteMutation = useDeleteMaterial()

  const handleDelete = () => {
    deleteMutation.mutate(material.id, {
      onSuccess: () => {
        toast.success("Material gelöscht")
        setDeleteDialogOpen(false)
      },
      onError: (error: Error) => {
        toast.error(error.message || "Löschen fehlgeschlagen")
      },
    })
  }

  return (
    <>
      <Card className="group relative overflow-hidden transition-all duration-200 hover:shadow-md hover:border-primary/30">
        {material.is_low_stock && (
          <div className="absolute top-0 left-0 right-0 h-1 bg-warning" />
        )}
        <CardContent className="p-5">
          <div className="flex items-start justify-between gap-3 mb-3">
            <Link to={`/inventory/${material.id}`} className="flex-1 min-w-0">
              <div className="flex items-center gap-2.5 mb-1">
                <div className={cn(
                  "flex h-9 w-9 flex-shrink-0 items-center justify-center rounded-lg",
                  material.is_low_stock
                    ? "bg-warning/15 text-warning"
                    : "bg-accent text-muted-foreground"
                )}>
                  <Package className="h-4 w-4" />
                </div>
                <div className="min-w-0">
                  <h3 className="font-semibold text-[15px] leading-tight line-clamp-1">{material.name}</h3>
                  {categoryName && (
                    <p className="text-sm text-muted-foreground line-clamp-1">
                      {categoryName}
                    </p>
                  )}
                </div>
              </div>
              {material.description && (
                <p className="text-sm text-muted-foreground line-clamp-1 mb-2 ml-[46px]">
                  {material.description}
                </p>
              )}
              {material.location && (
                <div className="flex items-center gap-1.5 text-sm text-muted-foreground mb-2 ml-[46px]">
                  <MapPin className="h-3.5 w-3.5 flex-shrink-0" />
                  <span className="line-clamp-1">{material.location}</span>
                </div>
              )}
              <div className="flex items-center justify-between pt-3 border-t border-border/60 ml-[46px]">
                <Badge variant="outline" className="font-mono text-xs">
                  {material.quantity} {material.unit}
                </Badge>
                <span className="text-xs text-muted-foreground">
                  Min: {material.min_quantity} {material.unit}
                </span>
              </div>
            </Link>
            <div className="flex flex-col items-center gap-1.5 flex-shrink-0">
              {material.is_low_stock && (
                <StatusBadge status="low_stock" />
              )}
              {!material.is_low_stock && material.qr_code && (
                <QrCode className="h-4 w-4 text-muted-foreground" />
              )}
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
          </div>
        </CardContent>
      </Card>

      <DeleteConfirmDialog
        open={deleteDialogOpen}
        onOpenChange={setDeleteDialogOpen}
        onConfirm={handleDelete}
        itemName={material.name}
        isPending={deleteMutation.isPending}
      />
    </>
  )
}

interface MaterialCardSkeletonProps {
  count?: number
}

export function MaterialCardSkeleton({ count = 1 }: MaterialCardSkeletonProps) {
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
            <div className="flex items-center justify-between pt-3 border-t border-border/60">
              <div className="h-5 bg-muted rounded-full w-16 animate-pulse" />
              <div className="h-3 bg-muted rounded w-20 animate-pulse" />
            </div>
          </CardContent>
        </Card>
      ))}
    </>
  )
}
