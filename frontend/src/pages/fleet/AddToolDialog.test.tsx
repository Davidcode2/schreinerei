import { describe, it, expect, vi, beforeEach } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { http, HttpResponse } from 'msw';
import { render } from '@/test/utils';
import { server } from '@/test/mocks/server';
import { mockData } from '@/test/mocks/handlers';
import { AddToolDialog } from './AddToolDialog';

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

    const submitButton = screen.getByRole('button', { name: /erstellen/i });
    expect(submitButton).toBeDisabled();
  });

  it('enables submit button when name is filled', async () => {
    const user = userEvent.setup();
    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'Bohrhammer');

    const submitButton = screen.getByRole('button', { name: /erstellen/i });
    expect(submitButton).toBeEnabled();
  });

  it('submits form with correct payload', async () => {
    const user = userEvent.setup();
    let submittedPayload: unknown = null;

    server.use(
      http.post('/api/v1/fleet/tools', async ({ request }) => {
        submittedPayload = await request.json();
        return HttpResponse.json({ id: 'new-tool', ...submittedPayload }, { status: 201 });
      })
    );

    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'Bohrhammer');
    await user.type(screen.getByLabelText(/kategorie/i), 'Elektrowerkzeug');
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
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(screen.getByText('Werkzeug erstellt')).toBeInTheDocument();
    });
  });

  it('closes dialog after successful submission', async () => {
    const user = userEvent.setup();
    render(<AddToolDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'Bohrhammer');
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(mockOnOpenChange).toHaveBeenCalledWith(false);
    });
  });
});
