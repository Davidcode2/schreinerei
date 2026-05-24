import { useState } from "react"
import { Mail, Clock3, Loader2, Shield, UserPlus, Users } from "lucide-react"

import { InviteUserDialog } from "@/components/settings/InviteUserDialog"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"
import {
  type PendingInviteResponse,
  usePendingInvites,
  useUsers,
} from "@/lib/api/hooks"
import { useAuthStore } from "@/lib/auth/authStore"

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
    case "employee":
    case "mitarbeiter":
      return "Mitarbeiter"
    default:
      return role
  }
}

function getDisplayName(user: { name: string | null; email: string }): string {
  return user.name ?? user.email.split("@")[0] ?? "User"
}

function formatInviteExpiry(expiresAt: string): string {
  return new Intl.DateTimeFormat("de-DE", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(expiresAt))
}

function PendingInviteRow({ invite }: { invite: PendingInviteResponse }) {
  return (
    <div className="flex items-center justify-between gap-3 rounded-xl border border-border/70 bg-background/80 p-3 shadow-sm shadow-black/5 transition-colors hover:bg-background">
      <div className="flex min-w-0 items-center gap-3">
        <span className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-accent text-muted-foreground">
          <Mail className="h-4 w-4" />
        </span>
        <div className="min-w-0">
          <p className="truncate font-medium text-foreground">{invite.email}</p>
          <div className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground">
            <span className="inline-flex items-center gap-1">
              <Clock3 className="h-3.5 w-3.5" />
              Gueltig bis {formatInviteExpiry(invite.expires_at)}
            </span>
          </div>
        </div>
      </div>
      <div className="flex shrink-0 flex-col items-end gap-2 sm:flex-row sm:items-center">
        <Badge variant="secondary" className="border border-border/70 bg-accent/80 text-foreground">
          Ausstehend
        </Badge>
        <Badge variant={invite.role === "admin" ? "default" : "outline"} className="gap-1">
          {invite.role === "admin" ? <Shield className="h-3 w-3" /> : null}
          {getRoleLabel(invite.role)}
        </Badge>
      </div>
    </div>
  )
}

export function UserManagementSection({ isAdmin }: UserManagementSectionProps) {
  const { data: users, isLoading: isUsersLoading, error: usersError } = useUsers()
  const {
    data: pendingInvites,
    isLoading: isPendingInvitesLoading,
    error: pendingInvitesError,
  } = usePendingInvites()
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
            <CardDescription className="mt-1.5">
              Verwalten Sie die Benutzer Ihrer Organisation
            </CardDescription>
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
        {(isUsersLoading || isPendingInvitesLoading) && isAuthenticated && (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
          </div>
        )}

        {pendingInvites && pendingInvites.length > 0 && (
          <div className="rounded-2xl border border-border/70 bg-accent/35 p-4 shadow-sm shadow-black/5">
            <div className="flex flex-wrap items-center justify-between gap-3">
              <div>
                <p className="text-sm font-semibold text-foreground">Ausstehende Einladungen</p>
                <p className="mt-1 text-sm text-muted-foreground">
                  Diese Personen wurden eingeladen, haben den Beitritt aber noch nicht abgeschlossen.
                </p>
              </div>
              <Badge variant="secondary" className="rounded-full px-3 py-1 text-xs font-medium">
                {pendingInvites.length} offen
              </Badge>
            </div>

            <div className="mt-4 space-y-3">
              {pendingInvites.map((invite) => (
                <PendingInviteRow key={invite.id} invite={invite} />
              ))}
            </div>
          </div>
        )}

        {pendingInvitesError && (
          <div className="rounded-xl border border-warning/30 bg-warning/10 p-3 text-sm text-foreground">
            Ausstehende Einladungen konnten nicht geladen werden.
          </div>
        )}

        {usersError && (
          <div className="text-center py-8 text-destructive">
            Benutzer konnten nicht geladen werden
          </div>
        )}

        {users && users.length === 0 && (!pendingInvites || pendingInvites.length === 0) && (
          <div className="text-center py-8 text-muted-foreground">
            Keine Benutzer gefunden
          </div>
        )}

        {users && users.length > 0 && (
          <div className="space-y-1">
            {users.map((apiUser, index) => (
              <div key={apiUser.id}>
                {index > 0 && <Separator className="my-3" />}
                <div className="-mx-2 flex flex-col items-start gap-3 rounded-lg p-2 transition-colors hover:bg-accent/30 sm:flex-row sm:items-center sm:justify-between">
                  <div className="flex min-w-0 items-center gap-3 self-stretch sm:self-auto">
                    <Avatar className="h-10 w-10 flex-shrink-0">
                      <AvatarFallback className="bg-accent text-xs font-medium text-foreground">
                        {getInitials(apiUser.name)}
                      </AvatarFallback>
                    </Avatar>
                    <div className="min-w-0">
                      <p className="truncate font-medium">{getDisplayName(apiUser)}</p>
                      <p className="truncate text-sm text-muted-foreground">{apiUser.email}</p>
                    </div>
                  </div>
                  <Badge
                    variant={apiUser.role === "admin" ? "default" : "outline"}
                    className="gap-1 sm:flex-shrink-0"
                  >
                    {apiUser.role === "admin" ? <Shield className="h-3 w-3" /> : null}
                    {getRoleLabel(apiUser.role)}
                  </Badge>
                </div>
              </div>
            ))}
          </div>
        )}

        <InviteUserDialog open={showInviteDialog} onOpenChange={setShowInviteDialog} />
      </CardContent>
    </Card>
  )
}
