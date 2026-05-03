import { WifiOff, RefreshCw } from 'lucide-react'
import { useOfflineSync } from '@/hooks/useOfflineSync'

export default function OfflineIndicator() {
  const { online, syncing } = useOfflineSync()

  if (online && !syncing) return null

  return (
    <div className="fixed bottom-16 md:bottom-4 left-4 z-50">
      <div
        className={`flex items-center gap-2.5 px-4 py-2.5 rounded-xl shadow-sm text-sm font-medium ${
          online
            ? 'bg-primary text-primary-foreground'
            : 'bg-destructive text-destructive-foreground'
        }`}
      >
        {syncing ? (
          <>
            <RefreshCw className="h-4 w-4 animate-spin" />
            <span>Synchronisiere...</span>
          </>
        ) : (
          <>
            <WifiOff className="h-4 w-4" />
            <span>Offline</span>
          </>
        )}
      </div>
    </div>
  )
}
