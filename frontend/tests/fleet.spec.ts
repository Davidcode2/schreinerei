import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';
import { createVehicle, getVehicle, createTool, listTools } from './helpers/api';
import { useCleanup, uniqueName } from './helpers/data';

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

    const addButton = page.locator(
      'button:has-text("Neues Fahrzeug"), button:has-text("Neu")'
    );
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

  test('should persist created vehicle via API', async ({ page }) => {
    const vehicleName = uniqueName('Test Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      license_plate: 'TEST-123',
      vehicle_type: 'car',
      description: 'Test vehicle for E2E',
    });
    track.vehicle(vehicle.id);

    const fetched = await getVehicle(page, vehicle.id);
    expect(fetched.name).toBe(vehicleName);
    expect(fetched.license_plate).toBe('TEST-123');
    expect(fetched.vehicle_type).toBe('car');
  });

  test('should persist created tool via API', async ({ page }) => {
    const toolName = uniqueName('Test Tool');
    const tool = await createTool(page, {
      name: toolName,
      category: 'hand_tool',
      description: 'Test tool for E2E',
    });
    track.tool(tool.id);

    const tools = await listTools(page);
    const found = tools.find((t) => t.id === tool.id);
    expect(found).toBeDefined();
    expect(found?.name).toBe(toolName);
    expect(found?.status).toBe('available');
  });
});
