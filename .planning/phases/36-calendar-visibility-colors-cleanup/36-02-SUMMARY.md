---
phase: 36-calendar-visibility-colors-cleanup
plan: 02
subsystem: ui
tags: [react-router, fleet, navigation, vitest]
requires:
  - phase: 34-fleet-page-calendar-integration
    provides: embedded calendar on /fleet
  - phase: 35-range-selection-confirmation-flow
    provides: preserved list-dialog reservation flow and two-tap calendar booking
provides:
  - /fleet as the single primary fleet booking route
  - removal of the standalone fleet calendar CTA and route
  - regression coverage for embedded-first fleet booking
affects: [app-routing, fleet-page-calendar, frontend-tests]
tech-stack:
  added: []
  patterns: [embedded-first booking navigation]
key-files:
  created: []
  modified:
    - frontend/src/pages/fleet/FleetPage.tsx
    - frontend/src/pages/fleet/FleetPage.test.tsx
    - frontend/src/App.tsx
    - frontend/src/pages/fleet/index.ts
key-decisions:
  - "Removed the standalone /fleet/calendar entry path instead of maintaining a second primary booking surface."
  - "Preserved the existing list-based ReservationDialog path so fleet list actions still book directly from resource cards."
patterns-established:
  - "Fleet booking starts from embedded calendar on /fleet."
requirements-completed: [FCAL-03]
duration: 3min
completed: 2026-05-01
---

# Phase 36 Plan 02: Fleet Entry Cleanup Summary

**The fleet page now serves as the single booking entry experience by keeping the embedded calendar on `/fleet` and removing the standalone calendar CTA and route.**

## Performance

- **Duration:** 3 min
- **Started:** 2026-05-01T17:03:00Z
- **Completed:** 2026-05-01T17:06:14Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Removed the old `Kalenderansicht öffnen` CTA from `FleetPage`.
- Removed the standalone `/fleet/calendar` route and stale barrel export wiring.
- Kept the embedded calendar first on `/fleet` and preserved the existing list-based `ReservationDialog` flow.

## task Commits

Not created per explicit user instruction.

## Files Created/Modified

- `frontend/src/pages/fleet/FleetPage.tsx` - removed standalone calendar CTA while keeping embedded calendar first
- `frontend/src/pages/fleet/FleetPage.test.tsx` - coverage for CTA removal and preserved list-dialog flow
- `frontend/src/App.tsx` - removed `/fleet/calendar` route
- `frontend/src/pages/fleet/index.ts` - removed now-unused `CalendarView` barrel export

## Decisions Made

- Cleaned up only the standalone calendar entry path and left the embedded calendar plus list-dialog booking intact.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `/fleet` is now the single primary fleet booking route in the frontend shell.
- Regression coverage protects both embedded calendar presence and list-dialog booking behavior.

## Known Stubs

None.

## Self-Check: PASSED

Summary file and referenced source files exist.
