import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import QrScanner from '@/components/qr/QrScanner'
import QrResultDialog from '@/components/qr/QrResultDialog'

export default function ScanPage() {
  const navigate = useNavigate()
  const [scannedCode, setScannedCode] = useState<string | null>(null)
  const [scannerOpen, setScannerOpen] = useState(true)

  const handleScan = (code: string) => {
    setScannedCode(code)
    setScannerOpen(false)
  }

  const handleClose = () => {
    navigate(-1)
  }

  return (
    <>
      {scannerOpen && (
        <QrScanner onScan={handleScan} onClose={handleClose} />
      )}
      <QrResultDialog
        qrCode={scannedCode}
        onClose={() => {
          setScannedCode(null)
          navigate(-1)
        }}
      />
    </>
  )
}
