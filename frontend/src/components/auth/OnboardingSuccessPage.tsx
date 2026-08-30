import { useEffect } from "react"
import { Navigate } from "react-router-dom"
import confetti from "canvas-confetti"
import { CheckCircle2 } from "lucide-react"
import { useAuth } from "@/hooks/useAuth"
import { startLogin } from "@/lib/auth/keycloak"

export function OnboardingSuccessPage() {
  const { isAuthenticated, isLoading } = useAuth()

  useEffect(() => {
    if (isLoading || isAuthenticated) return
    const end = Date.now() + 1200

    function frame() {
      confetti({
        particleCount: 4,
        angle: 60,
        spread: 55,
        origin: { x: 0, y: 0.7 },
        colors: ["#5b21b6", "#16a34a", "#fbbf24", "#e11d48"],
      })
      confetti({
        particleCount: 4,
        angle: 120,
        spread: 55,
        origin: { x: 1, y: 0.7 },
        colors: ["#5b21b6", "#16a34a", "#fbbf24", "#e11d48"],
      })
      if (Date.now() < end) requestAnimationFrame(frame)
    }

    frame()
  }, [isLoading, isAuthenticated])

  if (isLoading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="h-8 w-8 animate-spin rounded-full border-b-2 border-primary" />
      </div>
    )
  }

  if (isAuthenticated) {
    return <Navigate to="/" replace />
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-background p-4">
      <div className="w-full max-w-sm">
        <div className="mb-10 flex flex-col items-center">
          <div className="mb-5 flex h-14 w-14 items-center justify-center rounded-2xl bg-primary shadow-lg shadow-primary/20">
            <span className="font-display text-2xl font-bold text-primary-foreground">S</span>
          </div>
          <h1 className="font-display text-3xl tracking-tight">Schreinerei</h1>
          <p className="mt-2 text-sm text-muted-foreground">Baustellenverwaltung</p>
        </div>

        <div className="mb-8 flex flex-col items-center text-center">
          <span className="mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-emerald-50">
            <CheckCircle2 className="h-7 w-7 text-emerald-600" />
          </span>
          <h2 className="font-display text-2xl tracking-tight" role="heading" aria-level={1}>
            Willkommen bei Schreinerei
          </h2>
          <p className="mt-3 text-sm text-muted-foreground">
            Ihr Konto ist bereit. Melden Sie sich an, um Ihre Baustellenverwaltung zu starten.
          </p>
        </div>

        <button
          onClick={() => void startLogin()}
          className="w-full rounded-xl bg-primary px-4 py-3 text-sm font-medium text-primary-foreground shadow-sm transition-all hover:bg-primary/90 hover:shadow-md active:scale-[0.98]"
        >
          Zur Anmeldung
        </button>
      </div>
    </div>
  )
}
