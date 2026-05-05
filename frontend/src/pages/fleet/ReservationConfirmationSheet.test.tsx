import { describe, expect, it } from "vitest"
import { screen, waitFor } from '@testing-library/react'
import { render } from '@/test/utils'
import { mockData } from '@/test/mocks/handlers'
import { buildDefaultTimes } from "./ReservationConfirmationSheet"
import { ReservationConfirmationSheet } from './ReservationConfirmationSheet'

describe("buildDefaultTimes", () => {
  it("moves same-day quick bookings into the future", () => {
    const now = new Date(2026, 4, 3, 14, 10, 0)

    const defaults = buildDefaultTimes("2026-05-03", "2026-05-03", now)

    expect(defaults.start).toBe("2026-05-03T14:30")
    expect(defaults.end).toBe("2026-05-03T17:00")
  })

  it("extends the end time when the workday default has already passed", () => {
    const now = new Date(2026, 4, 3, 18, 20, 0)

    const defaults = buildDefaultTimes("2026-05-03", "2026-05-03", now)

    expect(defaults.start).toBe("2026-05-03T18:30")
    expect(defaults.end).toBe("2026-05-03T19:30")
  })
})

describe('ReservationConfirmationSheet', () => {
  it('shows project-aware selector labels', async () => {
    mockData.preferences = { active_site_id: 'site-1' }
    mockData.sites = [
      {
        id: 'site-1',
        project_type: 'internal_workshop',
        name: 'CNC Vorbereitung',
        customer_name: '',
        location: 'Werkstatt',
        description: null,
        status: 'planned',
        start_date: null,
        end_date: null,
        estimated_days: null,
        created_at: new Date().toISOString(),
      },
    ]

    render(
      <ReservationConfirmationSheet
        open={true}
        onOpenChange={() => {}}
        resourceId="vehicle-1"
        resourceType="vehicle"
        resourceName="Bulli 1"
        startDate="2026-05-05"
        endDate="2026-05-05"
      />
    )

    await waitFor(() => {
      expect(screen.getByText('Projekt (optional)')).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'CNC Vorbereitung (Werkstatt)' })).toBeInTheDocument()
    })
  })
})
