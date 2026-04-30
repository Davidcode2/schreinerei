# Issue Backlog

**Created:** 2026-04-30
**Source:** Phase 17 Feature Audit
**Requirements:** AUDIT-01, AUDIT-02, AUDIT-03, AUDIT-04, AUDIT-05, AUDIT-06

## Summary

| Category | Count |
|----------|-------|
| Critical Bugs | 0 |
| High Bugs | 1 |
| Medium Bugs | 5 |
| Low Bugs | 3 |
| Functional Issues | 4 |
| Missing Functionality | 11 |
| **Total** | **24** |

## By Feature

| Feature | Critical | High | Medium | Low | Total |
|---------|----------|------|--------|-----|-------|
| Baustellen | 0 | 0 | 1 | 0 | 1 |
| Time Booking | 0 | 1 | 1 | 0 | 2 |
| Inventory | 0 | 0 | 1 | 1 | 2 |
| Fleet | 0 | 0 | 1 | 0 | 1 |
| Reservations | 0 | 0 | 1 | 1 | 2 |
| E2E Testing | 0 | 0 | 5 | 0 | 5 |
| **Total** | 0 | 1 | 5 | 3 | 16 |

---

## Critical Priority

None identified.

---

## High Priority

### BUG-TIME-001: Hours Field Allows Invalid Zero Value
- **Feature:** Time Booking
- **Severity:** High
- **Location:** frontend/src/pages/sites/TimeEntryDialog.tsx:114
- **Description:** The hours input uses `parseFloat(e.target.value) || 0` which can result in 0 hours being submitted. Backend validation rejects hours <= 0 with validation error.
- **Impact:** Users see "Zeiterfassung fehlgeschlagen" error with no clear explanation
- **Reproduction:**
  1. Open TimeEntryDialog
  2. Clear hours input field
  3. Click "Speichern" (Save)
  4. Request sends hours: 0
  5. Backend returns 400 with "Hours must be positive"
- **Suggested Fix:** Add frontend validation to disable submit when hours <= 0, or initialize to minimum valid value (1)
- **Source:** 17-01-AUDIT-TIME-BOOKING.md

---

## Medium Priority

### BUG-BAU-001: Missing E2E Test Coverage for Update/Delete Operations
- **Feature:** Baustellen
- **Severity:** Medium
- **Description:** No E2E tests for site update, delete, or assignment operations. Only navigation and create are tested.
- **Impact:** Regression bugs in update/delete flows may go undetected
- **Suggested Fix:** Add E2E tests for update site, delete site, status transitions, user assignment
- **Source:** 17-01-AUDIT-BAUSTELLEN.md

### BUG-TIME-002: Missing Input Validation Feedback
- **Feature:** Time Booking
- **Severity:** Medium
- **Description:** The TimeEntryDialog shows no inline validation messages. Users only see a generic toast error after submission fails.
- **Impact:** Poor UX - users don't know what's wrong
- **Suggested Fix:** Add inline validation with error messages below fields
- **Source:** 17-01-AUDIT-TIME-BOOKING.md

### BUG-INV-001: Missing E2E Test Coverage for Update/Delete Operations
- **Feature:** Inventory
- **Severity:** Medium
- **Description:** No E2E tests for material update or delete operations. Only create and list are tested.
- **Impact:** Regression bugs in update/delete flows may go undetected
- **Suggested Fix:** Add E2E tests for update material, delete material, withdraw workflow
- **Source:** 17-02-AUDIT-INVENTORY.md

### BUG-FLEET-001: Missing E2E Test Coverage for Update/Delete Operations
- **Feature:** Fleet
- **Severity:** Medium
- **Description:** No E2E tests for vehicle/tool update, delete, or status transition operations.
- **Impact:** Regression bugs in update/delete flows may go undetected
- **Suggested Fix:** Add E2E tests for update vehicle, delete vehicle, status transitions
- **Source:** 17-03-AUDIT-FLEET.md

### BUG-RES-001: No E2E Test Coverage for Reservations
- **Feature:** Reservations
- **Severity:** Medium
- **Description:** Reservations is a core feature but has zero E2E test coverage. All operations are untested.
- **Impact:** Regression bugs in reservation flow will go undetected
- **Suggested Fix:** Add E2E tests for create, update, cancel, availability check
- **Source:** 17-03-AUDIT-RESERVATIONS.md

---

## Low Priority

### BUG-INV-002: QR Code Button Non-Functional
- **Feature:** Inventory
- **Severity:** Low
- **Location:** frontend/src/pages/inventory/InventoryListPage.tsx:68-70
- **Description:** QR code button exists but has no onClick handler - it's a static button.
- **Impact:** Users cannot initiate QR scan from this button
- **Suggested Fix:** Add onClick handler to open QR scanner dialog or navigate to QR scan page
- **Source:** 17-02-AUDIT-INVENTORY.md

### BUG-RES-002: Calendar View Does Not Support Creating Reservations
- **Feature:** Reservations
- **Severity:** Low
- **Location:** frontend/src/pages/fleet/CalendarView.tsx
- **Description:** Calendar view shows reservations but clicking on empty time slots does nothing.
- **Impact:** Users must go back to Fleet page to create reservations
- **Suggested Fix:** Add onClick handler to open ReservationDialog for selected time/resource
- **Source:** 17-03-AUDIT-RESERVATIONS.md

### ISSUE-AUTH-001: Token Exchange Failure During Auth Callback (BUG-001)
- **Feature:** Authentication
- **Severity:** Low (existing, intermittent)
- **Description:** During OAuth2 PKCE flow, one token exchange request fails with HTTP 400. Two token requests are made - one succeeds, one fails.
- **Impact:** Intermittent login failures, session instability
- **Source:** BUG-REPORT.md

---

## Functional Issues

### ISSUE-BAU-001: Backend Dependency Required for Full Testing
- **Description:** E2E data persistence tests fail when backend is not running.
- **Impact:** Cannot verify data persistence without running backend infrastructure
- **Source:** 17-01-AUDIT-BAUSTELLEN.md

### ISSUE-INV-001: Withdraw Functionality Status Unknown
- **Description:** The WithdrawDialog component may exist but its integration with InventoryListPage is unclear.
- **Impact:** Core inventory use case may be missing or broken
- **Source:** 17-02-AUDIT-INVENTORY.md

### ISSUE-TIME-001: No E2E Test Coverage for Time Booking
- **Description:** Time booking is a core feature but has no dedicated E2E tests.
- **Impact:** Regression bugs in time booking flow may go undetected
- **Source:** 17-01-AUDIT-TIME-BOOKING.md

### ISSUE-RES-001: Reservation Status Management Not Exposed in UI
- **Description:** Reservations have status transitions but UI does not expose buttons to transition status.
- **Impact:** Reservations may need manual status updates that cannot be done through UI
- **Source:** 17-03-AUDIT-RESERVATIONS.md

---

## Missing Functionality

### MISSING-BAU-001: Site Delete UI
- **Description:** No delete button visible in sites list or detail page. Backend route exists but UI does not expose delete functionality.
- **Impact:** Users cannot remove sites that were created in error
- **Source:** 17-01-AUDIT-BAUSTELLEN.md

### MISSING-BAU-002: Site Detail Page E2E Coverage
- **Description:** Site detail page has no E2E test coverage.
- **Impact:** Bugs in site detail view may go undetected
- **Source:** 17-01-AUDIT-BAUSTELLEN.md

### MISSING-TIME-001: Time Entry Edit/Delete
- **Description:** Users can create time entries but cannot edit or delete them.
- **Impact:** Users cannot correct mistakes in time entries
- **Source:** 17-01-AUDIT-TIME-BOOKING.md

### MISSING-TIME-002: Time Entry List View
- **Description:** No dedicated page to view all time entries.
- **Impact:** Limited visibility into time booking history
- **Source:** 17-01-AUDIT-TIME-BOOKING.md

### MISSING-INV-001: Material Delete UI
- **Description:** No delete button visible in materials list.
- **Impact:** Users cannot remove materials that were created in error
- **Source:** 17-02-AUDIT-INVENTORY.md

### MISSING-INV-002: Low Stock Alert System
- **Description:** Backend has min_quantity and is_low_stock but no UI indicator or alert system.
- **Impact:** Users may not know when materials need reordering
- **Source:** 17-02-AUDIT-INVENTORY.md

### MISSING-INV-003: Material Detail Page E2E Coverage
- **Description:** Material detail page has no E2E test coverage.
- **Impact:** Bugs in material detail view may go undetected
- **Source:** 17-02-AUDIT-INVENTORY.md

### MISSING-FLEET-001: Vehicle/Tool Delete UI
- **Description:** No delete button visible in vehicles or tools list.
- **Impact:** Users cannot remove vehicles/tools that were created in error
- **Source:** 17-03-AUDIT-FLEET.md

### MISSING-FLEET-002: Status Transition UI
- **Description:** Vehicle and tool status has no UI for transitions.
- **Impact:** Status tracking is manual, not reflected in app
- **Source:** 17-03-AUDIT-FLEET.md

### MISSING-FLEET-003: Calendar Click-to-Create
- **Description:** CalendarView shows reservations but clicking empty slots does not create new ones.
- **Impact:** Users must go through Fleet page to create reservations
- **Source:** 17-03-AUDIT-FLEET.md

### MISSING-RES-001: Reservation Edit UI
- **Description:** Users can create reservations but cannot edit them.
- **Impact:** Users must cancel and recreate to change reservations
- **Source:** 17-03-AUDIT-RESERVATIONS.md

### MISSING-RES-002: Reservation Status Transition UI
- **Description:** No UI buttons to confirm, start, complete, or cancel reservations.
- **Impact:** Reservations stuck in "pending" state, workflow incomplete
- **Source:** 17-03-AUDIT-RESERVATIONS.md

### MISSING-RES-003: Overlap Warning Details in Dialog
- **Description:** ReservationDialog shows availability warning but does not show which reservation conflicts.
- **Impact:** Users don't know who has the resource booked
- **Source:** 17-03-AUDIT-RESERVATIONS.md

---

## E2E Test Gaps

| Feature | Tested | Not Tested |
|---------|--------|------------|
| Baustellen | navigation, create, list | update, delete, status transitions, assignments, activities |
| Time Booking | (via API helpers only) | UI dialog, validation, create via UI |
| Inventory | navigation, create, list | update, delete, withdraw, QR scan |
| Fleet | navigation, create (API) | UI dialogs, update, delete, status transitions |
| Reservations | (none) | all operations |

---

## Resolved Issues

### BUG-004: Fleet "Neu" Button Non-Functional — FIXED ✓
- **Status:** Fixed in current codebase
- **Description:** The "Neu" button in Fleet page now properly opens a dropdown with options for "Fahrzeug" and "Werkzeug".
- **Implementation:** FleetPage.tsx uses DropdownMenu with onClick handlers that set dialogType state.
- **Source:** 17-03-AUDIT-FLEET.md

---

## Recommendations

### Immediate Actions (v1.6)

1. **Fix BUG-TIME-001** — Add hours > 0 validation in TimeEntryDialog
2. **Add E2E tests for reservations** — Cover create, cancel, availability check
3. **Add E2E tests for update/delete** — Sites, Inventory, Fleet all need coverage
4. **Wire QR code button** — Add onClick handler for QR scanning

### Short-term Actions (v1.7)

1. **Add delete UI** — Sites, Materials, Vehicles, Tools all need delete buttons
2. **Add inline validation** — TimeEntryDialog and other forms need validation feedback
3. **Add time entry edit/delete** — Allow users to modify existing entries
4. **Implement status transitions UI** — Fleet status, reservation status

### Medium-term Actions (v1.8+)

1. **Low stock alerts** — Implement notification system for inventory thresholds
2. **Reservation workflow UI** — Status transition buttons for reservations
3. **Calendar click-to-create** — Allow creating reservations from calendar view
4. **Fix auth token issues** — Address BUG-001, BUG-002, BUG-003 from BUG-REPORT.md

---

*Generated by Phase 17 Feature Audit*
*Total issues: 24 (1 high, 5 medium, 3 low, 4 functional issues, 11 missing functionality)*
