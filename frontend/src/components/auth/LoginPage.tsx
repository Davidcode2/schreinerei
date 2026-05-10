import { startLogin } from '../../lib/auth/keycloak'
import { Link } from 'react-router-dom'

export function LoginPage() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-4">
      <div className="max-w-sm w-full">
        <div className="flex flex-col items-center mb-10">
          <div className="flex h-14 w-14 items-center justify-center rounded-2xl bg-primary mb-5 shadow-lg shadow-primary/20">
            <span className="text-primary-foreground font-display text-2xl font-bold">S</span>
          </div>
          <h1 className="font-display text-3xl tracking-tight">Schreinerei</h1>
          <p className="text-sm text-muted-foreground mt-2">Baustellenverwaltung</p>
        </div>
        <button
          onClick={startLogin}
          className="w-full py-3 px-4 bg-primary text-primary-foreground rounded-xl font-medium text-sm hover:bg-primary/90 transition-all shadow-sm hover:shadow-md active:scale-[0.98]"
        >
          Mit Keycloak anmelden
        </button>
        <Link
          to="/signup"
          className="mt-3 block w-full rounded-xl border border-border px-4 py-3 text-center text-sm font-medium transition-all hover:bg-accent active:scale-[0.98]"
        >
          Neue Organisation erstellen
        </Link>
      </div>
    </div>
  )
}
