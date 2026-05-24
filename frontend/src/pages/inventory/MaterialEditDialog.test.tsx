import { describe, it, expect, vi } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { http, HttpResponse } from 'msw'
import { render } from '@/test/utils'
import { server } from '@/test/mocks/server'
import { createMaterial } from '@/test/factories'
import { MaterialEditDialog } from './MaterialEditDialog'

const apiRoute = (path: string) => `*/api/v1${path}`

describe('MaterialEditDialog', () => {
  it('submits updated billing defaults', async () => {
    const user = userEvent.setup()
    const material = createMaterial({
      id: 'mat-1',
      base_price_cents: 1000,
      price_markup_percentage: 10,
    })
    let submittedPayload: Record<string, unknown> | null = null

    server.use(
      http.patch(apiRoute('/inventory/materials/mat-1'), async ({ request }) => {
        submittedPayload = await request.json() as Record<string, unknown>
        return HttpResponse.json({ ...material, ...submittedPayload })
      })
    )

    render(
      <MaterialEditDialog open={true} onOpenChange={vi.fn()} material={material} />
    )

    await user.clear(screen.getByLabelText(/basispreis/i))
    await user.type(screen.getByLabelText(/basispreis/i), '25.50')
    await user.clear(screen.getByLabelText(/aufschlag/i))
    await user.type(screen.getByLabelText(/aufschlag/i), '18')
    await user.click(screen.getByRole('button', { name: /änderungen speichern/i }))

    await waitFor(() => {
      expect(submittedPayload).toMatchObject({
        min_quantity: material.min_quantity,
        location: material.location,
        base_price_cents: 2550,
        price_markup_percentage: 18,
      })
    })
  })

  it('clears billing defaults when the inputs are emptied', async () => {
    const user = userEvent.setup()
    const material = createMaterial({
      id: 'mat-2',
      base_price_cents: 1000,
      price_markup_percentage: 10,
    })
    let submittedPayload: Record<string, unknown> | null = null

    server.use(
      http.patch(apiRoute('/inventory/materials/mat-2'), async ({ request }) => {
        submittedPayload = await request.json() as Record<string, unknown>
        return HttpResponse.json({ ...material, ...submittedPayload })
      })
    )

    render(
      <MaterialEditDialog open={true} onOpenChange={vi.fn()} material={material} />
    )

    await user.clear(screen.getByLabelText(/basispreis/i))
    await user.clear(screen.getByLabelText(/aufschlag/i))
    await user.click(screen.getByRole('button', { name: /änderungen speichern/i }))

    await waitFor(() => {
      expect(submittedPayload).toMatchObject({
        clear_base_price_cents: true,
        clear_price_markup_percentage: true,
      })
    })
  })
})
