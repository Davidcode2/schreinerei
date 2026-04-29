import { Link } from "react-router-dom"
import { Button } from "@/components/ui/button"
import { Home } from "lucide-react"

export default function NotFoundPage() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] space-y-4 text-center">
      <div className="space-y-2">
        <h1 className="text-6xl font-bold text-primary">404</h1>
        <h2 className="text-2xl font-semibold">Seite nicht gefunden</h2>
        <p className="text-muted-foreground">
          Die angeforderte Seite existiert nicht oder wurde verschoben.
        </p>
      </div>
      <Button asChild className="gap-2">
        <Link to="/">
          <Home className="h-4 w-4" />
          Zurück zum Dashboard
        </Link>
      </Button>
    </div>
  )
}
