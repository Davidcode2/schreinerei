# E2E Test Patterns (Playwright)

Playwright patterns and learnings discovered during v1.0-v1.4 development.

## Playwright Learnings (QA-01)

### Authentication Pattern

**Multi-step Keycloak Login:**
```typescript
// 1. Click login button
await page.locator('button:has-text("Keycloak")').click();

// 2. Wait for Keycloak redirect
await page.waitForURL(/auth\.jakob-lingel\.dev|localhost:8080/);

// 3. Fill username, click Sign In
await page.locator('#username').fill(email);
await page.locator('#kc-login').click();

// 4. Fill password, submit
await page.locator('#password').fill(password);
await page.locator('#kc-login').click();

// 5. Wait for app redirect
await page.waitForURL(/localhost:5175/);
```

**Key Insight:** Keycloak uses multi-step auth (username first, then password). Tests must handle both steps.

**Port Configuration:** Frontend runs on port 5175 for local testing. Keycloak redirect URIs configured for `http://localhost:5175/*`.

### Navigation Patterns

**Sidebar Navigation:**
```typescript
// German labels in UI
await page.click('a:has-text("Inventar")');
await page.click('a:has-text("Baustellen")');
await page.click('a:has-text("Fuhrpark")');
```

**Direct URL Navigation:**
```typescript
await page.goto('/inventory');
await page.goto('/sites');
await page.goto('/fleet');
```

### Common Selectors

| Element | Selector Pattern |
|---------|-----------------|
| Main content | `main` |
| Headings | `h1:has-text("Text")` |
| Buttons | `button:has-text("Text")` |
| Inputs | `input[placeholder*="text"]` |
| Links | `a[href*="path"]` |
| German text | `:has-text("Inventar")` |

### Best Practices

1. **Always use `beforeEach` for login:**
   ```typescript
   test.beforeEach(async ({ page }) => {
     await login(page);
   });
   ```

2. **Wait for visibility, not just existence:**
   ```typescript
   await expect(locator).toBeVisible({ timeout: 5000 });
   ```

3. **Use flexible selectors:**
   ```typescript
   // Multiple possible button texts
   const btn = page.locator('button:has-text("hinzufügen"), button:has-text("Hinzufügen")');
   ```

4. **Check console for errors:**
   ```typescript
   const errors: string[] = [];
   page.on('console', msg => {
     if (msg.type() === 'error') errors.push(msg.text());
   });
   ```

### Local Testing Port Configuration

**Frontend Port 5175:**
- Frontend MUST run on port 5175 for local testing (enforced in `vite.config.ts`)
- Keycloak redirect URIs configured for `http://localhost:5175/*`
- This prevents port mismatch errors during E2E tests

**Keycloak Configuration:**
- Valid Redirect URIs: `http://localhost:5175/*`
- Web Origins: `http://localhost:5175`
- Test user: `schreiner@admin.test`

### Test Structure

```
frontend/tests/
├── helpers/
│   └── auth.ts          # Login/logout helpers
├── auth.spec.ts         # Authentication tests
├── inventory.spec.ts    # Material management
├── fleet.spec.ts        # Vehicles and tools
├── sites.spec.ts        # Construction sites
├── dashboard.spec.ts    # Dashboard overview
├── BUGS.md              # Discovered issues
└── TEST-SUMMARY.md      # Test results
```

### Running Tests

```bash
# All E2E tests
cd frontend && npm run test:e2e

# Specific test file
npx playwright test auth.spec.ts

# Interactive UI mode
npm run test:e2e:ui

# View test report
npx playwright show-report
```

## Discovered Issues and Resolutions

### BUG-01: Keycloak Redirect URI Invalid
- **Cause:** Port changed from 5173 to 5174 due to port conflict
- **Fix:** Ensure frontend always runs on port 5175 (enforced in vite.config.ts)

### BUG-02: Port 5173 Occupied
- **Cause:** Unknown process occupying default Vite port
- **Resolution:** Frontend now enforces port 5175 in configuration

## Test Coverage Summary

| Suite | Tests | Focus |
|-------|-------|-------|
| auth | 2 | Login, logout |
| inventory | 4 | Navigation, elements, add button, search |
| fleet | 5 | Navigation, vehicles, tools, reservations |
| sites | 4 | Navigation, site creation, list, time booking |
| dashboard | 3 | Load, sites overview, console errors |
| **Total** | **18** | All major user flows |

---

*Created: Phase 13 — Agent QA Playbook*
*Reference: See also QA-PLAYBOOK.md for validation procedures*
