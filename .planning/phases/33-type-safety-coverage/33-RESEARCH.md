# Phase 33: Type Safety & Coverage - Research

**Date:** 2026-05-01
**Status:** Ready for planning
**Phase:** 33
**Requirements:** quality gate for Phase 30-32 deliverables

## Goal

Close the remaining quality gap for the v1.9 inventory work by:
- eliminating drift between Rust DTOs and the frontend inventory contract,
- adding backend validation coverage around the new category and stock-in/edit endpoints,
- adding E2E coverage for the new settings, edit, stock-in, and enriched-history flows.

## What Already Exists

### Backend DTO export pipeline

- Inventory DTOs in `src/modules/inventory/api/routes.rs` already derive `TS` and export into `frontend/src/types/generated.ts`.
- CI already checks generated binding drift in `.github/workflows/ci.yml` by running `cargo test --all` and failing if `frontend/src/types/generated.ts` changes.
- The repo previously used `cargo test --features ts-rs/export`, but current CI and the current `ts-rs` setup rely on the normal test run regenerating bindings.

### Frontend type state

- `frontend/src/types/generated.ts` already contains the Phase 30-32 inventory DTOs, including `UpdateCategoryRequest`, `UpdateMaterialRequest`, `StockInRequest`, `EntryType`, and `EnrichedStockHistoryResponse`.
- `frontend/src/types/inventory.ts` still manually re-declares the same inventory request/response shapes.
- The frontend already has one proven generated-type import pattern in `frontend/src/lib/api/hooks/usePreferences.ts`.

This means the current gap is not missing ts-rs exports. The gap is that the inventory frontend still maintains a second handwritten contract beside the generated one.

### Backend validation coverage

- Domain validation for `UpdateCategory`, `UpdateMaterial`, and `StockIn` already exists in:
  - `src/modules/inventory/domain/category.rs`
  - `src/modules/inventory/domain/material.rs`
- Repository-level guard coverage exists only as lightweight SQL contract tests in `src/modules/inventory/infrastructure/material_repository.rs`.
- `src/modules/inventory/api/routes.rs` currently has no focused test block for the Phase 30 DTO conversion and PATCH semantics.

### Frontend test state

- Vitest coverage already exists for:
  - `frontend/src/lib/api/hooks/useInventory.test.tsx`
  - `frontend/src/pages/inventory/InventoryDetailPage.test.tsx`
- Playwright is installed and `frontend/tests/inventory.spec.ts` already covers navigation and basic API-backed persistence.
- The E2E harness already has reusable auth, cleanup, and API helper patterns in `frontend/tests/helpers/`.

## Established Patterns To Reuse

### Generated-type consumption pattern

Prefer the existing `usePreferences` approach: import DTO types from `@/types/generated` at the API boundary, then keep local type files only for UI-specific aliases or composition.

For inventory, this suggests:
- moving shared request/response DTOs to generated imports,
- keeping only client-specific aliases/helpers in `frontend/src/types/inventory.ts`, or turning that file into a thin re-export layer.

### Backend validation pattern

The repo favors small, direct unit tests colocated with the code they validate:
- domain command validation inside the domain module,
- route/DTO behavior inside the route module,
- SQL/guard invariants in the repository module.

Phase 33 should extend those seams instead of introducing real-Postgres integration tests, because `INT-01` is explicitly deferred to v2.

### Playwright verification pattern

The E2E README defines the house style:
- log in in `beforeEach`,
- create unique data with `uniqueName(...)`,
- track created entities for cleanup,
- verify persisted effects via API, not only UI success messages.

Phase 33 should keep those rules and extend `frontend/tests/helpers/api.ts` only where the new flows need missing helper coverage.

## Likely Files For Phase 33

### Type alignment

- `frontend/src/types/inventory.ts`
- `frontend/src/types/generated.ts`
- `frontend/src/lib/api/hooks/useInventory.ts`
- inventory pages/tests that import handwritten DTOs today
- `.github/workflows/ci.yml` only if the local verification command needs a clearer failure message (not required if current check is sufficient)

### Backend validation

- `src/modules/inventory/api/routes.rs`
- `src/modules/inventory/infrastructure/material_repository.rs`
- possibly `src/modules/inventory/domain/category.rs` or `src/modules/inventory/domain/material.rs` only for missing edge-case assertions

### E2E coverage

- `frontend/tests/inventory.spec.ts`
- `frontend/tests/helpers/api.ts`
- `frontend/tests/helpers/data.ts` only if extra cleanup tracking is required

## Constraints And Planning Implications

1. **Do not expand into real database integration tests.**
   `INT-01` is already deferred to v2, so Phase 33 should stay at unit, route-contract, Vitest, and Playwright level.

2. **Do not keep two sources of truth for DTOs.**
   If inventory continues to duplicate generated request/response types, the phase misses its core type-safety goal even if `generated.ts` is current.

3. **Reuse shipped UI behavior as the E2E target.**
   Phase 33 validates Phase 31-32; it should not invent new inventory UX.

4. **Keep verification commands aligned with CI reality.**
   The authoritative drift check today is `cargo test --all` plus `git diff --exit-code frontend/src/types/generated.ts`.

5. **Preserve tenant-safe API usage.**
   E2E helpers should keep using authenticated `page.request` so they exercise the same protected routes as the app.

## Recommended Plan Shape

The work naturally breaks into three plans:

1. **Type contract alignment**
   - consume generated inventory DTOs at the frontend API boundary
   - regenerate and verify `generated.ts`
   - remove or thin manual inventory DTO duplication

2. **Backend validation coverage**
   - add route-level tests for inventory DTO conversion/validation semantics
   - strengthen repository guard coverage for category delete conflicts

3. **Inventory E2E coverage**
   - cover settings page category management behavior
   - cover material edit and stock-in flows
   - cover enriched history rendering, attribution, and site links with API-backed assertions

## Risks / Common Pitfalls

- Updating `generated.ts` without switching inventory consumers off handwritten DTOs would preserve drift risk.
- Writing only domain validation tests would miss the API-layer PATCH semantics introduced in Phase 30.
- Adding Playwright UI assertions without API verification would miss persistence regressions.
- Trying to cover every inventory flow in one giant E2E test would create brittle setup and opaque failures.
- Reintroducing the outdated `cargo test --features ts-rs/export` command into verification would diverge from CI.

## Research Verdict

Research complete. Phase 33 should be planned as three focused plans: generated inventory type alignment, backend validation coverage, and API-verified inventory E2E tests.
