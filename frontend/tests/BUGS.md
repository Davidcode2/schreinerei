# Bug Report - Phase 9 Testing

**Date:** 2026-04-29
**Tester:** Playwright E2E Tests
**Environment:** localhost:5174 → localhost:3000 API

## Summary

| Bug ID | Severity | Test | Status |
|--------|----------|------|--------|
| BUG-01 | High | All auth tests | Open |
| BUG-02 | Medium | All tests | Open |

---

## BUG-01: Keycloak Redirect URI Invalid

**Test:** `tests/auth.spec.ts - should login successfully`

**Severity:** High (Blocks authentication)

**Steps to Reproduce:**
1. Navigate to http://localhost:5174
2. App redirects to Keycloak for authentication
3. Keycloak shows error page: "We are sorry... Invalid parameter: redirect_uri"

**Expected:** Keycloak should accept the redirect_uri and show login form.

**Actual:** Keycloak rejects the redirect_uri parameter.

**Error Message:**
```
heading "We are sorry..."
paragraph "Invalid parameter: redirect_uri"
```

**Root Cause:**
The Keycloak client `schreinerei_pwa` does not have `http://localhost:5174/auth/callback` registered as a valid redirect URI. The app is running on port 5174 (due to port 5173 being occupied), but Keycloak only accepts the configured redirect URIs.

**Fix Required:**
1. Register `http://localhost:5174/auth/callback` in Keycloak client settings
2. OR ensure tests run on port 5173 (free the occupied port)
3. OR use wildcard redirect URIs in Keycloak (less secure)

**Keycloak Admin Action:**
Go to Keycloak Admin Console → Clients → schreinerei_pwa → Valid Redirect URIs → Add `http://localhost:*/auth/callback`

---

## BUG-02: Port 5173 Occupied by Unknown Process

**Test:** All tests

**Severity:** Medium (Workaround available)

**Steps to Reproduce:**
1. Run `npm run dev` in frontend directory
2. Vite reports "Port 5173 is in use, trying another one..."
3. Server starts on port 5174 instead

**Expected:** Port 5173 should be available for Vite dev server.

**Actual:** An unknown process is occupying port 5173 and returning HTTP 426 "Upgrade Required".

**Root Cause:**
A lingering process is bound to port 5173. The process doesn't appear in `ps` output but shows in `ss` listening state.

**Workaround:**
Use port 5174 for testing (requires Keycloak redirect URI update - see BUG-01).

**Fix Required:**
Investigate and kill the process occupying port 5173. May require system reboot if process is a kernel-level listener.

---

## Test Infrastructure Status

**Successfully Set Up:**
- Playwright 1.59.1 installed and configured
- 5 test files created (auth, inventory, fleet, sites, dashboard)
- 18 E2E tests covering all major user flows
- Test report generated at `playwright-report/index.html`
- Screenshots and videos captured for all failures
- Tests ARE running correctly - discovering real issues!

**Test Coverage:**
- Authentication: 2 tests (login, logout)
- Inventory: 4 tests (navigation, category, material, list)
- Fleet: 5 tests (navigation, vehicle, tool, list, reservation)
- Sites: 4 tests (navigation, site creation, list, time booking)
- Dashboard: 3 tests (load, sites overview, console errors)
