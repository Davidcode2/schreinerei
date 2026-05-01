---
phase: 33-type-safety-coverage
plan: 01
subsystem: ui
tags: [react, inventory, ts-rs, typescript]
requires:
  - phase: 30-backend-api-foundation
    provides: Rust inventory DTO exports in generated frontend bindings
provides:
  - generated-type-backed inventory facade
  - inventory hooks typed against backend DTO exports
  - create and stock-in callers aligned with generated nullability
affects: [inventory-hooks, inventory-dialogs, generated-types]
tech-stack:
  added: []
  patterns: [thin inventory facade over generated DTOs, CI-aligned ts-rs drift verification]
key-files:
  created: []
  modified: [frontend/src/types/inventory.ts, frontend/src/lib/api/hooks/useInventory.ts, frontend/src/pages/inventory/AddMaterialDialog.tsx, frontend/src/pages/inventory/InventoryDetailPage.tsx]
key-decisions:
  - "Inventory keeps stable local type names through a thin facade, but the source of truth is now generated.ts."
  - "Create and stock mutations pass explicit nulls where the generated DTO contract requires them."
patterns-established:
  - "Frontend inventory hooks import request DTOs directly from generated bindings while UI aliases stay in types/inventory.ts."
requirements-completed: []
duration: 12min
completed: 2026-05-01
---

# Phase 33 Plan 01: Frontend contract alignment Summary

**Inventory hooks and dialogs now consume Rust-generated DTO contracts through a thin facade instead of handwritten duplicate request shapes.**

## Performance

- **Duration:** 12 min
- **Started:** 2026-05-01T17:40:00Z
- **Completed:** 2026-05-01T17:52:00Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Replaced duplicated inventory DTO definitions with generated-backed aliases in the local inventory facade.
- Retyped inventory mutation hooks against generated request DTOs.
- Updated create and detail-page mutation callers to satisfy generated nullability without changing runtime behavior.

## task Commits

1. **task 1: move inventory DTO imports to generated bindings** - `4b71d65` (feat)
2. **task 2: regenerate bindings and lock the drift check to the CI path** - no code diff; verified with `cargo test --all` and `git diff --exit-code frontend/src/types/generated.ts`

## Files Created/Modified
- `frontend/src/types/inventory.ts` - generated-backed inventory aliases and facade exports
- `frontend/src/lib/api/hooks/useInventory.ts` - generated DTO imports at the API boundary
- `frontend/src/pages/inventory/AddMaterialDialog.tsx` - explicit null payload fields for generated create DTOs
- `frontend/src/pages/inventory/InventoryDetailPage.tsx` - explicit null payload fields for generated stock mutations

## Decisions Made
- Kept `@/types/inventory` as the ergonomic import surface for UI code while removing it as a second DTO source of truth.
- Matched CI exactly for regeneration verification instead of reintroducing the older feature-flag export command.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Backend and E2E coverage can now rely on the same inventory DTO contract the Rust API exports.
- Generated binding drift is validated against the same `cargo test --all` path CI uses.

## Self-Check: PASSED

- Summary file exists.
- Commit `4b71d65` exists in git history.
