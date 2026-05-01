import Dexie, { type Table } from 'dexie'
import type { Material, Category } from '@/types/inventory'
import type { Site } from '@/types/sites'
import type { Vehicle, Tool, Reservation } from '@/types/fleet'

export interface PhotoUploadQueueData {
  siteId: string
  activityType: 'photo'
  content?: string
  mimeType: string
  fileName?: string
  fileDataUrl: string
}

export interface PendingAction {
  id?: number
  type: 'withdraw' | 'time_entry' | 'activity' | 'reservation' | 'photo_upload'
  data: Record<string, unknown> | PhotoUploadQueueData
  createdAt: Date
  retryCount: number
  lastError?: string
}

// Cached data with metadata
export interface CachedData<T> {
  id: string
  data: T
  cachedAt: Date
  expiresAt: Date
}

export class SchreinereiDB extends Dexie {
  materials!: Table<CachedData<Material>>
  categories!: Table<CachedData<Category>>
  sites!: Table<CachedData<Site>>
  vehicles!: Table<CachedData<Vehicle>>
  tools!: Table<CachedData<Tool>>
  reservations!: Table<CachedData<Reservation>>
  pendingActions!: Table<PendingAction>

  constructor() {
    super('SchreinereiDB')

    this.version(1).stores({
      materials: 'id, cachedAt',
      categories: 'id, cachedAt',
      sites: 'id, cachedAt',
      vehicles: 'id, cachedAt',
      tools: 'id, cachedAt',
      reservations: 'id, cachedAt',
      pendingActions: '++id, type, createdAt'
    })

    this.version(2).stores({
      materials: 'id, cachedAt',
      categories: 'id, cachedAt',
      sites: 'id, cachedAt',
      vehicles: 'id, cachedAt',
      tools: 'id, cachedAt',
      reservations: 'id, cachedAt',
      pendingActions: '++id, type, createdAt'
    })
  }
}

export const db = new SchreinereiDB()

// Helper functions
export const CACHE_DURATION = 24 * 60 * 60 * 1000 // 24 hours

export async function cacheMaterials(materials: Material[]) {
  const now = new Date()
  const expiresAt = new Date(now.getTime() + CACHE_DURATION)

  await db.materials.bulkPut(
    materials.map(m => ({
      id: m.id,
      data: m,
      cachedAt: now,
      expiresAt
    }))
  )
}

export async function getCachedMaterials(): Promise<Material[]> {
  const cached = await db.materials.toArray()
  const now = new Date()
  return cached
    .filter(c => c.expiresAt > now)
    .map(c => c.data)
}

export async function cacheCategories(categories: Category[]) {
  const now = new Date()
  const expiresAt = new Date(now.getTime() + CACHE_DURATION)

  await db.categories.bulkPut(
    categories.map(c => ({
      id: c.id,
      data: c,
      cachedAt: now,
      expiresAt
    }))
  )
}

export async function getCachedCategories(): Promise<Category[]> {
  const cached = await db.categories.toArray()
  const now = new Date()
  return cached
    .filter(c => c.expiresAt > now)
    .map(c => c.data)
}

export async function cacheSites(sites: Site[]) {
  const now = new Date()
  const expiresAt = new Date(now.getTime() + CACHE_DURATION)

  await db.sites.bulkPut(
    sites.map(s => ({
      id: s.id,
      data: s,
      cachedAt: now,
      expiresAt
    }))
  )
}

export async function getCachedSites(): Promise<Site[]> {
  const cached = await db.sites.toArray()
  const now = new Date()
  return cached
    .filter(c => c.expiresAt > now)
    .map(c => c.data)
}

export async function cacheVehicles(vehicles: Vehicle[]) {
  const now = new Date()
  const expiresAt = new Date(now.getTime() + CACHE_DURATION)

  await db.vehicles.bulkPut(
    vehicles.map(v => ({
      id: v.id,
      data: v,
      cachedAt: now,
      expiresAt
    }))
  )
}

export async function getCachedVehicles(): Promise<Vehicle[]> {
  const cached = await db.vehicles.toArray()
  const now = new Date()
  return cached
    .filter(c => c.expiresAt > now)
    .map(c => c.data)
}

export async function cacheTools(tools: Tool[]) {
  const now = new Date()
  const expiresAt = new Date(now.getTime() + CACHE_DURATION)

  await db.tools.bulkPut(
    tools.map(t => ({
      id: t.id,
      data: t,
      cachedAt: now,
      expiresAt
    }))
  )
}

export async function getCachedTools(): Promise<Tool[]> {
  const cached = await db.tools.toArray()
  const now = new Date()
  return cached
    .filter(c => c.expiresAt > now)
    .map(c => c.data)
}

// Clear all cached data (e.g., on logout)
export async function clearAllCache(): Promise<void> {
  await Promise.all([
    db.materials.clear(),
    db.categories.clear(),
    db.sites.clear(),
    db.vehicles.clear(),
    db.tools.clear(),
    db.reservations.clear()
  ])
}
