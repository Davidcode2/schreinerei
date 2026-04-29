import { useState, useEffect, useCallback } from 'react'
import { isOnline, fullSync, getLastSyncTime, getIsSyncing } from '@/lib/offline/sync'
import { getPendingCount, subscribeToPendingCount } from '@/lib/offline/queue'

export function useOfflineSync() {
  const [online, setOnline] = useState(isOnline())
  const [pendingCount, setPendingCount] = useState(0)
  const [lastSync, setLastSync] = useState(getLastSyncTime())
  const [syncing, setSyncing] = useState(getIsSyncing())

  useEffect(() => {
    const handleOnline = () => setOnline(true)
    const handleOffline = () => setOnline(false)

    window.addEventListener('online', handleOnline)
    window.addEventListener('offline', handleOffline)

    const unsubscribe = subscribeToPendingCount(setPendingCount)
    getPendingCount().then(setPendingCount)

    return () => {
      window.removeEventListener('online', handleOnline)
      window.removeEventListener('offline', handleOffline)
      unsubscribe()
    }
  }, [])

  const sync = useCallback(async () => {
    if (!online || syncing) return

    setSyncing(true)
    try {
      await fullSync()
      setLastSync(getLastSyncTime())
      const count = await getPendingCount()
      setPendingCount(count)
    } finally {
      setSyncing(false)
    }
  }, [online, syncing])

  return {
    online,
    pendingCount,
    lastSync,
    syncing,
    sync
  }
}
