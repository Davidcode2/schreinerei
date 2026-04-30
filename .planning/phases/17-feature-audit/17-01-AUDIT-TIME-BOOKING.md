# Time Booking Feature Audit

**Date:** 2026-04-30
**Auditor:** Automated + Manual Code Review
**Requirement:** AUDIT-03

## Summary

| Category | Count |
|----------|-------|
| Critical Bugs | 0 |
| High Bugs | 1 |
| Medium Bugs | 1 |
| Low Bugs | 0 |
| Functional Issues | 1 |
| Missing Functionality | 2 |

## Known Issue Investigation

### Pending Todo: Baustelle Time Booking 400 Error
- **Status:** Root cause identified
- **Root Cause:** Hours field can submit 0 value, which fails backend validation
- **Details:** The TimeEntryDialog sends `hours` as `parseFloat(e.target.value) || 0`. If the input is empty or contains invalid text, this evaluates to 0. Backend validation in `CreateTimeEntry::validate()` rejects `hours <= 0.0` with "Hours must be positive".

**Code Evidence:**

Frontend (TimeEntryDialog.tsx:114):
```typescript
onChange={(e) => setHours(parseFloat(e.target.value) || 0)}
```

Backend (time_entry.rs:40-41):
```rust
if self.hours <= 0.0 {
    return Err("Hours must be positive".to_string());
}
```

**Fix Required:**
1. Add frontend validation to prevent submitting when hours <= 0
2. OR: Initialize hours to 1 (minimum valid value) instead of allowing 0
3. Add disabled state to submit button when hours <= 0

## Test Coverage

**Existing E2E Tests:**
- [ ] None dedicated to time booking (tested only via API helpers)

**Missing E2E Tests:**
- [ ] Time entry dialog opens
- [ ] Work type selection (site, workshop, travel, other)
- [ ] Date picker functionality
- [ ] Hours input validation (min 0.5, max 24)
- [ ] Quick hour buttons (0.5h, 1h, 2h, 4h, 8h)
- [ ] Notes field
- [ ] Submit creates time entry via API
- [ ] Error handling for invalid hours
- [ ] Site association when work_type is "site"

## API Audit

### POST /api/v1/time-entries
- **Status:** Working (with valid hours)
- **Expected Payload:**
  ```typescript
  {
    site_id?: string,
    work_type: "site" | "workshop" | "travel" | "other",
    hours: number,  // Must be > 0 and <= 24
    work_date: string,  // YYYY-MM-DD format
    notes?: string
  }
  ```
- **Validation Errors:**
  - hours <= 0: "Hours must be positive"
  - hours > 24: "Hours cannot exceed 24 per day"
  - work_date in future: "Work date cannot be in the future"
  - invalid work_type: Parse error

### GET /api/v1/time-entries/my
- **Status:** Working
- **Returns:** Time entries for authenticated user

### GET /api/v1/sites/{id}/time-entries
- **Status:** Working
- **Returns:** Time entries for specific site

## UI Audit

### TimeEntryDialog.tsx
- **Opens correctly:** Yes (component exists and is properly structured)
- **Work type selection:** Working - 4 types with button group
- **Date picker:** Working - uses native date input
- **Hours input:** PARTIAL - allows 0 value which fails validation
- **Quick hour buttons:** Working - 0.5h, 1h, 2h, 4h, 8h options
- **Submit succeeds:** PARTIAL - fails with hours=0

### Form Validation
- **Work type:** Required, defaults to "site"
- **Date:** Required, defaults to today
- **Hours:** REQUIRED but allows 0 - BUG
- **Notes:** Optional

## Bugs Found

### BUG-TIME-001: Hours Field Allows Invalid Zero Value
- **Severity:** High
- **Location:** frontend/src/pages/sites/TimeEntryDialog.tsx:114
- **Description:** The hours input uses `parseFloat(e.target.value) || 0` which can result in 0 hours being submitted. Backend rejects hours <= 0 with validation error.
- **Reproduction:**
  1. Open TimeEntryDialog
  2. Clear hours input field
  3. Click "Speichern" (Save)
  4. Request sends hours: 0
  5. Backend returns 400 with "Hours must be positive"
- **Impact:** Users see "Zeiterfassung fehlgeschlagen" error with no clear explanation
- **Suggested Fix:**
  ```typescript
  // Option 1: Disable submit when hours <= 0
  disabled={!isFormValid || hours <= 0 || createMutation.isPending}
  
  // Option 2: Use minimum value instead of 0
  onChange={(e) => setHours(parseFloat(e.target.value) || 1)}
  ```

### BUG-TIME-002: Missing Input Validation Feedback
- **Severity:** Medium
- **Location:** frontend/src/pages/sites/TimeEntryDialog.tsx
- **Description:** The dialog shows no inline validation messages. Users only see a generic toast error after submission fails.
- **Reproduction:**
  1. Open TimeEntryDialog
  2. Enter hours = 25 (exceeds 24 limit)
  3. Submit
  4. Backend returns error, toast shows generic failure message
- **Impact:** Poor UX - users don't know what's wrong
- **Suggested Fix:** Add inline validation with error messages below fields

## Functional Issues

### ISSUE-TIME-001: No E2E Test Coverage for Time Booking
- **Description:** Time booking is a core feature but has no dedicated E2E tests. Only API helper exists for testing.
- **Impact:** Regression bugs in time booking flow may go undetected

## Missing Functionality

### MISSING-TIME-001: Time Entry Edit/Delete
- **Description:** Users can create time entries but cannot edit or delete them. No UI for modifying existing entries.
- **Requirement:** Standard CRUD operations
- **Impact:** Users cannot correct mistakes in time entries

### MISSING-TIME-002: Time Entry List View
- **Description:** No dedicated page to view all time entries. Users can only see entries in context of a site or via "my time entries" API.
- **Requirement:** User should see their time entries
- **Impact:** Limited visibility into time booking history

## Domain Logic Review

### Validation Rules (Backend)
| Rule | Implementation | Status |
|------|---------------|--------|
| Hours > 0 | `hours > 0.0` | ✓ Enforced |
| Hours <= 24 | `hours <= 24.0` | ✓ Enforced |
| Work date not future | `work_date <= today` | ✓ Enforced |
| Site ID optional | `Option<SiteId>` | ✓ Correct |
| Work type required | `WorkType` enum | ✓ Enforced |

### Work Type Options
| Type | German Label | Use Case |
|------|-------------|----------|
| site | Baustelle | Work on construction site |
| workshop | Werkstatt | Workshop work |
| travel | Fahrt | Travel time |
| other | Sonstiges | Other activities |

---

*Generated by Phase 17 Feature Audit*
*Source files reviewed: TimeEntryDialog.tsx, time_entry.rs, routes.rs, useSites.ts*
