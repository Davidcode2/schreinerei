import { Page } from '@playwright/test';

const TEST_USER = {
  email: 'schreiner@admin.test',
  password: 'T6&Mo2wnhypFEZ$P$8QqdWELZ3BP5Hhe',
};

export async function login(page: Page) {
  await page.goto('/');
  
  // Check if we're on the login page (need to click Keycloak button)
  const keycloakButton = page.locator('button:has-text("Keycloak"), button:has-text("anmelden")');
  if (await keycloakButton.isVisible()) {
    await keycloakButton.click();
  }
  
  // Wait for redirect to Keycloak if not already logged in
  await page.waitForTimeout(1000);
  
  // Check if we're on Keycloak login page
  if (page.url().includes('auth.jakob-lingel.dev') || page.url().includes('localhost:8080')) {
    // Try different possible username field selectors
    const usernameInput = page.locator('input[name="username"], input[id="username"], input[type="text"]').first();
    await usernameInput.fill(TEST_USER.email);
    
    const passwordInput = page.locator('input[name="password"], input[id="password"], input[type="password"]').first();
    await passwordInput.fill(TEST_USER.password);
    
    await page.click('button[type="submit"], input[type="submit"]');
    
    // Wait for redirect back to app
    await page.waitForURL(/localhost:5173|schreinerei/, { timeout: 30000 });
  }
  
  // Wait for app to load - the AppLayout has a <main> tag
  await page.waitForSelector('main', { timeout: 15000 });
}

export async function logout(page: Page) {
  const logoutButton = page.locator('button:has-text("Abmelden"), button:has-text("Logout")');
  if (await logoutButton.isVisible()) {
    await logoutButton.click();
    await page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080|login/, { timeout: 5000 });
  }
}
