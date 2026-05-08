import { useState } from "react"
import { useNavigate } from "react-router-dom"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus, QrCode, Settings, AlertTriangle, ArrowRight } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import {
  PageHeader,
  EmptyState,
  ErrorState,
} from "@/components/shared"
import { MaterialCard, MaterialCardSkeleton } from "@/components/inventory/MaterialCard"
import { CategoryFilter } from "@/components/inventory/CategoryFilter"
import { useCategories, useInventoryAlerts, useMaterials } from "@/lib/api/hooks"
import { AddMaterialDialog } from "./AddMaterialDialog"
import type { Material } from "@/types/inventory"

export default function InventoryListPage() {
  const navigate = useNavigate()
  const [selectedCategory, setSelectedCategory] = useState<string | undefined>()
  const [searchQuery, setSearchQuery] = useState("")
  const [addMaterialOpen, setAddMaterialOpen] = useState(false)

  const { data: categories, isLoading: categoriesLoading } = useCategories()
  const {
    data: materials,
    isLoading: materialsLoading,
    error,
    refetch,
  } = useMaterials(selectedCategory)
  const { data: inventoryAlerts = [] } = useInventoryAlerts()

  const categoryNames = new Map(
    (categories ?? []).map((category) => [category.id, category.name])
  )

  const filteredMaterials = materials?.filter((material: Material) =>
    material.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    material.description?.toLowerCase().includes(searchQuery.toLowerCase()) ||
    material.location?.toLowerCase().includes(searchQuery.toLowerCase())
  )

  return (
    <div className="space-y-6">
      <PageHeader
        title="Inventar"
        description="Materialverwaltung"
        action={
          <div className="flex items-center gap-2">
            <Button
              variant="outline"
              size="icon"
              onClick={() => navigate("/settings/inventory")}
              aria-label="Inventar-Einstellungen öffnen"
            >
              <Settings className="h-4 w-4" />
            </Button>
            <Button className="gap-2 h-10 shadow-sm" onClick={() => setAddMaterialOpen(true)}>
              <Plus className="h-4 w-4" />
              <span className="hidden sm:inline">Material hinzufügen</span>
              <span className="sm:hidden">Hinzufügen</span>
            </Button>
          </div>
        }
      />

      <div className="space-y-4">
        {!categoriesLoading && categories && categories.length > 0 && (
          <CategoryFilter
            categories={categories}
            selectedId={selectedCategory}
            onSelect={setSelectedCategory}
          />
        )}

        <div className="flex items-center gap-3">
          <div className="relative flex-1">
            <Search className="absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder="Material suchen..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10 h-10 bg-card border-border"
            />
          </div>
          <Button
            variant="outline"
            size="icon"
            className="flex-shrink-0 h-10 w-10"
            onClick={() => navigate("/scan")}
          >
            <QrCode className="h-4 w-4" />
          </Button>
        </div>
      </div>

      {inventoryAlerts.length > 0 && (
        <Card className="border-warning/30 bg-warning/5">
          <CardHeader className="pb-3">
            <CardTitle className="flex items-center gap-3 text-base font-semibold">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-warning/15">
                <AlertTriangle className="h-4 w-4 text-warning" />
              </div>
              Ablaufwarnungen
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            {inventoryAlerts.slice(0, 4).map((material: Material) => (
              <Button
                key={material.id}
                variant="ghost"
                className="h-auto w-full justify-between rounded-lg px-3 py-3"
                onClick={() => navigate(`/inventory/${material.id}`)}
              >
                <div className="text-left">
                  <p className="font-medium">{material.name}</p>
                  <p className="text-xs text-muted-foreground">
                    {material.expired_quantity > 0
                      ? `${material.expired_quantity} ${material.unit} abgelaufen`
                      : `${material.expiring_soon_quantity} ${material.unit} laufen bald ab`}
                  </p>
                </div>
                <ArrowRight className="h-4 w-4 text-muted-foreground" />
              </Button>
            ))}
          </CardContent>
        </Card>
      )}

      {/* Materials List */}
      {materialsLoading ? (
        <div className="space-y-4">
          <MaterialCardSkeleton count={5} />
        </div>
      ) : error ? (
        <ErrorState
          message="Materialien konnten nicht geladen werden"
          onRetry={() => refetch()}
        />
      ) : !filteredMaterials || filteredMaterials.length === 0 ? (
        <EmptyState
          icon={Search}
          title={searchQuery ? "Keine Ergebnisse" : "Kein Material"}
          description={
            searchQuery
              ? "Keine Materialien entsprechen Ihrer Suche."
              : "Fügen Sie Ihr erstes Material hinzu."
          }
          action={
            !searchQuery && (
              <Button className="gap-2 h-10 shadow-sm" onClick={() => setAddMaterialOpen(true)}>
                <Plus className="h-4 w-4" />
                Material hinzufügen
              </Button>
            )
          }
        />
      ) : (
        <div className="space-y-4">
          <p className="text-sm text-muted-foreground mb-0">
            {filteredMaterials.length} Material{filteredMaterials.length !== 1 ? "ien" : ""} gefunden
          </p>
          {filteredMaterials.map((material: Material) => {
            const categoryName = categoryNames.get(material.category_id)

            return (
              <MaterialCard
                key={material.id}
                material={material}
                {...(categoryName ? { categoryName } : {})}
              />
            )
          })}
        </div>
      )}

      {/* Add Material Dialog */}
      <AddMaterialDialog
        open={addMaterialOpen}
        onOpenChange={setAddMaterialOpen}
        categories={categories || []}
      />
    </div>
  )
}
