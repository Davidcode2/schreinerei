# Phase 17: Feature Audit - Research

**Gathered:** 2026-04-30
**Status:** Research Complete
**Phase Requirements:** AUDIT-01, AUDIT-02, AUDIT-03, AUDIT-04, AUDIT-05, AUDIT-06

## Executive Summary

This phase audits 5 core features (Baustellen, Inventory, Time Booking, Vehicles/Machines, Reservations) to document bugs, functional issues, and missing functionality. Research identified existing documentation, test coverage, and a systematic audit methodology.

---

## 1. Feature Inventory

### Feature 1: Baustellen (Construction Sites) — AUDIT-01

**Backend:**
- Location: `src/modules/sites/`
- API Routes: `/api/v1/sites`, `/api/v1/sites/{id}`, assignments, time-entries, activities
- Domain: Site aggregate with status state machine (Planned → Active → Completed → Archived)
- Time Entry: `CreateTimeEntry` command with validation (hours 0.01-24)

**Frontend:**
- Pages: `SitesListPage.tsx`, `SiteDetailPage.tsx`
- Components: `AddSiteDialog.tsx`, `TimeEntryDialog.tsx`, `ActivityFeed.tsx`
- Tests: `AddSiteDialog.test.tsx` (Phase 14)

**E2E Coverage:**
- `sites.spec.ts`: 4 navigation tests + 2 data persistence tests
- Tests cover: navigation, elements, add button, search, site creation, status verification

**Known Issues:**
- Time booking returns 400 error (pending todo)
- TimeEntryDialog may have validation or API mismatch

### Feature 2: Inventory (Materials) — AUDIT-02

**Backend:**
- Location: `src/modules/inventory/`
- API Routes: `/api/v1/inventory/materials`, `/api/v1/inventory/categories`
- Domain: Material aggregate with stock tracking, QR codes, low stock detection
- Commands: CreateMaterial, UpdateMaterial, WithdrawMaterial

**Frontend:**
- Pages: `InventoryListPage.tsx`, `InventoryDetailPage.tsx`
- Components: `AddMaterialDialog.tsx`, `WithdrawDialog.tsx`
- Tests: `AddMaterialDialog.test.tsx` (Phase 14)

**E2E Coverage:**
- `inventory.spec.ts`: 4 navigation tests + 2 data persistence tests
- Tests cover: navigation, elements, add button, search, material creation, list verification

**Known Issues:**
- None documented in BUG-REPORT.md for inventory
- INVT-08 (material dialog) marked as WORKING in bug report

### Feature 3: Time Booking — AUDIT-03

**Backend:**
- Location: `src/modules/sites/domain/time_entry.rs`
- API Route: `/api/v1/time-entries` (POST), `/api/v1/time-entries/my` (GET)
- Domain: TimeEntry with hours validation (0.01-24)
- Part of Sites module (not standalone)

**Frontend:**
- Component: `TimeEntryDialog.tsx` in sites pages
- No dedicated time booking page

**E2E Coverage:**
- No dedicated time booking tests
- Time entry creation via API tested in sites.spec.ts

**Known Issues:**
- **BUG-001 to BUG-008** from BUG-REPORT.md do not specifically mention time booking
- Pending todo: "Fix baustelle time booking 400 error"
- Time booking dialog may have validation or payload format issues

### Feature 4: Vehicles/Machines (Fleet) — AUDIT-04

**Backend:**
- Location: `src/modules/fleet/`
- API Routes: `/api/v1/fleet/vehicles`, `/api/v1/fleet/tools`
- Domain: Vehicle and Tool aggregates with status (Available, InUse, Maintenance, Retired)
- QR code support for both vehicles and tools

**Frontend:**
- Page: `FleetPage.tsx`
- Components: `AddVehicleDialog.tsx`, `AddToolDialog.tsx`, `VehiclesList.tsx`, `ToolsList.tsx`
- Tests: `AddVehicleDialog.test.tsx`, `AddToolDialog.test.tsx` (Phase 14)

**E2E Coverage:**
- `fleet.spec.ts`: 5 navigation tests + 2 data persistence tests
- Tests cover: navigation, elements, vehicles section, tools section, add button

**Known Issues:**
- **BUG-004**: Fleet "Neu" button non-functional (pending todo confirms this)
- FLEET-08 and FLEET-09 requirements (add vehicle/tool via dialog) NOT IMPLEMENTED

### Feature 5: Reservations — AUDIT-05

**Backend:**
- Location: `src/modules/fleet/domain/reservation.rs`
- API Routes: `/api/v1/fleet/reservations`, `/api/v1/fleet/reservations/my`, `/api/v1/fleet/calendar`, `/api/v1/fleet/availability`
- Domain: Reservation with status (Pending → Confirmed → InUse → Completed/Cancelled)
- Overlap detection logic tested

**Frontend:**
- Components: `ReservationDialog.tsx`, `ReservationsList.tsx`, `CalendarView.tsx`
- No dedicated reservations page (accessed via Fleet page)

**E2E Coverage:**
- No dedicated reservation tests
- Reservation creation via API not tested in fleet.spec.ts

**Known Issues:**
- Reservation dialog may not be wired to Fleet page UI
- No E2E coverage for reservation workflow

---

## 2. Existing Documentation

### BUG-REPORT.md (8 bugs documented)

| Bug ID | Severity | Component | Status |
|--------|----------|-----------|--------|
| BUG-001 | Critical | Auth/Token Exchange | Needs fix |
| BUG-002 | Critical | Auth/Token Refresh | Needs fix |
| BUG-003 | High | API/Service Worker | Needs fix |
| BUG-004 | High | Fleet UI | Confirmed by pending todo |
| BUG-005 | Medium | React Query | Needs investigation |
| BUG-006 | Medium | User Management | Needs fix |
| BUG-007 | Medium | User Management | Not implemented |
| BUG-008 | Medium | Offline Sync | Needs UX improvement |

### Pending Todos

1. `2026-04-29-fleet-neu-button-non-functional.md` — FLEET-08/FLEET-09 not implemented
2. `2026-04-29-baustelle-time-booking-400-error.md` — Time booking API fails

---

## 3. Audit Methodology

### Systematic Feature Audit Pattern

For each feature, audit should cover:

1. **Backend API Audit**
   - Run all API endpoints via Playwright
   - Check for error responses (4xx, 5xx)
   - Verify request/response payload formats
   - Test edge cases (empty, null, invalid)

2. **Frontend UI Audit**
   - Navigate to feature page
   - Test all interactive elements (buttons, forms, dialogs)
   - Check for console errors
   - Verify form validation
   - Test CRUD operations

3. **E2E Test Gap Analysis**
   - What tests exist?
   - What scenarios are untested?
   - Are there smoke tests for critical paths?

4. **Domain Logic Review**
   - Are state transitions correct?
   - Are validation rules enforced?
   - Are there race conditions?

### Issue Documentation Format

```markdown
## [FEATURE_NAME] Audit

### Bugs Found
- [BUG-XXX]: Description — Severity: Critical/High/Medium/Low

### Functional Issues
- [ISSUE-XXX]: Description — Impact: [user-facing/internal]

### Missing Functionality
- [MISSING-XXX]: Description — Requirement: [REQ-ID]

### E2E Test Gaps
- Untested scenario: [description]
```

---

## 4. ISSUE-BACKLOG.md Structure

Recommended structure for AUDIT-06:

```markdown
# Issue Backlog

**Created:** 2026-04-30
**Source:** Phase 17 Feature Audit

## Summary

| Category | Count |
|----------|-------|
| Critical Bugs | X |
| High Bugs | X |
| Medium Bugs | X |
| Low Bugs | X |
| Functional Issues | X |
| Missing Functionality | X |

## Critical Priority

### BUG-XXX: [Title]
- **Feature:** [Baustellen/Inventory/etc.]
- **Description:** [What's wrong]
- **Impact:** [User-facing impact]
- **Reproduction:** [Steps to reproduce]
- **Suggested Fix:** [If known]

## High Priority
[Same format]

## Medium Priority
[Same format]

## Low Priority / Nice to Have
[Same format]

## Future Enhancements
[Features not in scope but requested]
```

---

## 5. Recommended Audit Approach

### Wave 1: Automated Discovery (Parallel)
- Run existing E2E tests with extended logging
- Capture all console errors and network failures
- Generate API coverage report

### Wave 2: Manual Feature Audit (Sequential)
Each auditor (plan) takes one feature:
1. **Plan 17-01:** Baustellen + Time Booking (related features)
2. **Plan 17-02:** Inventory
3. **Plan 17-03:** Fleet (Vehicles/Tools) + Reservations (related features)

### Wave 3: Consolidation
- Merge all audit findings into ISSUE-BACKLOG.md
- Deduplicate issues
- Assign priorities

---

## 6. Dependencies

**Required Before Audit:**
- ✓ QA-PLAYBOOK.md (Phase 13 complete)
- ✓ E2E-PATTERNS.md (Phase 13 complete)
- ✓ E2E test infrastructure (Phase 16 complete)
- Backend running on port 3000
- Frontend running on port 5175
- Keycloak accessible

**Constraints:**
- This phase RECORDS issues only — no fixes
- Use QA Playbook procedures for validation
- All issues must be reproducible

---

## 7. Files to Audit

### Backend (by module)
- `src/modules/sites/` — Baustellen, Time Booking
- `src/modules/inventory/` — Inventory
- `src/modules/fleet/` — Vehicles, Tools, Reservations
- `src/modules/iam/` — User Management (related to BUG-006, BUG-007)

### Frontend (by page)
- `frontend/src/pages/sites/` — Baustellen, Time Booking
- `frontend/src/pages/inventory/` — Inventory
- `frontend/src/pages/fleet/` — Vehicles, Tools, Reservations
- `frontend/src/pages/settings/` — User Management

### Tests
- `frontend/tests/sites.spec.ts`
- `frontend/tests/inventory.spec.ts`
- `frontend/tests/fleet.spec.ts`
- `frontend/tests/dashboard.spec.ts`

---

## 8. Expected Output

**Per Feature:**
- Audit report with bugs, issues, missing functionality
- E2E test gap analysis

**Consolidated:**
- `.planning/ISSUE-BACKLOG.md` — Comprehensive issue list with priorities

---

## RESEARCH COMPLETE

**Findings Summary:**
- 5 features to audit across 4 backend modules and 3 frontend page groups
- 8 bugs already documented in BUG-REPORT.md
- 2 pending todos confirm specific issues
- E2E coverage exists but gaps in time booking and reservations
- Recommended 3-plan approach (Wave 1 automated, Wave 2 manual per feature group, Wave 3 consolidation)
