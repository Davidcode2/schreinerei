import { describe, expect, it, vi } from 'vitest'
import { render, screen } from '@/test/utils'
import userEvent from '@testing-library/user-event'
import PendingActionsBadge from './PendingActionsBadge'

vi.mock('@/hooks/useOfflineSync', () => ({
  useOfflineSync: vi.fn(),
}))

import { useOfflineSync } from '@/hooks/useOfflineSync'

describe('PendingActionsBadge', () => {
  it('shows queued action count', async () => {
    vi.mocked(useOfflineSync).mockReturnValue({ pendingCount: 2 } as never)
    render(<PendingActionsBadge />)
    expect(screen.getByText('2')).toBeInTheDocument()
    await userEvent.click(screen.getByRole('button'))
    expect(await screen.findByText(/auf Synchronisation/i)).toBeInTheDocument()
  })
})
