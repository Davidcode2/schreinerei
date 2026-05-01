import { expect, test, type Locator, type Page } from '@playwright/test';
import { login } from './helpers/auth';
import {
  createCategory,
  createMaterial,
  createSite,
  getCategory,
  getMaterial,
  listEnrichedMaterialHistory,
  withdrawMaterial,
} from './helpers/api';
import { uniqueName, useCleanup } from './helpers/data';

async function expectVisibleWithReload(page: Page, locator: Locator) {
  try {
    await expect(locator).toBeVisible({ timeout: 5000 });
  } catch {
    await page.reload();
    await expect(locator).toBeVisible({ timeout: 15000 });
  }
}

test.describe('Inventory phase 31-32 flows', () => {
  let cleanup: () => Promise<void> = async () => {};
  let track: ReturnType<typeof useCleanup>['track'];

  test.beforeEach(async ({ page }) => {
    await login(page);
    const helper = useCleanup(page);
    cleanup = helper.cleanup;
    track = helper.track;
  });

  test.afterEach(async () => {
    await cleanup();
  });

  test('navigates from inventory to settings and persists category edits', async ({ page }) => {
    const category = await createCategory(page, {
      name: uniqueName('Kategorie'),
      description: 'Altbestand',
    });
    track.category(category.id);
    const updatedName = uniqueName('Kategorie bearbeitet');

    await page.goto('/inventory');
    await page.getByRole('button', { name: 'Inventar-Einstellungen öffnen' }).click();
    await expect(page).toHaveURL(/\/settings\/inventory$/);

    const editCategoryButton = page.getByRole('button', { name: `${category.name} bearbeiten` });
    await expectVisibleWithReload(page, editCategoryButton);
    await editCategoryButton.click();
    await page.getByLabel('Name').fill(updatedName);
    await page.getByLabel('Beschreibung').fill('Neu beschrieben');
    await page.getByRole('button', { name: 'Speichern' }).click();

    await expect(page.getByText('Kategorie aktualisiert')).toBeVisible();

    const persisted = await getCategory(page, category.id);
    expect(persisted.name).toBe(updatedName);
    expect(persisted.description).toBe('Neu beschrieben');
  });

  test('keeps referenced categories undeleted and shows inline conflict feedback', async ({ page }) => {
    const category = await createCategory(page, {
      name: uniqueName('Blockierte Kategorie'),
      description: null,
    });
    track.category(category.id);

    const material = await createMaterial(page, {
      category_id: category.id,
      name: uniqueName('Referenziertes Material'),
      description: null,
      unit: 'Stück',
      quantity: 12,
      min_quantity: 2,
      location: 'Regal C',
    });
    track.material(material.id);

    await page.goto('/settings/inventory');
    const deleteCategoryButton = page.getByRole('button', { name: `${category.name} löschen` });
    await expectVisibleWithReload(page, deleteCategoryButton);
    await deleteCategoryButton.click();
    await page.getByRole('button', { name: 'Löschen' }).last().click();

    await expect(
      page.getByText(/Kategorie konnte nicht gelöscht werden|Cannot delete category/i)
    ).toBeVisible();

    const persisted = await getCategory(page, category.id);
    expect(persisted.id).toBe(category.id);
  });

  test('persists material location and minimum stock after editing in the UI', async ({ page }) => {
    const category = await createCategory(page, {
      name: uniqueName('Bearbeiten Kategorie'),
      description: null,
    });
    track.category(category.id);

    const material = await createMaterial(page, {
      category_id: category.id,
      name: uniqueName('Bearbeiten Material'),
      description: 'Vor dem Update',
      unit: 'Stück',
      quantity: 25,
      min_quantity: 5,
      location: 'Regal A',
    });
    track.material(material.id);

    await page.goto(`/inventory/${material.id}`);
    const editMaterialButton = page.getByRole('button', { name: 'Material bearbeiten' });
    await expectVisibleWithReload(page, editMaterialButton);
    await editMaterialButton.click();
    await page.getByLabel('Lagerort').fill('Regal B2');
    await page.getByLabel('Mindestbestand').fill('14');
    await page.getByLabel('Bestand korrigieren').fill('25');
    await page.getByRole('button', { name: 'Änderungen speichern' }).click();

    await expect(page.getByText('Material aktualisiert')).toBeVisible();

    const persisted = await getMaterial(page, material.id);
    expect(persisted.location).toBe('Regal B2');
    expect(persisted.min_quantity).toBe(14);
    expect(persisted.quantity).toBe(25);
  });

  test('stocks material in through the UI and verifies quantity plus history via API', async ({ page }) => {
    const category = await createCategory(page, {
      name: uniqueName('Einlagerung Kategorie'),
      description: null,
    });
    track.category(category.id);

    const material = await createMaterial(page, {
      category_id: category.id,
      name: uniqueName('Einlagerung Material'),
      description: null,
      unit: 'Stück',
      quantity: 8,
      min_quantity: 2,
      location: 'Wareneingang',
    });
    track.material(material.id);

    await page.goto(`/inventory/${material.id}`);
    const stockInButton = page.getByRole('button', { name: 'Material einlagern' });
    await expectVisibleWithReload(page, stockInButton);
    await stockInButton.click();
    await page.getByLabel('Menge').fill('6');
    await page.getByLabel('Notizen').fill('Lieferschein 4711');
    await page.getByRole('button', { name: /6 Stück einlagern/i }).click();

    await expect(page.getByText('6 Stück eingelagert')).toBeVisible();

    const persisted = await getMaterial(page, material.id);
    expect(persisted.quantity).toBe(14);

    const history = await listEnrichedMaterialHistory(page, material.id);
    expect(history[0]?.entry_type).toBe('material_added');
    expect(history[0]?.notes).toBe('Lieferschein 4711');
    expect(history[0]?.quantity_after).toBe(14);
  });

  test('renders enriched history badge, attribution, and Baustelle link for withdrawals', async ({ page }) => {
    const category = await createCategory(page, {
      name: uniqueName('Historie Kategorie'),
      description: null,
    });
    track.category(category.id);

    const site = await createSite(page, {
      name: uniqueName('Baustelle'),
      customer_name: 'Musterkunde',
      location: 'Hamburg',
    });
    track.site(site.id);

    const material = await createMaterial(page, {
      category_id: category.id,
      name: uniqueName('Historie Material'),
      description: null,
      unit: 'Stück',
      quantity: 18,
      min_quantity: 3,
      location: 'Regal D',
    });
    track.material(material.id);

    await withdrawMaterial(page, material.id, {
      quantity: 4,
      notes: 'Für Baustelle',
      site_id: site.id,
    });

    const history = await listEnrichedMaterialHistory(page, material.id);
    expect(history[0]?.entry_type).toBe('withdrawn');

    await page.goto(`/inventory/${material.id}`);

    const withdrawnBadge = page.getByText('Entnommen');
    await expectVisibleWithReload(page, withdrawnBadge);
    await expect(withdrawnBadge).toHaveClass(/bg-red-100/);
    await expect(withdrawnBadge).toHaveClass(/text-red-700/);
    await expect(withdrawnBadge).toHaveClass(/border-red-200/);
    await expect(page.getByText(`von ${history[0].user_name}`)).toBeVisible();
    await expect(page.getByRole('link', { name: site.name })).toHaveAttribute(
      'href',
      `/sites/${site.id}`
    );
  });
});
