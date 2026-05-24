import { beforeEach, describe, expect, it } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { render } from '@/test/utils'
import { mockData } from '@/test/mocks/handlers'
import { ActiveSiteIndicator } from './ActiveSiteIndicator'

describe('ActiveSiteIndicator', () => {
  beforeEach(() => {
    window.history.pushState({}, '', '/')
  })

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

  it('navigates to the active project detail page when clicked', async () => {
    mockData.preferences = { active_site_id: 'site-1' }
    mockData.sites = [
      {
        id: 'site-1',
        project_type: 'external_site',
        name: 'Villa Müller',
        customer_name: '',
        location: 'Leipzig',
        description: null,
        status: 'planned',
        start_date: null,
        end_date: null,
        estimated_days: null,
        created_at: new Date().toISOString(),
      },
    ]

    render(<ActiveSiteIndicator compact />)

    const user = userEvent.setup()
    await user.click(await screen.findByRole('button', { name: /projekt villa müller extern/i }))

    await waitFor(() => {
      expect(window.location.pathname).toBe('/sites/site-1')
    })
  })

  it('does not navigate when no active project exists', async () => {
    mockData.preferences = { active_site_id: null }
    mockData.sites = []

    render(<ActiveSiteIndicator compact />)

    const button = await screen.findByRole('button', { name: /projekt keine ausgewählt/i })

    expect(button).toBeDisabled()
    expect(window.location.pathname).not.toBe('/sites/site-1')
  })
})
