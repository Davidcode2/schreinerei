---
phase: 41
plan: 02
subsystem: frontend
tags: [mobile, modal, step-navigation, ux]
requires:
  - 41-01 (StepIndicator and StepContainer components)
provides:
  - Step-based AddMaterialDialog with mobile-optimized layout
affects:
  - AddMaterialDialog component
  - Material creation workflow
tech-stack:
  added:
    - StepIndicator component integration
    - StepContainer component integration
    - Step-based form validation
  patterns:
    - Two-step wizard pattern
    - Mobile-first modal height constraints
    - Swipe gesture navigation
key-files:
  created: []
  modified:
    - frontend/src/pages/inventory/AddMaterialDialog.tsx
    - frontend/src/pages/inventory/AddMaterialDialog.test.tsx
decisions:
  - Split form into Basisdaten (Step 1) and Details (Step 2) for mobile UX
  - Step 1 contains all required fields for basic material creation
  - Step 2 contains optional/conditional fields (Mindestbestand, MHD, Lagerort)
  - Max-height constraint of 90vh for mobile compatibility
  - Swipe gestures enabled via StepContainer for step navigation
metrics:
  duration: 10 minutes
  tasks_completed: 3
  files_modified: 2
  tests_added: 6
  tests_passed: 15
---

# Phase 41 Plan 02: AddMaterialDialog Step-Based Layout Summary

## One-Liner

Refactored AddMaterialDialog to a two-step wizard layout with dot indicators, swipe navigation, and mobile-optimized height constraints.

## What Was Done

### Task 1: Refactor AddMaterialDialog to Step-Based Layout

**Changes to AddMaterialDialog.tsx:**

1. Added `currentStep` state for step navigation
2. Defined step validation:
   - `isStep1Valid`: categoryId, name, quantity, unit (all required)
   - `isStep2Valid`: minQuantity + (expiresOn if category can expire)
3. Split form content into two steps:
   - **Step 1 - Basisdaten**: Kategorie, Name, Menge, Einheit
   - **Step 2 - Details**: Mindestbestand, MHD (conditional), Lagerort
4. Integrated `StepIndicator` component for dot navigation
5. Integrated `StepContainer` component for swipe gestures
6. Updated `DialogFooter` with step-appropriate buttons:
   - Step 1: Abbrechen, Weiter
   - Step 2: Zurück, Erstellen
7. Added `max-h-[90vh]` constraint to DialogContent for mobile

### Task 2: Add Tests for Step Navigation

**New test suite "Step Navigation" with 6 tests:**
- Starts on step 1
- Weiter button disabled when Step 1 incomplete
- Weiter button enabled when Step 1 complete
- Navigates to Step 2 when Weiter is clicked
- Shows Zurück button on Step 2
- Returns to Step 1 when Zurück is clicked

**Updated existing tests:**
- Refactored all existing tests to work with step-based layout
- Added helper function `fillStep1AndNavigate` for common test setup
- Used `fireEvent` for date input in MHD test (jsdom compatibility)

### Task 3: Verification

- TypeScript compiles without errors
- All 15 tests pass

## Deviations from Plan

None - plan executed exactly as written.

## Files Modified

| File | Changes |
|------|---------|
| `frontend/src/pages/inventory/AddMaterialDialog.tsx` | Added step-based navigation, integrated StepIndicator/StepContainer |
| `frontend/src/pages/inventory/AddMaterialDialog.test.tsx` | Added 6 step navigation tests, updated existing tests for step layout |

## Commits

| Commit | Message |
|--------|---------|
| `20852b1` | feat(41-02): refactor AddMaterialDialog to step-based layout |
| `042d22a` | test(41-02): add step navigation tests for AddMaterialDialog |

## Verification Results

- TypeScript: No errors
- Tests: 15 passed, 0 failed

## Next Steps

- Plan 41-03 can proceed with applying the same pattern to other tall modals

## Self-Check: PASSED

- Commits verified: 20852b1, 042d22a
- SUMMARY.md created at correct location
- All tests pass
