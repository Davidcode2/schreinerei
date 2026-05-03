import { Card, CardContent } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus } from "lucide-react"

export default function InventoryPage() {
  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="font-display text-2xl md:text-3xl font-normal tracking-tight">Inventar</h1>
          <p className="mt-1 text-sm text-muted-foreground">Materialverwaltung</p>
        </div>
        <Button className="gap-2 h-10 shadow-sm">
          <Plus className="h-4 w-4" />
          <span className="hidden sm:inline">Material hinzufügen</span>
          <span className="sm:hidden">Hinzufügen</span>
        </Button>
      </div>

      <div className="flex items-center gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input placeholder="Material suchen..." className="pl-10 h-10 bg-card border-border" />
        </div>
      </div>

      <Card>
        <CardContent className="p-5">
          <div className="space-y-4">
            {[1, 2, 3, 4, 5].map((i) => (
              <div key={i} className="flex items-center gap-4 border-b border-border/60 pb-4 last:border-0 last:pb-0">
                <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-muted">
                  <Skeleton className="h-4 w-4 rounded" />
                </div>
                <div className="flex-1 space-y-2">
                  <Skeleton className="h-4 w-[200px]" />
                  <Skeleton className="h-3 w-[100px]" />
                </div>
                <Skeleton className="h-5 w-[80px] rounded-full" />
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
