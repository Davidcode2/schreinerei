import { Page } from '@playwright/test';

const TEST_USER = {
  email: 'schreiner@admin.test',
  password: 'T6&Mo2wnhypFEZ$P$8QqdWELZ3BP5Hhe',
};

export async function login(page: Page) {
  await page.goto('/');
  
  if (page.url().includes('auth.jakob-lingel.dev') || page.url().includes('localhost:8080')) {
    await page.fill('input[name="username"]', TEST_USER.email);
    await page.fill('input[name="password"]', TEST_USER.password);
    await page.click('button[type="submit"]');
    await page.waitForURL(/localhost:5173|schreinerei/, { timeout: 30000 });
  }
  
  await page.waitForSelector('[data-testid="dashboard"], main', { timeout: 10000 });
}

export async function logout(page: Page) {
  const logoutButton = page.locator('button:has-text("Abmelden"), button:has-text("Logout")');
  if (await logoutButton.isVisible()) {
    await logoutButton.click();
    await page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080|login/, { timeout: 5000 });
  }
}
