# 44-03 Summary

## Outcome

The main downstream surfaces that depended on Baustelle-only language now understand the broadened project model and can clearly distinguish internal workshop projects from external ones.

## Changes

- Updated `TimeEntryDialog` to use project-aware wording and show project-type cues in the selector
- Updated `ReservationConfirmationSheet` and `ReservationDialog` to label project linkage as `Projekt` and show internal/external cues
- Updated `ActiveSiteIndicator` to read as active project context instead of active Baustelle context
- Updated `DashboardPage` copy to use project-aware wording while leaving the existing active-only data behavior unchanged
- Added targeted tests for time booking, reservation, active-context, dashboard, and confirmation-sheet wording

## Verification

- `npm --prefix frontend run test -- TimeEntryDialog.test.tsx ReservationConfirmationSheet.test.tsx ReservationDialog.test.tsx DashboardPage.test.tsx ActiveSiteIndicator.test.tsx`
- `npm --prefix frontend run build`

## Notes

- This plan intentionally does not change the hidden dashboard filtering logic; that remains Phase 47 scope.
