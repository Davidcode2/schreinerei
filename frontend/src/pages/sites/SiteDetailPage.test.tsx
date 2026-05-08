import { describe, expect, it, vi } from 'vitest'
import userEvent from '@testing-library/user-event'
import { screen, waitFor, within } from '@testing-library/react'
import { Route, Routes } from 'react-router-dom'
import { http, HttpResponse } from 'msw'
import { render } from '@/test/utils'
import { server } from '@/test/mocks/server'
import { useAuthStore } from '@/lib/auth/authStore'
import SiteDetailPage from './SiteDetailPage'

const site = {
  id: 'site-1',
  project_type: 'internal_workshop' as const,
  name: 'CNC Vorbereitung',
  customer_name: '',
  location: 'Werkstatt',
  description: 'Vorbereitung',
  status: 'planned' as const,
  start_date: null,
  end_date: null,
  estimated_days: 2,
  budget_amount_cents: 320000,
  billing_reference: 'BR-2',
  billing_notes: 'Schlussrechnung nach Abnahme',
  quote_reference: 'ANG-2026-09',
  created_at: new Date().toISOString(),
}

function emptyCalendarResponse() {
  return HttpResponse.json({ resources: [] })
}

describe('SiteDetailPage', () => {
  function setAdminUser() {
    useAuthStore.setState({
      user: {
        id: 'user-1',
        email: 'admin@example.com',
        name: 'Admin',
        role: 'admin',
        created_at: new Date().toISOString(),
      },
      tokens: null,
      isAuthenticated: true,
      isLoading: false,
    })
  }

  it('labels the main feed as Projekt-Timeline with canonical context copy', async () => {
    window.history.pushState({}, '', '/sites/site-1')

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([])),
      http.get('*/api/v1/fleet/calendar*', emptyCalendarResponse)
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    await waitFor(() => {
      expect(screen.getAllByText('Werkstattprojekt').length).toBeGreaterThan(0)
      expect(screen.getAllByText('Projekt-Timeline').length).toBeGreaterThan(0)
      expect(
        screen.getByText(/Der zentrale Ort für Notizen, Fotos und Dokumente/i)
      ).toBeInTheDocument()
    })
  })

  it('opens the unified composer from the primary entry CTA while keeping the camera shortcut available', async () => {
    window.history.pushState({}, '', '/sites/site-1')

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([])),
      http.get('*/api/v1/fleet/calendar*', emptyCalendarResponse)
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    const user = userEvent.setup()

    const helperText = await screen.findByText(/Der zentrale Ort für Notizen, Fotos und Dokumente/i)
    const timelineSection =
      (helperText.closest('[class*="rounded"]') as HTMLElement | null) ?? document.body

    expect(within(timelineSection).getByRole('button', { name: /kamera/i })).toBeInTheDocument()

    await user.click(within(timelineSection).getByRole('button', { name: /eintrag/i }))

    expect(await screen.findByText('Dokumente hinzufügen')).toBeInTheDocument()
  })

  it('keeps material history reachable inside the same timeline surface', async () => {
    window.history.pushState({}, '', '/sites/site-1')

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([])),
      http.get('*/api/v1/fleet/calendar*', emptyCalendarResponse)
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    await waitFor(() => {
      expect(screen.getByRole('tab', { name: /Material/i })).toBeInTheDocument()
      expect(screen.getByText('Projektplanung')).toBeInTheDocument()
    })
  })

  it('shows project labor and material aggregates on the detail surface', async () => {
    window.history.pushState({}, '', '/sites/site-1')

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () =>
        HttpResponse.json({
          labor: {
            total_hours: 12.5,
            entry_count: 4,
            site_hours: 7.5,
            workshop_hours: 5,
            last_work_date: '2026-05-08',
          },
          materials: {
            distinct_material_count: 2,
            withdrawal_count: 3,
            lines: [
              {
                material_id: 'mat-1',
                material_name: 'Montageschaum',
                category_name: 'Chemie',
                unit: 'Stück',
                total_withdrawn: 4,
                withdrawal_count: 2,
                last_withdrawn_at: new Date().toISOString(),
              },
            ],
          },
        })
      ),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([])),
      http.get('*/api/v1/fleet/calendar*', emptyCalendarResponse)
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    expect(await screen.findByText('Projektkennzahlen')).toBeInTheDocument()
    expect(screen.getAllByText('12.5h').length).toBeGreaterThan(0)
    expect(screen.getByText('Montageschaum')).toBeInTheDocument()
    expect(screen.getByText(/4 Stück/i)).toBeInTheDocument()
    expect(screen.getByText('Budget & Abrechnung')).toBeInTheDocument()
    expect(screen.getByText(/3\.200,00/)).toBeInTheDocument()
    expect(screen.getByText('ANG-2026-09')).toBeInTheDocument()
  })

  it('lets admins export the invoice-ready project summary', async () => {
    window.history.pushState({}, '', '/sites/site-1')
    setAdminUser()

    const createObjectUrlSpy = vi.spyOn(URL, 'createObjectURL').mockReturnValue('blob:invoice-summary')
    const revokeObjectUrlSpy = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => undefined)

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 12.5, entry_count: 4, site_hours: 7.5, workshop_hours: 5, last_work_date: '2026-05-08' },
        materials: { distinct_material_count: 2, withdrawal_count: 3, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/invoice-summary', () => HttpResponse.json({
        export_version: 'v1',
        generated_at: new Date().toISOString(),
        project: {
          id: 'site-1',
          name: 'CNC Vorbereitung',
          project_type: 'internal_workshop',
          customer_name: '',
          location: 'Werkstatt',
          status: 'planned',
          start_date: null,
          end_date: null,
          estimated_days: 2,
        },
        billing: {
          budget_amount_cents: 320000,
          quote_reference: 'ANG-2026-09',
          billing_reference: 'BR-2',
          billing_notes: 'Schlussrechnung nach Abnahme',
        },
        labor: { total_hours: 12.5, entry_count: 4, site_hours: 7.5, workshop_hours: 5, last_work_date: '2026-05-08' },
        materials: { distinct_material_count: 2, withdrawal_count: 3, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([])),
      http.get('*/api/v1/fleet/calendar*', emptyCalendarResponse)
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    const user = userEvent.setup()
    await user.click(await screen.findByRole('button', { name: /projektübersicht exportieren/i }))

    await waitFor(() => {
      expect(createObjectUrlSpy).toHaveBeenCalled()
      expect(revokeObjectUrlSpy).toHaveBeenCalledWith('blob:invoice-summary')
    })
  })

  it('shows the embedded project planning calendar filtered to the current project', async () => {
    window.history.pushState({}, '', '/sites/site-1')

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 0, entry_count: 0, site_hours: 0, workshop_hours: 0, last_work_date: null },
        materials: { distinct_material_count: 0, withdrawal_count: 0, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([])),
      http.get('*/api/v1/fleet/calendar*', ({ request }) => {
        const url = new URL(request.url)
        expect(url.searchParams.get('site_id')).toBe('site-1')
        return HttpResponse.json({
          resources: [
            {
              resource_type: 'vehicle',
              resource_id: 'veh-1',
              resource_name: 'Sprinter',
              reservations: [
                {
                  id: 'res-1',
                  start_time: new Date().toISOString(),
                  end_time: new Date().toISOString(),
                  user_name: 'Max Mustermann',
                  site_id: 'site-1',
                  site_name: 'CNC Vorbereitung',
                  status: 'confirmed',
                },
              ],
            },
          ],
        })
      })
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    expect(await screen.findByText('Reservierungen im Projektkontext')).toBeInTheDocument()
    expect(await screen.findByText('Sprinter')).toBeInTheDocument()
  })
})
