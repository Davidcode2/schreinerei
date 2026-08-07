import { expect, test, type Page } from '@playwright/test';
import path from 'node:path';
import { login } from './helpers/auth';

const screenshotDirectory = path.resolve(
  process.cwd(),
  '../.planning/frontend-ui-audit/screenshots',
);

async function settle(page: Page) {
  await page.waitForLoadState('domcontentloaded');
  await page.waitForTimeout(500);
}

async function capture(page: Page, name: string, fullPage = true) {
  await settle(page);
  await page.screenshot({
    path: path.join(screenshotDirectory, `${name}.png`),
    fullPage,
  });
}

async function closeOverlay(page: Page) {
  await page.keyboard.press('Escape');
  await expect(page.getByRole('dialog')).toHaveCount(0);
}

test.describe('UI audit screenshots', () => {
  test('captures desktop pages and overlays', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 1000 });
    await login(page);

    await page.goto('/');
    await capture(page, '01-dashboard-desktop');

    await page.goto('/inventory');
    await capture(page, '02-inventory-list');
    await page.getByRole('button', { name: /Material hinzuf/i }).click();
    await capture(page, '03-dialog-add-material', false);
    await closeOverlay(page);

    await page.goto('/inventory/00000000-0000-0000-0000-000000000311');
    await capture(page, '04-inventory-detail-expiry-low-stock');
    await page.getByRole('button', { name: 'Material bearbeiten' }).click();
    await capture(page, '05-dialog-edit-material', false);
    await closeOverlay(page);
    await page.getByRole('button', { name: 'Material entnehmen' }).first().click();
    await capture(page, '06-dialog-withdraw-expired-material', false);
    await closeOverlay(page);
    await page.getByRole('button', { name: /Einlagern/ }).first().click();
    await capture(page, '07-dialog-stock-in-expiry', false);
    await closeOverlay(page);

    await page.goto('/sites');
    await capture(page, '08-project-list');
    await page.getByRole('button', { name: /Projekt anlegen/i }).click();
    await capture(page, '09-dialog-add-project', false);
    await closeOverlay(page);

    await page.goto('/sites/00000000-0000-0000-0000-000000000401');
    await capture(page, '10-project-overview');
    await page.getByRole('button', { name: 'Planen' }).click();
    await capture(page, '11-sheet-project-planning', false);
    await closeOverlay(page);
    await page.getByRole('button', { name: 'Zeit buchen' }).click();
    await capture(page, '12-dialog-book-time', false);
    await closeOverlay(page);
    await page.getByText('Aktiv', { exact: true }).first().click();
    await capture(page, '13-dialog-project-status', false);
    await closeOverlay(page);
    await page.getByRole('button', { name: 'Termin', exact: true }).click();
    await capture(page, '14-dialog-project-appointment', false);
    await closeOverlay(page);

    await page.goto('/sites/00000000-0000-0000-0000-000000000401/details');
    await capture(page, '15-project-details');
    await page.getByRole('button', { name: 'Rechnung erstellen' }).click();
    await capture(page, '16-dialog-create-invoice', false);
    await closeOverlay(page);

    await page.goto('/sites/00000000-0000-0000-0000-000000000401/time');
    await capture(page, '17-project-time-list');

    await page.goto('/fleet');
    await capture(page, '18-fleet-calendar-and-list');
    await page.getByRole('button', { name: /Fahrzeug hinzuf/i }).click();
    await capture(page, '19-dialog-add-vehicle', false);
    await closeOverlay(page);

    await page.goto('/tools');
    await capture(page, '20-tools-calendar-and-list');
    const reservationEntry = page.locator('[role="button"]').filter({
      hasText: /Martin Brenner|Lukas Eisele/,
    }).first();
    await expect(reservationEntry).toBeVisible();
    await reservationEntry.click();
    await expect(page.getByRole('dialog', { name: 'Reservierung bearbeiten' })).toBeVisible();
    await capture(page, '21-dialog-edit-tool-reservation-priority', false);
    await page.getByRole('button', { name: 'Löschen' }).click();
    await expect(page.getByRole('alertdialog', { name: 'Reservierung stornieren?' })).toBeVisible();
    await capture(page, '35-dialog-confirm-reservation-cancellation', false);
    await page.getByRole('alertdialog').getByRole('button', { name: 'Abbrechen' }).click();
    await closeOverlay(page);

    const emptyToolSlots = page.locator('button[data-selection-state="idle"]');
    await emptyToolSlots.nth(0).click();
    await emptyToolSlots.nth(1).click();
    await expect(page.getByRole('dialog')).toBeVisible();
    await capture(page, '22-sheet-calendar-reservation-confirmation', false);
    await closeOverlay(page);

    await page.goto('/tools/00000000-0000-0000-0000-000000000603');
    await capture(page, '23-tool-detail-maintenance');
    await page.getByRole('button', { name: 'Planen' }).click();
    await capture(page, '24-dialog-maintenance-plan', false);
    await closeOverlay(page);

    await page.goto('/settings');
    await capture(page, '25-settings');
    await page.getByRole('button', { name: 'Einladen' }).click();
    await capture(page, '26-dialog-invite-user', false);
    await closeOverlay(page);

    await page.goto('/settings/inventory');
    await capture(page, '27-inventory-settings');
    await page.getByRole('button', { name: 'Kategorie anlegen' }).click();
    await capture(page, '28-dialog-add-category', false);
    await closeOverlay(page);

    await page.goto('/sites/history');
    await capture(page, '29-project-history');

    await page.goto('/does-not-exist');
    await capture(page, '30-not-found');
  });

  test('captures mobile navigation and priority dialog', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await login(page);

    await page.goto('/');
    await capture(page, '31-dashboard-mobile');
    await page.getByRole('button', { name: 'Menü öffnen' }).click();
    await capture(page, '32-mobile-navigation', false);
    await closeOverlay(page);

    await page.goto('/tools');
    await capture(page, '33-tools-mobile');
    const reservationEntry = page.locator('[role="button"]').filter({
      hasText: /Martin Brenner|Lukas Eisele/,
    }).first();
    await expect(reservationEntry).toBeVisible();
    await reservationEntry.click();
    await expect(page.getByRole('dialog', { name: 'Reservierung bearbeiten' })).toBeVisible();
    await capture(page, '34-dialog-edit-tool-reservation-mobile-priority', false);
  });
});
