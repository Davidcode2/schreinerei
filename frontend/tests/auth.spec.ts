import { test, expect } from '@playwright/test';
import { login, logout } from './helpers/auth';

test.describe('Authentication', () => {
  test('should login successfully', async ({ page }) => {
    await login(page);
    await expect(page).toHaveURL(/localhost:5173|schreinerei/);
    await expect(page.locator('main')).toBeVisible();
  });

  test('should logout successfully', async ({ page }) => {
    await login(page);
    await logout(page);
    await expect(page).not.toHaveURL(/localhost:5173.*dashboard/);
  });
});
