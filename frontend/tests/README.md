# E2E Testing Guide

End-to-end tests for the Schreinerei SaaS application using Playwright.

## Running Tests

```bash
# All E2E tests
cd frontend && npm run test:e2e

# Specific test file
npx playwright test inventory.spec.ts

# Interactive UI mode
npm run test:e2e:ui

# View test report
npx playwright show-report
```

## Test Structure

```
frontend/tests/
├── helpers/
│   ├── auth.ts    # Keycloak login/logout
│   ├── api.ts     # API helper functions
│   └── data.ts    # Test data and cleanup
├── auth.spec.ts
├── inventory.spec.ts
├── sites.spec.ts
├── fleet.spec.ts
└── dashboard.spec.ts
```

## Data Assertions

**Problem:** Tests that only verify UI elements can miss data persistence bugs.

**Solution:** Use API calls to verify data was actually saved to the database.

### Pattern

```typescript
import { createMaterial, getMaterial } from './helpers/api';
import { useCleanup, uniqueName } from './helpers/data';

test.describe('Data Persistence', () => {
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

  test('should persist created material', async ({ page }) => {
    // 1. Create via API (or UI)
    const material = await createMaterial(page, {
      category_id: 'existing-category-id',
      name: uniqueName('Test'),
      unit: 'Stück',
      quantity: 10,
      min_quantity: 2,
    });
    track.material(material.id);

    // 2. Verify via API
    const fetched = await getMaterial(page, material.id);
    expect(fetched.name).toBe(material.name);
    expect(fetched.quantity).toBe(10);
  });
});
```

### Key Principles

1. **Always verify via API** — Don't trust UI success states
2. **Use unique names** — `uniqueName('prefix')` prevents test conflicts
3. **Track for cleanup** — `track.material(id)` ensures cleanup even if test fails
4. **Clean up in afterEach** — Leaves database clean for next test run

### API Helpers Available

| Function | Purpose |
|----------|---------|
| `createMaterial(page, data)` | Create a material, returns created object |
| `getMaterial(page, id)` | Fetch material by ID |
| `listMaterials(page)` | List all materials |
| `deleteMaterial(page, id)` | Delete a material |
| `createSite(page, data)` | Create a site |
| `getSite(page, id)` | Fetch site by ID |
| `listSites(page)` | List all sites |
| `deleteSite(page, id)` | Delete a site |
| `createVehicle(page, data)` | Create a vehicle |
| `getVehicle(page, id)` | Fetch vehicle by ID |
| `listVehicles(page)` | List all vehicles |
| `deleteVehicle(page, id)` | Delete a vehicle |
| `createTool(page, data)` | Create a tool |
| `listTools(page)` | List all tools |
| `deleteTool(page, id)` | Delete a tool |
| `listCategories(page)` | List inventory categories |
| `createCategory(page, data)` | Create a category |

## Authentication

All tests use the `login(page)` helper from `./helpers/auth.ts`:

```typescript
test.beforeEach(async ({ page }) => {
  await login(page);
});
```

The helper handles Keycloak's multi-step authentication flow.

## Port Configuration

- Frontend runs on port **5175** (enforced in vite.config.ts)
- Keycloak redirect URIs configured for `http://localhost:5175/*`
- Backend runs on port **3000**

## Troubleshooting

### Tests fail with auth errors
- Verify frontend runs on port 5175
- Check Keycloak is accessible
- Verify test user credentials in `helpers/auth.ts`

### Data persists between tests
- Ensure `cleanup()` is called in `afterEach`
- Check that all created resources are tracked with `track.*()`

### API calls return 401
- Ensure `login(page)` is called in `beforeEach`
- API calls inherit auth from page context

---

*See also: [QA-PLAYBOOK.md](../../.planning/QA-PLAYBOOK.md) for validation procedures*
