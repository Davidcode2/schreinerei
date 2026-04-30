# Feature Research

**Domain:** Testing & Quality Foundation for Rust + React SaaS
**Researched:** 2026-04-30
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

Testing features that are non-negotiable for a production SaaS. Missing these = fragile codebase and slow development velocity.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Backend unit tests | Business logic validation without DB | MEDIUM | Rust built-in #[test] + domain layer testing |
| Frontend unit tests | Component behavior validation | MEDIUM | Vitest + React Testing Library |
| Integration tests | API + database together | MEDIUM | SQLx #[sqlx::test] with auto-migrations |
| E2E critical paths | User flows work end-to-end | LOW | Playwright already in place (18 tests) |
| CI test automation | Tests run on every PR | LOW | GitHub Actions integration |
| Multi-tenant isolation tests | Tenant data never leaks | MEDIUM | Critical for SaaS security |

### Differentiators (Competitive Advantage)

Testing features that elevate code quality and developer velocity beyond the basics.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Domain layer unit tests | Pure business logic, no DB dependency | LOW | Hexagonal architecture enables this |
| Mock-free service tests | Test services with test database | MEDIUM | Use test containers or test DB |
| Offline scenario tests | PWA works without network | MEDIUM | Playwright offline emulation |
| Frontend-backend contract tests | API mismatches caught early | MEDIUM | Type-safe API client generation |
| Test fixtures factory | Consistent test data creation | LOW | Builder pattern for entities |
| Performance regression tests | Catch slow queries before prod | HIGH | SQLx query analysis |

### Anti-Features (Commonly Requested, Often Problematic)

Testing approaches that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| 100% code coverage | "More coverage = better quality" | Chases metrics, not bugs; brittle tests | Target critical paths + business logic |
| Mocking repositories directly | "Unit test everything in isolation" | Tests mock, not real code; false confidence | Integration tests with test DB |
| Snapshot testing everywhere | "Easy to write tests" | Breaks on any UI change; noisy failures | Use for stable outputs (API responses) |
| Parallel E2E tests | "Faster test runs" | State conflicts in DB; flaky tests | Sequential E2E with isolated tenants |
| Test data seeded once | "Faster test setup" | Tests depend on each other; order-sensitive | Fresh test DB per test (SQLx auto) |
| UI snapshot tests | "Visual regression" | Breaks on styling changes; maintenance burden | Focus on behavior, not appearance |

## Feature Dependencies

```
Backend Unit Tests
    └──requires──> Domain layer isolation (already have)
    └──requires──> Test fixtures for domain entities

Backend Integration Tests
    └──requires──> SQLx test framework setup
    └──requires──> Test database (auto-created per test)
    └──requires──> Migrations run automatically

Frontend Unit Tests
    └──requires──> Vitest configuration
    └──requires──> React Testing Library
    └──requires──> MSW for API mocking
    └──requires──> Test fixtures for component props

E2E Tests (Extended)
    └──requires──> Playwright (already have)
    └──enhances──> Offline scenario testing
    └──enhances──> Multi-tenant isolation verification

CI Integration
    └──requires──> Backend tests passing
    └──requires──> Frontend tests passing
    └──requires──> E2E tests passing
```

### Dependency Notes

- **Frontend unit tests require MSW:** Components make API calls; MSW intercepts these in test environment
- **Backend integration tests require SQLx test framework:** `#[sqlx::test]` macro creates isolated test databases automatically
- **E2E tests enhance offline testing:** Playwright has built-in `context.setOffline(true)` for offline scenarios
- **Test fixtures factory enables all test types:** Consistent data creation across unit, integration, and E2E tests

## MVP Definition

### Launch With (v1.5)

Minimum viable testing strategy — what's needed to prevent regressions and validate features.

- [ ] Backend unit tests for domain layer — Pure business logic validation without database dependency
- [ ] Backend integration tests for each module — Inventory, Sites, Fleet with real database
- [ ] Frontend unit tests for critical components — Forms, dialogs, navigation
- [ ] E2E offline scenario tests — PWA works without network
- [ ] CI integration — Tests run on every push to main

### Add After Validation (v1.x)

Testing enhancements once core coverage is established.

- [ ] Frontend-backend contract tests — OpenAPI spec generation + type-safe client
- [ ] Performance regression tests — Query execution time thresholds
- [ ] Visual regression tests for key pages — Playwright screenshots
- [ ] Test data factory pattern — Builder pattern for complex entities

### Future Consideration (v2+)

Advanced testing strategies for mature product.

- [ ] Mutation testing — Verify tests actually catch bugs
- [ ] Chaos testing — Database failover, network latency
- [ ] Load testing — API performance under load
- [ ] Security testing automation — SQL injection, XSS attempts

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Backend unit tests (domain) | HIGH | LOW | P1 |
| Backend integration tests | HIGH | MEDIUM | P1 |
| Frontend unit tests | MEDIUM | MEDIUM | P1 |
| E2E offline tests | HIGH | LOW | P1 |
| Multi-tenant isolation tests | HIGH | LOW | P1 |
| CI integration | HIGH | LOW | P1 |
| Test fixtures factory | MEDIUM | LOW | P2 |
| Contract tests | MEDIUM | MEDIUM | P2 |
| Performance tests | MEDIUM | HIGH | P3 |
| Visual regression tests | LOW | MEDIUM | P3 |

**Priority key:**
- P1: Must have for launch (testing foundation)
- P2: Should have, add when possible
- P3: Nice to have, future consideration

## Testing Patterns by Layer

### Backend (Rust)

| Layer | Test Type | Framework | Pattern |
|-------|-----------|-----------|---------|
| Domain | Unit | `#[test]` | Pure functions, no DB |
| Application | Integration | `#[sqlx::test]` | Real DB, test fixtures |
| Infrastructure | Integration | `#[sqlx::test]` | Repository with test DB |
| API | Integration | `axum::test` | HTTP client against test server |

### Frontend (React)

| Layer | Test Type | Framework | Pattern |
|-------|-----------|-----------|---------|
| Components | Unit | Vitest + RTL | User interactions, not implementation |
| Hooks | Unit | Vitest + @testing-library/react-hooks | Custom hook testing |
| API client | Unit | Vitest + MSW | Mock responses |
| E2E | Integration | Playwright | Full user flows |

## Implementation Complexity Notes

### Backend Unit Tests (LOW complexity)

Rust has built-in test support. Domain layer is already isolated (hexagonal architecture).

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_can_withdraw_sufficient_stock() {
        let material = Material { quantity: 10, min_quantity: 2, ..default() };
        assert!(material.can_withdraw(5));
    }

    #[test]
    fn material_cannot_withdraw_insufficient_stock() {
        let material = Material { quantity: 3, min_quantity: 2, ..default() };
        assert!(!material.can_withdraw(5));
    }
}
```

### Backend Integration Tests (MEDIUM complexity)

SQLx provides `#[sqlx::test]` macro that creates isolated test databases. Already used in `tenant_isolation_test.rs`.

```rust
#[sqlx::test(migrator = "crate::MIGRATOR")]
async fn test_create_material(pool: PgPool) {
    let repo = MaterialRepository::new(pool);
    let material = repo.create_material(&create_cmd, tenant_id).await.unwrap();
    assert!(material.id.0 != Uuid::nil());
}
```

### Frontend Unit Tests (MEDIUM complexity)

Requires Vitest setup, React Testing Library, and MSW for API mocking.

```typescript
// Vitest + RTL pattern
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { http, HttpResponse } from 'msw';
import { setupServer } from 'msw/node';

const server = setupServer(
  http.get('/api/materials', () => HttpResponse.json([]))
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());

test('shows material list', async () => {
  render(<MaterialList />);
  await waitFor(() => {
    expect(screen.getByText('Material')).toBeInTheDocument();
  });
});
```

### E2E Offline Tests (LOW complexity)

Playwright has built-in offline emulation.

```typescript
test('works offline', async ({ context, page }) => {
  await context.setOffline(true);
  await page.goto('/inventory');
  // App should show cached data
  await expect(page.locator('[data-offline-indicator]')).toBeVisible();
});
```

## Current State Analysis

| Category | Current State | Gap | Priority |
|----------|---------------|-----|----------|
| E2E Tests | 18 tests in Playwright | Offline scenarios | P1 |
| Backend Integration | 1 test (tenant isolation) | Module-specific tests | P1 |
| Backend Unit | None | Domain layer tests | P1 |
| Frontend Unit | None | Component tests | P1 |
| CI Integration | Not set up | GitHub Actions | P1 |

## Sources

- Rust testing: https://doc.rust-lang.org/book/ch11-00-testing.html (HIGH confidence)
- SQLx testing: https://github.com/launchbadge/sqlx (HIGH confidence)
- React Testing Library: https://testing-library.com/docs/react-testing-library/intro (HIGH confidence)
- Vitest: https://vitest.dev (HIGH confidence)
- Playwright: https://playwright.dev (HIGH confidence)
- MSW: https://mswjs.io (HIGH confidence)
- Mockall: https://docs.rs/mockall (HIGH confidence)
- Project codebase analysis (HIGH confidence)

---
*Feature research for: Testing & Quality Foundation*
*Researched: 2026-04-30*
