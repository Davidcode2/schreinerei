import { cacheMaterials, cacheSites, cacheVehicles, cacheTools } from './db'
import { processAction, getPendingActions } from './queue'
import { apiClient } from '@/lib/api/client'
import type { Material } from '@/types/inventory'
import type { Site } from '@/types/sites'
import type { Vehicle, Tool } from '@/types/fleet'
import { toast } from 'sonner'

let isSyncing = false
let lastSyncTime: Date | null = null

export function isOnline(): boolean {
  return navigator.onLine
}

export function getLastSyncTime(): Date | null {
  return lastSyncTime
}

// Sync all data from server to cache
export async function syncFromServer(): Promise<void> {
  if (!isOnline()) return

  try {
    const [materials, sites, vehicles, tools] = await Promise.all([
      apiClient.get<Material[]>('/api/v1/inventory/materials'),
      apiClient.get<Site[]>('/api/v1/sites'),
      apiClient.get<Vehicle[]>('/api/v1/fleet/vehicles'),
      apiClient.get<Tool[]>('/api/v1/fleet/tools')
    ])

    await Promise.all([
      cacheMaterials(materials),
      cacheSites(sites),
      cacheVehicles(vehicles),
      cacheTools(tools)
    ])

    lastSyncTime = new Date()
    toast.success('Daten synchronisiert')
  } catch (error) {
    console.error('Sync from server failed:', error)
    toast.error('Synchronisierung fehlgeschlagen', {
      description: 'Die Daten konnten nicht aktualisiert werden. Bitte versuchen Sie es später erneut.',
      action: {
        label: 'Erneut versuchen',
        onClick: () => fullSync()
      }
    })
    throw error
  }
}

// Sync pending actions to server
export async function syncPendingActions(): Promise<{ success: number; failed: number }> {
  if (!isOnline()) {
    return { success: 0, failed: 0 }
  }

  const actions = await getPendingActions()
  let success = 0
  let failed = 0

  for (const action of actions) {
    const result = await processAction(action)
    if (result) {
      success++
    } else {
      failed++
    }
  }

  if (success > 0) {
    toast.success(`${success} Änderung${success > 1 ? 'en' : ''} synchronisiert`)
  }
  if (failed > 0) {
    toast.error(`${failed} Änderung${failed > 1 ? 'en' : ''} konnten nicht synchronisiert werden`)
  }

  return { success, failed }
}

// Full sync: both directions
export async function fullSync(): Promise<void> {
  if (isSyncing || !isOnline()) return

  isSyncing = true
  toast.info('Synchronisierung gestartet...')

  try {
    await syncPendingActions()
    await syncFromServer()
  } catch (error) {
    // Error already toasted in syncFromServer
  } finally {
    isSyncing = false
  }
}

// Initialize sync on app load
export function initSync(): () => void {
  // Sync when online
  const handleOnline = () => {
    console.log('Back online, syncing...')
    fullSync()
  }

  window.addEventListener('online', handleOnline)

  // Initial sync if online
  if (isOnline()) {
    fullSync()
  }

  // Cleanup
  return () => {
    window.removeEventListener('online', handleOnline)
  }
}

// Check if currently syncing
export function getIsSyncing(): boolean {
  return isSyncing
}
