---
phase: 32-enriched-history
plan: 02
subsystem: ui
tags: [react, inventory, history, routing]
requires:
  - phase: 32-enriched-history
    provides: enriched material history query hook
provides:
  - dedicated inventory history presentation component
  - enriched detail-page rendering with badges, attribution, and site links
affects: [inventory-detail-page, history-verification, phase-33-tests]
tech-stack:
  added: []
  patterns: [inventory history presentation stays separate from sites activity feed]
key-files:
  created:
    - frontend/src/pages/inventory/MaterialHistoryFeed.tsx
  modified:
    - frontend/src/pages/inventory/InventoryDetailPage.tsx
    - frontend/src/pages/inventory/InventoryDetailPage.test.tsx
key-decisions:
  - "Inventory detail history uses a dedicated `MaterialHistoryFeed` instead of reusing the sites activity feed."
  - "History query failures stay isolated to the history card through `ErrorState` with retry."
patterns-established:
  - "Inventory stock-movement rows render badge, timestamp, signed quantity, attribution, and optional site metadata in a fixed order."
requirements-completed: [STOCK-02, HIST-01, HIST-02, HIST-03]
duration: 4min
completed: 2026-05-01
---

# Phase 32 Plan 02: Enriched history UI summary

**Inventory detail history now renders enriched stock movements with color-coded badges, actor attribution, signed quantities, and clickable Baustelle links.**

## Performance

- **Duration:** 4 min
- **Started:** 2026-05-01T15:26:13Z
- **Completed:** 2026-05-01T15:27:26Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Replaced legacy history mocks with enriched fixtures and failing-first UI regression tests.
- Added `MaterialHistoryFeed` with the fixed label and badge-class map for all five history entry types.
- Wired `InventoryDetailPage` to the enriched hook with isolated error handling and retry.

## task Commits

1. **task 1: write detail-page regression tests for enriched history rendering** - `92f9746` (test)
2. **task 2: extract and wire a dedicated enriched history feed component** - `b725265` (feat)

## Files Created/Modified
- `frontend/src/pages/inventory/InventoryDetailPage.test.tsx` - covers badges, attribution, Baustelle links, and the enriched empty state.
- `frontend/src/pages/inventory/MaterialHistoryFeed.tsx` - renders the inventory-specific enriched history feed.
- `frontend/src/pages/inventory/InventoryDetailPage.tsx` - switches to the enriched history hook and local error handling.

## Decisions Made
- Kept inventory history UI separate from the sites activity feed because the layout and semantics are inventory-specific.
- Limited Baustelle links to withdrawal rows with both `site_id` and `site_name` to preserve internal-only routing.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 33 can extend automated coverage around the enriched feed without additional UI extraction work.
- The inventory detail page now exposes the observable behaviors required for STOCK-02 and HIST-01/02/03 verification.

## Self-Check: PASSED

- Summary file exists.
- Commits `92f9746` and `b725265` exist in git history.
