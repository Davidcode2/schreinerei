---
phase: 25-deduction-history-site-name-end-to-end-wiring
plan: 01
subsystem: ui
tags: [react, react-query, vitest, msw, inventory]
requires:
  - phase: 25-deduction-history-site-name-end-to-end-wiring
    provides: backend stock history endpoint with site_name
provides:
  - typed frontend material history contract and query hook
  - inventory detail history UI with conditional site label rendering
  - regression tests for site_name present/null and empty-state behavior
affects: [inventory, deduction-history, frontend-tests]
tech-stack:
  added: []
  patterns: [react-query hook per endpoint, conditional text-only rendering for optional fields]
key-files:
  created:
    - frontend/src/lib/api/hooks/useInventory.test.tsx
    - frontend/src/pages/inventory/InventoryDetailPage.test.tsx
  modified:
    - frontend/src/types/inventory.ts
    - frontend/src/lib/api/hooks/useInventory.ts
    - frontend/src/pages/inventory/InventoryDetailPage.tsx
key-decisions:
  - "Expose history rows through a dedicated useMaterialHistory hook keyed by material id for cache isolation."
  - "Render notes/site_name as plain React text nodes with conditional blocks to satisfy tampering mitigation."
patterns-established:
  - "Inventory detail pages consume endpoint-specific React Query hooks rather than inline fetch logic."
  - "History UI uses explicit empty-state copy when arrays are empty."
requirements-completed: [DEDU-03]
duration: 2min
completed: 2026-05-01
---

# Phase 25 Plan 01: Deduction history site name end-to-end wiring Summary

**Inventory detail now consumes material stock history and shows linked Baustelle names from `site_name` with regression coverage for linked, unlinked, and empty history states.**

## Performance

- **Duration:** 2 min
- **Started:** 2026-04-30T22:05:50Z
- **Completed:** 2026-04-30T22:07:30Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments
- Added `MaterialStockHistoryEntry` and `useMaterialHistory(id)` with endpoint `/api/v1/inventory/materials/${id}/history`.
- Added a new "Historie" card in `InventoryDetailPage` with conditional `site_name` rendering and empty-state text.
- Added deterministic MSW regression tests that cover `site_name` visible, `site_name: null`, and empty history responses.

## Task Commits

1. **Task 1: Add typed material history consumer hook for DEDU-03** - `2ba8e34` (test), `4960c4d` (feat)
2. **Task 2: Render deduction history with conditional site name in inventory detail** - `de72b81` (feat)
3. **Task 3: Add regression test for site_name end-to-end render path** - `32dc4a2` (test)

## Files Created/Modified
- `frontend/src/types/inventory.ts` - added `MaterialStockHistoryEntry` including nullable `site_name`.
- `frontend/src/lib/api/hooks/useInventory.ts` - added `useMaterialHistory` React Query hook.
- `frontend/src/pages/inventory/InventoryDetailPage.tsx` - added history section and conditional site label row.
- `frontend/src/lib/api/hooks/useInventory.test.tsx` - verifies history endpoint call, key presence, and disabled behavior.
- `frontend/src/pages/inventory/InventoryDetailPage.test.tsx` - verifies site_name rendering rules and empty-state copy.

## Decisions Made
- Kept history rendering lightweight with direct map output and no expensive client transforms (threat mitigation T-25-03).
- Used conditional UI blocks for `site_name` and `notes` without HTML injection APIs (threat mitigation T-25-01).

## Deviations from Plan
None - plan executed exactly as written.

## Issues Encountered
- `gsd-sdk` CLI was unavailable in this environment; plan execution proceeded with direct repository operations.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- DEDU-03 frontend integration gap is closed with tests guarding regressions.
- Ready for phase verification and manual smoke of `/inventory/:id` history rendering.

## Self-Check: PASSED
- FOUND: `.planning/phases/25-deduction-history-site-name-end-to-end-wiring/25-01-SUMMARY.md`
- FOUND: `2ba8e34`
- FOUND: `4960c4d`
- FOUND: `de72b81`
- FOUND: `32dc4a2`
