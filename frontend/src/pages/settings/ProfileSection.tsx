import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Separator } from "@/components/ui/separator"
import { User, Shield, Info, LogOut } from "lucide-react"
import { useAuth } from "@/hooks/useAuth"
import { useAuthStore } from "@/lib/auth/authStore"

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
        <CardTitle className="flex items-center gap-2">
          <User className="h-5 w-5" />
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
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="email">E-Mail</Label>
            <Input
              id="email"
              value={user.email}
              readOnly
            />
          </div>
        </div>

        <Separator />

        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium">Rolle</p>
            <p className="text-sm text-muted-foreground">
              Ihre Berechtigungen im System
            </p>
          </div>
          <Badge variant={user.role === "admin" ? "default" : "outline"}>
            {getRoleLabel(user.role)}
          </Badge>
        </div>
      </CardContent>
    </Card>
  )
}
