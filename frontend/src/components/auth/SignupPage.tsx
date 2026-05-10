import { Link, useSearchParams } from "react-router-dom"
import { Building2, ChevronLeft, Loader2, Mail } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { usePublicInvite } from "@/lib/api/hooks"
import { startLogin } from "@/lib/auth/keycloak"

export function SignupPage() {
  const [searchParams] = useSearchParams()
  const inviteToken = searchParams.get("invite")
  const { data: invite, isLoading, error } = usePublicInvite(inviteToken)

  if (inviteToken) {
    const isPendingInvite = invite?.status === "pending"

    return (
      <div className="min-h-screen bg-background p-4">
        <div className="mx-auto flex min-h-screen w-full max-w-md flex-col justify-center">
          <Link
            to="/login"
            className="mb-6 inline-flex items-center gap-2 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground"
          >
            <ChevronLeft className="h-4 w-4" />
            Zur Anmeldung
          </Link>

          <Card>
            <CardHeader className="space-y-3">
              <span className="flex h-11 w-11 items-center justify-center rounded-lg bg-accent">
                <Mail className="h-5 w-5" />
              </span>
              <div>
                <CardTitle className="font-display text-2xl" role="heading" aria-level={1}>
                  Einladung annehmen
                </CardTitle>
                <CardDescription className="mt-2">
                  Melden Sie sich mit der eingeladenen E-Mail-Adresse an.
                </CardDescription>
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              {isLoading && (
                <div className="flex items-center justify-center py-8">
                  <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
                </div>
              )}

              {error && (
                <div className="rounded-lg border border-destructive/30 bg-destructive/10 p-3 text-sm text-destructive">
                  Einladung konnte nicht geladen werden.
                </div>
              )}

              {invite && (
                <>
                  <div className="space-y-2">
                    <Label>E-Mail-Adresse</Label>
                    <Input value={invite.email} readOnly className="h-10 bg-background" />
                  </div>
                  <div className="text-sm text-muted-foreground">
                    Gueltig bis {new Date(invite.expires_at).toLocaleString("de-DE")}
                  </div>
                  <Button
                    className="w-full gap-2"
                    disabled={!isPendingInvite}
                    onClick={() => void startLogin()}
                  >
                    <Mail className="h-4 w-4" />
                    {isPendingInvite ? "Weiter zur Registrierung" : "Einladung abgelaufen"}
                  </Button>
                </>
              )}
            </CardContent>
          </Card>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-background p-4">
      <div className="mx-auto flex min-h-screen w-full max-w-md flex-col justify-center">
        <Link
          to="/login"
          className="mb-6 inline-flex items-center gap-2 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground"
        >
          <ChevronLeft className="h-4 w-4" />
          Zur Anmeldung
        </Link>

        <Card>
          <CardHeader className="space-y-3">
            <span className="flex h-11 w-11 items-center justify-center rounded-lg bg-accent">
              <Building2 className="h-5 w-5" />
            </span>
            <div>
              <CardTitle className="font-display text-2xl" role="heading" aria-level={1}>
                Organisation erstellen
              </CardTitle>
              <CardDescription className="mt-2">
                Der erste Schritt fuer neue Kunden wird hier angebunden.
              </CardDescription>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="organization-name">Organisationsname</Label>
              <Input id="organization-name" placeholder="Schreinerei Beispiel" disabled />
            </div>
            <div className="space-y-2">
              <Label htmlFor="admin-email">E-Mail des Admins</Label>
              <Input id="admin-email" type="email" placeholder="name@betrieb.de" disabled />
            </div>
            <Button className="w-full gap-2" disabled>
              <Mail className="h-4 w-4" />
              Onboarding starten
            </Button>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
