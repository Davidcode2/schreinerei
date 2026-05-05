import { describe, it, expect, vi, beforeEach } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { http, HttpResponse } from 'msw';
import { render } from '@/test/utils';
import { server } from '@/test/mocks/server';
import { mockData } from '@/test/mocks/handlers';
import { AddSiteDialog } from './AddSiteDialog';

const apiRoute = (path: string) => `*/api/v1${path}`;

describe('AddSiteDialog', () => {
  const mockOnOpenChange = vi.fn();

  beforeEach(() => {
    mockOnOpenChange.mockClear();
    mockData.sites = [];
  });

  it('renders dialog with correct title when open', () => {
    render(<AddSiteDialog open={true} onOpenChange={mockOnOpenChange} />);

    expect(screen.getByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText('Projekt anlegen')).toBeInTheDocument();
  });

  it('does not render when closed', () => {
    render(<AddSiteDialog open={false} onOpenChange={mockOnOpenChange} />);

    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('has submit button disabled when required fields are empty', () => {
    render(<AddSiteDialog open={true} onOpenChange={mockOnOpenChange} />);

    const submitButton = screen.getByRole('button', { name: /erstellen/i });
    expect(submitButton).toBeDisabled();
  });

  it('enables submit button when external project name and customer are filled', async () => {
    const user = userEvent.setup();
    render(<AddSiteDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/projektname/i), 'Villa Müller');
    await user.type(screen.getByLabelText(/kunde/i), 'Familie Müller');

    const submitButton = screen.getByRole('button', { name: /erstellen/i });
    expect(submitButton).toBeEnabled();
  });

  it('allows internal workshop project without customer name', async () => {
    const user = userEvent.setup();
    render(<AddSiteDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.click(screen.getByRole('combobox'));
    await user.click(screen.getByRole('option', { name: /werkstatt intern/i }));
    await user.type(screen.getByLabelText(/projektname/i), 'CNC Vorbereitung');

    expect(screen.getByRole('button', { name: /projekt erstellen/i })).toBeEnabled();
  });

  it('submits form with correct payload', async () => {
    const user = userEvent.setup();
    let submittedPayload: Record<string, unknown> | null = null;

    server.use(
      http.post(apiRoute('/sites'), async ({ request }) => {
        const body = await request.json() as Record<string, unknown>;
        submittedPayload = body;
        return HttpResponse.json({ id: 'new-site', ...body }, { status: 201 });
      })
    );

    render(<AddSiteDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/projektname/i), 'Villa Müller');
    await user.type(screen.getByLabelText(/kunde/i), 'Familie Müller');
    await user.type(screen.getByLabelText(/standort/i), 'Musterstraße 1, Berlin');
    await user.type(screen.getByLabelText(/planungsnotiz/i), 'Küchenumbau');

    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(submittedPayload).toEqual({
        project_type: 'external_site',
        name: 'Villa Müller',
        customer_name: 'Familie Müller',
        location: 'Musterstraße 1, Berlin',
        description: 'Küchenumbau',
      });
    });
  });

  it('shows success toast after submission', async () => {
    const user = userEvent.setup();
    render(<AddSiteDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/projektname/i), 'Villa Müller');
    await user.type(screen.getByLabelText(/kunde/i), 'Familie Müller');
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(screen.getByText('Baustelle erstellt')).toBeInTheDocument();
    });
  });

  it('closes dialog after successful submission', async () => {
    const user = userEvent.setup();
    render(<AddSiteDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/projektname/i), 'Villa Müller');
    await user.type(screen.getByLabelText(/kunde/i), 'Familie Müller');
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(mockOnOpenChange).toHaveBeenCalledWith(false);
    });
  });
});
