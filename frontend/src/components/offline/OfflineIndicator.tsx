import { WifiOff } from 'lucide-react'
import { useOfflineSync } from '@/hooks/useOfflineSync'
import { Badge } from '@/components/ui/badge'

export default function OfflineIndicator() {
  const { online, syncing } = useOfflineSync()

  if (online && !syncing) return null

  return (
    <div className="fixed bottom-16 md:bottom-4 left-4 z-50">
      <Badge
        variant={online ? 'default' : 'destructive'}
        className="flex items-center gap-2 px-3 py-2"
      >
        {syncing ? (
          <>
            <div className="h-3 w-3 animate-spin rounded-full border-2 border-current border-t-transparent" />
            <span>Synchronisiere...</span>
          </>
        ) : (
          <>
            <WifiOff className="h-4 w-4" />
            <span>Offline</span>
          </>
        )}
      </Badge>
    </div>
  )
}
