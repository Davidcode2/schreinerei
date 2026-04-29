import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';

test.describe('Fleet Management', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should navigate to fleet page', async ({ page }) => {
    await page.click('a[href*="fleet"], a:has-text("Fuhrpark")');
    await expect(page).toHaveURL(/fleet/);
  });

  test('should display fleet page elements', async ({ page }) => {
    await page.goto('/fleet');
    
    await page.waitForSelector('h1:has-text("Fuhrpark")', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content?.length).toBeGreaterThan(0);
    expect(content).toContain('Fahrzeuge');
    expect(content).toContain('Werkzeuge');
  });

  test('should have add vehicle/tool button', async ({ page }) => {
    await page.goto('/fleet');
    
    const addButton = page.locator('button:has-text("Neues Fahrzeug"), button:has-text("Neu")');
    await expect(addButton.first()).toBeVisible({ timeout: 5000 });
  });

  test('should display vehicles section', async ({ page }) => {
    await page.goto('/fleet');
    
    await page.waitForSelector('text=Fahrzeuge', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content).toContain('Fahrzeuge');
  });

  test('should display tools section', async ({ page }) => {
    await page.goto('/fleet');
    
    await page.waitForSelector('text=Werkzeuge', { timeout: 5000 });
    
    const content = await page.locator('main').textContent();
    expect(content).toContain('Werkzeuge');
  });
});
