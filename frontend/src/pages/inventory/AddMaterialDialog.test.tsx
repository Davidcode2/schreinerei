import { describe, it, expect, vi, beforeEach } from 'vitest';
import { screen, waitFor, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { http, HttpResponse } from 'msw';
import { render } from '@/test/utils';
import { server } from '@/test/mocks/server';
import { mockData } from '@/test/mocks/handlers';
import { createCategory } from '@/test/factories';
import { AddMaterialDialog } from './AddMaterialDialog';

const apiRoute = (path: string) => `*/api/v1${path}`;

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

// Helper to fill Step 1 and navigate to Step 2
async function fillStep1AndNavigate(
  user: ReturnType<typeof userEvent.setup>,
  category: string = 'Schrauben'
) {
  await selectOption(user, /kategorie/i, category);
  await user.type(screen.getByLabelText(/name/i), 'Schrauben M8');
  await user.type(screen.getByLabelText(/menge/i), '100');
  await selectOption(user, /einheit/i, 'Stück');
  await user.click(screen.getByRole('button', { name: 'Weiter' }));
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

  describe('Step Navigation', () => {
    it('starts on step 1', () => {
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      expect(screen.getByRole('tab', { name: /Schritt 1 von 2/ })).toHaveAttribute('aria-selected', 'true');
    });

    it('disables Weiter button when Step 1 is incomplete', () => {
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      const weiterButton = screen.getByRole('button', { name: 'Weiter' });
      expect(weiterButton).toBeDisabled();
    });

    it('enables Weiter button when Step 1 is complete', async () => {
      const user = userEvent.setup();
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      // Fill Step 1 required fields
      await selectOption(user, /kategorie/i, 'Schrauben');
      await user.type(screen.getByLabelText(/name/i), 'Schrauben M8');
      await user.type(screen.getByLabelText(/menge/i), '100');
      await selectOption(user, /einheit/i, 'Stück');

      const weiterButton = screen.getByRole('button', { name: 'Weiter' });
      expect(weiterButton).not.toBeDisabled();
    });

    it('navigates to Step 2 when Weiter is clicked', async () => {
      const user = userEvent.setup();
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      // Fill Step 1
      await selectOption(user, /kategorie/i, 'Schrauben');
      await user.type(screen.getByLabelText(/name/i), 'Schrauben M8');
      await user.type(screen.getByLabelText(/menge/i), '100');
      await selectOption(user, /einheit/i, 'Stück');

      await user.click(screen.getByRole('button', { name: 'Weiter' }));

      expect(screen.getByRole('tab', { name: /Schritt 2 von 2/ })).toHaveAttribute('aria-selected', 'true');
      expect(screen.getByLabelText(/mindestbestand/i)).toBeInTheDocument();
    });

    it('shows Zurück button on Step 2', async () => {
      const user = userEvent.setup();
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      // Navigate to Step 2
      await fillStep1AndNavigate(user);

      expect(screen.getByRole('button', { name: 'Zurück' })).toBeInTheDocument();
    });

    it('returns to Step 1 when Zurück is clicked', async () => {
      const user = userEvent.setup();
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      // Navigate to Step 2 then back
      await fillStep1AndNavigate(user);
      await user.click(screen.getByRole('button', { name: 'Zurück' }));

      expect(screen.getByRole('tab', { name: /Schritt 1 von 2/ })).toHaveAttribute('aria-selected', 'true');
    });
  });

  describe('Basic Dialog', () => {
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

    it('has Weiter button on Step 1', () => {
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      expect(screen.getByRole('button', { name: 'Weiter' })).toBeInTheDocument();
      expect(screen.queryByRole('button', { name: /erstellen/i })).not.toBeInTheDocument();
    });

    it('has Erstellen button disabled on Step 2 when form is incomplete', async () => {
      const user = userEvent.setup();
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      // Navigate to Step 2 without filling minQuantity
      await fillStep1AndNavigate(user);

      const submitButton = screen.getByRole('button', { name: /erstellen/i });
      expect(submitButton).toBeDisabled();
    });

    it('enables Erstellen button when all required fields are filled', async () => {
      const user = userEvent.setup();
      render(
        <AddMaterialDialog
          open={true}
          onOpenChange={mockOnOpenChange}
          categories={categories}
        />
      );

      // Fill Step 1 and navigate to Step 2
      await fillStep1AndNavigate(user);
      await user.type(screen.getByLabelText(/mindestbestand/i), '10');

      const submitButton = screen.getByRole('button', { name: /erstellen/i });
      expect(submitButton).toBeEnabled();
    });
  });

  describe('Form Submission', () => {
    it('submits form with correct payload', async () => {
      const user = userEvent.setup();
      let submittedPayload: Record<string, unknown> | null = null;

      server.use(
        http.post(apiRoute('/inventory/materials'), async ({ request }) => {
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

      // Fill Step 1 and navigate to Step 2
      await fillStep1AndNavigate(user);
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

      // Fill Step 1 with a category that can expire
      await selectOption(user, /kategorie/i, 'Lacke');
      await user.type(screen.getByLabelText(/name/i), 'Lack rot');
      await user.type(screen.getByLabelText(/menge/i), '10');
      await selectOption(user, /einheit/i, 'Liter');
      await user.click(screen.getByRole('button', { name: 'Weiter' }));

      // Now on Step 2
      await user.type(screen.getByLabelText(/mindestbestand/i), '2');

      const mhdInput = screen.getByLabelText(/mhd/i);
      expect(mhdInput).toBeInTheDocument();
      expect(screen.getByRole('button', { name: /erstellen/i })).toBeDisabled();

      // Use fireEvent for date input as userEvent.type may not work correctly
      fireEvent.change(mhdInput, { target: { value: '2026-05-20' } });

      await waitFor(() => {
        expect(screen.getByRole('button', { name: /erstellen/i })).toBeEnabled();
      });
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
      await fillStep1AndNavigate(user);
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
      await fillStep1AndNavigate(user);
      await user.type(screen.getByLabelText(/mindestbestand/i), '10');
      await user.click(screen.getByRole('button', { name: /erstellen/i }));

      await waitFor(() => {
        expect(mockOnOpenChange).toHaveBeenCalledWith(false);
      });
    });
  });
});
