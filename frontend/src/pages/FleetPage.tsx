import { PageHeader } from "@/components/shared"
import { Card, CardContent } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus, Car, Wrench } from "lucide-react"

export default function FleetPage() {
  return (
    <div className="space-y-6">
      <PageHeader
        title="Fuhrpark & Werkzeuge"
        description="Fahrzeuge und Werkzeuge verwalten"
        action={
          <Button className="gap-2 h-10 shadow-sm">
            <Plus className="h-4 w-4" />
            <span className="hidden sm:inline">Neu</span>
          </Button>
        }
      />

      <div className="flex items-center gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input placeholder="Fahrzeug oder Werkzeug suchen..." className="pl-10 h-10 bg-card border-border" />
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardContent className="p-5">
            <div className="flex items-center gap-3 mb-4">
              <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
                <Car className="h-4 w-4 text-muted-foreground" />
              </div>
              <h3 className="font-display text-lg">Fahrzeuge</h3>
            </div>
            <div className="space-y-4">
              {[1, 2, 3].map((i) => (
                <div key={i} className="flex items-center gap-4 border-b border-border/60 pb-4 last:border-0 last:pb-0">
                  <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-muted">
                    <Skeleton className="h-4 w-4 rounded" />
                  </div>
                  <div className="flex-1 space-y-2">
                    <Skeleton className="h-4 w-[150px]" />
                    <Skeleton className="h-3 w-[100px]" />
                  </div>
                  <Skeleton className="h-5 w-[60px] rounded-full" />
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-5">
            <div className="flex items-center gap-3 mb-4">
              <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
                <Wrench className="h-4 w-4 text-muted-foreground" />
              </div>
              <h3 className="font-display text-lg">Werkzeuge</h3>
            </div>
            <div className="space-y-4">
              {[1, 2, 3].map((i) => (
                <div key={i} className="flex items-center gap-4 border-b border-border/60 pb-4 last:border-0 last:pb-0">
                  <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-muted">
                    <Skeleton className="h-4 w-4 rounded" />
                  </div>
                  <div className="flex-1 space-y-2">
                    <Skeleton className="h-4 w-[150px]" />
                    <Skeleton className="h-3 w-[100px]" />
                  </div>
                  <Skeleton className="h-5 w-[60px] rounded-full" />
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
