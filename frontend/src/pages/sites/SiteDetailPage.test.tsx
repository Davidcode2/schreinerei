import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
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

function emptyAppointmentsResponse() {
  return HttpResponse.json([])
}

describe('SiteDetailPage', () => {
  beforeEach(() => {
    useAuthStore.setState({
      user: {
        id: 'user-2',
        tenant_id: 'tenant-1',
        email: 'employee@example.com',
        name: 'Employee',
        role: 'mitarbeiter',
        created_at: new Date().toISOString(),
      },
      tokens: null,
      isAuthenticated: true,
      isLoading: false,
    })
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  function setAdminUser() {
    useAuthStore.setState({
      user: {
        id: 'user-1',
        tenant_id: 'tenant-1',
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
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
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
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
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
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
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
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
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

  it('shows existing invoices to admins without exposing a missing PDF download route', async () => {
    window.history.pushState({}, '', '/sites/site-1')
    setAdminUser()

    let pdfRequested = false

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 12.5, entry_count: 4, site_hours: 7.5, workshop_hours: 5, last_work_date: '2026-05-08' },
        materials: { distinct_material_count: 2, withdrawal_count: 3, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/invoices', () => HttpResponse.json([
        {
          id: 'inv-1',
          site_id: 'site-1',
          invoice_number: 1,
          invoice_number_display: '2026-00001',
          status: 'generated',
          sender_name: null,
          sender_address: null,
          issued_at: '2026-05-10T08:00:00.000Z',
          due_on: null,
          voided_at: null,
          pdf_artifact: {
            storage_path: 'invoices/tenant/inv-1.pdf',
            sha256_hash: 'hash',
            content_type: 'application/pdf',
            size_bytes: 12,
            created_at: '2026-05-10T08:01:00.000Z',
          },
          created_by: 'user-1',
          created_at: '2026-05-10T08:00:00.000Z',
          updated_at: '2026-05-10T08:01:00.000Z',
        },
      ])),
      http.get('*/api/v1/billing/invoices/inv-1/pdf', () => {
        pdfRequested = true
        return new HttpResponse(new Blob(['pdf'], { type: 'application/pdf' }), {
          headers: { 'Content-Type': 'application/pdf' },
        })
      }),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    expect(await screen.findByText('Rechnungen')).toBeInTheDocument()
    expect(await screen.findByText('2026-00001')).toBeInTheDocument()
    expect(screen.getByText('Erstellt')).toBeInTheDocument()
    expect(screen.getByText(/PDF noch nicht verfügbar/i)).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /pdf/i })).not.toBeInTheDocument()
    expect(pdfRequested).toBe(false)
  })

  it('lets admins create an invoice from the site detail page', async () => {
    window.history.pushState({}, '', '/sites/site-1')
    setAdminUser()
    let createRequested = false
    let invoiceListCalls = 0

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 12.5, entry_count: 4, site_hours: 7.5, workshop_hours: 5, last_work_date: '2026-05-08' },
        materials: { distinct_material_count: 2, withdrawal_count: 3, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/invoices', () => {
        invoiceListCalls += 1
        return HttpResponse.json(invoiceListCalls > 1 ? [
          {
            id: 'inv-2',
            site_id: 'site-1',
            invoice_number: 2,
            invoice_number_display: '2026-00002',
            status: 'draft',
            sender_name: null,
            sender_address: null,
            issued_at: null,
            due_on: null,
            voided_at: null,
            pdf_artifact: null,
            created_by: 'user-1',
            created_at: '2026-05-10T09:00:00.000Z',
            updated_at: '2026-05-10T09:00:00.000Z',
          },
        ] : [])
      }),
      http.post('*/api/v1/billing/projects/site-1/invoices', () => {
        createRequested = true
        return HttpResponse.json({
          invoice: {
            id: 'inv-2',
            site_id: 'site-1',
            invoice_number: 2,
            invoice_number_display: '2026-00002',
            status: 'draft',
            sender_name: null,
            sender_address: null,
            issued_at: null,
            due_on: null,
            voided_at: null,
            pdf_artifact: null,
            created_by: 'user-1',
            created_at: '2026-05-10T09:00:00.000Z',
            updated_at: '2026-05-10T09:00:00.000Z',
          },
          project: {},
          billing: {},
          labor: {},
          materials: {},
          line_items: [],
        })
      }),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    const user = userEvent.setup()
    await user.click(await screen.findByRole('button', { name: /rechnung erstellen/i }))

    await waitFor(() => expect(createRequested).toBe(true))
    expect(await screen.findByText('2026-00002')).toBeInTheDocument()
    expect(screen.getByText(/PDF noch nicht verfügbar/i)).toBeInTheDocument()
  })

  it('keeps the invoice action usable when invoice creation fails', async () => {
    window.history.pushState({}, '', '/sites/site-1')
    setAdminUser()
    let createAttempts = 0

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 12.5, entry_count: 4, site_hours: 7.5, workshop_hours: 5, last_work_date: '2026-05-08' },
        materials: { distinct_material_count: 2, withdrawal_count: 3, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/invoices', () => HttpResponse.json([])),
      http.post('*/api/v1/billing/projects/site-1/invoices', () => {
        createAttempts += 1
        return HttpResponse.json({ error: 'invoice failed' }, { status: 500 })
      }),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    const user = userEvent.setup()
    const createButton = await screen.findByRole('button', { name: /rechnung erstellen/i })
    await user.click(createButton)

    await waitFor(() => expect(createAttempts).toBe(1))
    expect(await screen.findByRole('button', { name: /rechnung erstellen/i })).toBeEnabled()
  })

  it('keeps invoice creation controls hidden for non-admins', async () => {
    window.history.pushState({}, '', '/sites/site-1')
    let invoiceListRequested = false

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 0, entry_count: 0, site_hours: 0, workshop_hours: 0, last_work_date: null },
        materials: { distinct_material_count: 0, withdrawal_count: 0, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/invoices', () => {
        invoiceListRequested = true
        return HttpResponse.json([])
      }),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    expect(await screen.findByText('Budget & Abrechnung')).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /rechnung erstellen/i })).not.toBeInTheDocument()
    expect(screen.queryByText('Rechnungen')).not.toBeInTheDocument()
    expect(invoiceListRequested).toBe(false)
  })

  it('shows the dedicated appointment planner instead of the fleet reservation calendar', async () => {
    window.history.pushState({}, '', '/sites/site-1')

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 0, entry_count: 0, site_hours: 0, workshop_hours: 0, last_work_date: null },
        materials: { distinct_material_count: 0, withdrawal_count: 0, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([
        {
          id: 'assignment-1',
          site_id: 'site-1',
          user_id: 'user-1',
          role: 'worker',
          created_at: new Date().toISOString(),
        },
      ])),
      http.get('*/api/v1/users', () => HttpResponse.json([
        {
          id: 'user-1',
          email: 'max@example.com',
          name: 'Max Mustermann',
          role: 'employee',
          created_at: new Date().toISOString(),
        },
      ])),
      http.get('*/api/v1/sites/site-1/appointments*', ({ request }) => {
        const url = new URL(request.url)
        expect(url.searchParams.get('start_date')).toBeTruthy()
        expect(url.searchParams.get('end_date')).toBeTruthy()
        return HttpResponse.json([
          {
            id: 'appt-1',
            site_id: 'site-1',
            title: 'Abnahme vor Ort',
            appointment_kind: 'customer_appointment',
            starts_at: '2026-05-06T08:30:00.000Z',
            ends_at: '2026-05-06T10:00:00.000Z',
            notes: 'Mit Bauherr',
            assigned_user_ids: ['user-1'],
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
          },
        ])
      }),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    const planningCard = (await screen.findByText('Projektplanung')).closest('.rounded-xl')

    expect(planningCard).toHaveClass('md:col-span-2')
    expect(await screen.findByText('Terminplan')).toBeInTheDocument()
    expect(await screen.findByText('Abnahme vor Ort')).toBeInTheDocument()
    expect(screen.queryByText('Reservierungen im Projektkontext')).not.toBeInTheDocument()
  })

  it('shows booking author and only offers edit for the creator entry', async () => {
    window.history.pushState({}, '', '/sites/site-1')
    setAdminUser()

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 6.5, entry_count: 2, site_hours: 6.5, workshop_hours: 0, last_work_date: '2026-05-08' },
        materials: { distinct_material_count: 0, withdrawal_count: 0, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/invoices', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([
        {
          id: 'entry-own',
          site_id: 'site-1',
          user_id: 'user-1',
          creator_name: 'Admin',
          can_edit: true,
          can_delete: true,
          work_type: 'site',
          hours: 4,
          work_date: '2026-05-08',
          notes: 'Montage',
          created_at: new Date().toISOString(),
        },
        {
          id: 'entry-other',
          site_id: 'site-1',
          user_id: 'user-2',
          creator_name: 'Anna Tischler',
          can_edit: false,
          can_delete: false,
          work_type: 'travel',
          hours: 2.5,
          work_date: '2026-05-07',
          notes: 'Anfahrt',
          created_at: new Date().toISOString(),
        },
      ])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    expect(await screen.findByText('Erfasst von Admin')).toBeInTheDocument()
    expect(screen.getByText('Erfasst von Anna Tischler')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /bearbeiten/i })).toBeInTheDocument()
    expect(screen.getAllByRole('button', { name: /bearbeiten/i })).toHaveLength(1)
  })

  it('opens the time dialog in edit mode for creator-owned entries', async () => {
    window.history.pushState({}, '', '/sites/site-1')
    setAdminUser()

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/summary', () => HttpResponse.json({
        labor: { total_hours: 4, entry_count: 1, site_hours: 4, workshop_hours: 0, last_work_date: '2026-05-08' },
        materials: { distinct_material_count: 0, withdrawal_count: 0, lines: [] },
      })),
      http.get('*/api/v1/sites/site-1/invoices', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/users', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/appointments*', emptyAppointmentsResponse),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([
        {
          id: 'entry-own',
          site_id: 'site-1',
          user_id: 'user-1',
          creator_name: 'Admin',
          can_edit: true,
          can_delete: true,
          work_type: 'site',
          hours: 4,
          work_date: '2026-05-08',
          notes: 'Montage',
          created_at: new Date().toISOString(),
        },
      ])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([])),
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    const user = userEvent.setup()
    await user.click(await screen.findByRole('button', { name: /bearbeiten/i }))

    expect(await screen.findByText('Zeit bearbeiten')).toBeInTheDocument()
    expect(screen.getByDisplayValue('Montage')).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /^löschen$/i })).toBeInTheDocument()
  })
})
