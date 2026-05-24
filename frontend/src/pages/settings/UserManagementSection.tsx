import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Separator } from "@/components/ui/separator"
import { Users, UserPlus, Shield, Loader2 } from "lucide-react"
import { useUsers } from "@/lib/api/hooks"
import { useAuthStore } from "@/lib/auth/authStore"
import { InviteUserDialog } from "@/components/settings/InviteUserDialog"

interface UserManagementSectionProps {
  isAdmin: boolean
}

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
  const { isAuthenticated } = useAuthStore((state) => state)
  const [showInviteDialog, setShowInviteDialog] = useState(false)

  if (!isAdmin) {
    return null
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
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
            className="w-full gap-2 shadow-sm transition-transform active:scale-[0.97] sm:w-auto"
            onClick={() => setShowInviteDialog(true)}
          >
            <UserPlus className="h-4 w-4" />
            Einladen
          </Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
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
                <div className="flex flex-col items-start gap-3 rounded-lg p-2 -mx-2 transition-colors hover:bg-accent/30 sm:flex-row sm:items-center sm:justify-between">
                  <div className="flex min-w-0 items-center gap-3 self-stretch sm:self-auto">
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
                    className="gap-1 sm:flex-shrink-0"
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
        />
      </CardContent>
    </Card>
  )
}
