import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';
import {
  createMaterial,
  getMaterial,
  listMaterials,
  listCategories,
  createCategory,
} from './helpers/api';
import { useCleanup, uniqueName } from './helpers/data';

test.describe('Inventory Management', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should navigate to inventory page', async ({ page }) => {
    await page.click('a[href*="inventory"], a:has-text("Inventar")');
    await expect(page).toHaveURL(/inventory/);
  });

  test('should display inventory page elements', async ({ page }) => {
    await page.goto('/inventory');

    await page.waitForSelector('h1:has-text("Inventar")', { timeout: 5000 });

    const content = await page.locator('main').textContent();
    expect(content?.length).toBeGreaterThan(0);
    expect(content).toContain('Inventar');
  });

  test('should have add material button', async ({ page }) => {
    await page.goto('/inventory');

    const addButton = page.locator(
      'button:has-text("Material hinzufügen"), button:has-text("hinzufügen")'
    );
    await expect(addButton.first()).toBeVisible({ timeout: 5000 });
  });

  test('should have search input', async ({ page }) => {
    await page.goto('/inventory');

    const searchInput = page.locator(
      'input[placeholder*="suchen"], input[placeholder*="Material"]'
    );
    await expect(searchInput.first()).toBeVisible({ timeout: 5000 });
  });
});

test.describe('Data Persistence', () => {
  let cleanup: () => Promise<void>;
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

  test('should persist created material via API', async ({ page }) => {
    const categories = await listCategories(page);
    let categoryId: string;

    if (categories.length > 0) {
      categoryId = categories[0].id;
    } else {
      const category = await createCategory(page, {
        name: uniqueName('Test Category'),
      });
      categoryId = category.id;
    }

    const materialName = uniqueName('Test Material');
    const material = await createMaterial(page, {
      category_id: categoryId,
      name: materialName,
      description: 'Test material for E2E',
      unit: 'Stück',
      quantity: 100,
      min_quantity: 10,
      location: 'Test Location',
    });
    track.material(material.id);

    const fetched = await getMaterial(page, material.id);
    expect(fetched.name).toBe(materialName);
    expect(fetched.quantity).toBe(100);
    expect(fetched.id).toBe(material.id);
    expect(fetched.category_id).toBe(categoryId);
  });

  test('should list created material in materials list', async ({ page }) => {
    const categories = await listCategories(page);
    let categoryId: string;

    if (categories.length > 0) {
      categoryId = categories[0].id;
    } else {
      const category = await createCategory(page, {
        name: uniqueName('Test Category'),
      });
      categoryId = category.id;
    }

    const materialName = uniqueName('List Test');
    const material = await createMaterial(page, {
      category_id: categoryId,
      name: materialName,
      unit: 'kg',
      quantity: 50,
      min_quantity: 5,
    });
    track.material(material.id);

    const materials = await listMaterials(page);
    const found = materials.find((m) => m.id === material.id);
    expect(found).toBeDefined();
    expect(found?.name).toBe(materialName);
  });
});
