import { vi } from 'vitest'
import userEvent from '@testing-library/user-event'
import { render, screen } from '@/test/utils'

const scanner = vi.hoisted(() => ({
  start: vi.fn(),
  stop: vi.fn().mockResolvedValue(undefined),
}))

vi.mock('html5-qrcode', () => ({
  Html5Qrcode: class {
    start = scanner.start
    stop = scanner.stop
  },
}))

import QrScanner from './QrScanner'

describe('QrScanner', () => {
  it('opens manual entry after camera access fails', async () => {
    scanner.start.mockRejectedValueOnce(new Error('camera denied'))
    vi.spyOn(console, 'error').mockImplementation(() => {})
    const user = userEvent.setup()
    render(<QrScanner onScan={vi.fn()} onClose={vi.fn()} />)

    await user.click(await screen.findByRole('button', { name: 'Code manuell eingeben' }))

    expect(screen.getByPlaceholderText('QR-Code eingeben...')).toBeInTheDocument()
    expect(screen.queryByText(/Kamera-Zugriff verweigert/)).not.toBeInTheDocument()
  })
})
