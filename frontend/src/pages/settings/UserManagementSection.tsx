import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Separator } from "@/components/ui/separator"
import { Input } from "@/components/ui/input"
import { Users, UserPlus, Shield, Copy, Link2, Loader2 } from "lucide-react"
import { useUsers } from "@/lib/api/hooks"
import { useAuthStore } from "@/lib/auth/authStore"
import { toast } from "sonner"

interface UserManagementSectionProps {
  isAdmin: boolean
}

const KEYCLOAK_URL = import.meta.env.VITE_KEYCLOAK_URL || 'http://localhost:8080'
const REALM = import.meta.env.VITE_KEYCLOAK_REALM || 'schreinerei'

function getInitials(name: string | null): string {
  if (!name) return "??"
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

function getDisplayName(user: { name: string | null; email: string }): string {
  return user.name || user.email.split("@")[0]
}

export function UserManagementSection({ isAdmin }: UserManagementSectionProps) {
  const [inviteUrlCopied, setInviteUrlCopied] = useState(false)
  const { data: users, isLoading, error } = useUsers()
  const user = useAuthStore((state) => state.user)

  if (!isAdmin) {
    return null
  }

  const orgId = user?.tenant_id || ""
  const inviteUrl = `${KEYCLOAK_URL}/realms/${REALM}/org/${orgId}/inviting`

  const copyInviteUrl = async () => {
    try {
      await navigator.clipboard.writeText(inviteUrl)
      setInviteUrlCopied(true)
      toast.success("Einladungslink kopiert")
      setTimeout(() => setInviteUrlCopied(false), 2000)
    } catch {
      toast.error("Link konnte nicht kopiert werden")
    }
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
          <Button size="sm" className="gap-2" onClick={copyInviteUrl}>
            <UserPlus className="h-4 w-4" />
            Einladen
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        {/* Organization invite link section */}
        <div className="mb-6 p-4 bg-muted/50 rounded-lg">
          <div className="flex items-center gap-2 mb-2">
            <Link2 className="h-4 w-4 text-muted-foreground" />
            <span className="text-sm font-medium">Organisation beitreten</span>
          </div>
          <p className="text-sm text-muted-foreground mb-3">
            Teilen Sie diesen Link, um neue Mitarbeiter einzuladen
          </p>
          <div className="flex gap-2">
            <Input
              value={inviteUrl}
              readOnly
              className="text-sm bg-background"
            />
            <Button size="sm" variant="outline" onClick={copyInviteUrl}>
              <Copy className="h-4 w-4" />
            </Button>
          </div>
        </div>

        <Separator className="mb-4" />

        {/* User list */}
        {isLoading && (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
          </div>
        )}

        {error && (
          <div className="text-center py-8 text-destructive">
            Benutzer konnten nicht geladen werden
          </div>
        )}

        {users && (
          <div className="space-y-4">
            {users.map((apiUser, index) => (
              <div key={apiUser.id}>
                {index > 0 && <Separator className="mb-4" />}
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <Avatar className="h-10 w-10">
                      <AvatarFallback>{getInitials(apiUser.name)}</AvatarFallback>
                    </Avatar>
                    <div>
                      <p className="font-medium">{getDisplayName(apiUser)}</p>
                      <p className="text-sm text-muted-foreground">{apiUser.email}</p>
                    </div>
                  </div>
                  <Badge
                    variant={apiUser.role === "admin" ? "default" : "outline"}
                    className="gap-1"
                  >
                    {apiUser.role === "admin" && <Shield className="h-3 w-3" />}
                    {getRoleLabel(apiUser.role)}
                  </Badge>
                </div>
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
