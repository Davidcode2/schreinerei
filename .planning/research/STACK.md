# Stack Research

**Domain:** Testing & Quality Foundation for Schreinerei SaaS
**Researched:** 2026-04-30
**Confidence:** HIGH

## Existing Stack (No Changes Needed)

These are already in place and working:

| Technology | Version | Purpose |
|------------|---------|---------|
| Playwright | ^1.59.1 | E2E testing |
| SQLx test macros | 0.8 | Database integration tests |
| Rust built-in test | Stable | Unit test framework |
| Tokio test | 1 | Async test runtime |

## Recommended Additions

### Rust Backend Testing

| Library | Version | Purpose | Why Recommended |
|---------|---------|---------|-----------------|
| tower | 0.5 | Service trait for testing Axum | Industry standard for testing Axum handlers without HTTP server |
| http-body-util | 0.1 | Body extraction utilities | Required for reading response bodies in tower tests |
| testcontainers | 0.27 | Docker containers for integration tests | Real PostgreSQL for true integration tests, better than mocks |
| wiremock | 0.6 | HTTP mocking for external services | Mock Keycloak responses in tests, no real Keycloak needed |
| mockall | 0.14 | Trait mocking | For unit testing domain logic with mocked repositories |

### React Frontend Testing

| Library | Version | Purpose | Why Recommended |
|---------|---------|---------|-----------------|
| vitest | ^4.1 | Test runner with Vite integration | Native Vite support, fast, Jest-compatible API |
| @testing-library/react | ^16.3 | React component testing | React 19 compatible, user-centric testing approach |
| @testing-library/user-event | ^14.6 | Realistic user interactions | Better than fireEvent, simulates real browser events |
| @testing-library/jest-dom | ^6.9 | Extended DOM matchers | Readable assertions like `toBeInTheDocument()` |
| jsdom | ^29.1 | DOM environment for tests | Required for component testing without browser |
| msw | ^2.14 | API mocking | Intercept fetch/XHR, works with React Query |
| @vitest/coverage-v8 | ^4.1 | Code coverage | Native V8 coverage, fast |

## Installation

### Rust Backend

```toml
# Cargo.toml - add to [dev-dependencies]
[dev-dependencies]
tower = "0.5"
http-body-util = "0.1"
testcontainers = "0.27"
testcontainers-modules = { version = "0.12", features = ["postgres"] }
wiremock = "0.6"
mockall = "0.14"
```

```bash
cargo add --dev tower http-body-util testcontainers testcontainers-modules --features testcontainers-modules/postgres wiremock mockall
```

### React Frontend

```bash
cd frontend

# Core testing
npm install -D vitest @vitest/coverage-v8 jsdom

# React Testing Library
npm install -D @testing-library/react @testing-library/user-event @testing-library/jest-dom

# API mocking
npm install -D msw
```

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Jest | Slow with Vite, requires complex config | Vitest (native Vite integration) |
| Enzyme | Deprecated, doesn't work with React 18+ | @testing-library/react |
| Cypress | Playwright already in use, redundant | Extend Playwright coverage |
| Nock | Node.js only, doesn't work in browser | MSW (works in both) |
| sinon | Verbose mock setup | mockall (auto-generates mocks) |
| Mock database with handwritten fakes | Time-consuming, error-prone | testcontainers (real DB) |

## Test Patterns by Layer

### Domain Layer (Rust)
- **Unit tests** with `#[test]` and `mockall`
- Mock repositories and external services
- Test business rules in isolation

```rust
#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    
    #[test]
    fn test_material_stock_cannot_go_negative() {
        // Pure domain logic test
    }
}
```

### Application Layer (Rust)
- **Unit tests** with mocked ports (repositories, external APIs)
- Use `mockall` to generate mock implementations

```rust
#[automock]
pub trait MaterialRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Material>>;
}

#[tokio::test]
async fn test_reserve_material_decreases_stock() {
    let mut mock_repo = MockMaterialRepository::new();
    mock_repo.expect_find_by_id()
        .returning(|_| Ok(Some(test_material())));
    // Test use case with mocked repo
}
```

### Infrastructure Layer (Rust)
- **Integration tests** with `testcontainers` for real PostgreSQL
- **API tests** with `tower::Service` (no HTTP server needed)

```rust
use tower::ServiceExt; // for oneshot

#[tokio::test]
async fn test_get_material_endpoint() {
    let app = create_test_app().await;
    
    let response = app
        .oneshot(Request::builder()
            .uri("/api/materials/test-id")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
```

### External Services (Rust)
- **Wiremock** for mocking Keycloak HTTP responses

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_keycloak_token_validation() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/.well-known/openid-configuration"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(keycloak_config()))
        .mount(&mock_server)
        .await;
    
    // Test with mocked Keycloak
}
```

### React Components
- **Vitest + Testing Library** for unit/component tests

```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./tests/setup.ts'],
    include: ['src/**/*.{test,spec}.{ts,tsx}'],
  },
})
```

```typescript
// tests/setup.ts
import '@testing-library/jest-dom'
import { server } from './mocks/server'

beforeAll(() => server.listen())
afterEach(() => server.resetHandlers())
afterAll(() => server.close())
```

### React API Integration
- **MSW** for mocking backend API in tests

```typescript
// tests/mocks/handlers.ts
import { http, HttpResponse } from 'msw'

export const handlers = [
  http.get('/api/materials', () => {
    return HttpResponse.json([
      { id: '1', name: 'Test Material', stock: 10 }
    ])
  }),
]
```

```typescript
// tests/mocks/server.ts
import { setupServer } from 'msw/node'
import { handlers } from './handlers'

export const server = setupServer(...handlers)
```

### E2E Tests (Playwright)
- **Extend existing tests** with more scenarios
- Keep using existing helper patterns

```typescript
// Already established pattern - extend it
test('should create material via dialog', async ({ page }) => {
  await login(page);
  await page.goto('/inventory');
  
  await page.click('button:has-text("Material hinzufügen")');
  await page.fill('input[name="name"]', 'Test Material');
  await page.fill('input[name="quantity"]', '10');
  await page.click('button:has-text("Speichern")');
  
  await expect(page.locator('text=Test Material')).toBeVisible();
});
```

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| vitest ^4.1 | Vite ^6.0 \|\| ^7.0 \|\| ^8.0 | Native Vite integration |
| @testing-library/react ^16.3 | React ^18.0 \|\| ^19.0 | React 19 support confirmed |
| testcontainers 0.27 | tokio 1.x | Async runtime required |
| wiremock 0.6 | tokio 1.x | Async runtime required |
| msw 2.x | All modern browsers + Node | Works with Service Worker |

## Test Strategy Summary

| Layer | Test Type | Tools | Speed |
|-------|-----------|-------|-------|
| Domain | Unit | Rust + mockall | Fast |
| Application | Unit | Rust + mockall | Fast |
| Infrastructure | Integration | testcontainers + tower | Medium |
| External APIs | Unit | wiremock | Fast |
| React Components | Unit | Vitest + Testing Library | Fast |
| React API Calls | Unit | MSW | Fast |
| Full Stack | E2E | Playwright | Slow |

## Integration with Existing Architecture

The testing stack integrates with the hexagonal architecture:

```
Domain Layer        → Unit tests with mockall
Application Layer   → Unit tests with mockall  
Infrastructure Layer → Integration tests with testcontainers
                    → API tests with tower::Service
                    
Frontend Components → Vitest + Testing Library
Frontend API Calls  → MSW mocking backend
E2E Flows          → Playwright (real backend + real Keycloak)
```

## Sources

- /vitest-dev/vitest — React testing setup, configuration
- /testing-library/react-testing-library — Render, queries, userEvent patterns
- /mswjs/msw — Node.js integration, handler setup
- /testcontainers/testcontainers-rs — PostgreSQL async setup, wait strategies
- /lukemathwalker/wiremock-rs — HTTP mocking, request matching
- /asomers/mockall — Trait mocking, automock attribute
- npm registry — Version verification for all npm packages
- crates.io — Version verification for all Rust crates

---
*Stack research for: Schreinerei Testing & Quality Foundation*
*Researched: 2026-04-30*
