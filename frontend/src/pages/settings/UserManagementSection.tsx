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
import { InviteUserDialog } from "@/components/settings/InviteUserDialog"

interface UserManagementSectionProps {
  isAdmin: boolean
}

const KEYCLOAK_URL = import.meta.env.VITE_KEYCLOAK_URL || 'https://auth.jakob-lingel.dev'
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
  return user.name ?? user.email.split("@")[0] ?? "User"
}

export function UserManagementSection({ isAdmin }: UserManagementSectionProps) {
  const { data: users, isLoading, error } = useUsers()
  const { user, isAuthenticated } = useAuthStore((state) => state)
  const [showInviteDialog, setShowInviteDialog] = useState(false)

  if (!isAdmin) {
    return null
  }

  const orgId = user?.tenant_id || ""
  const inviteUrl = `${KEYCLOAK_URL}/realms/${REALM}/org/${orgId}/inviting`

  const copyInviteUrl = async () => {
    try {
      await navigator.clipboard.writeText(inviteUrl)
      toast.success("Einladungslink kopiert")
    } catch {
      toast.error("Link konnte nicht kopiert werden")
    }
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between gap-4">
          <div>
            <CardTitle className="flex items-center gap-3 font-display text-lg">
              <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
                <Users className="h-4 w-4" />
              </span>
              Benutzerverwaltung
            </CardTitle>
            <CardDescription className="mt-1.5">Verwalten Sie die Benutzer Ihrer Organisation</CardDescription>
          </div>
          <Button
            size="sm"
            className="gap-2 shadow-sm active:scale-[0.97] transition-transform"
            onClick={() => setShowInviteDialog(true)}
          >
            <UserPlus className="h-4 w-4" />
            Einladen
          </Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="rounded-xl bg-accent/50 p-4">
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
            <Button
              size="sm"
              variant="outline"
              onClick={copyInviteUrl}
              className="shadow-sm active:scale-[0.97] transition-transform"
            >
              <Copy className="h-4 w-4" />
            </Button>
          </div>
        </div>

        <Separator />

        {isLoading && isAuthenticated && (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
          </div>
        )}

        {error && (
          <div className="text-center py-8 text-destructive">
            Benutzer konnten nicht geladen werden
          </div>
        )}

        {users && users.length === 0 && (
          <div className="text-center py-8 text-muted-foreground">
            Keine Benutzer gefunden
          </div>
        )}

        {users && users.length > 0 && (
          <div className="space-y-1">
            {users.map((apiUser, index) => (
              <div key={apiUser.id}>
                {index > 0 && <Separator className="my-3" />}
                <div className="flex items-center justify-between gap-3 rounded-lg p-2 -mx-2 transition-colors hover:bg-accent/30">
                  <div className="flex items-center gap-3 min-w-0">
                    <Avatar className="h-10 w-10 flex-shrink-0">
                      <AvatarFallback className="bg-accent text-foreground text-xs font-medium">
                        {getInitials(apiUser.name)}
                      </AvatarFallback>
                    </Avatar>
                    <div className="min-w-0">
                      <p className="font-medium truncate">{getDisplayName(apiUser)}</p>
                      <p className="text-sm text-muted-foreground truncate">{apiUser.email}</p>
                    </div>
                  </div>
                  <Badge
                    variant={apiUser.role === "admin" ? "default" : "outline"}
                    className="gap-1 flex-shrink-0"
                  >
                    {apiUser.role === "admin" && <Shield className="h-3 w-3" />}
                    {getRoleLabel(apiUser.role)}
                  </Badge>
                </div>
              </div>
            ))}
          </div>
        )}

        <InviteUserDialog
          open={showInviteDialog}
          onOpenChange={setShowInviteDialog}
          inviteUrl={inviteUrl}
        />
      </CardContent>
    </Card>
  )
}
