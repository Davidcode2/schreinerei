import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';
import {
  createTimeEntry,
  getTimeEntry,
  updateTimeEntry,
  deleteTimeEntry,
  createReservation,
  getReservation,
  updateReservation,
  createVehicle,
} from './helpers/api';
import { useCleanup, uniqueName } from './helpers/data';

test.describe('Time Entry Edit Operations', () => {
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

  test('should edit time entry hours', async ({ page }) => {
    // Create time entry via API
    const today = new Date().toISOString().split('T')[0];
    const timeEntry = await createTimeEntry(page, {
      work_type: 'workshop',
      hours: 4,
      work_date: today,
      notes: 'Original note',
    });
    track.timeEntry(timeEntry.id);

    // Navigate to sites page (where time entries are typically shown)
    await page.goto('/sites');
    await page.waitForSelector('h1:has-text("Baustellen")', { timeout: 5000 });

    // Look for time entry in the page or navigate to a time entry view
    // Try to find and click edit button for the time entry
    // This might be in a time entries section or on a specific site page

    // For now, test API update directly since UI location may vary
    const updated = await updateTimeEntry(page, timeEntry.id, { hours: 8 });
    expect(updated.hours).toBe(8);

    // Verify persistence via API
    const fetched = await getTimeEntry(page, timeEntry.id);
    expect(fetched.hours).toBe(8);
  });

  test('should edit time entry work type', async ({ page }) => {
    // Create time entry via API
    const today = new Date().toISOString().split('T')[0];
    const timeEntry = await createTimeEntry(page, {
      work_type: 'workshop',
      hours: 4,
      work_date: today,
    });
    track.timeEntry(timeEntry.id);

    // Update work type via API
    const updated = await updateTimeEntry(page, timeEntry.id, { work_type: 'site' });
    expect(updated.work_type).toBe('site');

    // Verify persistence via API
    const fetched = await getTimeEntry(page, timeEntry.id);
    expect(fetched.work_type).toBe('site');
  });

  test('should delete time entry', async ({ page }) => {
    // Create time entry via API
    const today = new Date().toISOString().split('T')[0];
    const timeEntry = await createTimeEntry(page, {
      work_type: 'other',
      hours: 2,
      work_date: today,
    });
    track.timeEntry(timeEntry.id);

    // Delete via API
    await deleteTimeEntry(page, timeEntry.id);

    // Verify deletion via API (should return 404)
    const response = await page.request.get(`/api/v1/time-entries/${timeEntry.id}`);
    expect(response.status()).toBe(404);
  });
});

test.describe('Reservation Edit Operations', () => {
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

  test('should edit reservation notes', async ({ page }) => {
    // Create vehicle and reservation via API
    const vehicleName = uniqueName('Reservation Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      vehicle_type: 'van',
    });
    track.vehicle(vehicle.id);

    const now = new Date();
    const startTime = new Date(now.getTime() + 24 * 60 * 60 * 1000); // Tomorrow
    const endTime = new Date(startTime.getTime() + 8 * 60 * 60 * 1000); // 8 hours later

    const reservation = await createReservation(page, {
      resource_type: 'vehicle',
      resource_id: vehicle.id,
      start_time: startTime.toISOString(),
      end_time: endTime.toISOString(),
      notes: 'Original notes',
    });
    track.reservation(reservation.id);

    // Update notes via API
    const updated = await updateReservation(page, reservation.id, {
      notes: 'Updated notes for E2E test',
    });
    expect(updated.notes).toBe('Updated notes for E2E test');

    // Verify persistence via API
    const fetched = await getReservation(page, reservation.id);
    expect(fetched.notes).toBe('Updated notes for E2E test');
  });

  test('should edit reservation time range', async ({ page }) => {
    // Create vehicle and reservation via API
    const vehicleName = uniqueName('Time Range Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      vehicle_type: 'car',
    });
    track.vehicle(vehicle.id);

    const now = new Date();
    const startTime = new Date(now.getTime() + 48 * 60 * 60 * 1000); // Day after tomorrow
    const endTime = new Date(startTime.getTime() + 4 * 60 * 60 * 1000); // 4 hours

    const reservation = await createReservation(page, {
      resource_type: 'vehicle',
      resource_id: vehicle.id,
      start_time: startTime.toISOString(),
      end_time: endTime.toISOString(),
    });
    track.reservation(reservation.id);

    // Update time range
    const newStartTime = new Date(startTime.getTime() + 2 * 60 * 60 * 1000); // 2 hours later
    const newEndTime = new Date(newStartTime.getTime() + 8 * 60 * 60 * 1000); // 8 hours

    await updateReservation(page, reservation.id, {
      start_time: newStartTime.toISOString(),
      end_time: newEndTime.toISOString(),
    });

    // Verify times were updated
    const fetched = await getReservation(page, reservation.id);
    expect(new Date(fetched.start_time).getTime()).toBe(newStartTime.getTime());
    expect(new Date(fetched.end_time).getTime()).toBe(newEndTime.getTime());
  });
});
