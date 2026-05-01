import { describe, expect, it, vi, beforeEach } from 'vitest'
import { initSync, syncPendingActions } from './sync'

const getPendingActions = vi.fn()
const processAction = vi.fn()

vi.mock('./queue', () => ({
  getPendingActions: () => getPendingActions(),
  processAction: (...args: unknown[]) => processAction(...args),
}))

vi.mock('./db', () => ({
  cacheMaterials: vi.fn(),
  cacheSites: vi.fn(),
  cacheVehicles: vi.fn(),
  cacheTools: vi.fn(),
}))

vi.mock('@/lib/api/client', () => ({
  apiClient: {
    get: vi.fn().mockResolvedValue([]),
  },
}))

vi.mock('sonner', () => ({
  toast: {
    success: vi.fn(),
    error: vi.fn(),
    info: vi.fn(),
  },
}))

describe('offline sync', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    Object.defineProperty(navigator, 'onLine', {
      configurable: true,
      get: () => true,
    })
  })

  it('processes queued actions when online event fires', async () => {
    getPendingActions.mockResolvedValue([{ id: 1 }, { id: 2 }])
    processAction.mockResolvedValue(true)

    const cleanup = initSync()
    window.dispatchEvent(new Event('online'))

    await new Promise(resolve => setTimeout(resolve, 0))

    expect(getPendingActions).toHaveBeenCalled()
    expect(processAction).toHaveBeenCalledTimes(2)
    cleanup()
  })

  it('returns success and failed counts for badge updates', async () => {
    getPendingActions.mockResolvedValue([{ id: 1 }, { id: 2 }, { id: 3 }])
    processAction.mockResolvedValueOnce(true).mockResolvedValueOnce(false).mockResolvedValueOnce(true)

    const result = await syncPendingActions()

    expect(result).toEqual({ success: 2, failed: 1 })
  })
})
