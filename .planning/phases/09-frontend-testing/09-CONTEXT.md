# Phase 9 Context: Frontend Testing

## Goal

Comprehensive frontend testing with Playwright to discover remaining bugs and validate all user flows work correctly.

## Phase Boundary

This phase focuses on:
- Setting up Playwright E2E testing infrastructure
- Writing comprehensive tests for all user flows
- Running tests to discover remaining bugs
- Recording navigation patterns for future testing

NOT in scope:
- Unit tests (separate concern)
- API integration tests (backend concern)
- Performance testing (future phase)

---

## Testing Scope

From ROADMAP.md:

1. **Inventory Flow**
   - Create inventory category
   - Add materials with category
   - Withdraw items from inventory

2. **Fleet Flow**
   - Create vehicles
   - Create tools
   - Book/reserve vehicles via calendar

3. **Sites Flow**
   - Create sites
   - Book time on sites

4. **Dashboard**
   - View sites overview
   - View statistics

5. **Settings**
   - View users list
   - Invite users (admin only)

---

## Technical Context

**Frontend Stack:**
- Vite 6 + React 19 + TypeScript
- React Router 7
- TanStack Query 5
- Tailwind CSS 4 + shadcn/ui
- Zustand for state

**Auth:**
- Keycloak OAuth2 PKCE
- Test credentials available in Phase 8 context

**Test URLs:**
- Dev: http://localhost:5173
- API: http://localhost:3000
- Keycloak: http://localhost:8080 (or https://auth.jakob-lingel.dev)

**Test Credentials:**
- Email: schreiner@admin.test
- Password: T6&Mo2wnhypFEZ$P$8QqdWELZ3BP5Hhe

---

## Decisions

### D-01: Playwright for E2E Testing
Use Playwright for end-to-end testing because:
- Cross-browser support (Chromium, Firefox, WebKit)
- Built-in assertions and locators
- Screenshot/video on failure
- Good TypeScript support
- Works with Vite dev server

### D-02: Test Structure
Organize tests by user flow:
- `tests/auth.spec.ts` - Login/logout
- `tests/inventory.spec.ts` - Material CRUD
- `tests/fleet.spec.ts` - Vehicle/tool management
- `tests/sites.spec.ts` - Site management
- `tests/dashboard.spec.ts` - Dashboard overview

### D-03: Test Runner
Use Playwright's built-in test runner with:
- `playwright test` for CI
- `playwright test --ui` for debugging
- Screenshots on failure
- Trace files for debugging

### the agent's Discretion
- Specific test assertions and edge cases
- Test data cleanup strategy
- Parallel vs sequential test execution

---

## Specifics

### Test Prerequisites

1. Backend running on localhost:3000
2. Keycloak running with test user
3. Database with clean test data (or ability to create)

### Known Fixed Issues (Phase 8)

- Dashboard sites API 500 error ✓
- Logout button ✓
- Vehicle creation 400 error ✓
- Category creation ✓
- Reservation date format ✓

### Known Pending Issues (from STATE.md)

- Fix fleet page "Neu" button
- Fix baustelle time booking 400 error

---

## Deferred Ideas

- CI/CD integration for tests (run in GitHub Actions)
- Visual regression testing
- Mobile viewport testing
- Performance metrics collection

---

*Phase: 09-frontend-testing*
*Context created: 2026-04-29*
