# 44-02 Summary

## Outcome

The existing `/sites` create and detail flows now behave as the main project planning surface for both external Baustellen and internal workshop projects.

## Changes

- Extended frontend site types with `project_type`
- Added `useRemoveAssignment()` to the site hooks
- Added `ProjectPlanningSheet` as a mobile-first planning editor
- Added `ProjectAssignmentsSection` for in-place team assignment management
- Updated `AddSiteDialog` to support project type, planning dates, estimated days, and project-aware copy
- Updated `SiteDetailPage`, `SitesListPage`, and `SiteCard` to surface the broader project model
- Added/updated targeted tests and mock handlers for the new flows

## Verification

- `npm --prefix frontend run test -- AddSiteDialog.test.tsx ProjectPlanningSheet.test.tsx SiteDetailPage.test.tsx`
- `npm --prefix frontend run build`

## Notes

- The runtime routes remain `/sites` in this phase; only the user-facing planning surface and copy are broadened toward the project model.
