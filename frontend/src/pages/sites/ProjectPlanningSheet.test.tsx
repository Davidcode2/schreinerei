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
    await user.click(screen.getByRole('button', { name: /speichern/i }))

    await waitFor(() => {
      expect(submitted).toMatchObject({
        project_type: 'internal_workshop',
        estimated_days: 5,
      })
      expect(onOpenChange).toHaveBeenCalledWith(false)
    })
  })
})
