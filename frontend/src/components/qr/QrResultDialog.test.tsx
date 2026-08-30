import { http, HttpResponse } from 'msw'
import userEvent from '@testing-library/user-event'
import { render, screen, waitFor } from '@/test/utils'
import { server } from '@/test/mocks/server'
import QrResultDialog from './QrResultDialog'

describe('QrResultDialog', () => {
  it.each([
    {
      type: 'material',
      response: { id: 'material-42', name: 'Multiplex', quantity: 12, unit: 'Stück', location: null },
      button: 'Zum Material',
      path: '/inventory/material-42',
    },
    {
      type: 'vehicle',
      response: { resource_type: 'vehicle', resource_id: 'vehicle-42', resource_name: 'Montagebus', status: 'available', location: null },
      button: 'Details anzeigen',
      path: '/fleet/vehicle-42',
    },
    {
      type: 'tool',
      response: { resource_type: 'tool', resource_id: 'tool-42', resource_name: 'Tauchsäge', status: 'available', location: null },
      button: 'Details anzeigen',
      path: '/tools/tool-42',
    },
  ])('navigates a $type result to its detail page', async ({ type, response, button, path }) => {
    server.use(
      http.get('*/api/v1/inventory/qr/fixture-code', () =>
        type === 'material' ? HttpResponse.json(response) : HttpResponse.json({}, { status: 404 }),
      ),
      http.get('*/api/v1/fleet/qr/fixture-code', () => HttpResponse.json(response)),
    )
    const user = userEvent.setup()
    render(<QrResultDialog qrCode="fixture-code" onClose={() => {}} />)

    await user.click(await screen.findByRole('button', { name: button }))

    await waitFor(() => expect(window.location.pathname).toBe(path))
  })
})
