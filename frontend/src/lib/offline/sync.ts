import { db, cacheMaterials, cacheSites, cacheVehicles, cacheTools } from './db'
import { processAction, getPendingActions } from './queue'
import { apiClient } from '@/lib/api/client'
import type { Material, Category } from '@/types/inventory'
import type { Site } from '@/types/sites'
import type { Vehicle, Tool } from '@/types/fleet'

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
    // Fetch and cache all data in parallel
    const [materials, categories, sites, vehicles, tools] = await Promise.all([
      apiClient.get<Material[]>('/api/v1/inventory/materials'),
      apiClient.get<Category[]>('/api/v1/inventory/categories'),
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
  } catch (error) {
    console.error('Sync from server failed:', error)
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

  return { success, failed }
}

// Full sync: both directions
export async function fullSync(): Promise<void> {
  if (isSyncing || !isOnline()) return

  isSyncing = true

  try {
    // First, send pending actions
    await syncPendingActions()

    // Then, fetch fresh data
    await syncFromServer()
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
