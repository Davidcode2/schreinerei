# Roadmap: Schreinerei SaaS

**Project:** Schreinerei SaaS
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.
**Timeline:** 1 day

---

## Milestones

- ✅ **v1.0 MVP** — Phases 1-5 (shipped 2026-04-29)
- ✅ **v1.1 Organization-Based Tenancy** — Phase 6 (shipped 2026-04-29)
- ✅ **v1.2 Frontend Polish** — Phase 7 (shipped 2026-04-29)
- ✅ **v1.3 Bug Fixes** — Phases 8-10 (shipped 2026-04-29)

---

## Phases

<details>
<summary>✅ v1.0 MVP (Phases 1-5) — SHIPPED 2026-04-29</summary>

- [x] Phase 1: Auth & IAM Foundation (2/2 plans) — completed 2026-04-28
- [x] Phase 2: Inventar Management (2/2 plans) — completed 2026-04-28
- [x] Phase 3: Baustellen Management (2/2 plans) — completed 2026-04-28
- [x] Phase 4: Fuhrpark & Werkzeuge (2/2 plans) — completed 2026-04-28
- [x] Phase 5: PWA & Mobile (4/4 plans) — completed 2026-04-29

</details>

<details>
<summary>✅ v1.1 Organization-Based Tenancy (Phase 6) — SHIPPED 2026-04-29</summary>

- [x] Phase 6: Organization-Based Tenancy (3/3 plans) — completed 2026-04-29

</details>

---

## Phase 7: Frontend Polish ✅

**Goal:** Fix non-functional buttons and connect UI to backend APIs

**Requirements:** INVT-08, INVT-09, SITE-09, FLEET-08, FLEET-09, USER-01, USER-02, ERR-01, ERR-02

**Plans:** 3 plans (3/3 complete)

**Duration:** 1 day

**Status:** Complete (2026-04-29)

### Plans

**Wave 1**
- [x] 07-01 — Add Material & Site Dialogs (INVT-08, INVT-09, SITE-09)

**Wave 2**
- [x] 07-02 — Add Fleet Dialogs (FLEET-08, FLEET-09)

**Wave 3**
- [x] 07-03 — User Management & Error Handling (USER-01, USER-02, ERR-01, ERR-02)

---

## Phase 8: Bug Fixes ✅

**Goal:** Fix all frontend-accessible functionalities - ensure end-to-end flows work correctly

**Issues discovered during Phase 7 testing:**
- Dashboard sites API 500 error (NUMERIC vs FLOAT8 type mismatch)
- Logout button non-functional
- Vehicle creation 400 error ("Invalid vehicle type: van")
- Cannot create materials - no category selection/creation available
- Calendar reservation 400 error (date format)

**Plans:** 2 plans (2/2 complete)

**Duration:** 1 day

**Status:** Complete (2026-04-29)

### Plans

**Wave 1**
- [x] 08-01 — Backend Bug Fixes (Dashboard type mismatch, VehicleType enum)

**Wave 2**
- [x] 08-02 — Frontend Bug Fixes (Logout, Category creation, Date format)

### Success Criteria

1. User can add material via dialog with name, quantity, unit, location
2. User can create site via dialog with name, customer, location
3. User can add vehicle via dialog with name, plate, status
4. User can add tool via dialog with name, status
5. Admin can invite user via email dialog
6. Settings displays real users from API (not mock data)
7. QR scanner shows friendly error when camera denied
8. QR scanner provides retry button after error

---

## Phase 9: Frontend Testing ✅

**Goal:** Comprehensive frontend testing with Playwright to discover remaining bugs

**Requirements:** TEST-01, TEST-02, TEST-03, TEST-04, TEST-05, TEST-06

**Plans:** 3 plans (3/3 complete)

**Duration:** 1 day

**Status:** Complete (2026-04-29)

### Plans

**Wave 1**
- [x] 09-01 — Playwright Setup & Auth/Inventory Tests (TEST-01, TEST-02)

**Wave 2**
- [x] 09-02 — Fleet, Sites & Dashboard Tests (TEST-03, TEST-04, TEST-05)

**Wave 3**
- [x] 09-03 — Run Tests & Document Bugs (TEST-06)

### Success Criteria

1. ✅ Playwright installed and configured
2. ✅ All user flows have E2E tests (18 tests)
3. ⚠️ Tests can authenticate with Keycloak (blocked by config)
4. ✅ Bug report generated with findings (2 bugs found)

### Bugs Discovered

| Bug | Description | Severity |
|-----|-------------|----------|
| BUG-01 | Keycloak redirect_uri invalid for port 5174 | High |
| BUG-02 | Port 5173 occupied by unknown process | Medium |

---

## Phase 10: Bug Fixes Round 2 ✅

**Goal:** Fix all 8 bugs discovered during Phase 9 automated Playwright testing

**Requirements:** BUG-001, BUG-002, BUG-003, BUG-004, BUG-005, BUG-006, BUG-007, BUG-008

**Plans:** 4 plans (4/4 complete)

**Duration:** 1 day

**Status:** Complete (2026-04-29)

### Plans

**Wave 1 (Parallel)**
- [x] 10-01 — Authentication Fixes (BUG-001, BUG-002)
- [x] 10-02 — Fleet Button Fix (BUG-004)
- [x] 10-03 — PWA & Query Fixes (BUG-003, BUG-005)

**Wave 2**
- [x] 10-04 — User Management & Sync (BUG-006, BUG-007, BUG-008)

### Success Criteria

1. Token exchange succeeds on first attempt (no double-exchange failures)
2. Token refresh retries before logout (exponential backoff)
3. API calls go through Vite proxy correctly
4. Fleet "Neu" button opens dropdown with Vehicle/Tool options
5. No redundant API calls within 30 second window
6. User list displays users from API
7. Admin can invite users via email dialog
8. Sync status visible to user with toast notifications

### Bugs to Fix

| Bug | Description | Severity | Location |
|-----|-------------|----------|----------|
| BUG-001 | Token Exchange Failure During Auth Callback | Critical | keycloak.ts, AuthCallback.tsx |
| BUG-002 | Token Refresh Cascade Failure | Critical | client.ts |
| BUG-003 | Wrong API URL (Frontend Port Instead of Backend) | High | vite.config.ts |
| BUG-004 | Fleet "Neu" Button Non-Functional | High | FleetPage.tsx |
| BUG-005 | Redundant API Calls | Medium | hooks/*.ts |
| BUG-006 | User List Not Displaying (Loading State Stuck) | Medium | UserManagementSection.tsx |
| BUG-007 | No Email Invite Dialog | Medium | UserManagementSection.tsx |
| BUG-008 | Offline Sync Fails Silently | Medium | sync.ts |

---

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Auth & IAM | v1.0 | 2/2 | Complete | 2026-04-28 |
| 2. Inventar | v1.0 | 2/2 | Complete | 2026-04-28 |
| 3. Baustellen | v1.0 | 2/2 | Complete | 2026-04-28 |
| 4. Fuhrpark | v1.0 | 2/2 | Complete | 2026-04-28 |
| 5. PWA & Mobile | v1.0 | 4/4 | Complete | 2026-04-29 |
| 6. Org Tenancy | v1.1 | 3/3 | Complete | 2026-04-29 |
| 7. Frontend Polish | v1.2 | 3/3 | Complete | 2026-04-29 |
| 8. Bug Fixes | v1.3 | 2/2 | Complete | 2026-04-29 |
| 9. Frontend Testing | v1.3 | 3/3 | Complete | 2026-04-29 |
| 10. Bug Fixes Round 2 | v1.3 | 4/4 | Complete | 2026-04-29 |

---

## Summary

| Phase | Goal | Duration | Requirements |
|-------|------|----------|--------------|
| 7 | Frontend Polish | 1 day | 9 |
| 8 | Bug Fixes | 1 day | 5 bugs |
| 9 | Frontend Testing | 1 day | 6 test requirements |
| 10 | Bug Fixes Round 2 | 1 day | 8 bugs |

**Total for v1.2+v1.3:** 4 days

---

*Last updated: 2026-04-29 after Phase 10 planning*
