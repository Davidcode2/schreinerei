---
phase: 15-ts-rs-type-generation
plan: 01
subsystem: type-generation
tags: [typescript, rust, codegen, types]
dependencies:
  requires: []
  provides: [typescript-types]
  affects: [frontend, backend]
tech_stack:
  added:
    - ts-rs v12 for Rust → TypeScript type generation
  patterns:
    - DTO types auto-generated from Rust structs
key_files:
  created:
    - .cargo/config.toml
    - frontend/src/types/generated.ts
    - .github/workflows/ci.yml
  modified:
    - Cargo.toml
    - src/modules/inventory/api/routes.rs
    - src/modules/sites/api/routes.rs
    - src/modules/fleet/api/routes.rs
    - src/modules/iam/api/routes.rs
decisions:
  - ts-rs v12 instead of v10 (v10 lacks export functionality)
  - export_to path with TS_RS_EXPORT_DIR="" for direct output
metrics:
  duration: ~15 minutes
  completed_date: 2026-04-30
  dtos_updated: 49
  types_generated: 49
---

# Phase 15 Plan 01: ts-rs Type Generation Summary

## One-liner

Generated TypeScript type definitions from Rust DTOs using ts-rs to prevent frontend-backend type drift.

## What Was Done

Added ts-rs derive macros to all 49 backend DTOs across 4 modules (inventory, sites, fleet, IAM) and configured automatic TypeScript type generation to `frontend/src/types/generated.ts`.

### Tasks Completed

1. **Add ts-rs dependency** — Added ts-rs v12 with serde-compat, chrono-impl, uuid-impl, url-impl features
2. **Inventory DTOs** — Added TS derive and #[ts(export)] to 14 DTOs
3. **Sites DTOs** — Added TS derive and #[ts(export)] to 12 DTOs
4. **Fleet DTOs** — Added TS derive and #[ts(export)] to 19 DTOs
5. **IAM DTOs** — Added TS derive and #[ts(export)] to 4 DTOs
6. **Generate TypeScript types** — Configured export and generated `frontend/src/types/generated.ts` (111 lines, 49 types)
7. **Add CI check** — Added `.github/workflows/ci.yml` with type drift detection

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking Issue] Upgraded ts-rs from v10 to v12**
- **Found during:** Task 1
- **Issue:** ts-rs v10 doesn't have the `export` feature mentioned in the plan. The `export` feature doesn't exist in any ts-rs version.
- **Fix:** Upgraded to ts-rs v12. In ts-rs, types are exported automatically during `cargo test` when `#[ts(export)]` attribute is present. No special feature flag is needed.
- **Files modified:** Cargo.toml
- **Commit:** e6cf6ed

**2. [Rule 3 - Blocking Issue] Added .cargo/config.toml for export directory**
- **Found during:** Task 6
- **Issue:** Types were exported to `./bindings/frontend/src/types/generated.ts` instead of `frontend/src/types/generated.ts` because `TS_RS_EXPORT_DIR` defaults to `./bindings`.
- **Fix:** Created `.cargo/config.toml` with `TS_RS_EXPORT_DIR = ""` to export directly to project root, allowing `export_to = "frontend/src/types/generated.ts"` to work correctly.
- **Files modified:** .cargo/config.toml (new)
- **Commit:** e6cf6ed

### None - plan executed exactly as written.

## Output

### Generated Types

The file `frontend/src/types/generated.ts` contains 49 TypeScript type definitions matching the backend DTOs:

- **Inventory (14 types):** CategoryResponse, CreateCategoryRequest, MaterialResponse, CreateMaterialRequest, WithdrawRequest, AdjustStockRequest, ListMaterialsQuery, QrCodeResponse, QrSvgResponse, OrderRequestResponse, CreateOrderRequestDto, ApproveOrderRequestDto, FulfillOrderRequestDto, OrderStatusQuery
- **Sites (12 types):** SiteResponse, CreateSiteRequest, UpdateSiteRequest, ListSitesQuery, AssignmentResponse, AssignUserRequest, TimeEntryResponse, CreateTimeEntryRequest, ActivityResponse, CreateActivityRequest, ActivityQuery, DashboardSiteResponse
- **Fleet (19 types):** VehicleResponse, CreateVehicleRequest, UpdateVehicleRequest, ListVehiclesQuery, ToolResponse, CreateToolRequest, UpdateToolRequest, ListToolsQuery, ReservationResponse, CreateReservationRequest, UpdateReservationRequest, ListReservationsQuery, CalendarResponse, CalendarEntryResponse, ReservationSummaryResponse, CalendarQuery, AvailabilityResponse, AvailabilityQuery, QrStatusResponse
- **IAM (4 types):** UserResponse, InviteUserRequest, UpdateRoleRequest, UpdateProfileRequest

### CI Integration

The CI workflow will fail if:
1. Generated types differ from committed version (type drift detected)
2. Tests fail
3. Code formatting issues
4. Clippy warnings

## Verification

- [x] `cargo check` passes
- [x] `cargo test` passes (165 tests)
- [x] `frontend/src/types/generated.ts` exists with 111 lines
- [x] All 49 DTOs have `#[ts(export)]` attribute
- [x] CI workflow includes type drift check

## Commits

| Commit | Message |
|--------|---------|
| 5f29a54 | feat(15-01): add CI workflow with type drift check |
| e6cf6ed | feat(15-01): generate TypeScript types from Rust DTOs |
| 73a3ad3 | feat(15-01): add ts-rs derive macros to IAM DTOs |
| ef516ef | feat(15-01): add ts-rs derive macros to fleet DTOs |
| dd9b1e4 | feat(15-01): add ts-rs derive macros to sites DTOs |
| 69c1572 | feat(15-01): add ts-rs derive macros to inventory DTOs |
| 062f6fb | feat(15-01): add ts-rs dependency for TypeScript type generation |

## Known Stubs

None — all types are fully generated from backend DTOs.

## Threat Flags

None — this phase only adds type generation metadata to existing DTOs. No new network endpoints, auth paths, or trust boundaries were introduced.

## Self-Check: PASSED

- [x] All files exist at expected paths
- [x] All commits exist in git log
- [x] Verification criteria met
