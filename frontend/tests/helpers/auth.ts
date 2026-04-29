import { Page } from '@playwright/test';

const TEST_USER = {
  email: 'schreiner@admin.test',
  password: 'T6&Mo2wnhypFEZ$P$8QqdWELZ3BP5Hhe',
};

export async function login(page: Page) {
  await page.goto('/');
  await page.waitForLoadState('domcontentloaded');
  await page.waitForTimeout(1000);
  
  const keycloakButton = page.locator('button:has-text("Keycloak"), button:has-text("anmelden")');
  const mainContent = page.locator('main');
  
  // Check if already logged in
  const hasMain = await mainContent.isVisible().catch(() => false);
  if (hasMain) return;
  
  const hasLoginButton = await keycloakButton.isVisible().catch(() => false);
  if (!hasLoginButton) {
    // Wait longer for page to load
    await page.waitForTimeout(2000);
    if (await mainContent.isVisible().catch(() => false)) return;
  }
  
  // Click Keycloak login
  await keycloakButton.click();
  await page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080/, { timeout: 15000 });
  await page.waitForTimeout(500);
  
  // Fill username (Keycloak multi-step auth)
  const usernameInput = page.locator('input#username, #username').first();
  await usernameInput.waitFor({ state: 'visible', timeout: 10000 });
  await usernameInput.fill(TEST_USER.email);
  
  // Click Sign In to show password
  await page.locator('button:has-text("Sign In"), #kc-login').first().click();
  await page.waitForTimeout(500);
  
  // Fill password
  const passwordInput = page.locator('input#password, #password').first();
  await passwordInput.waitFor({ state: 'visible', timeout: 10000 });
  await passwordInput.fill(TEST_USER.password);
  
  // Submit
  await page.locator('button:has-text("Sign In"), #kc-login').first().click();
  
  // Wait for app
  await page.waitForURL(/localhost:5175/, { timeout: 20000 });
  await page.waitForTimeout(1000);
  await mainContent.waitFor({ state: 'visible', timeout: 10000 });
}

export async function logout(page: Page) {
  const logoutBtn = page.locator('button:has-text("Abmelden"), button:has-text("Logout")');
  if (await logoutBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
    await logoutBtn.click();
    await page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080|localhost:5175\/login/, { timeout: 10000 });
  }
}
