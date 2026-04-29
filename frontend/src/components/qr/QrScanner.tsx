import { useEffect, useRef, useState } from 'react'
import { Html5Qrcode } from 'html5-qrcode'
import { Button } from '@/components/ui/button'
import { AlertCircle } from 'lucide-react'

interface QrScannerProps {
  onScan: (result: string) => void
  onClose: () => void
}

export default function QrScanner({ onScan, onClose }: QrScannerProps) {
  const [error, setError] = useState<string | null>(null)
  const [scanning, setScanning] = useState(true)
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
      setError('Kamera-Zugriff verweigert oder nicht verfügbar')
      console.error('QR Scanner error:', err)
    })

    return () => {
      if (scannerRef.current && scanning) {
        scannerRef.current.stop().catch(() => {})
      }
    }
  }, [scanning, onScan])

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
        <div className="absolute bottom-20 left-4 right-4 bg-destructive text-white p-4 rounded-lg flex items-center gap-2">
          <AlertCircle className="h-5 w-5" />
          <p>{error}</p>
        </div>
      )}

      <div className="absolute bottom-4 left-0 right-0 text-center text-white/60 text-sm">
        Halten Sie die Kamera auf einen QR-Code
      </div>
    </div>
  )
}
