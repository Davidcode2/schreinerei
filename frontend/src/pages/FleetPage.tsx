import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Search, Plus } from "lucide-react"
import { Badge } from "@/components/ui/badge"

export default function FleetPage() {
  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-2xl font-bold">Fuhrpark & Werkzeuge</h1>
          <p className="text-muted-foreground">Fahrzeuge und Werkzeuge verwalten</p>
        </div>
        <Button className="gap-2">
          <Plus className="h-4 w-4" />
          Neues Fahrzeug/Werkzeug
        </Button>
      </div>

      <div className="flex items-center gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input placeholder="Fahrzeug oder Werkzeug suchen..." className="pl-10" />
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        {/* Vehicles section */}
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Fahrzeuge</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {[1, 2, 3].map((i) => (
                <div key={i} className="flex items-center gap-4 border-b pb-4 last:border-0 last:pb-0">
                  <Skeleton className="h-10 w-10 rounded" />
                  <div className="flex-1 space-y-2">
                    <Skeleton className="h-4 w-[150px]" />
                    <Skeleton className="h-3 w-[100px]" />
                  </div>
                  <Badge variant="outline">
                    <Skeleton className="h-4 w-[60px]" />
                  </Badge>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* Tools section */}
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Werkzeuge</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {[1, 2, 3].map((i) => (
                <div key={i} className="flex items-center gap-4 border-b pb-4 last:border-0 last:pb-0">
                  <Skeleton className="h-10 w-10 rounded" />
                  <div className="flex-1 space-y-2">
                    <Skeleton className="h-4 w-[150px]" />
                    <Skeleton className="h-3 w-[100px]" />
                  </div>
                  <Badge variant="outline">
                    <Skeleton className="h-4 w-[60px]" />
                  </Badge>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
