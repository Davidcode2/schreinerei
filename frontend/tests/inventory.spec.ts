import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';

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
    
    const addButton = page.locator('button:has-text("Material hinzufügen"), button:has-text("hinzufügen")');
    await expect(addButton.first()).toBeVisible({ timeout: 5000 });
  });

  test('should have search input', async ({ page }) => {
    await page.goto('/inventory');
    
    const searchInput = page.locator('input[placeholder*="suchen"], input[placeholder*="Material"]');
    await expect(searchInput.first()).toBeVisible({ timeout: 5000 });
  });
});
