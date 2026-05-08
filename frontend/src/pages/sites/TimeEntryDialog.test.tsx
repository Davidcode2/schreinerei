import { describe, expect, it } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { render } from '@/test/utils'
import { mockData } from '@/test/mocks/handlers'
import { TimeEntryDialog } from './TimeEntryDialog'

describe('TimeEntryDialog', () => {
  it('shows project-aware wording and project type cues', async () => {
    mockData.preferences = { active_site_id: 'site-2' }
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
      {
        id: 'site-2',
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

    render(<TimeEntryDialog open={true} onOpenChange={() => {}} />)

    await waitFor(() => {
      expect(screen.getByText('Projekt')).toBeInTheDocument()
      const options = screen.getAllByRole('option').map((option) => option.textContent)
      expect(options).toContain('Villa Müller (Extern)')
      expect(options).toContain('CNC Vorbereitung (Werkstatt)')
    })
  })

  it('requires a project for workshop time before submit', async () => {
    const user = userEvent.setup()

    mockData.preferences = { active_site_id: null }
    mockData.sites = [
      {
        id: 'site-2',
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

    render(<TimeEntryDialog open={true} onOpenChange={() => {}} />)

    await user.click(await screen.findByRole('button', { name: 'Werkstattprojekt' }))

    expect(screen.getByText('Produktive Zeit wird immer einem Projekt zugeordnet.')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /buchen/i })).toBeDisabled()

    await user.selectOptions(screen.getByRole('combobox'), 'site-2')

    expect(screen.getByRole('button', { name: /buchen/i })).toBeEnabled()
  })
})
