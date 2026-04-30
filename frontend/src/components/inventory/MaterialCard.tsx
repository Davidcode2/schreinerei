import { useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Package, QrCode, AlertTriangle, Trash2 } from "lucide-react"
import { Link } from "react-router-dom"
import { StatusBadge } from "@/components/shared"
import { DeleteConfirmDialog } from "@/components/shared/DeleteConfirmDialog"
import { useDeleteMaterial } from "@/lib/api/hooks"
import { toast } from "sonner"
import type { Material } from "@/types/inventory"

interface MaterialCardProps {
  material: Material
}

export function MaterialCard({ material }: MaterialCardProps) {
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
      <Card className="hover:border-primary/50 transition-colors">
        <CardContent className="p-4">
          <div className="flex items-start justify-between gap-4">
            <Link to={`/inventory/${material.id}`} className="flex items-start gap-3 min-w-0 flex-1">
              <div className="rounded-lg bg-muted p-2 flex-shrink-0">
                <Package className="h-5 w-5 text-muted-foreground" />
              </div>
              <div className="min-w-0">
                <h3 className="font-semibold line-clamp-1">{material.name}</h3>
                {material.description && (
                  <p className="text-sm text-muted-foreground line-clamp-1">
                    {material.description}
                  </p>
                )}
                {material.location && (
                  <p className="text-xs text-muted-foreground mt-1">
                    📍 {material.location}
                  </p>
                )}
              </div>
            </Link>
            <div className="flex items-center gap-2 flex-shrink-0">
              {material.qr_code && (
                <QrCode className="h-4 w-4 text-muted-foreground" />
              )}
              {material.is_low_stock && (
                <AlertTriangle className="h-4 w-4 text-yellow-500" />
              )}
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
          <Link to={`/inventory/${material.id}`}>
            <div className="flex items-center justify-between mt-3 pt-3 border-t">
              <div className="flex items-center gap-2">
                <Badge variant="outline" className="font-mono">
                  {material.quantity} {material.unit}
                </Badge>
                {material.is_low_stock && (
                  <StatusBadge status="low_stock" />
                )}
              </div>
              <span className="text-xs text-muted-foreground">
                Min: {material.min_quantity} {material.unit}
              </span>
            </div>
          </Link>
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
          <CardContent className="p-4">
            <div className="flex items-start gap-3">
              <div className="rounded-lg bg-muted p-2">
                <Package className="h-5 w-5 text-muted-foreground animate-pulse" />
              </div>
              <div className="flex-1 space-y-2">
                <div className="h-4 bg-muted rounded w-3/4 animate-pulse" />
                <div className="h-3 bg-muted rounded w-1/2 animate-pulse" />
              </div>
            </div>
            <div className="flex items-center justify-between mt-3 pt-3 border-t">
              <div className="h-5 bg-muted rounded w-16 animate-pulse" />
              <div className="h-3 bg-muted rounded w-20 animate-pulse" />
            </div>
          </CardContent>
        </Card>
      ))}
    </>
  )
}
