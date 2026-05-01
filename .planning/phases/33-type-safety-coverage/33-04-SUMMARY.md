---
phase: 33-type-safety-coverage
plan: 04
subsystem: ui
tags: [react, inventory, ts-rs, typescript, testing]
requires:
  - phase: 33-type-safety-coverage
    provides: generated inventory facade and CI-aligned drift gate from plan 01
provides:
  - generated-backed order request aliases in the inventory facade
  - inventory order hooks typed through generated DTO contracts
  - regression coverage for generated order hook wiring
affects: [inventory-hooks, order-requests, generated-types]
tech-stack:
  added: []
  patterns: [thin inventory facade over generated DTOs, source-level regression checks for facade drift]
key-files:
  created: []
  modified: [frontend/src/types/inventory.ts, frontend/src/lib/api/hooks/useInventory.ts, frontend/src/lib/api/hooks/useInventory.test.tsx]
key-decisions:
  - "Order request UI imports stay on @/types/inventory while the facade now aliases generated OrderRequestResponse and sibling DTOs directly."
  - "Hook regression coverage checks both runtime order endpoints and the facade source so handwritten DTO drift fails fast in frontend tests."
patterns-established:
  - "Inventory order hooks consume generated DTO contracts through the local facade instead of handwritten interfaces."
  - "Source-level tests may pin facade re-exports when type-only drift is otherwise invisible to Vitest runtime assertions."
requirements-completed: []
duration: 8min
completed: 2026-05-01
---

# Phase 33 Plan 04: Order DTO gap closure Summary

**Inventory order hooks now consume generated ts-rs DTOs through the local facade, with regression tests that catch handwritten facade drift.**

## Performance

- **Duration:** 8 min
- **Started:** 2026-05-01T20:27:35Z
- **Completed:** 2026-05-01T20:30:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Replaced the remaining handwritten order DTO definitions in `@/types/inventory` with generated aliases.
- Retyped `useOrderRequests` and `useCreateOrderRequest` through the generated-backed inventory facade.
- Added hook regression coverage for order endpoints plus source-level checks that the facade stays generated-backed.

## task Commits

Each task was committed atomically:

1. **task 1: replace the remaining handwritten inventory order DTO aliases with generated re-exports** - `2978cb5`, `7f3a1fa` (test, feat)
2. **task 2: add a regression test for generated-backed order hooks and rerun the CI drift gate** - `dcfce8c` (test)

## Files Created/Modified
- `frontend/src/types/inventory.ts` - re-exports generated order DTOs instead of handwritten interfaces
- `frontend/src/lib/api/hooks/useInventory.ts` - imports order request/query types from the local generated-backed facade
- `frontend/src/lib/api/hooks/useInventory.test.tsx` - covers order hooks and guards the facade against handwritten DTO drift

## Decisions Made
- Kept the ergonomic `OrderRequest` name by aliasing `OrderRequestResponse` in the inventory facade instead of renaming UI imports.
- Used source-level assertions in the hook suite because Vitest runtime execution alone would not catch handwritten type duplication.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected the source-file path in the new facade regression test**
- **Found during:** task 1 (replace the remaining handwritten inventory order DTO aliases with generated re-exports)
- **Issue:** The first RED attempt pointed at `frontend/src/lib/types/inventory.ts`, so the suite failed on an ENOENT instead of the intended handwritten DTO check.
- **Fix:** Updated the path to the real `frontend/src/types/inventory.ts` location before rerunning the RED phase.
- **Files modified:** frontend/src/lib/api/hooks/useInventory.test.tsx
- **Verification:** `npm run test:run -- src/lib/api/hooks/useInventory.test.tsx`
- **Committed in:** `2978cb5` / `7f3a1fa`

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** The fix kept the new regression targeted at the intended facade drift behavior. No scope creep.

## Issues Encountered
- The initial type-only RED assertion passed because Vitest transpiles away compile-time generic checks; source-level assertions provided a stable runtime guard for the same contract.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- The remaining Phase 33 frontend gap is isolated to history badge styling and its tests.
- The generated DTO drift gate now covers the inventory order flow as well as the earlier material/category mutations.

## Self-Check: PASSED

- Summary file exists.
- Commits `2978cb5`, `7f3a1fa`, and `dcfce8c` exist in git history.
