import { describe, expect, it } from 'vitest'
import userEvent from '@testing-library/user-event'
import { http, HttpResponse } from 'msw'
import { render, screen, waitFor } from '@/test/utils'
import { server } from '@/test/mocks/server'
import { SitePlanningCalendar } from './SitePlanningCalendar'

describe('SitePlanningCalendar', () => {
  it('opens a new appointment draft from an empty calendar hour', async () => {
    const user = userEvent.setup()
    let submitted: Record<string, unknown> | null = null

    server.use(
      http.get('*/api/v1/sites/site-1/appointments*', () => HttpResponse.json([])),
      http.post('*/api/v1/sites/site-1/appointments', async ({ request }) => {
        submitted = (await request.json()) as Record<string, unknown>
        return HttpResponse.json({
          id: 'appointment-1',
          site_id: 'site-1',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          ...submitted,
        }, { status: 201 })
      })
    )

    render(<SitePlanningCalendar siteId="site-1" assignments={[]} canEdit />)

    const [slot] = await screen.findAllByRole('button', {
      name: /termin am .* um 11:00 erstellen/i,
    })
    expect(slot).toBeDefined()

    await user.click(slot!)

    expect(await screen.findByRole('heading', { name: /termin planen/i })).toBeInTheDocument()
    expect((screen.getByLabelText(/start/i) as HTMLInputElement).value).toMatch(/T11:00$/)
    expect((screen.getByLabelText(/ende/i) as HTMLInputElement).value).toMatch(/T13:00$/)

    await user.type(screen.getByLabelText(/titel/i), 'Aufmaß')
    await user.click(screen.getByRole('button', { name: /anlegen/i }))

    await waitFor(() => {
      expect(submitted).toMatchObject({
        title: 'Aufmaß',
        appointment_kind: 'customer_appointment',
        assigned_user_ids: [],
        notes: null,
      })
    })
  })
})
