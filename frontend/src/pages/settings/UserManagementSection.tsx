import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Separator } from "@/components/ui/separator"
import { Users, UserPlus, Shield } from "lucide-react"
import { useAuth } from "@/hooks/useAuth"

interface UserManagementSectionProps {
  isAdmin: boolean
}

// Mock users for now - would come from API
const mockUsers = [
  { id: "1", name: "Max Mustermann", email: "max@example.com", role: "admin" },
  { id: "2", name: "Anna Schmidt", email: "anna@example.com", role: "mitarbeiter" },
  { id: "3", name: "Peter Weber", email: "peter@example.com", role: "mitarbeiter" },
]

function getInitials(name: string): string {
  return name
    .split(" ")
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2)
}

function getRoleLabel(role: string): string {
  switch (role) {
    case "admin":
      return "Admin"
    case "mitarbeiter":
      return "Mitarbeiter"
    default:
      return role
  }
}

export function UserManagementSection({ isAdmin }: UserManagementSectionProps) {
  if (!isAdmin) {
    return null
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle className="flex items-center gap-2">
              <Users className="h-5 w-5" />
              Benutzerverwaltung
            </CardTitle>
            <CardDescription>Verwalten Sie die Benutzer Ihrer Organisation</CardDescription>
          </div>
          <Button size="sm" className="gap-2">
            <UserPlus className="h-4 w-4" />
            Einladen
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {mockUsers.map((user, index) => (
            <div key={user.id}>
              {index > 0 && <Separator className="mb-4" />}
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <Avatar className="h-10 w-10">
                    <AvatarFallback>{getInitials(user.name)}</AvatarFallback>
                  </Avatar>
                  <div>
                    <p className="font-medium">{user.name}</p>
                    <p className="text-sm text-muted-foreground">{user.email}</p>
                  </div>
                </div>
                <Badge
                  variant={user.role === "admin" ? "default" : "outline"}
                  className="gap-1"
                >
                  {user.role === "admin" && <Shield className="h-3 w-3" />}
                  {getRoleLabel(user.role)}
                </Badge>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
