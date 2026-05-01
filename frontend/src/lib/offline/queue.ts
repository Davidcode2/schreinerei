import { db, type PendingAction, type PhotoUploadQueueData } from './db'
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
    await apiClient.post('/api/v1/time-entries', {
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
  },

  photo_upload: async (data) => {
    const payload = parsePhotoUploadPayload(data)
    const file = await fileFromDataUrl(payload.fileDataUrl, payload.mimeType, payload.fileName)
    const formData = new FormData()
    formData.append('file', file)

    const uploadResponse = await apiClient.post<{ photo_url: string }>(
      `/api/v1/sites/${payload.siteId}/attachments/photo`,
      formData
    )

    await apiClient.post(`/api/v1/sites/${payload.siteId}/activities`, {
      activity_type: 'photo',
      content: payload.content,
      photo_url: uploadResponse.photo_url,
    })
  }
}

function parsePhotoUploadPayload(data: Record<string, unknown>): PhotoUploadQueueData {
  const payload = data as Partial<PhotoUploadQueueData>
  if (
    payload.activityType !== 'photo' ||
    typeof payload.siteId !== 'string' ||
    typeof payload.fileDataUrl !== 'string' ||
    typeof payload.mimeType !== 'string'
  ) {
    throw new Error('Malformed photo upload queue payload')
  }

  return {
    siteId: payload.siteId,
    activityType: 'photo',
    content: typeof payload.content === 'string' ? payload.content : undefined,
    mimeType: payload.mimeType,
    fileName: typeof payload.fileName === 'string' ? payload.fileName : undefined,
    fileDataUrl: payload.fileDataUrl,
  }
}

async function fileFromDataUrl(dataUrl: string, mimeType: string, fileName?: string): Promise<File> {
  const base64Marker = 'base64,'
  const markerIndex = dataUrl.indexOf(base64Marker)
  if (markerIndex === -1) {
    throw new Error('Malformed file data URL')
  }

  const base64 = dataUrl.slice(markerIndex + base64Marker.length)
  const binary = atob(base64)
  const bytes = Uint8Array.from(binary, char => char.charCodeAt(0))
  const blob = new Blob([bytes], { type: mimeType })
  return new File([blob], fileName ?? 'offline-photo', { type: mimeType })
}

export async function serializeFileToDataUrl(file: File): Promise<string> {
  return await new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => {
      if (typeof reader.result === 'string') {
        resolve(reader.result)
        return
      }
      reject(new Error('Failed to serialize file'))
    }
    reader.onerror = () => reject(new Error('Failed to serialize file'))
    reader.readAsDataURL(file)
  })
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

export async function queuePhotoUploadAction(params: {
  siteId: string
  file: File
  content?: string
}): Promise<number> {
  const fileDataUrl = await serializeFileToDataUrl(params.file)

  return queueAction('photo_upload', {
    siteId: params.siteId,
    activityType: 'photo',
    content: params.content,
    mimeType: params.file.type,
    fileName: params.file.name,
    fileDataUrl,
  })
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
