import { Link, useSearchParams } from "react-router-dom"
import { AlertTriangle, Building2, CheckCircle2, ChevronLeft, Loader2, Mail } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { useOnboardingSession } from "@/lib/api/hooks"
import { startLogin } from "@/lib/auth/keycloak"

const STATUS_COPY: Record<string, string> = {
  pending_payment: "Zahlung wird bestaetigt.",
  payment_confirmed: "Zahlung bestaetigt, Organisation wird vorbereitet.",
  provisioning: "Organisation und Einladung werden erstellt.",
  completed: "Ihre Organisation ist bereit.",
  payment_failed: "Die Zahlung wurde nicht bestaetigt.",
  keycloak_failed: "Die Organisation konnte nicht erstellt werden.",
}

function statusMessage(status?: string) {
  return status ? STATUS_COPY[status] ?? "Status wird geladen." : "Status wird geladen."
}

export function OnboardingCompletePage() {
  const [searchParams] = useSearchParams()
  const sessionId = searchParams.get("session")
  const { data, isLoading, error } = useOnboardingSession(sessionId)

  const isCompleted = data?.status === "completed"
  const isFailed = data?.status === "payment_failed" || data?.status === "keycloak_failed"

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
              {isCompleted ? (
                <CheckCircle2 className="h-5 w-5 text-emerald-600" />
              ) : isFailed ? (
                <AlertTriangle className="h-5 w-5 text-destructive" />
              ) : (
                <Building2 className="h-5 w-5" />
              )}
            </span>
            <div>
              <CardTitle className="font-display text-2xl" role="heading" aria-level={1}>
                Onboarding abschliessen
              </CardTitle>
              <CardDescription className="mt-2">
                {sessionId
                  ? "Wir pruefen den Status Ihrer neuen Organisation."
                  : "Die Rueckkehr aus dem Checkout enthaelt keine Session-ID."}
              </CardDescription>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            {!sessionId && (
              <div className="rounded-lg border border-destructive/30 bg-destructive/10 p-3 text-sm text-destructive">
                Session-ID fehlt. Starten Sie das Onboarding erneut.
              </div>
            )}

            {sessionId && isLoading && (
              <div className="flex items-center justify-center gap-3 rounded-lg border bg-muted/40 p-4 text-sm text-muted-foreground">
                <Loader2 className="h-4 w-4 animate-spin" />
                Status wird geladen
              </div>
            )}

            {sessionId && error && (
              <div className="rounded-lg border border-destructive/30 bg-destructive/10 p-3 text-sm text-destructive">
                {error.message || "Onboarding-Status konnte nicht geladen werden."}
              </div>
            )}

            {data && (
              <>
                <div className="rounded-lg border bg-muted/30 p-4">
                  <div className="text-sm font-medium">{statusMessage(data.status)}</div>
                  <div className="mt-1 text-sm text-muted-foreground">
                    Organisation: {data.organization_slug}
                  </div>
                  {data.error_message && (
                    <div className="mt-2 text-sm text-destructive">{data.error_message}</div>
                  )}
                </div>

                {isCompleted ? (
                  <Button className="w-full gap-2" onClick={() => void startLogin()}>
                    <Mail className="h-4 w-4" />
                    Weiter zur Anmeldung
                  </Button>
                ) : isFailed ? (
                  <Link to="/signup" className="block">
                    <Button className="w-full" variant="outline">
                      Onboarding erneut starten
                    </Button>
                  </Link>
                ) : (
                  <div className="flex items-center justify-center gap-3 rounded-lg border bg-muted/20 p-4 text-sm text-muted-foreground">
                    <Loader2 className="h-4 w-4 animate-spin" />
                    Wir aktualisieren den Status automatisch.
                  </div>
                )}
              </>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
