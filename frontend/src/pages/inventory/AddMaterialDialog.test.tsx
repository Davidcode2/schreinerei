import { describe, it, expect, vi, beforeEach } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { http, HttpResponse } from 'msw';
import { render } from '@/test/utils';
import { server } from '@/test/mocks/server';
import { mockData } from '@/test/mocks/handlers';
import { createCategory } from '@/test/factories';
import { AddMaterialDialog } from './AddMaterialDialog';

// Helper to select an option from a Radix UI Select
async function selectOption(
  user: ReturnType<typeof userEvent.setup>,
  label: string | RegExp,
  optionText: string
) {
  // Find and click the select trigger
  const trigger = screen.getByRole('combobox', { name: label });
  await user.click(trigger);

  // Wait for the portal content to appear and find the option
  // Radix UI renders select content in a portal at the body level
  const option = await screen.findByText(optionText, {}, { timeout: 2000 });
  await user.click(option);
}

describe('AddMaterialDialog', () => {
  const mockOnOpenChange = vi.fn();
  const categories = [
    createCategory({ id: 'cat-1', name: 'Schrauben' }),
    createCategory({ id: 'cat-2', name: 'Lacke', can_expire: true }),
  ];

  beforeEach(() => {
    mockOnOpenChange.mockClear();
    mockData.materials = [];
    mockData.categories = categories;
  });

  it('renders dialog with correct title when open', () => {
    render(
      <AddMaterialDialog
        open={true}
        onOpenChange={mockOnOpenChange}
        categories={categories}
      />
    );

    expect(screen.getByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText('Material hinzufügen')).toBeInTheDocument();
  });

  it('does not render when closed', () => {
    render(
      <AddMaterialDialog
        open={false}
        onOpenChange={mockOnOpenChange}
        categories={categories}
      />
    );

    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('has submit button disabled when form is invalid', () => {
    render(
      <AddMaterialDialog
        open={true}
        onOpenChange={mockOnOpenChange}
        categories={categories}
      />
    );

    const submitButton = screen.getByRole('button', { name: /erstellen/i });
    expect(submitButton).toBeDisabled();
  });

  it('enables submit button when all required fields are filled', async () => {
    const user = userEvent.setup();
    render(
      <AddMaterialDialog
        open={true}
        onOpenChange={mockOnOpenChange}
        categories={categories}
      />
    );

    // Fill all required fields using helper
    await selectOption(user, /kategorie/i, 'Schrauben');
    await user.type(screen.getByLabelText(/name/i), 'Schrauben M8');
    await user.type(screen.getByLabelText(/menge/i), '100');
    await selectOption(user, /einheit/i, 'Stück');
    await user.type(screen.getByLabelText(/mindestbestand/i), '10');

    const submitButton = screen.getByRole('button', { name: /erstellen/i });
    expect(submitButton).toBeEnabled();
  });

  it('submits form with correct payload', async () => {
    const user = userEvent.setup();
    let submittedPayload: Record<string, unknown> | null = null;

    server.use(
      http.post('/api/v1/inventory/materials', async ({ request }) => {
        const body = await request.json() as Record<string, unknown>;
        submittedPayload = body;
        return HttpResponse.json({ id: 'new-material', ...body }, { status: 201 });
      })
    );

    render(
      <AddMaterialDialog
        open={true}
        onOpenChange={mockOnOpenChange}
        categories={categories}
      />
    );

    // Fill form
    await selectOption(user, /kategorie/i, 'Schrauben');
    await user.type(screen.getByLabelText(/name/i), 'Schrauben M8');
    await user.type(screen.getByLabelText(/menge/i), '100');
    await selectOption(user, /einheit/i, 'Stück');
    await user.type(screen.getByLabelText(/mindestbestand/i), '10');
    await user.type(screen.getByLabelText(/lagerort/i), 'Regal A1');

    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(submittedPayload).toEqual({
        category_id: 'cat-1',
        name: 'Schrauben M8',
        description: null,
        quantity: 100,
        unit: 'Stück',
        min_quantity: 10,
        location: 'Regal A1',
        expires_on: null,
      });
    });
  });

  it('requires MHD when the selected category can expire', async () => {
    const user = userEvent.setup();

    render(
      <AddMaterialDialog
        open={true}
        onOpenChange={mockOnOpenChange}
        categories={categories}
      />
    );

    await selectOption(user, /kategorie/i, 'Lacke');
    await user.type(screen.getByLabelText(/name/i), 'Lack rot');
    await user.type(screen.getByLabelText(/menge/i), '10');
    await selectOption(user, /einheit/i, 'Liter');
    await user.type(screen.getByLabelText(/mindestbestand/i), '2');

    expect(screen.getByLabelText(/mhd/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /erstellen/i })).toBeDisabled();

    await user.type(screen.getByLabelText(/mhd/i), '2026-05-20');

    expect(screen.getByRole('button', { name: /erstellen/i })).toBeEnabled();
  });

  it('shows success toast after submission', async () => {
    const user = userEvent.setup();
    render(
      <AddMaterialDialog
        open={true}
        onOpenChange={mockOnOpenChange}
        categories={categories}
      />
    );

    // Fill and submit form
    await selectOption(user, /kategorie/i, 'Schrauben');
    await user.type(screen.getByLabelText(/name/i), 'Schrauben M8');
    await user.type(screen.getByLabelText(/menge/i), '100');
    await selectOption(user, /einheit/i, 'Stück');
    await user.type(screen.getByLabelText(/mindestbestand/i), '10');
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(screen.getByText('Material erstellt')).toBeInTheDocument();
    });
  });

  it('closes dialog after successful submission', async () => {
    const user = userEvent.setup();
    render(
      <AddMaterialDialog
        open={true}
        onOpenChange={mockOnOpenChange}
        categories={categories}
      />
    );

    // Fill and submit form
    await selectOption(user, /kategorie/i, 'Schrauben');
    await user.type(screen.getByLabelText(/name/i), 'Schrauben M8');
    await user.type(screen.getByLabelText(/menge/i), '100');
    await selectOption(user, /einheit/i, 'Stück');
    await user.type(screen.getByLabelText(/mindestbestand/i), '10');
    await user.click(screen.getByRole('button', { name: /erstellen/i }));

    await waitFor(() => {
      expect(mockOnOpenChange).toHaveBeenCalledWith(false);
    });
  });
});
