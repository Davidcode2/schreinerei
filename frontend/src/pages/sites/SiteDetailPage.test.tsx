import { describe, expect, it } from 'vitest'
import { screen, waitFor } from '@testing-library/react'
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
  it('shows project planning and project type cues', async () => {
    window.history.pushState({}, '', '/sites/site-1')

    server.use(
      http.get('*/api/v1/sites/site-1', () => HttpResponse.json(site)),
      http.get('*/api/v1/sites/site-1/assignments', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/time-entries', () => HttpResponse.json([])),
      http.get('*/api/v1/sites/site-1/activities', () => HttpResponse.json([]))
    )

    render(
      <Routes>
        <Route path="/sites/:id" element={<SiteDetailPage />} />
      </Routes>
    )

    await waitFor(() => {
      expect(screen.getByText('Werkstattprojekt')).toBeInTheDocument()
      expect(screen.getByText('Projektplanung')).toBeInTheDocument()
      expect(screen.getByRole('button', { name: /planen/i })).toBeInTheDocument()
    })
  })
})
