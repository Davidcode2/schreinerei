import { Page } from '@playwright/test';

const TEST_USER = {
  email: 'schreiner@admin.test',
  password: 'T6&Mo2wnhypFEZ$P$8QqdWELZ3BP5Hhe',
};

export async function login(page: Page) {
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  
  // Check if we need to click Keycloak login button
  const keycloakButton = page.locator('button:has-text("Keycloak"), button:has-text("anmelden")');
  if (await keycloakButton.isVisible({ timeout: 2000 }).catch(() => false)) {
    await keycloakButton.click();
    await page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080/, { timeout: 10000 });
  }
  
  // Check for Keycloak error
  const errorText = await page.locator('body').textContent().catch(() => '');
  if (errorText.includes('Invalid parameter') || errorText.includes('We are sorry')) {
    throw new Error(`Keycloak error: redirect_uri not configured for port 5175`);
  }
  
  // Handle Keycloak login if on auth page
  let url = page.url();
  while (url.includes('auth.jakob-lingel.dev') || url.includes('localhost:8080')) {
    // Wait for visible inputs
    await page.waitForSelector('input:visible', { timeout: 5000 });
    
    // Get all visible input fields
    const inputs = page.locator('input:visible');
    const inputCount = await inputs.count();
    
    // Fill visible empty inputs
    for (let i = 0; i < inputCount; i++) {
      const input = inputs.nth(i);
      const value = await input.inputValue().catch(() => '');
      const type = await input.getAttribute('type').catch(() => 'text');
      
      if (!value) {
        if (type === 'password') {
          await input.fill(TEST_USER.password);
        } else {
          await input.fill(TEST_USER.email);
        }
      }
    }
    
    // Submit
    await page.locator('button:has-text("Sign In")').click();
    
    // Wait a moment for navigation
    await page.waitForTimeout(1000);
    
    // Check if we're still on Keycloak (multi-step auth)
    url = page.url();
    if (!url.includes('auth.jakob-lingel.dev') && !url.includes('localhost:8080')) {
      break;
    }
  }
  
  // Wait for main app
  await page.waitForSelector('main', { timeout: 15000 });
}

export async function logout(page: Page) {
  const logoutBtn = page.locator('button:has-text("Abmelden"), button:has-text("Logout")');
  if (await logoutBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
    await logoutBtn.click();
    await page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080|login/, { timeout: 5000 });
  }
}
