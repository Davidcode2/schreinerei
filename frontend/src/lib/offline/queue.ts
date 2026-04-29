import { db, type PendingAction } from './db'
import { apiClient } from '@/lib/api/client'

const MAX_RETRIES = 3

// Action handlers map action type to API call
const actionHandlers: Record<string, (data: Record<string, unknown>) => Promise<void>> = {
  withdraw: async (data) => {
    await apiClient.post(`/api/v1/inventory/materials/${data.materialId}/withdraw`, {
      quantity: data.quantity,
      notes: data.notes
    })
  },

  time_entry: async (data) => {
    await apiClient.post('/api/v1/sites/time-entries', {
      site_id: data.siteId,
      work_type: data.workType,
      hours: data.hours,
      work_date: data.workDate,
      notes: data.notes
    })
  },

  activity: async (data) => {
    await apiClient.post(`/api/v1/sites/${data.siteId}/activities`, {
      activity_type: data.activityType,
      content: data.content,
      photo_url: data.photoUrl
    })
  },

  reservation: async (data) => {
    await apiClient.post('/api/v1/fleet/reservations', {
      resource_type: data.resourceType,
      resource_id: data.resourceId,
      site_id: data.siteId,
      start_time: data.startTime,
      end_time: data.endTime,
      notes: data.notes
    })
  }
}

// Add action to queue
export async function queueAction(
  type: PendingAction['type'],
  data: Record<string, unknown>
): Promise<number> {
  const action: PendingAction = {
    type,
    data,
    createdAt: new Date(),
    retryCount: 0
  }

  return await db.pendingActions.add(action)
}

// Get all pending actions
export async function getPendingActions(): Promise<PendingAction[]> {
  return await db.pendingActions.orderBy('createdAt').toArray()
}

// Get count of pending actions
export async function getPendingCount(): Promise<number> {
  return await db.pendingActions.count()
}

// Remove action from queue
export async function removeAction(id: number): Promise<void> {
  await db.pendingActions.delete(id)
}

// Update action error
export async function updateActionError(
  id: number,
  error: string,
  incrementRetry: boolean = true
): Promise<void> {
  const action = await db.pendingActions.get(id)
  if (!action) return

  await db.pendingActions.update(id, {
    lastError: error,
    retryCount: incrementRetry ? action.retryCount + 1 : action.retryCount
  })
}

// Process a single action
export async function processAction(action: PendingAction): Promise<boolean> {
  const handler = actionHandlers[action.type]
  if (!handler) {
    await removeAction(action.id!)
    return false
  }

  try {
    await handler(action.data)
    await removeAction(action.id!)
    return true
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error'

    if (action.retryCount >= MAX_RETRIES) {
      // Keep in queue but don't retry automatically
      await updateActionError(action.id!, errorMessage, false)
      return false
    }

    await updateActionError(action.id!, errorMessage)
    return false
  }
}

// Subscribe to pending count changes
export function subscribeToPendingCount(callback: (count: number) => void): () => void {
  // Initial count
  db.pendingActions.count().then(callback)

  // Dexie doesn't have built-in live queries for count
  // We'll poll every 5 seconds
  const interval = setInterval(() => {
    db.pendingActions.count().then(callback)
  }, 5000)

  return () => clearInterval(interval)
}

// Clear all pending actions (for cleanup)
export async function clearPendingActions(): Promise<void> {
  await db.pendingActions.clear()
}
