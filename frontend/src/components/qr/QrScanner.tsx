import { useEffect, useRef, useState } from 'react'
import { Html5Qrcode } from 'html5-qrcode'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { AlertCircle } from 'lucide-react'

interface QrScannerProps {
  onScan: (result: string) => void
  onClose: () => void
}

export default function QrScanner({ onScan, onClose }: QrScannerProps) {
  const [error, setError] = useState<string | null>(null)
  const [scanning, setScanning] = useState(true)
  const [retryCount, setRetryCount] = useState(0)
  const [manualCode, setManualCode] = useState('')
  const [showManualEntry, setShowManualEntry] = useState(false)
  const scannerRef = useRef<Html5Qrcode | null>(null)

  useEffect(() => {
    if (!scanning) return

    const scanner = new Html5Qrcode('qr-reader')
    scannerRef.current = scanner

    scanner.start(
      { facingMode: 'environment' },
      {
        fps: 10,
        qrbox: { width: 250, height: 250 }
      },
      (decodedText) => {
        scanner.stop()
        setScanning(false)
        onScan(decodedText)
      },
      () => {} // Ignore scan errors (no QR found)
    ).catch((err) => {
      setError('Kamera-Zugriff verweigert. Bitte Kamera-Berechtigung erteilen oder Code manuell eingeben.')
      console.error('QR Scanner error:', err)
    })

    return () => {
      if (scannerRef.current && scanning) {
        scannerRef.current.stop().catch(() => {})
      }
    }
  }, [scanning, onScan, retryCount])

  const handleRetry = () => {
    setError(null)
    setRetryCount(c => c + 1)
  }

  const handleManualSubmit = () => {
    if (manualCode.trim()) {
      onScan(manualCode.trim())
    }
  }

  return (
    <div className="fixed inset-0 bg-black z-50">
      <div className="absolute top-0 left-0 right-0 bg-gradient-to-b from-black/80 to-transparent p-4 pb-8 z-10">
        <div className="flex justify-between items-center">
          <h2 className="text-white font-display text-xl">QR-Code scannen</h2>
          <Button
            variant="ghost"
            onClick={onClose}
            className="text-white h-10 rounded-lg active:scale-[0.97] transition-transform"
          >
            Abbrechen
          </Button>
        </div>
      </div>

      <div
        id="qr-reader"
        className="w-full h-full"
      />

      {error && (
        <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/90 to-transparent p-6 pt-12 space-y-3">
          <div className="bg-destructive/90 backdrop-blur-sm text-white p-4 rounded-xl flex items-center gap-3 shadow-sm">
            <div className="rounded-lg bg-white/20 p-1.5 flex-shrink-0">
              <AlertCircle className="h-4 w-4" />
            </div>
            <p className="text-sm leading-relaxed">{error}</p>
          </div>
          <Button
            onClick={handleRetry}
            className="w-full h-12 rounded-lg text-base active:scale-[0.97] transition-transform"
          >
            Erneut versuchen
          </Button>
          <Button
            variant="outline"
            onClick={() => setShowManualEntry(true)}
            className="w-full h-12 rounded-lg text-base bg-white text-black active:scale-[0.97] transition-transform"
          >
            Code manuell eingeben
          </Button>
        </div>
      )}

      {showManualEntry && !error && (
        <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/90 to-transparent p-6 pt-12 space-y-3">
          <div className="bg-card backdrop-blur-sm p-5 rounded-xl space-y-3 shadow-sm">
            <p className="text-sm text-muted-foreground">Geben Sie den QR-Code manuell ein:</p>
            <Input
              value={manualCode}
              onChange={(e) => setManualCode(e.target.value)}
              placeholder="QR-Code eingeben..."
              className="w-full h-12 rounded-lg text-base"
              onKeyDown={(e) => e.key === 'Enter' && handleManualSubmit()}
            />
            <Button
              onClick={handleManualSubmit}
              className="w-full h-12 rounded-lg text-base active:scale-[0.97] transition-transform"
            >
              Bestätigen
            </Button>
          </div>
        </div>
      )}

      {!error && !showManualEntry && (
        <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent pb-6 pt-12 text-center">
          <p className="text-white/60 text-sm">Halten Sie die Kamera auf einen QR-Code</p>
        </div>
      )}
    </div>
  )
}
