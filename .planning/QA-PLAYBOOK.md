# Agent QA Playbook

A guide for future agent sessions to efficiently validate features, run tests, and navigate the codebase without repeated exploration.

## 1. Running Tests

### Backend Unit Tests
```bash
cargo test --lib
```
- All tests are inline in domain files
- No database required (pure unit tests)
- Should complete in under 1 second
- Look for `#[cfg(test)]` blocks in `src/modules/*/domain/*.rs`

### Frontend E2E Tests
```bash
cd frontend
npm run test:e2e
```
- Playwright tests in `frontend/tests/`
- Requires backend running on port 3000
- Requires frontend running on port 5175 (Vite dev server)
- Requires Keycloak accessible
- Test user: `schreiner@admin.test` (see `frontend/tests/helpers/auth.ts`)

### Running Specific Test Files
```bash
# Single E2E test file
cd frontend
npx playwright test auth.spec.ts

# Interactive UI mode
npm run test:e2e:ui

# View test report
npx playwright show-report
```

## 2. Checking Logs

### Backend Logs (Kubernetes)
```bash
kubectl logs -n schreinerei deployment/schreinerei-backend
kubectl logs -n schreinerei deployment/schreinerei-backend --tail=100
```

### Backend Logs (Local)
```bash
cargo run
# Logs appear in terminal
```

### Frontend Logs
- Browser DevTools Console
- Network tab for API requests

## 3. Validating Features

### General Pattern
1. Navigate to feature page (use sidebar links)
2. Check for expected UI elements (headings, buttons, forms)
3. Test CRUD operations (create, read, update, delete)
4. Verify data persists (check database or refresh page)

### Navigation Paths
- Dashboard: `/`
- Inventory: `/inventory`
- Sites: `/sites`
- Fleet: `/fleet` (vehicles and tools)

### E2E Test Pattern
```typescript
import { test, expect } from '@playwright/test';
import { login } from './helpers/auth';

test.describe('Feature Name', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('should do something', async ({ page }) => {
    await page.goto('/feature-path');
    await expect(page.locator('h1')).toBeVisible();
  });
});
```

## 4. Common Selectors (from existing tests)

| Element | Selector Pattern |
|---------|-----------------|
| Main content | `main` |
| Navigation | `a[href*="inventory"], a:has-text("Inventar")` |
| Headings | `h1:has-text("Inventar")` |
| Buttons | `button:has-text("hinzufügen")` |
| Inputs | `input[placeholder*="suchen"]` |

## 5. Authentication for E2E

Use the login helper from `frontend/tests/helpers/auth.ts`:
```typescript
import { login, logout } from './helpers/auth';

test.beforeEach(async ({ page }) => {
  await login(page);
});
```

The helper handles Keycloak's multi-step login flow.

### Keycloak Login Flow Details

The login helper (`frontend/tests/helpers/auth.ts`) handles:
1. Click login button
2. Wait for Keycloak redirect
3. Fill username, click Sign In
4. Fill password, submit
5. Wait for app redirect

**Port Configuration:** Frontend runs on port 5175 for local testing. Keycloak redirect URIs configured for `http://localhost:5175/*`.

**Test Credentials:**
- Email: `schreiner@admin.test`
- Password: See `frontend/tests/helpers/auth.ts`

## 6. Troubleshooting

### E2E Tests Fail with Auth Error
- Verify frontend runs on port 5175 (check `vite.config.ts`)
- Check Keycloak redirect URIs include `http://localhost:5175/*`
- Verify test user credentials in `frontend/tests/helpers/auth.ts`
- Ensure Keycloak is accessible

### Backend Tests Fail
- Check for compilation errors first: `cargo check`
- Tests should have no external dependencies
- Look for `#[cfg(test)]` module at bottom of domain files

### Database Issues
- Backend uses SQLx with runtime queries
- No migrations needed for unit tests
- For integration tests, see `tests/tenant_isolation_test.rs`

### Port Conflicts
- Frontend MUST run on port 5175 (enforced in `vite.config.ts`)
- If port is occupied, investigate what process is using it:
  ```bash
  lsof -i :5175
  ```

## 7. Preventing Frontend-Backend Parameter Mismatches

Parameter mismatches occur when frontend sends parameters that don't match backend expectations. Prevention strategy:

### Type Safety (ts-rs)

**Problem:** Manual TypeScript types drift from Rust DTOs.

**Solution:** Auto-generate TypeScript from Rust structs using ts-rs.

```rust
// Backend: src/modules/inventory/api/dto.rs
#[derive(Serialize, TS)]
#[ts(export)]
pub struct CreateMaterialRequest {
    pub name: String,
    pub quantity: i32,
    pub category_id: Uuid,
}
```

Generated TypeScript:
```typescript
// Frontend: src/types/generated.ts
export interface CreateMaterialRequest {
    name: string;
    quantity: number;
    category_id: string;
}
```

### Runtime Validation (Zod)

**Problem:** API responses might not match expected shape at runtime.

**Solution:** Validate API responses with Zod schemas.

```typescript
import { z } from 'zod';
import type { Material } from './types/generated';

const MaterialSchema = z.object({
    id: z.string(),
    name: z.string(),
    quantity: z.number(),
    category_id: z.string(),
});

// Validate before use
const response = await fetch('/api/materials');
const data = await response.json();
const material = MaterialSchema.parse(data) as Material;
```

### Checklist for New Endpoints

1. [ ] Add `#[derive(TS)]` and `#[ts(export)]` to Rust DTO
2. [ ] Run `cargo test --features ts-rs/export`
3. [ ] Import generated type in frontend
4. [ ] Add Zod schema for runtime validation (optional but recommended)
5. [ ] Add E2E test for the endpoint

### Common Mismatch Patterns to Avoid

| Pattern | Issue | Prevention |
|---------|-------|-------------|
| `snake_case` vs `camelCase` | Field name mismatch | Use `#[serde(rename_all = "camelCase")]` consistently |
| Optional fields | `null` vs `undefined` | Use `Option<T>` in Rust, `field?: T` in TS |
| Date format | String vs Date | Use `chrono::DateTime` with `#[ts(type = "Date")]` |
| UUID format | String vs object | UUIDs are strings in JSON, not objects |

### Debugging Mismatches

1. Check backend logs for deserialization errors
2. Check browser network tab for request/response shapes
3. Compare Rust DTO with TypeScript type
4. Run ts-rs generation to sync types

---

## Quick Reference

| Command | Purpose |
|---------|---------|
| `cargo test --lib` | Run backend unit tests |
| `cd frontend && npm run test:e2e` | Run all E2E tests |
| `npx playwright test auth.spec.ts` | Run specific test file |
| `kubectl logs -n schreinerei deployment/schreinerei-backend` | View backend logs |
| `cargo run` | Run backend locally with logs |

---

*Created: Phase 13 — Agent QA Playbook*
*Purpose: Reduce context cost for future development sessions*
