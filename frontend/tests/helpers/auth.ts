import { Page } from '@playwright/test';

const TEST_USER = {
  email: 'schreiner@admin.test',
  password: 'T6&Mo2wnhypFEZ$P$8QqdWELZ3BP5Hhe',
};

export async function login(page: Page, attempt = 1) {
  await page.goto('/');
  await page.waitForLoadState('domcontentloaded');
  await page.waitForTimeout(1000);
  
  const keycloakButton = page.locator('button:has-text("Keycloak"), button:has-text("anmelden")');
  const mainContent = page.locator('main');
  const hasStoredToken = async () =>
    page.evaluate(() => {
      const raw = window.localStorage.getItem('auth-storage');
      if (!raw) return false;
      const parsed = JSON.parse(raw) as {
        state?: { tokens?: { access_token?: string } };
      };
      return Boolean(parsed.state?.tokens?.access_token);
    });
  
  // Check if already logged in
  const hasMain = await mainContent.isVisible().catch(() => false);
  if (hasMain && await hasStoredToken().catch(() => false)) return;
  
  const hasLoginButton = await keycloakButton.isVisible().catch(() => false);
  if (!hasLoginButton) {
    // Wait longer for page to load
    await page.waitForTimeout(2000);
    if (await mainContent.isVisible().catch(() => false) && await hasStoredToken().catch(() => false)) return;
  }
  
  // Click Keycloak login
  await keycloakButton.click();
  await Promise.race([
    page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080/, { timeout: 15000 }),
    page.locator('input#username, #username').first().waitFor({ state: 'visible', timeout: 15000 }),
  ]);
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
  try {
    await page.waitForURL(/localhost:5175/, { timeout: 20000 });
  } catch (error) {
    if (attempt >= 2) {
      throw error;
    }

    await page.goto('/');
    if (await mainContent.isVisible().catch(() => false) && await hasStoredToken().catch(() => false)) {
      return;
    }

    return login(page, attempt + 1);
  }
  await page.waitForTimeout(1000);
  await mainContent.waitFor({ state: 'visible', timeout: 10000 });
  await page.waitForFunction(() => {
    const raw = window.localStorage.getItem('auth-storage');
    if (!raw) return false;
    const parsed = JSON.parse(raw) as {
      state?: { tokens?: { access_token?: string } };
    };
    return Boolean(parsed.state?.tokens?.access_token);
  });
}

export async function logout(page: Page) {
  const logoutBtn = page.locator('button:has-text("Abmelden"), button:has-text("Logout")');
  if (await logoutBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
    await logoutBtn.click();
    await page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080|localhost:5175\/login/, { timeout: 10000 });
  }
}
