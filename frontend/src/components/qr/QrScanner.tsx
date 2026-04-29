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
      <div className="absolute top-4 left-4 right-4 flex justify-between items-center z-10">
        <h2 className="text-white text-lg font-medium">QR-Code scannen</h2>
        <Button variant="ghost" onClick={onClose} className="text-white">
          Abbrechen
        </Button>
      </div>

      <div
        id="qr-reader"
        className="w-full h-full"
      />

      {error && (
        <div className="absolute bottom-20 left-4 right-4 space-y-3">
          <div className="bg-destructive text-white p-4 rounded-lg flex items-center gap-2">
            <AlertCircle className="h-5 w-5 flex-shrink-0" />
            <p>{error}</p>
          </div>
          <Button
            onClick={handleRetry}
            className="w-full"
          >
            Erneut versuchen
          </Button>
          <Button
            variant="outline"
            onClick={() => setShowManualEntry(true)}
            className="w-full bg-white text-black"
          >
            Code manuell eingeben
          </Button>
        </div>
      )}

      {showManualEntry && !error && (
        <div className="absolute bottom-20 left-4 right-4 space-y-3 bg-white p-4 rounded-lg">
          <p className="text-sm text-muted-foreground">Geben Sie den QR-Code manuell ein:</p>
          <Input
            value={manualCode}
            onChange={(e) => setManualCode(e.target.value)}
            placeholder="QR-Code eingeben..."
            className="w-full"
          />
          <Button onClick={handleManualSubmit} className="w-full">
            Bestätigen
          </Button>
        </div>
      )}

      {!error && !showManualEntry && (
        <div className="absolute bottom-4 left-0 right-0 text-center text-white/60 text-sm">
          Halten Sie die Kamera auf einen QR-Code
        </div>
      )}
    </div>
  )
}
