import { describe, expect, it, vi } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { http, HttpResponse } from 'msw'
import { render } from '@/test/utils'
import { server } from '@/test/mocks/server'
import { ProjectPlanningSheet } from './ProjectPlanningSheet'

const site = {
  id: 'site-1',
  project_type: 'external_site' as const,
  name: 'Villa Müller',
  customer_name: 'Familie Müller',
  location: 'Leipzig',
  description: 'Bestandsprojekt',
  status: 'planned' as const,
  start_date: null,
  end_date: null,
  estimated_days: 3,
  budget_amount_cents: 250000,
  billing_reference: 'BR-1',
  billing_notes: 'Teilrechnung nach Montage',
  quote_reference: 'ANG-2026-01',
  created_at: new Date().toISOString(),
}

describe('ProjectPlanningSheet', () => {
  it('saves updated project planning fields', async () => {
    const user = userEvent.setup()
    const onOpenChange = vi.fn()
    let submitted: Record<string, unknown> | null = null

    server.use(
      http.patch('*/api/v1/sites/site-1', async ({ request }) => {
        submitted = (await request.json()) as Record<string, unknown>
        return HttpResponse.json({ ...site, ...(submitted ?? {}) })
      })
    )

    render(<ProjectPlanningSheet open={true} onOpenChange={onOpenChange} site={site} />)

    await user.click(screen.getByRole('combobox'))
    await user.click(screen.getByRole('option', { name: /werkstatt intern/i }))
    await user.clear(screen.getByLabelText(/kunde/i))
    await user.clear(screen.getByLabelText(/geplante tage/i))
    await user.type(screen.getByLabelText(/geplante tage/i), '5')
    await user.clear(screen.getByLabelText(/budget/i))
    await user.type(screen.getByLabelText(/budget/i), '3000')
    await user.clear(screen.getByLabelText(/abrechnungsreferenz/i))
    await user.type(screen.getByLabelText(/abrechnungsreferenz/i), 'BR-2')
    await user.click(screen.getByRole('button', { name: /speichern/i }))

    await waitFor(() => {
      expect(submitted).toMatchObject({
        project_type: 'internal_workshop',
        estimated_days: 5,
        budget_amount_cents: 300000,
        billing_reference: 'BR-2',
      })
      expect(onOpenChange).toHaveBeenCalledWith(false)
    })
  })

  it('clears budget and billing metadata when fields are emptied', async () => {
    const user = userEvent.setup()
    let submitted: Record<string, unknown> | null = null

    server.use(
      http.patch('*/api/v1/sites/site-1', async ({ request }) => {
        submitted = (await request.json()) as Record<string, unknown>
        return HttpResponse.json({ ...site, ...(submitted ?? {}) })
      })
    )

    render(<ProjectPlanningSheet open={true} onOpenChange={vi.fn()} site={site} />)

    await user.clear(screen.getByLabelText(/budget/i))
    await user.clear(screen.getByLabelText(/angebotsreferenz/i))
    await user.clear(screen.getByLabelText(/abrechnungsreferenz/i))
    await user.clear(screen.getByLabelText(/abrechnungshinweise/i))
    await user.click(screen.getByRole('button', { name: /speichern/i }))

    await waitFor(() => {
      expect(submitted).toMatchObject({
        clear_budget_amount: true,
        clear_quote_reference: true,
        clear_billing_reference: true,
        clear_billing_notes: true,
      })
    })
  })
})
