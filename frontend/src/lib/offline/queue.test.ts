import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { queuePhotoUploadAction, getPendingActions, processAction } from './queue'
import { apiClient } from '@/lib/api/client'

const pendingStore: Array<Record<string, unknown>> = []

vi.mock('./db', () => ({
  db: {
    pendingActions: {
      add: vi.fn(async (action: Record<string, unknown>) => {
        const id = pendingStore.length + 1
        pendingStore.push({ ...action, id })
        return id
      }),
      orderBy: vi.fn(() => ({
        toArray: vi.fn(async () => [...pendingStore]),
      })),
      delete: vi.fn(async (id: number) => {
        const index = pendingStore.findIndex(entry => entry.id === id)
        if (index >= 0) pendingStore.splice(index, 1)
      }),
      get: vi.fn(async (id: number) => pendingStore.find(entry => entry.id === id)),
      update: vi.fn(async (id: number, payload: Record<string, unknown>) => {
        const index = pendingStore.findIndex(entry => entry.id === id)
        if (index >= 0) pendingStore[index] = { ...pendingStore[index], ...payload }
      }),
      clear: vi.fn(async () => {
        pendingStore.splice(0, pendingStore.length)
      }),
      count: vi.fn(async () => pendingStore.length),
    },
  },
}))

vi.mock('@/lib/api/client', () => ({
  apiClient: {
    post: vi.fn(),
  },
}))

describe('offline photo upload queue', () => {
  beforeEach(async () => {
    pendingStore.splice(0, pendingStore.length)
    vi.clearAllMocks()
  })

  afterEach(async () => {
    pendingStore.splice(0, pendingStore.length)
  })

  it('stores photo payload with metadata in queue storage', async () => {
    const file = new File(['img'], 'photo.jpg', { type: 'image/jpeg' })

    await queuePhotoUploadAction({
      siteId: 'site-1',
      content: 'Mangel dokumentiert',
      file,
    })

    const actions = await getPendingActions()
    expect(actions).toHaveLength(1)
    expect(actions[0]?.type).toBe('photo_upload')
    expect(actions[0]?.data).toMatchObject({
      siteId: 'site-1',
      content: 'Mangel dokumentiert',
      mimeType: 'image/jpeg',
    })
    expect((actions[0]?.data as { fileDataUrl?: string } | undefined)?.fileDataUrl).toMatch(/^data:image\/jpeg;base64,/)
  })

  it('processes deserialized queued photo upload after reload', async () => {
    vi.mocked(apiClient.post)
      .mockResolvedValueOnce({ photo_url: '/api/v1/sites/site-1/attachments/a-1' })
      .mockResolvedValueOnce({ id: 'act-1' })

    const deserializedAction = {
      id: 999,
      type: 'photo_upload',
      createdAt: new Date('2026-05-01T10:00:00.000Z'),
      retryCount: 0,
      data: {
        siteId: 'site-1',
        activityType: 'photo',
        content: 'Reload test',
        mimeType: 'image/jpeg',
        fileDataUrl: 'data:image/jpeg;base64,ZmFrZS1iaW5hcnk=',
      },
    } as never

    const processed = await processAction(deserializedAction)

    expect(processed).toBe(true)
    expect(apiClient.post).toHaveBeenNthCalledWith(
      1,
      '/api/v1/sites/site-1/attachments/photo',
      expect.any(FormData)
    )
    const firstCall = vi.mocked(apiClient.post).mock.calls[0]
    expect(firstCall).toBeDefined()
    const uploadFormData = firstCall?.[1] as FormData
    expect(uploadFormData.get('photo')).toBeInstanceOf(File)
    expect(uploadFormData.get('file')).toBeNull()
    expect(apiClient.post).toHaveBeenNthCalledWith(
      2,
      '/api/v1/sites/site-1/activities',
      expect.objectContaining({
        activity_type: 'photo',
        content: 'Reload test',
        photo_url: '/api/v1/sites/site-1/attachments/a-1',
      })
    )
  })
})
