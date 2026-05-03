import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { Label } from "@/components/ui/label"
import { Button } from "@/components/ui/button"
import { PageHeader } from "@/components/shared"
import { User, Building2 } from "lucide-react"

export default function SettingsPage() {
  return (
    <div className="space-y-6">
      <PageHeader
        title="Einstellungen"
        description="App-Einstellungen"
      />

      <div className="grid gap-4">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-3 font-display text-lg">
              <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
                <User className="h-4 w-4" />
              </span>
              Profil
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 sm:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="firstName">Vorname</Label>
                <Skeleton className="h-10 w-full" />
              </div>
              <div className="space-y-2">
                <Label htmlFor="lastName">Nachname</Label>
                <Skeleton className="h-10 w-full" />
              </div>
            </div>
            <div className="space-y-2">
              <Label htmlFor="email">E-Mail</Label>
              <Skeleton className="h-10 w-full" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-3 font-display text-lg">
              <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
                <Building2 className="h-4 w-4" />
              </span>
              Firma
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="companyName">Firmenname</Label>
              <Skeleton className="h-10 w-full" />
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="street">Straße</Label>
                <Skeleton className="h-10 w-full" />
              </div>
              <div className="space-y-2">
                <Label htmlFor="city">Stadt</Label>
                <Skeleton className="h-10 w-full" />
              </div>
            </div>
          </CardContent>
        </Card>

        <div className="flex justify-end gap-3">
          <Button variant="outline" className="active:scale-[0.97] transition-transform">Abbrechen</Button>
          <Button className="active:scale-[0.97] transition-transform">Speichern</Button>
        </div>
      </div>
    </div>
  )
}
