---
phase: 32-enriched-history
plan: 01
subsystem: ui
tags: [react, react-query, inventory, history]
requires:
  - phase: 30-backend-api-foundation
    provides: enriched inventory history endpoint with typed payload fields
provides:
  - dedicated enriched material history React Query hook
  - regression coverage for enriched history endpoint and cache key
affects: [inventory-detail-page, enriched-history-ui]
tech-stack:
  added: []
  patterns: [dedicated React Query contract per backend history shape]
key-files:
  created: []
  modified:
    - frontend/src/lib/api/hooks/useInventory.ts
    - frontend/src/lib/api/hooks/useInventory.test.tsx
key-decisions:
  - "Enriched inventory history uses its own React Query cache key so legacy history data cannot masquerade as enriched payloads."
  - "The frontend fetches enriched inventory history through a dedicated hook instead of page-local API calls."
patterns-established:
  - "Inventory history variants get separate hooks when their payload contracts diverge."
requirements-completed: [STOCK-02, HIST-02, HIST-03]
duration: 3min
completed: 2026-05-01
---

# Phase 32 Plan 01: Enriched history hook summary

**Dedicated React Query wiring for enriched inventory history via `/history/enriched` with its own cache namespace.**

## Performance

- **Duration:** 3 min
- **Started:** 2026-05-01T15:24:45Z
- **Completed:** 2026-05-01T15:25:03Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Added failing-first regression coverage for the enriched history endpoint contract.
- Exported `useEnrichedMaterialHistory` with the exact enriched endpoint path and cache key.
- Preserved the legacy history hook so the UI migration could land in the next plan without contract drift.

## task Commits

1. **task 1: add enriched-history hook regression coverage first** - `abf411f` (test)
2. **task 2: implement the dedicated enriched-history query hook** - `74c2310` (feat)

## Files Created/Modified
- `frontend/src/lib/api/hooks/useInventory.test.tsx` - pins the enriched endpoint path, cache key, and disabled-empty-id behavior.
- `frontend/src/lib/api/hooks/useInventory.ts` - exports `useEnrichedMaterialHistory` using the enriched backend contract.

## Decisions Made
- Used a dedicated `material-history-enriched` query key to isolate enriched responses from the legacy history cache.
- Kept `enabled: !!id` on the new hook to avoid empty-id request churn.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The detail page can now consume enriched history through a stable hook instead of page-local fetch logic.
- Phase 32-02 can switch the UI to badges, attribution, and Baustelle links without inventing a new data path.

## Self-Check: PASSED

- Summary file exists.
- Commits `abf411f` and `74c2310` exist in git history.
