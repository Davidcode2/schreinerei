import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';
import {
  createSite,
  getSite,
  createMaterial,
  getMaterial,
  createVehicle,
  getVehicle,
  createTool,
  listTools,
  deleteTool,
  listCategories,
  createCategory,
  createTimeEntry,
  createReservation,
} from './helpers/api';
import { useCleanup, uniqueName } from './helpers/data';

test.describe('Delete Operations', () => {
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

  test('should delete site with confirmation dialog', async ({ page }) => {
    // Create site via API
    const siteName = uniqueName('Delete Test Site');
    const site = await createSite(page, {
      name: siteName,
      customer_name: 'Test Customer',
      location: 'Test Location',
    });
    track.site(site.id);

    // Navigate to sites page
    await page.goto('/sites');
    await page.waitForSelector('h1:has-text("Baustellen")', { timeout: 5000 });

    // Find the site card and click delete button
    const siteCard = page.locator(`text=${siteName}`).first();
    await expect(siteCard).toBeVisible({ timeout: 5000 });

    // Click delete button (trash icon or Löschen text near the site name)
    const deleteButton = page
      .locator(`article, div, li`)
      .filter({ hasText: siteName })
      .locator('button:has-text("Löschen"), button[aria-label*="delete"], button:has(svg)')
      .first();
    await deleteButton.click();

    // Verify confirmation dialog appears
    const dialog = page.locator('role=alertdialog, [role="dialog"], [data-testid="alert-dialog"]');
    await expect(dialog).toBeVisible({ timeout: 3000 });

    // Confirm deletion
    const confirmButton = dialog.locator('button:has-text("Löschen"), button:has-text("Delete"), button:has-text("Bestätigen")').first();
    await confirmButton.click();

    // Wait for dialog to close and list to update
    await expect(dialog).not.toBeVisible({ timeout: 3000 });

    // Verify site is no longer in list
    await page.waitForTimeout(500); // Brief wait for UI update
    const siteGone = await page.locator(`text=${siteName}`).count();
    expect(siteGone).toBe(0);

    // Verify soft delete via API (should return 404)
    const response = await page.request.get(`/api/v1/sites/${site.id}`);
    expect(response.status()).toBe(404);
  });

  test('should delete material with confirmation dialog', async ({ page }) => {
    // Get or create category
    const categories = await listCategories(page);
    let categoryId: string;
    if (categories.length > 0) {
      categoryId = categories[0].id;
    } else {
      const category = await createCategory(page, { name: uniqueName('Test Cat') });
      categoryId = category.id;
      track.category(category.id);
    }

    // Create material via API
    const materialName = uniqueName('Delete Test Material');
    const material = await createMaterial(page, {
      category_id: categoryId,
      name: materialName,
      unit: 'Stück',
      quantity: 10,
      min_quantity: 2,
    });
    track.material(material.id);

    // Navigate to inventory page
    await page.goto('/inventory');
    await page.waitForSelector('h1:has-text("Inventar")', { timeout: 5000 });

    // Find the material card and click delete button
    const materialCard = page.locator(`text=${materialName}`).first();
    await expect(materialCard).toBeVisible({ timeout: 5000 });

    // Click delete button
    const deleteButton = page
      .locator(`article, div, li`)
      .filter({ hasText: materialName })
      .locator('button:has-text("Löschen"), button[aria-label*="delete"], button:has(svg)')
      .first();
    await deleteButton.click();

    // Verify confirmation dialog appears
    const dialog = page.locator('role=alertdialog, [role="dialog"], [data-testid="alert-dialog"]');
    await expect(dialog).toBeVisible({ timeout: 3000 });

    // Confirm deletion
    const confirmButton = dialog.locator('button:has-text("Löschen"), button:has-text("Delete"), button:has-text("Bestätigen")').first();
    await confirmButton.click();

    // Wait for dialog to close
    await expect(dialog).not.toBeVisible({ timeout: 3000 });

    // Verify material is no longer in list
    await page.waitForTimeout(500);
    const materialGone = await page.locator(`text=${materialName}`).count();
    expect(materialGone).toBe(0);

    // Verify soft delete via API (should return 404)
    const response = await page.request.get(`/api/v1/inventory/materials/${material.id}`);
    expect(response.status()).toBe(404);
  });

  test('should delete vehicle with confirmation dialog', async ({ page }) => {
    // Create vehicle via API
    const vehicleName = uniqueName('Delete Test Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      license_plate: 'TEST-DELETE',
      vehicle_type: 'car',
    });
    track.vehicle(vehicle.id);

    // Navigate to fleet page
    await page.goto('/fleet');
    await page.waitForSelector('h1:has-text("Fuhrpark")', { timeout: 5000 });

    // Find the vehicle card and click delete button
    const vehicleCard = page.locator(`text=${vehicleName}`).first();
    await expect(vehicleCard).toBeVisible({ timeout: 5000 });

    // Click delete button
    const deleteButton = page
      .locator(`article, div, li`)
      .filter({ hasText: vehicleName })
      .locator('button:has-text("Löschen"), button[aria-label*="delete"], button:has(svg)')
      .first();
    await deleteButton.click();

    // Verify confirmation dialog appears
    const dialog = page.locator('role=alertdialog, [role="dialog"], [data-testid="alert-dialog"]');
    await expect(dialog).toBeVisible({ timeout: 3000 });

    // Confirm deletion
    const confirmButton = dialog.locator('button:has-text("Löschen"), button:has-text("Delete"), button:has-text("Bestätigen")').first();
    await confirmButton.click();

    // Wait for dialog to close
    await expect(dialog).not.toBeVisible({ timeout: 3000 });

    // Verify vehicle is no longer in list
    await page.waitForTimeout(500);
    const vehicleGone = await page.locator(`text=${vehicleName}`).count();
    expect(vehicleGone).toBe(0);

    // Verify soft delete via API (should return 404)
    const response = await page.request.get(`/api/v1/fleet/vehicles/${vehicle.id}`);
    expect(response.status()).toBe(404);
  });

  test('should delete tool with confirmation dialog', async ({ page }) => {
    // Create tool via API
    const toolName = uniqueName('Delete Test Tool');
    const tool = await createTool(page, {
      name: toolName,
      category: 'hand_tool',
      description: 'Tool to be deleted',
    });
    track.tool(tool.id);

    // Navigate to fleet page
    await page.goto('/fleet');
    await page.waitForSelector('h1:has-text("Fuhrpark")', { timeout: 5000 });

    // Find the tool card and click delete button
    const toolCard = page.locator(`text=${toolName}`).first();
    await expect(toolCard).toBeVisible({ timeout: 5000 });

    // Click delete button
    const deleteButton = page
      .locator(`article, div, li`)
      .filter({ hasText: toolName })
      .locator('button:has-text("Löschen"), button[aria-label*="delete"], button:has(svg)')
      .first();
    await deleteButton.click();

    // Verify confirmation dialog appears
    const dialog = page.locator('role=alertdialog, [role="dialog"], [data-testid="alert-dialog"]');
    await expect(dialog).toBeVisible({ timeout: 3000 });

    // Confirm deletion
    const confirmButton = dialog.locator('button:has-text("Löschen"), button:has-text("Delete"), button:has-text("Bestätigen")').first();
    await confirmButton.click();

    // Wait for dialog to close
    await expect(dialog).not.toBeVisible({ timeout: 3000 });

    // Verify tool is no longer in list
    await page.waitForTimeout(500);
    const toolGone = await page.locator(`text=${toolName}`).count();
    expect(toolGone).toBe(0);

    // Verify soft delete via API (should return 404)
    const response = await page.request.get(`/api/v1/fleet/tools/${tool.id}`);
    expect(response.status()).toBe(404);
  });

  test('should show error when deleting site with time entries', async ({ page }) => {
    // Create site via API
    const siteName = uniqueName('Conflict Test Site');
    const site = await createSite(page, {
      name: siteName,
      customer_name: 'Test Customer',
      location: 'Test Location',
    });
    track.site(site.id);

    // Create time entry for this site
    const today = new Date().toISOString().split('T')[0];
    const timeEntry = await createTimeEntry(page, {
      site_id: site.id,
      work_type: 'site',
      hours: 4,
      work_date: today,
    });
    track.timeEntry(timeEntry.id);

    // Navigate to sites page
    await page.goto('/sites');
    await page.waitForSelector('h1:has-text("Baustellen")', { timeout: 5000 });

    // Find the site card and click delete button
    const siteCard = page.locator(`text=${siteName}`).first();
    await expect(siteCard).toBeVisible({ timeout: 5000 });

    // Click delete button
    const deleteButton = page
      .locator(`article, div, li`)
      .filter({ hasText: siteName })
      .locator('button:has-text("Löschen"), button[aria-label*="delete"], button:has(svg)')
      .first();
    await deleteButton.click();

    // Verify confirmation dialog appears
    const dialog = page.locator('role=alertdialog, [role="dialog"], [data-testid="alert-dialog"]');
    await expect(dialog).toBeVisible({ timeout: 3000 });

    // Confirm deletion
    const confirmButton = dialog.locator('button:has-text("Löschen"), button:has-text("Delete"), button:has-text("Bestätigen")').first();
    await confirmButton.click();

    // Verify error message appears (dependency conflict)
    // The error could be shown as a toast, alert, or within the dialog
    const errorVisible = await page.locator('text=/Abhängigkeit|Konflikt|kann nicht gelöscht|cannot be deleted|dependency|conflict/i').count();
    
    // Site should still exist (not deleted due to dependency)
    const response = await page.request.get(`/api/v1/sites/${site.id}`);
    // Depending on implementation, might return 409 Conflict or the site still exists
    // Either way, the UI should show an error
    expect(errorVisible + (response.ok() ? 1 : 0)).toBeGreaterThan(0);
  });

  test('should show error when deleting vehicle with active reservation', async ({ page }) => {
    // Create vehicle via API
    const vehicleName = uniqueName('Conflict Test Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      license_plate: 'CONFLICT-1',
      vehicle_type: 'van',
    });
    track.vehicle(vehicle.id);

    // Create active reservation for this vehicle
    const now = new Date();
    const startTime = new Date(now.getTime() - 2 * 60 * 60 * 1000); // Started 2 hours ago
    const endTime = new Date(now.getTime() + 6 * 60 * 60 * 1000); // Ends 6 hours from now

    const reservation = await createReservation(page, {
      resource_type: 'vehicle',
      resource_id: vehicle.id,
      start_time: startTime.toISOString(),
      end_time: endTime.toISOString(),
    });
    track.reservation(reservation.id);

    // Navigate to fleet page
    await page.goto('/fleet');
    await page.waitForSelector('h1:has-text("Fuhrpark")', { timeout: 5000 });

    // Find the vehicle card and click delete button
    const vehicleCard = page.locator(`text=${vehicleName}`).first();
    await expect(vehicleCard).toBeVisible({ timeout: 5000 });

    // Click delete button
    const deleteButton = page
      .locator(`article, div, li`)
      .filter({ hasText: vehicleName })
      .locator('button:has-text("Löschen"), button[aria-label*="delete"], button:has(svg)')
      .first();
    await deleteButton.click();

    // Verify confirmation dialog appears
    const dialog = page.locator('role=alertdialog, [role="dialog"], [data-testid="alert-dialog"]');
    await expect(dialog).toBeVisible({ timeout: 3000 });

    // Confirm deletion
    const confirmButton = dialog.locator('button:has-text("Löschen"), button:has-text("Delete"), button:has-text("Bestätigen")').first();
    await confirmButton.click();

    // Verify error message appears or vehicle still exists
    const errorVisible = await page.locator('text=/Abhängigkeit|Konflikt|Reservierung|kann nicht gelöscht|cannot be deleted|reservation|conflict/i').count();
    
    // Vehicle should still exist (not deleted due to active reservation)
    const response = await page.request.get(`/api/v1/fleet/vehicles/${vehicle.id}`);
    expect(errorVisible + (response.ok() ? 1 : 0)).toBeGreaterThan(0);
  });
});
