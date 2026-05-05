import { describe, expect, it } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import { render } from '@/test/utils'
import { mockData } from '@/test/mocks/handlers'
import { ActiveSiteIndicator } from './ActiveSiteIndicator'

describe('ActiveSiteIndicator', () => {
  it('shows project-aware wording and type label', async () => {
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

    render(<ActiveSiteIndicator compact />)

    await waitFor(() => {
      expect(screen.getByText('Projekt')).toBeInTheDocument()
      expect(screen.getByText('Werkstatt')).toBeInTheDocument()
    })
  })
})
