import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';
import { createSite, getSite } from './helpers/api';
import { useCleanup, uniqueName } from './helpers/data';

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

    const addButton = page.locator(
      'button:has-text("Baustelle anlegen"), button:has-text("anlegen")'
    );
    await expect(addButton.first()).toBeVisible({ timeout: 5000 });
  });

  test('should have search input', async ({ page }) => {
    await page.goto('/sites');

    const searchInput = page.locator(
      'input[placeholder*="suchen"], input[placeholder*="Baustelle"]'
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

  test('should persist created site via API', async ({ page }) => {
    const siteName = uniqueName('Test Site');
    const site = await createSite(page, {
      name: siteName,
      customer_name: 'Test Customer',
      location: 'Test Address 123',
      description: 'Test site for E2E',
    });
    track.site(site.id);

    const fetched = await getSite(page, site.id);
    expect(fetched.name).toBe(siteName);
    expect(fetched.customer_name).toBe('Test Customer');
    expect(fetched.location).toBe('Test Address 123');
  });

  test('should verify site status after creation', async ({ page }) => {
    const site = await createSite(page, {
      name: uniqueName('Status Test'),
      customer_name: 'Customer',
      location: 'Address',
    });
    track.site(site.id);

    const fetched = await getSite(page, site.id);
    expect(fetched.status).toBe('planned');
  });
});
