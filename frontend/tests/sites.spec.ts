import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';

test.describe('Sites Management', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should navigate to sites page', async ({ page }) => {
    await page.click('a[href*="sites"], a:has-text("Baustellen")');
    await expect(page).toHaveURL(/sites/);
  });

  test('should display sites page elements', async ({ page }) => {
    await page.goto('/sites');
    
    await page.waitForSelector('h1:has-text("Baustellen")', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content?.length).toBeGreaterThan(0);
  });

  test('should have add site button', async ({ page }) => {
    await page.goto('/sites');
    
    const addButton = page.locator('button:has-text("Baustelle anlegen"), button:has-text("anlegen")');
    await expect(addButton.first()).toBeVisible({ timeout: 5000 });
  });

  test('should have search input', async ({ page }) => {
    await page.goto('/sites');
    
    const searchInput = page.locator('input[placeholder*="suchen"], input[placeholder*="Baustelle"]');
    await expect(searchInput.first()).toBeVisible({ timeout: 5000 });
  });
});
