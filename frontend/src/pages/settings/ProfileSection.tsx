import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Badge } from "@/components/ui/badge"
import { Separator } from "@/components/ui/separator"
import { User, Shield } from "lucide-react"
import { useAuth } from "@/hooks/useAuth"

function getRoleLabel(role: string): string {
  switch (role) {
    case "admin":
      return "Administrator"
    case "mitarbeiter":
      return "Mitarbeiter"
    default:
      return role
  }
}

export function ProfileSection() {
  const { user } = useAuth()

  if (!user) {
    return null
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-3 font-display text-lg">
          <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
            <User className="h-4 w-4" />
          </span>
          Profil
        </CardTitle>
        <CardDescription>Ihre persönlichen Informationen</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid gap-4 sm:grid-cols-2">
          <div className="space-y-2">
            <Label htmlFor="name">Name</Label>
            <Input
              id="name"
              value={user.name || ""}
              placeholder="Nicht gesetzt"
              readOnly
              className="bg-accent/30"
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="email">E-Mail</Label>
            <Input
              id="email"
              value={user.email}
              readOnly
              className="bg-accent/30"
            />
          </div>
        </div>

        <Separator />

        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
              <Shield className="h-4 w-4" />
            </span>
            <div>
              <p className="text-sm font-medium">Rolle</p>
              <p className="text-sm text-muted-foreground">
                Ihre Berechtigungen im System
              </p>
            </div>
          </div>
          <Badge
            variant={user.role === "admin" ? "default" : "outline"}
            className={
              user.role === "admin"
                ? "bg-primary text-primary-foreground"
                : ""
            }
          >
            {getRoleLabel(user.role)}
          </Badge>
        </div>
      </CardContent>
    </Card>
  )
}
