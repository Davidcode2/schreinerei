import { expect, test } from '@playwright/test';
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

    await page.getByRole('button', { name: `${category.name} bearbeiten` }).click();
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
    await page.getByRole('button', { name: `${category.name} löschen` }).click();
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
    await page.getByRole('button', { name: 'Material bearbeiten' }).click();
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
    await page.getByRole('button', { name: 'Material einlagern' }).click();
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

    await expect(page.getByText('Entnommen')).toBeVisible();
    await expect(page.getByText(`von ${history[0].user_name}`)).toBeVisible();
    await expect(page.getByRole('link', { name: site.name })).toHaveAttribute(
      'href',
      `/sites/${site.id}`
    );
  });
});
