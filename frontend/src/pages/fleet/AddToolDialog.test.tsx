import { describe, it, expect, vi, beforeEach } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { http, HttpResponse } from 'msw';
import { render } from '@/test/utils';
import { server } from '@/test/mocks/server';
import { mockData } from '@/test/mocks/handlers';
import { AddToolDialog } from './AddToolDialog';

const apiRoute = (path: string) => `*/api/v1${path}`;

describe('AddToolDialog', () => {
  const mockOnOpenChange = vi.fn();

  beforeEach(() => {
    mockOnOpenChange.mockClear();
    mockData.tools = [];
  });

  it('renders dialog with correct title when open', () => {
    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    expect(screen.getByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText('Werkzeug hinzufügen')).toBeInTheDocument();
  });

  it('does not render when closed', () => {
    render(<AddToolDialog open={false} onOpenChange={mockOnOpenChange} />);

    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('has submit button disabled when name is empty', () => {
    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    const weiterButton = screen.getByRole('button', { name: /weiter/i });
    expect(weiterButton).toBeDisabled();
  });

  it('enables Weiter when name is filled', async () => {
    const user = userEvent.setup();
    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'Bohrhammer');

    const weiterButton = screen.getByRole('button', { name: /weiter/i });
    expect(weiterButton).toBeEnabled();
  });

  it('shows step navigation and opens detail step', async () => {
    const user = userEvent.setup();
    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    expect(screen.getByRole('tab', { name: /schritt 1 von 2/i })).toHaveAttribute('aria-selected', 'true');

    await user.type(screen.getByLabelText(/name/i), 'Bohrhammer');
    await user.click(screen.getByRole('button', { name: /weiter/i }));

    expect(screen.getByRole('tab', { name: /schritt 2 von 2/i })).toHaveAttribute('aria-selected', 'true');
    expect(screen.getByLabelText(/standort/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /zurück/i })).toBeInTheDocument();
  });

  it('submits form with correct payload', async () => {
    const user = userEvent.setup();
    let submittedPayload: Record<string, unknown> | null = null;

    server.use(
      http.post(apiRoute('/fleet/tools'), async ({ request }) => {
        const body = await request.json() as Record<string, unknown>;
        submittedPayload = body;
        return HttpResponse.json({ id: 'new-tool', ...body }, { status: 201 });
      })
    );

    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'Bohrhammer');
    await user.type(screen.getByLabelText(/kategorie/i), 'Elektrowerkzeug');
    await user.click(screen.getByRole('button', { name: /weiter/i }));
    await user.type(screen.getByLabelText(/standort/i), 'Werkstatt');
    await user.type(screen.getByLabelText(/beschreibung/i), 'Bosch GBH 2-21');

    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(submittedPayload).toEqual({
        name: 'Bohrhammer',
        category: 'Elektrowerkzeug',
        location: 'Werkstatt',
        description: 'Bosch GBH 2-21',
      });
    });
  });

  it('shows success toast after submission', async () => {
    const user = userEvent.setup();
    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'Bohrhammer');
    await user.click(screen.getByRole('button', { name: /weiter/i }));
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(screen.getByText('Werkzeug erstellt')).toBeInTheDocument();
    });
  });

  it('closes dialog after successful submission', async () => {
    const user = userEvent.setup();
    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'Bohrhammer');
    await user.click(screen.getByRole('button', { name: /weiter/i }));
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(mockOnOpenChange).toHaveBeenCalledWith(false);
    });
  });
});
