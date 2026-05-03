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
      className="h-10 w-10 rounded-lg active:scale-[0.97] transition-transform"
    >
      <RefreshCw className={`h-4 w-4 ${syncing ? 'animate-spin' : ''}`} />
    </Button>
  )
}
