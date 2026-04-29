import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';

test.describe('Fleet Management', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should navigate to fleet page', async ({ page }) => {
    await page.click('a[href*="fuhrpark"], a:has-text("Fuhrpark")');
    await expect(page).toHaveURL(/fuhrpark|fleet/);
  });

  test('should create a new vehicle', async ({ page }) => {
    await page.goto('/fuhrpark');
    
    await page.click('button:has-text("Neu"), button:has-text("Hinzufügen")');
    await page.waitForSelector('[role="dialog"]');
    
    const dialog = page.locator('[role="dialog"]');
    
    const nameInput = dialog.locator('input[name="name"], input[placeholder*="Name"]').first();
    await nameInput.fill(`Test Vehicle ${Date.now()}`);
    
    const plateInput = dialog.locator('input[name="license_plate"], input[placeholder*="Kennzeichen"]').first();
    await plateInput.fill('B-TEST 123');
    
    const typeSelect = dialog.locator('select[name="vehicle_type"], button:has-text("Typ")').first();
    if (await typeSelect.isVisible()) {
      await typeSelect.click();
      await page.keyboard.press('ArrowDown');
      await page.keyboard.press('Enter');
    }
    
    await dialog.locator('button[type="submit"], button:has-text("Speichern")').click();
    
    await expect(dialog).not.toBeVisible({ timeout: 5000 });
  });

  test('should create a new tool', async ({ page }) => {
    await page.goto('/fuhrpark');
    
    const toolsTab = page.locator('button:has-text("Werkzeuge"), [role="tab"]:has-text("Tools")');
    if (await toolsTab.isVisible()) {
      await toolsTab.click();
    }
    
    await page.click('button:has-text("Neu"), button:has-text("Hinzufügen")');
    await page.waitForSelector('[role="dialog"]');
    
    const dialog = page.locator('[role="dialog"]');
    
    const nameInput = dialog.locator('input[name="name"], input[placeholder*="Name"]').first();
    await nameInput.fill(`Test Tool ${Date.now()}`);
    
    await dialog.locator('button[type="submit"], button:has-text("Speichern")').click();
    
    await expect(dialog).not.toBeVisible({ timeout: 5000 });
  });

  test('should display fleet items', async ({ page }) => {
    await page.goto('/fuhrpark');
    
    await page.waitForSelector('table, [data-testid="fleet-list"], .vehicle-card', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content?.length).toBeGreaterThan(0);
  });

  test('should open reservation dialog', async ({ page }) => {
    await page.goto('/fuhrpark');
    await page.waitForSelector('table, [data-testid="fleet-list"]', { timeout: 5000 });
    
    const reserveButton = page.locator('button:has-text("Reservieren"), button:has-text("Kalender")').first();
    if (await reserveButton.isVisible()) {
      await reserveButton.click();
      await page.waitForSelector('[role="dialog"]', { timeout: 3000 });
      const dialog = page.locator('[role="dialog"]');
      await expect(dialog).toBeVisible();
    }
  });
});
