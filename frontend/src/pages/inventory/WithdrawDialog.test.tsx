import { describe, expect, it, vi } from 'vitest'
import { render, screen, waitFor } from '@/test/utils'
import userEvent from '@testing-library/user-event'
import { WithdrawDialog } from './WithdrawDialog'

const material = {
  id: 'mat-1',
  category_id: 'cat-1',
  name: 'Betonschraube',
  description: null,
  unit: 'Stück',
  quantity: 20,
  min_quantity: 5,
  can_expire: false,
  legacy_quantity: 20,
  expired_quantity: 0,
  expiring_soon_quantity: 0,
  next_expiry_on: null,
  expiry_batches: [],
  location: null,
  qr_code: null,
  is_low_stock: false,
  created_at: new Date().toISOString(),
}

const expiringMaterial = {
  ...material,
  can_expire: true,
  expired_quantity: 2,
  next_expiry_on: '2026-05-01',
}

const sites = [
  { id: 'site-1', name: 'Villa Müller', project_type: 'external_site' as const },
  { id: 'site-2', name: 'CNC Vorbereitung', project_type: 'internal_workshop' as const },
]

describe('WithdrawDialog', () => {
  it('blocks normal withdrawal until a project is selected', () => {
    render(
      <WithdrawDialog
        open={true}
        onOpenChange={() => {}}
        material={material as never}
        onConfirm={vi.fn()}
        isLoading={false}
        sites={sites}
        initialSiteId={null}
      />
    )

    expect(screen.getByLabelText(/projekt \*/i)).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /stück entnehmen/i })).toBeDisabled()
    expect(screen.getByText(/müssen einem projekt zugeordnet sein/i)).toBeInTheDocument()
  })

  it('uses the initial project and enables normal withdrawal immediately', () => {
    render(
      <WithdrawDialog
        open={true}
        onOpenChange={() => {}}
        material={material as never}
        onConfirm={vi.fn()}
        isLoading={false}
        sites={sites}
        initialSiteId={'site-2'}
      />
    )

    expect(screen.getByRole('button', { name: /stück entnehmen/i })).toBeEnabled()
    expect(screen.getByRole('option', { name: 'CNC Vorbereitung (Werkstatt)' })).toBeInTheDocument()
  })

  it('allows disposal without requiring a project', async () => {
    const onConfirm = vi.fn()
    const user = userEvent.setup()
    render(
      <WithdrawDialog
        open={true}
        onOpenChange={() => {}}
        material={expiringMaterial as never}
        onConfirm={onConfirm}
        isLoading={false}
        sites={sites}
        initialSiteId={null}
      />
    )

    await user.click(screen.getByText('Entsorgung'))
    const disposeButton = await waitFor(() => screen.getByRole('button', { name: /entsorgen/i }))
    expect(disposeButton).toBeEnabled()

    await user.click(disposeButton)
    expect(onConfirm).toHaveBeenCalledWith(1, undefined, null, true)
  })
})
