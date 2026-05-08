import { describe, expect, it } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { render } from '@/test/utils'
import { mockData } from '@/test/mocks/handlers'
import DashboardPage from './DashboardPage'

describe('DashboardPage', () => {
  it('uses project-aware wording while keeping the active-project section', async () => {
    mockData.preferences = { active_site_id: null }
    mockData.sites = [
      {
        id: 'site-1',
        project_type: 'external_site',
        name: 'Villa Müller',
        customer_name: 'Familie Müller',
        location: 'Leipzig',
        description: null,
        status: 'active',
        start_date: null,
        end_date: null,
        estimated_days: null,
        created_at: new Date().toISOString(),
      },
    ]
    mockData.timeEntries = []

    render(<DashboardPage />)

    await waitFor(() => {
      expect(screen.getByText('Projekte')).toBeInTheDocument()
      expect(screen.getByRole('button', { name: /alle anzeigen/i })).toBeInTheDocument()
    })
  })

  it('shows completed projects by default and lets the user filter them explicitly', async () => {
    const user = userEvent.setup()

    mockData.preferences = { active_site_id: null }
    mockData.sites = [
      {
        id: 'site-1',
        project_type: 'external_site',
        name: 'Villa Müller',
        customer_name: 'Familie Müller',
        location: 'Leipzig',
        description: null,
        status: 'active',
        start_date: null,
        end_date: null,
        estimated_days: null,
        created_at: new Date().toISOString(),
      },
      {
        id: 'site-2',
        project_type: 'internal_workshop',
        name: 'CNC Nacharbeit',
        customer_name: '',
        location: 'Werkstatt',
        description: null,
        status: 'completed',
        start_date: null,
        end_date: null,
        estimated_days: null,
        created_at: new Date().toISOString(),
      },
    ]
    mockData.timeEntries = []

    render(<DashboardPage />)

    expect(await screen.findByText('CNC Nacharbeit')).toBeInTheDocument()

    await user.click(screen.getByRole('button', { name: 'Abgeschlossen' }))

    await waitFor(() => {
      expect(screen.queryByText('CNC Nacharbeit')).not.toBeInTheDocument()
    })
  })
})
