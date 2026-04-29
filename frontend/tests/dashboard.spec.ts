import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';

test.describe('Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should load dashboard after login', async ({ page }) => {
    await page.waitForSelector('main', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content?.length).toBeGreaterThan(0);
  });

  test('should display sites overview section', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('main', { timeout: 5000 });
    
    const sitesSection = page.locator('text=Aktive Baustellen').first();
    await expect(sitesSection).toBeVisible({ timeout: 5000 });
  });

  test('should display stats cards', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('main', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content).toContain('Aktive Baustellen');
    expect(content).toContain('Niedrige Bestände');
    expect(content).toContain('Heute gebucht');
  });
});
