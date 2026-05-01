import { PageHeader } from "@/components/shared"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { useCategories } from "@/lib/api/hooks"

export default function InventorySettingsPage() {
  const { data: categories, isLoading } = useCategories()

  return (
    <div className="space-y-6 max-w-2xl mx-auto">
      <PageHeader
        title="Inventar-Einstellungen"
        description="Kategorien und Materialpflege zentral verwalten"
      />

      <Card>
        <CardHeader>
          <CardTitle>Kategorien</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          {isLoading ? (
            <p className="text-sm text-muted-foreground">Kategorien werden geladen...</p>
          ) : categories && categories.length > 0 ? (
            categories.map((category) => (
              <div
                key={category.id}
                className="rounded-lg border border-border/60 p-3"
              >
                <p className="font-medium">{category.name}</p>
                <p className="text-sm text-muted-foreground">
                  {category.description || "Keine Beschreibung hinterlegt."}
                </p>
              </div>
            ))
          ) : (
            <p className="text-sm text-muted-foreground">
              Kategorien erscheinen hier, sobald sie verfügbar sind.
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
