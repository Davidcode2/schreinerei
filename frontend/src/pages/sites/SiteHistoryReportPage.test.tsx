import { describe, expect, it } from 'vitest'
import { screen } from '@testing-library/react'
import { http, HttpResponse } from 'msw'
import { render } from '@/test/utils'
import { server } from '@/test/mocks/server'
import { useAuthStore } from '@/lib/auth/authStore'
import SiteHistoryReportPage from './SiteHistoryReportPage'

describe('SiteHistoryReportPage', () => {
  it('renders historical project rows from the reporting endpoint', async () => {
    useAuthStore.setState({
      user: {
        id: 'user-1',
        email: 'admin@example.com',
        name: 'Admin',
        role: 'admin',
        tenant_id: 'tenant-1',
        created_at: new Date().toISOString(),
      },
      tokens: null,
      isAuthenticated: true,
      isLoading: false,
    })

    server.use(
      http.get('*/api/v1/sites/history-report', () =>
        HttpResponse.json([
          {
            site_id: 'site-1',
            project_type: 'external_site',
            name: 'Villa Müller',
            customer_name: 'Familie Müller',
            status: 'completed',
            start_date: '2026-05-01',
            end_date: '2026-05-10',
            estimated_days: 5,
            budget_amount_cents: 320000,
            billing_reference: 'BR-1',
            quote_reference: 'ANG-1',
            total_hours: 12.5,
            worker_count: 2,
            distinct_material_count: 3,
            withdrawal_count: 6,
            cost_basis: 'budget_vs_actual',
          },
        ])
      ),
      http.get('*/api/v1/users', () => HttpResponse.json([]))
    )

    render(<SiteHistoryReportPage />)

    expect(await screen.findByText('Villa Müller')).toBeInTheDocument()
    expect(screen.getByText('Familie Müller')).toBeInTheDocument()
    expect(screen.getByText('budget_vs_actual')).toBeInTheDocument()
  })
})
