import { Clock } from 'lucide-react'
import { useOfflineSync } from '@/hooks/useOfflineSync'
import { Badge } from '@/components/ui/badge'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'

export default function PendingActionsBadge() {
  const { pendingCount } = useOfflineSync()

  if (pendingCount === 0) return null

  return (
    <Popover>
      <PopoverTrigger asChild>
        <button className="relative">
          <Badge variant="secondary" className="flex items-center gap-1">
            <Clock className="h-3 w-3" />
            <span>{pendingCount}</span>
          </Badge>
        </button>
      </PopoverTrigger>
      <PopoverContent className="w-64">
        <div className="space-y-2">
          <p className="text-sm font-medium">
            Ausstehende Aktionen
          </p>
          <p className="text-xs text-muted-foreground">
            {pendingCount} Aktion{pendingCount !== 1 ? 'en' : ''} wartet{pendingCount !== 1 ? 'en' : ''} auf Synchronisation.
          </p>
          <p className="text-xs text-muted-foreground">
            Werden automatisch gesendet wenn wieder online.
          </p>
        </div>
      </PopoverContent>
    </Popover>
  )
}
