import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';

test.describe('Inventory Management', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should navigate to inventory page', async ({ page }) => {
    await page.click('a[href*="inventar"], a:has-text("Inventar")');
    await expect(page).toHaveURL(/inventar|inventory/);
  });

  test('should create a new category', async ({ page }) => {
    await page.goto('/inventar');
    
    await page.click('button:has-text("Neu"), button:has-text("Hinzufügen")');
    await page.waitForSelector('[role="dialog"]');
    
    const categorySection = page.locator('[role="dialog"]');
    const newCategoryButton = categorySection.locator('button:has-text("Neu")').first();
    
    if (await newCategoryButton.isVisible()) {
      await newCategoryButton.click();
      const categoryInput = categorySection.locator('input[placeholder*="Kategorie"], input[placeholder*="category"]').first();
      if (await categoryInput.isVisible()) {
        await categoryInput.fill(`Test Category ${Date.now()}`);
      }
    }
  });

  test('should add a new material', async ({ page }) => {
    await page.goto('/inventar');
    
    await page.click('button:has-text("Neu"), button:has-text("Hinzufügen")');
    await page.waitForSelector('[role="dialog"]');
    
    const dialog = page.locator('[role="dialog"]');
    
    const nameInput = dialog.locator('input[name="name"], input[placeholder*="Name"]').first();
    await nameInput.fill(`Test Material ${Date.now()}`);
    
    const quantityInput = dialog.locator('input[name="quantity"], input[placeholder*="Menge"]').first();
    await quantityInput.fill('10');
    
    const unitSelect = dialog.locator('select[name="unit"], button:has-text("Einheit")').first();
    if (await unitSelect.isVisible()) {
      await unitSelect.click();
      await page.keyboard.press('ArrowDown');
      await page.keyboard.press('Enter');
    }
    
    await dialog.locator('button[type="submit"], button:has-text("Speichern")').click();
    
    await expect(dialog).not.toBeVisible({ timeout: 5000 });
  });

  test('should display materials in list', async ({ page }) => {
    await page.goto('/inventar');
    
    await page.waitForSelector('table, [data-testid="material-list"], .material-card', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content?.length).toBeGreaterThan(0);
  });
});
