import { RefreshCw } from 'lucide-react'
import { useOfflineSync } from '@/hooks/useOfflineSync'
import { Button } from '@/components/ui/button'

export default function SyncButton() {
  const { online, syncing, sync } = useOfflineSync()

  return (
    <Button
      variant="ghost"
      size="icon"
      onClick={sync}
      disabled={!online || syncing}
      title={online ? 'Synchronisieren' : 'Offline'}
    >
      <RefreshCw className={`h-4 w-4 ${syncing ? 'animate-spin' : ''}`} />
    </Button>
  )
}
