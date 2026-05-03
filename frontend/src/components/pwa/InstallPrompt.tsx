import { useState, useEffect } from 'react'
import { Download, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'

interface BeforeInstallPromptEvent extends Event {
  prompt: () => Promise<void>
  userChoice: Promise<{ outcome: 'accepted' | 'dismissed' }>
}

export default function InstallPrompt() {
  const [deferredPrompt, setDeferredPrompt] = useState<BeforeInstallPromptEvent | null>(null)
  const [showPrompt, setShowPrompt] = useState(false)

  useEffect(() => {
    const handler = (e: Event) => {
      e.preventDefault()
      setDeferredPrompt(e as BeforeInstallPromptEvent)
      setShowPrompt(true)
    }

    window.addEventListener('beforeinstallprompt', handler)

    return () => {
      window.removeEventListener('beforeinstallprompt', handler)
    }
  }, [])

  const handleInstall = async () => {
    if (!deferredPrompt) return

    deferredPrompt.prompt()
    const { outcome } = await deferredPrompt.userChoice

    if (outcome === 'accepted') {
      setShowPrompt(false)
    }

    setDeferredPrompt(null)
  }

  if (!showPrompt) return null

  return (
    <div className="fixed bottom-20 md:bottom-4 left-4 right-4 md:left-auto md:right-4 md:w-80 z-40">
      <Card className="shadow-lg">
        <CardContent className="flex items-center gap-4 p-4">
          <div className="rounded-2xl bg-accent p-2.5 flex-shrink-0">
            <Download className="h-5 w-5 text-primary" />
          </div>
          <div className="flex-1 min-w-0">
            <p className="font-display text-sm">App installieren</p>
            <p className="text-xs text-muted-foreground mt-0.5">
              Für bessere Leistung und Offline-Zugriff
            </p>
          </div>
          <div className="flex gap-1.5 flex-shrink-0">
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setShowPrompt(false)}
              className="h-10 w-10 rounded-lg active:scale-[0.97] transition-transform"
            >
              <X className="h-4 w-4" />
            </Button>
            <Button
              size="sm"
              onClick={handleInstall}
              className="h-10 rounded-lg active:scale-[0.97] transition-transform"
            >
              Installieren
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
