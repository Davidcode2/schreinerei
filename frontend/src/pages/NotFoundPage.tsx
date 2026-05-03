import { Link } from "react-router-dom"
import { Button } from "@/components/ui/button"
import { Home } from "lucide-react"

export default function NotFoundPage() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] space-y-6 text-center">
      <div className="rounded-2xl bg-accent/60 p-5">
        <span className="font-display text-6xl text-primary">404</span>
      </div>
      <div className="space-y-2">
        <h1 className="font-display text-2xl md:text-3xl tracking-tight">Seite nicht gefunden</h1>
        <p className="text-muted-foreground max-w-sm leading-relaxed">
          Die angeforderte Seite existiert nicht oder wurde verschoben.
        </p>
      </div>
      <Button asChild className="gap-2 rounded-lg active:scale-[0.97] transition-transform">
        <Link to="/">
          <Home className="h-4 w-4" />
          Zurück zum Dashboard
        </Link>
      </Button>
    </div>
  )
}
