# Bug Report - Phase 9 Testing

**Date:** 2026-04-29
**Tester:** Playwright E2E Tests
**Environment:** localhost:5173 → localhost:3000 API

## Summary

| Bug ID | Severity | Test | Status |
|--------|----------|------|--------|
| BUG-01 | High | All 18 tests | Open |

---

## BUG-01: Vite Dev Server Returns 426 Upgrade Required

**Test:** All tests - login helper in `tests/helpers/auth.ts:36`

**Severity:** High (Blocks all E2E testing)

**Steps to Reproduce:**
1. Start Playwright test against localhost:5173
2. Test navigates to http://localhost:5173
3. Page shows "Upgrade Required" instead of the React app
4. Test times out waiting for `main` element

**Expected:** Vite dev server should serve the React app with HTML that includes `<main>` element.

**Actual:** Vite returns HTTP 426 "Upgrade Required" - this indicates a WebSocket/HTTP protocol mismatch. Multiple Vite processes may be conflicting on the same port.

**Error Message:**
```
TimeoutError: page.waitForSelector: Timeout 15000ms exceeded.
Call log:
  - waiting for locator('main') to be visible

Page snapshot:
- generic [ref=e2]: Upgrade Required
```

**Root Cause:**
The Vite dev server is returning a 426 status code. This typically happens when:
1. Multiple Vite instances are running on the same port
2. HTTP/2 or WebSocket upgrade is required but failing
3. HMR (Hot Module Replacement) WebSocket connection issues

**Recommendation:**
1. Kill all existing Vite processes: `pkill -f vite`
2. Ensure only one Vite instance runs on port 5173
3. Consider adding a health check before running tests
4. For CI, use `webServer` config in playwright.config.ts to manage the dev server

**Workaround:**
Tests should be run in a clean environment where the Vite dev server is freshly started:
```bash
# Kill all Vite processes
pkill -f vite

# Start fresh dev server
cd frontend && npm run dev &

# Wait for server to be ready
sleep 5

# Run tests
npx playwright test
```

---

## Test Infrastructure Status

**Successfully Set Up:**
- Playwright 1.59.1 installed and configured
- 5 test files created (auth, inventory, fleet, sites, dashboard)
- 18 E2E tests covering all major user flows
- Test report generated at `playwright-report/index.html`
- Screenshots and videos captured for all failures

**Test Coverage:**
- Authentication: 2 tests (login, logout)
- Inventory: 4 tests (navigation, category, material, list)
- Fleet: 5 tests (navigation, vehicle, tool, list, reservation)
- Sites: 4 tests (navigation, site creation, list, time booking)
- Dashboard: 3 tests (load, sites overview, console errors)
