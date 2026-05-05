import { describe, expect, it } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import { render } from '@/test/utils'
import { mockData } from '@/test/mocks/handlers'
import { ReservationDialog } from './ReservationDialog'

describe('ReservationDialog', () => {
  it('renders project-aware site selector labels', async () => {
    mockData.preferences = { active_site_id: 'site-1' }
    mockData.sites = [
      {
        id: 'site-1',
        project_type: 'external_site',
        name: 'Villa Müller',
        customer_name: 'Familie Müller',
        location: 'Leipzig',
        description: null,
        status: 'planned',
        start_date: null,
        end_date: null,
        estimated_days: null,
        created_at: new Date().toISOString(),
      },
    ]
    mockData.vehicles = [{ id: 'vehicle-1', name: 'Bulli 1' }]

    render(
      <ReservationDialog
        open={true}
        onOpenChange={() => {}}
        mode="create"
        resourceType="vehicle"
        initialStartTime="2026-05-05T08:00"
        initialEndTime="2026-05-05T10:00"
      />
    )

    await waitFor(() => {
      expect(screen.getByText('Projekt (optional)')).toBeInTheDocument()
      expect(screen.getByRole('option', { name: 'Villa Müller (Extern)' })).toBeInTheDocument()
    })
  })
})
