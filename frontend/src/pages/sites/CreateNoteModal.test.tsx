import { describe, expect, it, vi, beforeEach } from 'vitest'
import userEvent from '@testing-library/user-event'
import { render, screen, waitFor } from '@/test/utils'
import { CreateNoteModal } from './CreateNoteModal'

const createActivityMutate = vi.fn()
const uploadPhotoMutate = vi.fn()
const queuePhotoUploadAction = vi.fn()

vi.mock('@/lib/api/hooks', () => ({
  useCreateActivity: () => ({ isPending: false, mutateAsync: createActivityMutate }),
  useUploadSitePhoto: () => ({ isPending: false, mutateAsync: uploadPhotoMutate }),
}))

vi.mock('@/lib/offline/sync', () => ({
  isOnline: vi.fn(),
}))

vi.mock('@/lib/offline/queue', () => ({
  queuePhotoUploadAction: (...args: unknown[]) => queuePhotoUploadAction(...args),
}))

import { isOnline } from '@/lib/offline/sync'

describe('CreateNoteModal offline photo flow', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('queues photo upload while offline instead of uploading immediately', async () => {
    vi.mocked(isOnline).mockReturnValue(false)

    const onSuccess = vi.fn()
    const onOpenChange = vi.fn()
    const { container } = render(
      <CreateNoteModal
        open
        onOpenChange={onOpenChange}
        siteId="site-1"
        onSuccess={onSuccess}
      />
    )

    await userEvent.click(screen.getByRole('button', { name: 'Foto' }))

    const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement
    const file = new File(['data'], 'offline.jpg', { type: 'image/jpeg' })
    await userEvent.upload(fileInput, file)

    await userEvent.click(screen.getByRole('button', { name: 'Speichern' }))

    await waitFor(() => {
      expect(queuePhotoUploadAction).toHaveBeenCalledWith({
        siteId: 'site-1',
        file,
        content: undefined,
      })
    })

    expect(uploadPhotoMutate).not.toHaveBeenCalled()
    expect(createActivityMutate).not.toHaveBeenCalled()
    expect(onSuccess).toHaveBeenCalledOnce()
    expect(onOpenChange).toHaveBeenCalledWith(false)
  })
})
