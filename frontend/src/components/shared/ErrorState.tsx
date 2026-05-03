import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { AlertCircle, RefreshCw } from "lucide-react"

interface ErrorStateProps {
  message: string
  onRetry?: () => void
}

export function ErrorState({ message, onRetry }: ErrorStateProps) {
  return (
    <Card className="border-destructive/30 bg-destructive/5">
      <CardContent className="flex flex-col items-center justify-center py-16 text-center">
        <div className="rounded-2xl bg-destructive/10 p-4 mb-5">
          <AlertCircle className="h-8 w-8 text-destructive" />
        </div>
        <h3 className="font-display text-xl mb-2">Fehler aufgetreten</h3>
        <p className="text-sm text-muted-foreground mb-5 max-w-sm leading-relaxed">
          {message}
        </p>
        {onRetry && (
          <Button variant="outline" onClick={onRetry} className="gap-2 h-10">
            <RefreshCw className="h-4 w-4" />
            Erneut versuchen
          </Button>
        )}
      </CardContent>
    </Card>
  )
}
