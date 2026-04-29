import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';

test.describe('Sites Management', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should navigate to sites page', async ({ page }) => {
    await page.click('a[href*="baustellen"], a:has-text("Baustellen")');
    await expect(page).toHaveURL(/baustellen|sites/);
  });

  test('should create a new site', async ({ page }) => {
    await page.goto('/baustellen');
    
    await page.click('button:has-text("Neu"), button:has-text("Hinzufügen")');
    await page.waitForSelector('[role="dialog"]');
    
    const dialog = page.locator('[role="dialog"]');
    
    const nameInput = dialog.locator('input[name="name"], input[placeholder*="Name"]').first();
    await nameInput.fill(`Test Site ${Date.now()}`);
    
    const customerInput = dialog.locator('input[name="customer"], input[placeholder*="Kunde"]').first();
    if (await customerInput.isVisible()) {
      await customerInput.fill('Test Customer');
    }
    
    const locationInput = dialog.locator('input[name="location"], input[placeholder*="Ort"]').first();
    if (await locationInput.isVisible()) {
      await locationInput.fill('Berlin');
    }
    
    await dialog.locator('button[type="submit"], button:has-text("Speichern")').click();
    
    await expect(dialog).not.toBeVisible({ timeout: 5000 });
  });

  test('should display sites list', async ({ page }) => {
    await page.goto('/baustellen');
    
    await page.waitForSelector('table, [data-testid="sites-list"], .site-card', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content?.length).toBeGreaterThan(0);
  });

  test('should handle time booking without errors', async ({ page }) => {
    await page.goto('/baustellen');
    await page.waitForSelector('table, [data-testid="sites-list"]', { timeout: 5000 });
    
    const timeButton = page.locator('button:has-text("Zeit"), button:has-text("Buchen")').first();
    if (await timeButton.isVisible()) {
      await timeButton.click();
      await page.waitForSelector('[role="dialog"]', { timeout: 3000 });
      
      const dialog = page.locator('[role="dialog"]');
      await expect(dialog).toBeVisible();
      
      const errorAlert = dialog.locator('[role="alert"], .error');
      await expect(errorAlert).not.toBeVisible();
    }
  });
});
