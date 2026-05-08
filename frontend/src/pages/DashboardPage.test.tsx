import { describe, expect, it } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { http, HttpResponse } from 'msw'
import { render } from '@/test/utils'
import { mockData } from '@/test/mocks/handlers'
import { server } from '@/test/mocks/server'
import DashboardPage from './DashboardPage'

const apiPath = (path: string) => `*/api/v1${path}`

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

  it('shows expiry alerts in the dashboard warning surfaces', async () => {
    mockData.preferences = { active_site_id: null }
    mockData.sites = []
    mockData.timeEntries = []
    mockData.materials = [
      {
        id: 'mat-1',
        category_id: 'cat-1',
        name: 'Leim',
        description: null,
        unit: 'Liter',
        quantity: 6,
        min_quantity: 2,
        can_expire: true,
        legacy_quantity: 0,
        expired_quantity: 0,
        expiring_soon_quantity: 2,
        next_expiry_on: '2026-05-20',
        expiry_batches: [],
        location: 'Regal A',
        qr_code: null,
        is_low_stock: false,
        created_at: new Date().toISOString(),
      },
    ]

    server.use(
      http.get(apiPath('/inventory/alerts'), () => HttpResponse.json(mockData.materials)),
      http.get(apiPath('/inventory/low-stock'), () => HttpResponse.json([]))
    )

    render(<DashboardPage />)

    expect(await screen.findByText('Materialwarnungen')).toBeInTheDocument()
    expect(await screen.findByText(/2 Liter bald ablaufend/i)).toBeInTheDocument()
  })

  it('shows low-stock warnings in the same top material warnings section', async () => {
    mockData.preferences = { active_site_id: null }
    mockData.sites = []
    mockData.timeEntries = []
    mockData.materials = [
      {
        id: 'mat-2',
        category_id: 'cat-1',
        name: 'Schrauben',
        description: null,
        unit: 'Stück',
        quantity: 2,
        min_quantity: 10,
        can_expire: false,
        legacy_quantity: 2,
        expired_quantity: 0,
        expiring_soon_quantity: 0,
        next_expiry_on: null,
        expiry_batches: [],
        location: 'Regal B',
        qr_code: null,
        is_low_stock: true,
        created_at: new Date().toISOString(),
      },
    ]

    server.use(
      http.get(apiPath('/inventory/alerts'), () => HttpResponse.json([])),
      http.get(apiPath('/inventory/low-stock'), () => HttpResponse.json(mockData.materials))
    )

    render(<DashboardPage />)

    expect(await screen.findByText('Materialwarnungen')).toBeInTheDocument()
    expect(await screen.findByText(/Bestand unter Minimum/i)).toBeInTheDocument()
    expect(screen.queryByText('Niedrige Bestände')).not.toBeInTheDocument()
  })
})
