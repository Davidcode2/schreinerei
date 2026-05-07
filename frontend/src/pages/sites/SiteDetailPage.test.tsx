import { describe, expect, it } from 'vitest'
import userEvent from '@testing-library/user-event'
import { screen, waitFor, within } from '@testing-library/react'
import { Route, Routes } from 'react-router-dom'
import { http, HttpResponse } from 'msw'
import { render } from '@/test/utils'
import { server } from '@/test/mocks/server'
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
  created_at: new Date().toISOString(),
}

describe('SiteDetailPage', () => {
  it('labels the main feed as Projekt-Timeline with canonical context copy', async () => {
    window.history.pushState({}, '', '/sites/site-1')

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
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
      expect(screen.getByText('Werkstattprojekt')).toBeInTheDocument()
      expect(screen.getByText('Projekt-Timeline')).toBeInTheDocument()
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
      http.get('*/api/v1/inventory/sites/site-1/history', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    const user = userEvent.setup()

    const timelineCard = await screen.findByText('Projekt-Timeline')
    const timelineSection = timelineCard.closest('[class*="rounded"]') ?? document.body

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
})
