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

  it('renders the edit delete action like the sites calendar modal', async () => {
    mockData.sites = []

    render(
      <ReservationDialog
        open={true}
        onOpenChange={() => {}}
        mode="edit"
        initialData={{
          id: 'reservation-1',
          resource_id: 'tool-1',
          resource_type: 'tool',
          resource_name: 'Akkuschrauber',
          site_id: null,
          site_name: null,
          project_id: null,
          project_name: null,
          user_id: 'user-1',
          user_name: 'Max Mustermann',
          status: 'confirmed',
          start_time: '2026-05-05T08:00:00.000Z',
          end_time: '2026-05-05T10:00:00.000Z',
          purpose: null,
          notes: null,
          current_holder: null,
          created_at: '2026-05-01T08:00:00.000Z',
          updated_at: '2026-05-02T08:00:00.000Z',
        }}
      />
    )

    const deleteButton = await screen.findByRole('button', { name: 'Löschen' })

    expect(deleteButton).toHaveClass('text-destructive')
    expect(deleteButton).toHaveClass('gap-2')
  })
})
