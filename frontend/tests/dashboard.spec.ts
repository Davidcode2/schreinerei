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

  test('should display sites overview', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('main', { timeout: 5000 });
    
    const sitesSection = page.locator('[data-testid="sites-overview"], h2:has-text("Baustellen"), h2:has-text("Sites")');
    
    await expect(sitesSection.first()).toBeVisible({ timeout: 5000 });
  });

  test('should not have console errors', async ({ page }) => {
    const errors: string[] = [];
    
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });
    
    page.on('pageerror', err => {
      errors.push(err.message);
    });
    
    await page.goto('/');
    await page.waitForSelector('main', { timeout: 5000 });
    
    await page.waitForTimeout(2000);
    
    const criticalErrors = errors.filter(e => 
      !e.includes('extension') && 
      !e.includes('favicon') &&
      !e.includes('manifest')
    );
    
    expect(criticalErrors).toHaveLength(0);
  });
});
