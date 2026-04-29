# Bug Report - Frontend Testing Session

**Date:** 2026-04-29
**Tester:** Claude (automated testing via playwright-cli)
**Environment:** localhost:5175 (frontend), localhost:3000 (backend)

## Summary

Systematic testing of the Schreinerei SaaS frontend revealed **8 bugs**, ranging from critical authentication issues to non-functional UI buttons.

---

## Critical Bugs

### BUG-001: Token Exchange Failure During Auth Callback

**Severity:** Critical
**Location:** `frontend/src/lib/auth/keycloak.ts:62-63`
**Component:** Authentication

**Description:**
During the OAuth2 PKCE authentication flow, one of the token exchange requests fails with HTTP 400. The console shows:
```
[ERROR] Failed to load resource: the server responded with a status of 400 () @ https://auth.jakob-lingel.dev/realms/schreinerei/protocol/openid-connect/token
[ERROR] Auth callback failed: Error: Token exchange failed
```

**Network Evidence:**
```
[POST] https://auth.jakob-lingel.dev/realms/schreinerei/protocol/openid-connect/token => [200]
[POST] https://auth.jakob-lingel.dev/realms/schreinerei/protocol/openid-connect/token => [400]
```

Two token requests are made - one succeeds (200), one fails (400). This causes the auth callback to fail, though the app continues to function using cached tokens.

**Impact:**
- Intermittent login failures
- Session instability
- Token refresh cascade failures

**Likely Cause:**
The PKCE verifier or state might not be stored correctly in sessionStorage, or the code is being exchanged twice.

---

### BUG-002: Token Refresh Cascade Failure

**Severity:** Critical
**Location:** `frontend/src/lib/api/client.ts:19-28`
**Component:** Authentication / API Client

**Description:**
When the access token expires and refresh fails, it triggers a cascade of 401 Unauthorized errors on all subsequent API calls, eventually forcing logout.

**Console Evidence:**
```
[ERROR] Failed to load resource: the server responded with a status of 400 () @ https://auth.jakob-lingel.dev/realms/schreinerei/protocol/openid-connect/token
[ERROR] Failed to load resource: the server responded with a status of 401 (Unauthorized) @ http://localhost:3000/api/v1/users
[ERROR] Failed to load resource: the server responded with a status of 401 (Unauthorized) @ http://localhost:3000/api/v1/fleet/vehicles
[ERROR] Failed to load resource: the server responded with a status of 401 (Unauthorized) @ http://localhost:3000/api/v1/inventory/materials
...
```

**Impact:**
- Users are forcibly logged out
- Data loss if unsaved
- Poor user experience

---

### BUG-003: Wrong API URL (Frontend Port Instead of Backend)

**Severity:** High
**Location:** Unknown - possibly service worker or incorrect fetch
**Component:** API Client / Service Worker

**Description:**
Some API calls are being made to `http://localhost:5175/api/v1/...` instead of `http://localhost:3000/api/v1/...`, resulting in 401 Unauthorized errors.

**Network Evidence:**
```
[GET] http://localhost:5175/api/v1/dashboard/sites => [401] Unauthorized
[GET] http://localhost:3000/api/v1/dashboard/sites => [200] OK
```

**Impact:**
- Inconsistent API behavior
- 401 errors that should not occur
- Possible race condition between proxy and direct calls

**Likely Cause:**
Service worker or PWA caching may be intercepting some requests before the Vite proxy can handle them.

---

## High Severity Bugs

### BUG-004: Fleet "Neu" Button Non-Functional

**Severity:** High
**Location:** `frontend/src/pages/fleet/FleetPage.tsx:49-52`
**Component:** Fleet Management

**Description:**
The "Neu" button in the Fleet page has no onClick handler and does nothing when clicked.

**Code:**
```tsx
<Button className="gap-2">
  <Plus className="h-4 w-4" />
  <span className="hidden sm:inline">Neu</span>
</Button>
```

**Requirement Impact:**
- FLEET-08: User can add new vehicle via dialog form - **NOT IMPLEMENTED**
- FLEET-09: User can add new tool via dialog form - **NOT IMPLEMENTED**

**Expected Behavior:**
Should open a dialog to add a new vehicle or tool.

---

### BUG-005: Redundant API Calls

**Severity:** Medium
**Location:** Multiple components
**Component:** React Query / Data Fetching

**Description:**
The same API endpoints are called multiple times unnecessarily, indicating poor query deduplication or unnecessary re-renders.

**Network Evidence:**
```
[GET] http://localhost:3000/api/v1/inventory/materials => [200] OK
[GET] http://localhost:3000/api/v1/inventory/materials => [200] OK
[GET] http://localhost:3000/api/v1/sites => [200] OK
[GET] http://localhost:3000/api/v1/sites => [200] OK
[GET] http://localhost:3000/api/v1/fleet/vehicles => [200] OK
[GET] http://localhost:3000/api/v1/fleet/vehicles => [200] OK
```

**Impact:**
- Unnecessary server load
- Slower page loads
- Bandwidth waste

**Likely Cause:**
- Multiple components calling the same hooks
- Query invalidation triggering refetches
- Offline sync running in parallel with React Query

---

## Medium Severity Bugs

### BUG-006: User List Not Displaying (Loading State Stuck)

**Severity:** Medium
**Location:** `frontend/src/pages/settings/UserManagementSection.tsx`
**Component:** User Management

**Description:**
The users API call succeeds (200 OK), but the user list shows a loading spinner instead of the users.

**Network Evidence:**
```
[GET] http://localhost:3000/api/v1/users => [200] OK
```

**UI Observation:**
The snapshot shows `img [ref=e103]` (loading spinner) instead of the user list.

**Impact:**
- USER-02: Settings page displays real users from API - **PARTIALLY WORKING** (API works, UI broken)

**Likely Cause:**
The component may be stuck in loading state due to auth state changes triggering re-renders, or the query is being invalidated before completion.

---

### BUG-007: No Email Invite Dialog

**Severity:** Medium
**Location:** `frontend/src/pages/settings/UserManagementSection.tsx:75-78`
**Component:** User Management

**Description:**
The "Einladen" button only copies an organization invite link to clipboard. There is no dialog for inviting users by email.

**Code:**
```tsx
<Button size="sm" className="gap-2" onClick={copyInviteUrl}>
  <UserPlus className="h-4 w-4" />
  Einladen
</Button>
```

**Requirement Impact:**
- USER-01: Admin can invite user via email dialog - **NOT IMPLEMENTED**

**Expected Behavior:**
Should open a dialog with an email input field for sending invitations.

---

### BUG-008: Offline Sync Fails Silently

**Severity:** Medium
**Location:** `frontend/src/lib/offline/sync.ts:20-44`
**Component:** Offline Support

**Description:**
When sync fails, the error is logged but the UI shows no feedback to the user. The sync button is disabled but users don't know why.

**Console Evidence:**
```
[ERROR] Sync from server failed: Error: API request failed
    at ApiClient.request (http://localhost:5175/src/lib/api/client.ts:41:13)
    at async syncFromServer (http://localhost:5175/src/lib/offline/sync.ts:15:49)
```

**Impact:**
- Users don't know their data isn't syncing
- No retry mechanism visible to user
- Data inconsistency between devices

---

## Working Features

The following features were tested and work correctly:

### Inventory Management
- **INVT-08**: Add material dialog with form submission - **WORKING**
  - Category selection works
  - Form validation works
  - POST /api/v1/inventory/materials returns 201 Created
  - New material appears in list immediately

### Sites Management
- **SITE-09**: Add site dialog with form submission - **WORKING**
  - Form validation works
  - POST /api/v1/sites returns 201 Created
  - New site appears in list immediately

### Authentication
- Keycloak OAuth2 login flow works (despite BUG-001)
- User profile displays correctly
- Logout works

### Navigation
- Sidebar navigation works correctly
- All pages load without errors
- Mobile navigation works

---

## Recommendations

### Immediate Actions (Critical)

1. **Fix Token Exchange (BUG-001)**
   - Investigate why two token requests are made
   - Ensure PKCE verifier is stored before redirect
   - Add better error handling for token exchange failures

2. **Fix Token Refresh (BUG-002)**
   - Implement exponential backoff for token refresh
   - Don't logout immediately on first refresh failure
   - Show toast notification when session is expiring

3. **Fix Wrong API URL (BUG-003)**
   - Audit all fetch calls
   - Ensure service worker uses correct URLs
   - Add request interceptor to fix URLs

### Short-term Actions (High)

4. **Implement Fleet Add Dialog (BUG-004)**
   - Create AddVehicleDialog component
   - Create AddToolDialog component
   - Wire up "Neu" button onClick handler

5. **Reduce Redundant API Calls (BUG-005)**
   - Review React Query staleTime settings
   - Ensure queries are properly shared
   - Consider using React Query's structural sharing

### Medium-term Actions

6. **Fix User List Display (BUG-006)**
   - Debug why loading state is stuck
   - Add error boundary around UserManagementSection

7. **Implement Email Invite Dialog (BUG-007)**
   - Create InviteUserDialog component
   - Add email validation
   - Integrate with backend invitation endpoint

8. **Improve Offline Sync Feedback (BUG-008)**
   - Show sync status to user
   - Add manual retry button
   - Display last successful sync time

---

## Test Session Details

**Login Credentials Used:**
- Username: schreiner@admin.test
- Password: [provided]

**Pages Tested:**
- Dashboard (/)
- Inventory (/inventory)
- Sites (/sites)
- Fleet (/fleet)
- Settings (/settings)

**Test Duration:** ~10 minutes

**Browser:** Chromium (headless via playwright-cli)
