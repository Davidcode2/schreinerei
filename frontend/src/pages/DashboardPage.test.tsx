import { describe, expect, it } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
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
      expect(screen.getAllByText('Aktive Projekte').length).toBeGreaterThan(0)
      expect(screen.getByRole('button', { name: /alle anzeigen/i })).toBeInTheDocument()
    })
  })
})
