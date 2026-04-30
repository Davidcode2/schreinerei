import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';
import {
  createReservation,
  getReservation,
  updateReservation,
  createVehicle,
  createSite,
} from './helpers/api';
import { useCleanup, uniqueName } from './helpers/data';
import type { ReservationStatus } from './helpers/api';

test.describe('Reservation Status Transitions', () => {
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

  test('should confirm a pending reservation', async ({ page }) => {
    // Create vehicle and pending reservation via API
    const vehicleName = uniqueName('Confirm Test Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      vehicle_type: 'van',
    });
    track.vehicle(vehicle.id);

    const now = new Date();
    const startTime = new Date(now.getTime() + 24 * 60 * 60 * 1000);
    const endTime = new Date(startTime.getTime() + 8 * 60 * 60 * 1000);

    const reservation = await createReservation(page, {
      resource_type: 'vehicle',
      resource_id: vehicle.id,
      start_time: startTime.toISOString(),
      end_time: endTime.toISOString(),
    });
    track.reservation(reservation.id);

    // New reservations should be pending
    expect(reservation.status).toBe('pending');

    // Navigate to fleet page
    await page.goto('/fleet');
    await page.waitForSelector('h1:has-text("Fuhrpark")', { timeout: 5000 });

    // Update status to confirmed via API (simulating UI button click)
    const updated = await updateReservation(page, reservation.id, {
      status: 'confirmed' as ReservationStatus,
    });
    expect(updated.status).toBe('confirmed');

    // Verify persistence via API
    const fetched = await getReservation(page, reservation.id);
    expect(fetched.status).toBe('confirmed');
  });

  test('should start a confirmed reservation', async ({ page }) => {
    // Create vehicle and confirmed reservation via API
    const vehicleName = uniqueName('Start Test Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      vehicle_type: 'car',
    });
    track.vehicle(vehicle.id);

    const now = new Date();
    const startTime = new Date(now.getTime() + 2 * 60 * 60 * 1000); // 2 hours from now
    const endTime = new Date(startTime.getTime() + 8 * 60 * 60 * 1000);

    const reservation = await createReservation(page, {
      resource_type: 'vehicle',
      resource_id: vehicle.id,
      start_time: startTime.toISOString(),
      end_time: endTime.toISOString(),
    });
    track.reservation(reservation.id);

    // First confirm it
    await updateReservation(page, reservation.id, {
      status: 'confirmed' as ReservationStatus,
    });

    // Navigate to fleet page
    await page.goto('/fleet');
    await page.waitForSelector('h1:has-text("Fuhrpark")', { timeout: 5000 });

    // Update status to in_use
    const updated = await updateReservation(page, reservation.id, {
      status: 'in_use' as ReservationStatus,
    });
    expect(updated.status).toBe('in_use');

    // Verify persistence
    const fetched = await getReservation(page, reservation.id);
    expect(fetched.status).toBe('in_use');
  });

  test('should complete an in-use reservation', async ({ page }) => {
    // Create vehicle and in_use reservation via API
    const vehicleName = uniqueName('Complete Test Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      vehicle_type: 'truck',
    });
    track.vehicle(vehicle.id);

    const now = new Date();
    const startTime = new Date(now.getTime() - 4 * 60 * 60 * 1000); // Started 4 hours ago
    const endTime = new Date(now.getTime() + 4 * 60 * 60 * 1000); // Ends 4 hours from now

    const reservation = await createReservation(page, {
      resource_type: 'vehicle',
      resource_id: vehicle.id,
      start_time: startTime.toISOString(),
      end_time: endTime.toISOString(),
    });
    track.reservation(reservation.id);

    // Transition to confirmed then in_use
    await updateReservation(page, reservation.id, { status: 'confirmed' as ReservationStatus });
    await updateReservation(page, reservation.id, { status: 'in_use' as ReservationStatus });

    // Navigate to fleet page
    await page.goto('/fleet');
    await page.waitForSelector('h1:has-text("Fuhrpark")', { timeout: 5000 });

    // Update status to completed
    const updated = await updateReservation(page, reservation.id, {
      status: 'completed' as ReservationStatus,
    });
    expect(updated.status).toBe('completed');

    // Verify persistence
    const fetched = await getReservation(page, reservation.id);
    expect(fetched.status).toBe('completed');
  });

  test('should cancel a pending reservation', async ({ page }) => {
    // Create vehicle and pending reservation via API
    const vehicleName = uniqueName('Cancel Test Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      vehicle_type: 'van',
    });
    track.vehicle(vehicle.id);

    const now = new Date();
    const startTime = new Date(now.getTime() + 72 * 60 * 60 * 1000); // 3 days from now
    const endTime = new Date(startTime.getTime() + 8 * 60 * 60 * 1000);

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

    // Update status to cancelled
    const updated = await updateReservation(page, reservation.id, {
      status: 'cancelled' as ReservationStatus,
    });
    expect(updated.status).toBe('cancelled');

    // Verify persistence
    const fetched = await getReservation(page, reservation.id);
    expect(fetched.status).toBe('cancelled');
  });

  test('should transition through full workflow', async ({ page }) => {
    // Create vehicle and pending reservation
    const vehicleName = uniqueName('Full Workflow Vehicle');
    const vehicle = await createVehicle(page, {
      name: vehicleName,
      vehicle_type: 'car',
    });
    track.vehicle(vehicle.id);

    const now = new Date();
    const startTime = new Date(now.getTime() + 12 * 60 * 60 * 1000);
    const endTime = new Date(startTime.getTime() + 8 * 60 * 60 * 1000);

    const reservation = await createReservation(page, {
      resource_type: 'vehicle',
      resource_id: vehicle.id,
      start_time: startTime.toISOString(),
      end_time: endTime.toISOString(),
    });
    track.reservation(reservation.id);

    // Verify initial status
    expect(reservation.status).toBe('pending');

    // Transition: pending → confirmed
    let updated = await updateReservation(page, reservation.id, {
      status: 'confirmed' as ReservationStatus,
    });
    expect(updated.status).toBe('confirmed');

    // Transition: confirmed → in_use
    updated = await updateReservation(page, reservation.id, {
      status: 'in_use' as ReservationStatus,
    });
    expect(updated.status).toBe('in_use');

    // Transition: in_use → completed
    updated = await updateReservation(page, reservation.id, {
      status: 'completed' as ReservationStatus,
    });
    expect(updated.status).toBe('completed');

    // Verify final state via API
    const fetched = await getReservation(page, reservation.id);
    expect(fetched.status).toBe('completed');
  });
});
