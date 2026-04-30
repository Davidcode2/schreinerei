# Pitfalls Research

**Domain:** Testing & Quality Foundation for Rust/React Multi-Tenant SaaS
**Researched:** 2026-04-30
**Confidence:** HIGH

## Critical Pitfalls

### Pitfall 1: Frontend-Backend Type Drift

**What goes wrong:**
Frontend TypeScript types (`frontend/src/types/inventory.ts`) are manually maintained copies of backend Rust DTOs (`src/modules/inventory/api/routes.rs`). When backend changes a field name, type, or adds a new field, the frontend type doesn't update automatically, causing runtime errors or silent data mismatches.

**Why it happens:**
No automated type synchronization. Developers update backend DTOs and forget to update corresponding frontend types, or vice versa. The types look similar but aren't guaranteed to match.

**How to avoid:**
1. **Use ts-rs** to generate TypeScript types from Rust structs:
   ```rust
   #[derive(TS, Serialize)]
   #[ts(export)]
   pub struct MaterialResponse {
       pub id: String,
       pub category_id: String,
       // ... fields auto-sync to TypeScript
   }
   ```
2. Run `cargo test` to regenerate TypeScript bindings
3. Import generated types in frontend instead of manual definitions
4. CI check: fail build if generated types differ from committed types

**Warning signs:**
- Frontend receives `undefined` for expected fields
- API calls return data that frontend can't display
- TypeScript compilation passes but runtime errors occur
- Backend adds field, frontend doesn't show it for weeks

**Phase to address:** TEST-01 (Backend Unit Tests) — Add ts-rs derive macros and type generation

---

### Pitfall 2: Incomplete Multi-Tenant Test Coverage

**What goes wrong:**
Tests verify happy paths but miss cross-tenant data leakage. A developer adds a new query that forgets `WHERE tenant_id = $1`, and existing tests pass because they only test single-tenant scenarios.

**Why it happens:**
Testing multi-tenant isolation requires explicit cross-tenant test scenarios. Most tests create one tenant and verify functionality, not isolation. SQLx compile-time checks don't verify tenant scoping.

**How to avoid:**
1. Every test that queries data must test cross-tenant isolation
2. Create two tenants with similar data, verify queries only return current tenant's data
3. Test pattern from existing `tenant_isolation_test.rs`:
   ```rust
   // Create tenant A and B with users
   let tenant_a = create_test_tenant(&pool, "Tenant A").await;
   let tenant_b = create_test_tenant(&pool, "Tenant B").await;
   
   // Query with tenant_a context, verify no tenant_b data
   let count = sqlx::query_scalar::<_, i64>(
       "SELECT COUNT(*) FROM users WHERE tenant_id = $1"
   )
   .bind(tenant_a)  // MUST be in every query
   .fetch_one(&pool)
   .await?;
   
   assert_eq!(count, expected_tenant_a_count);
   ```
4. Add test macro or helper that enforces tenant context in all queries

**Warning signs:**
- New query doesn't have `tenant_id` in WHERE clause
- Test only creates one tenant
- Integration tests skip tenant parameter
- Admin features that "see all data" bypass tenant check

**Phase to address:** TEST-03/04/05 (Integration Tests) — Require cross-tenant test for each module

---

### Pitfall 3: SQLx Runtime Queries Untested

**What goes wrong:**
SQLx is configured with runtime queries (not compile-time checked) for this project. Queries with wrong syntax, missing parameters, or incorrect types compile fine but fail at runtime in production.

**Why it happens:**
Compile-time checked queries require database at build time. Project uses `DATABASE_URL` with runtime checking, which defers validation to when queries actually execute.

**How to avoid:**
1. Use `#[sqlx::test]` macro which creates isolated test databases:
   ```rust
   #[sqlx::test]
   async fn test_query(pool: PgPool) -> sqlx::Result<()> {
       let result = sqlx::query("SELECT * FROM materials WHERE tenant_id = $1")
           .bind(tenant_id)
           .fetch_all(&pool)
           .await?;
       Ok(())
   }
   ```
2. Every route handler must have at least one integration test
3. Test query success AND failure cases
4. CI runs `cargo test` against real PostgreSQL instance

**Warning signs:**
- Query compiles but fails at runtime
- Missing `DATABASE_URL` environment variable
- Tests only mock database calls, don't execute real queries
- New route added without integration test

**Phase to address:** TEST-03/04/05 (Integration Tests) — Every route needs at least one test

---

### Pitfall 4: React Query Cache Interference in Tests

**What goes wrong:**
Tests pass individually but fail when run together. One test's data appears in another test. Queries return stale data. Tests are flaky and inconsistent.

**Why it happens:**
React Query (TanStack Query) caches responses. Tests share the same QueryClient instance. One test's mutation or query pollutes another test's cache.

**How to avoid:**
1. Create fresh QueryClient for each test:
   ```typescript
   const queryClient = new QueryClient({
     defaultOptions: {
       queries: { retry: false },
     },
   });
   
   const wrapper = ({ children }) => (
     <QueryClientProvider client={queryClient}>
       {children}
     </QueryClientProvider>
   );
   ```
2. Clear cache between tests:
   ```typescript
   afterEach(() => {
     queryClient.clear();
   });
   ```
3. Use MSW (Mock Service Worker) for API mocking:
   ```typescript
   import { setupServer } from 'msw/node';
   const server = setupServer(...handlers);
   beforeAll(() => server.listen());
   afterEach(() => server.resetHandlers());
   afterAll(() => server.close());
   ```

**Warning signs:**
- Tests fail when run in different order
- Tests pass in isolation, fail in full suite
- Stale data appears in tests
- Flaky tests that sometimes pass, sometimes fail

**Phase to address:** TEST-02 (Frontend Unit Tests) — Establish testing patterns

---

### Pitfall 5: Missing Error State Tests

**What goes wrong:**
Tests only verify happy paths. Error handling code is untested. Frontend shows wrong error messages, or swallows errors silently. Backend error responses don't match what frontend expects.

**Why it happens:**
Developers focus on "it works" scenarios. Error states are harder to trigger in tests. MSW makes it easy but developers don't add error handlers.

**How to avoid:**
1. Every API call needs success AND error test cases:
   ```typescript
   // Success case
   test('creates material successfully', async () => {
     server.use(
       http.post('/api/v1/inventory/materials', () => 
         HttpResponse.json({ id: '123', ... }, { status: 201 })
       )
     );
     // ... test success
   });
   
   // Error case
   test('handles validation error', async () => {
     server.use(
       http.post('/api/v1/inventory/materials', () => 
         HttpResponse.json({ message: 'Invalid category' }, { status: 400 })
       )
     );
     // ... test error handling
   });
   ```
2. Test loading states
3. Test network failure scenarios
4. Verify error messages are user-friendly, not technical

**Warning signs:**
- Test file only has happy path tests
- `catch` blocks are untested
- Frontend shows generic "Something went wrong"
- Backend returns 500 when it should return 400

**Phase to address:** TEST-02 (Frontend Unit Tests) — Mandate error state tests

---

### Pitfall 6: E2E Tests Don't Assert Critical Data

**What goes wrong:**
E2E tests verify navigation and UI presence but not actual functionality. "Submit button exists" but not "submit actually creates the record". Tests pass but feature is broken.

**Why it happens:**
UI tests are easier to write than data verification. Requires API calls or database queries to verify. Tests focus on visible elements.

**How to avoid:**
1. Every E2E test must verify data persistence:
   ```typescript
   test('should create material', async ({ page, request }) => {
     await page.goto('/inventory');
     await page.click('button:has-text("Material hinzufügen")');
     await page.fill('input[name="name"]', 'Test Material');
     await page.click('button[type="submit"]');
     
     // DON'T STOP HERE - verify data was created
     const response = await request.get('/api/v1/inventory/materials');
     const materials = await response.json();
     expect(materials.find(m => m.name === 'Test Material')).toBeDefined();
   });
   ```
2. Use Playwright's `request` context for API verification
3. Assert specific field values, not just existence
4. Clean up test data after test

**Warning signs:**
- E2E tests only check `toBeVisible()`
- No API calls in E2E tests
- Tests pass but feature doesn't work
- Tests don't verify database state

**Phase to address:** TEST-06 (E2E Coverage Extended) — Require data assertions

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Manual type sync between frontend/backend | No build step changes, fast initial development | Silent type mismatches, runtime errors, debugging nightmare | Never — use ts-rs from day one |
| Skip cross-tenant tests | Faster test writing, simpler test setup | Security vulnerabilities, data leakage incidents | Never — multi-tenant is core architecture |
| Mock database in all tests | Faster tests, no DB setup | SQLx queries untested, runtime query failures | Unit tests only — integration tests must use real DB |
| Copy-paste test setup | Quick to add new test | Divergent patterns, hard to change, test tech debt | Prototyping only — extract shared helpers |

---

## Integration Gotchas

Common mistakes when connecting frontend and backend.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| API routes | Backend expects `category_id`, frontend sends `categoryId` (camelCase) | Use same field names — derive `serde(rename = "...")` only when needed, document the rename |
| UUIDs | Backend returns UUID as string, frontend expects different format | Backend serializes UUIDs as strings (`"uuid"`) — frontend must parse as strings, not validate format |
| Pagination | Backend returns `{ items, total, page, pageSize }`, frontend expects `{ data, count, offset, limit }` | Agree on pagination format upfront, use shared types |
| Dates | Backend returns RFC3339, frontend expects ISO8601 | RFC3339 is ISO8601 subset — frontend can parse both, but document the format |
| Errors | Backend returns `{ message, code }`, frontend expects `{ error, details }` | Standardize error response format, use ts-rs to generate error types |
| Null vs undefined | Backend sends `null`, TypeScript uses `undefined` for optional | TypeScript `| null` union for nullable fields, explicit null checks |

---

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| N+1 queries in tests | Tests slow, 100+ DB queries per test | Assert query count in tests, use `sqlx::query_as!` with JOINs | 100+ test records |
| No pagination in list endpoints | Test returns all 5 records fine, production returns 50,000 | Add pagination from start, test with 100+ records | 1000+ records |
| Frontend fetches all data | Test uses 10 items, production renders 5000 items in dropdown | Test with large datasets, add virtualization | 100+ items in any list |
| Sync tests in parallel | Tests pass in sequence, fail when parallelized | Ensure test isolation (separate tenants, cleanup) | Any parallel test execution |

---

## Security Mistakes

Domain-specific security issues beyond general web security.

| Mistake | Risk | Prevention |
|---------|------|------------|
| Missing tenant_id in query | Data leakage between tenants | Every query MUST include `WHERE tenant_id = $1` — test all queries with cross-tenant scenarios |
| Admin endpoint without tenant check | Admin sees all tenants' data | Admin endpoints also need tenant context — test with multiple tenants |
| QR code includes tenant ID | Cross-tenant QR code usage | QR codes must include tenant prefix or validate tenant on lookup |
| JWT tenant_id trusted blindly | Tenant spoofing via crafted JWT | Validate JWT signature, verify organization claim matches database tenant |

---

## Testing Anti-Patterns

Common testing mistakes specific to this stack.

| Anti-Pattern | Why Bad | Instead |
|--------------|---------|---------|
| Testing implementation details | Breaks on refactor, doesn't verify behavior | Test behavior through public API |
| Mocking what you don't own | Mock doesn't match real behavior | Use real dependencies (DB) or official test utilities (`#[sqlx::test]`) |
| One giant test function | Hard to debug, unclear what failed | One test per scenario, descriptive test names |
| Skipping assertions to make test pass | False confidence | Every test must have meaningful assertions |
| Testing private functions | Brittle, tests coupling not behavior | Test through public interface |
| Only testing success path | Errors go unhandled in production | Test success, error, and edge cases |
| Using production DB for tests | Data corruption, unreliable tests | Use `#[sqlx::test]` which creates isolated test databases |

---

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Backend Tests:** Often missing cross-tenant isolation tests — verify with two tenants
- [ ] **Frontend Tests:** Often missing error state tests — verify error UI shows
- [ ] **E2E Tests:** Often missing data persistence verification — verify API/database state
- [ ] **Type Definitions:** Often missing generated types from ts-rs — verify types auto-generated
- [ ] **Query Tests:** Often missing tenant_id parameter — verify all queries include it
- [ ] **Auth Tests:** Often missing multi-tenant auth tests — verify user can't access other tenant
- [ ] **Offline Tests:** Often missing sync conflict tests — verify conflict resolution

---

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Type drift detected | MEDIUM | 1. Add ts-rs to affected types 2. Run `cargo test` to generate 3. Update frontend imports 4. Remove manual types |
| Cross-tenant data leak | HIGH | 1. Audit ALL queries for tenant_id 2. Add integration tests for each query 3. Consider data migration if leak occurred 4. Notify affected tenants |
| Flaky tests from cache | LOW | 1. Add QueryClient cleanup to each test 2. Use MSW instead of real API 3. Run tests in isolation to identify culprit |
| Missing error handling | MEDIUM | 1. Add error test cases to existing tests 2. Verify error UI components exist 3. Add error boundaries 4. Test error scenarios in E2E |
| Untested queries failing | MEDIUM | 1. Add `#[sqlx::test]` for each route 2. Set up CI with DATABASE_URL 3. Run integration tests in CI pipeline |

---

## Pitfall-to-Phase Mapping

How milestone phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Frontend-Backend Type Drift | TEST-01 (Backend) | `cargo test` generates types, CI checks for drift |
| Incomplete Multi-Tenant Tests | TEST-03/04/05 (Integration) | Every module has cross-tenant test |
| SQLx Runtime Queries Untested | TEST-03/04/05 (Integration) | Every route has integration test |
| React Query Cache Interference | TEST-02 (Frontend) | All tests use fresh QueryClient |
| Missing Error State Tests | TEST-02 (Frontend) | Every hook has error test |
| E2E Tests Don't Assert Data | TEST-06 (E2E Extended) | Every E2E test verifies API/DB state |

---

## Sources

- Context7 documentation for ts-rs: https://github.com/aleph-alpha/ts-rs
- Context7 documentation for SQLx testing: https://github.com/launchbadge/sqlx
- Context7 documentation for Vitest with MSW: https://github.com/vitest-dev/vitest
- Context7 documentation for Playwright API testing: https://github.com/microsoft/playwright
- Existing test patterns: `tests/tenant_isolation_test.rs`
- Existing E2E tests: `frontend/tests/*.spec.ts`
- API types: `frontend/src/types/inventory.ts`, `src/modules/inventory/api/routes.rs`

---

*Pitfalls research for: Testing & Quality Foundation (v1.5)*
*Researched: 2026-04-30*
