import { describe, it, expect, vi, beforeEach } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { http, HttpResponse } from 'msw';
import { render } from '@/test/utils';
import { server } from '@/test/mocks/server';
import { mockData } from '@/test/mocks/handlers';
import { AddVehicleDialog } from './AddVehicleDialog';

const apiRoute = (path: string) => `*/api/v1${path}`;

// Helper to select an option from a Radix UI Select
async function selectOption(
  user: ReturnType<typeof userEvent.setup>,
  label: string | RegExp,
  optionText: string
) {
  const trigger = screen.getByRole('combobox', { name: label });
  await user.click(trigger);
  const option = await screen.findByText(optionText, {}, { timeout: 2000 });
  await user.click(option);
}

describe('AddVehicleDialog', () => {
  const mockOnOpenChange = vi.fn();

  beforeEach(() => {
    mockOnOpenChange.mockClear();
    mockData.vehicles = [];
  });

  it('renders dialog with correct title when open', () => {
    render(<AddVehicleDialog open={true} onOpenChange={mockOnOpenChange} />);

    expect(screen.getByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText('Fahrzeug hinzufügen')).toBeInTheDocument();
  });

  it('does not render when closed', () => {
    render(<AddVehicleDialog open={false} onOpenChange={mockOnOpenChange} />);

    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('has submit button disabled when required fields are empty', () => {
    render(<AddVehicleDialog open={true} onOpenChange={mockOnOpenChange} />);

    const weiterButton = screen.getByRole('button', { name: /weiter/i });
    expect(weiterButton).toBeDisabled();
  });

  it('enables Weiter when name and vehicle type are filled', async () => {
    const user = userEvent.setup();
    render(<AddVehicleDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'VW Transporter');
    await selectOption(user, /fahrzeugtyp/i, 'Transporter');

    const weiterButton = screen.getByRole('button', { name: /weiter/i });
    expect(weiterButton).toBeEnabled();
  });

  it('shows step indicator and moves to details step', async () => {
    const user = userEvent.setup();
    render(<AddVehicleDialog open={true} onOpenChange={mockOnOpenChange} />);

    expect(screen.getByRole('tab', { name: /schritt 1 von 2/i })).toHaveAttribute('aria-selected', 'true');

    await user.type(screen.getByLabelText(/name/i), 'VW Transporter');
    await selectOption(user, /fahrzeugtyp/i, 'Transporter');
    await user.click(screen.getByRole('button', { name: /weiter/i }));

    expect(screen.getByRole('tab', { name: /schritt 2 von 2/i })).toHaveAttribute('aria-selected', 'true');
    expect(screen.getByRole('button', { name: /zurück/i })).toBeInTheDocument();
    expect(screen.getByLabelText(/standort/i)).toBeInTheDocument();
  });

  it('submits form with correct payload', async () => {
    const user = userEvent.setup();
    let submittedPayload: Record<string, unknown> | null = null;

    server.use(
      http.post(apiRoute('/fleet/vehicles'), async ({ request }) => {
        const body = await request.json() as Record<string, unknown>;
        submittedPayload = body;
        return HttpResponse.json({ id: 'new-vehicle', ...body }, { status: 201 });
      })
    );

    render(<AddVehicleDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'VW Transporter');
    await user.type(screen.getByLabelText(/kennzeichen/i), 'B-AB 1234');
    await selectOption(user, /fahrzeugtyp/i, 'Transporter');
    await user.click(screen.getByRole('button', { name: /weiter/i }));
    await user.type(screen.getByLabelText(/standort/i), 'Hof 1');

    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(submittedPayload).toEqual({
        name: 'VW Transporter',
        vehicle_type: 'van',
        license_plate: 'B-AB 1234',
        location: 'Hof 1',
      });
    });
  });

  it('shows success toast after submission', async () => {
    const user = userEvent.setup();
    render(<AddVehicleDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.type(screen.getByLabelText(/name/i), 'VW Transporter');
    await selectOption(user, /fahrzeugtyp/i, 'Transporter');
    await user.click(screen.getByRole('button', { name: /weiter/i }));
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(screen.getByText('Fahrzeug erstellt')).toBeInTheDocument();
    });
  });

  it('contains all vehicle type options', async () => {
    const user = userEvent.setup();
    render(<AddVehicleDialog open={true} onOpenChange={mockOnOpenChange} />);

    await user.click(screen.getByRole('combobox', { name: /fahrzeugtyp/i }));

    await waitFor(() => {
      expect(screen.getByText('PKW')).toBeInTheDocument();
      expect(screen.getByText('Transporter')).toBeInTheDocument();
      expect(screen.getByText('LKW')).toBeInTheDocument();
      expect(screen.getByText('Anhänger')).toBeInTheDocument();
      expect(screen.getByText('Sonstige')).toBeInTheDocument();
    });
  });
});
