---
phase: 30-backend-api-foundation
plan: 02
subsystem: api
tags: [inventory-api, patch-routes, stock-in, enriched-history, ts-rs]

# Dependency graph
requires:
  - plan: 30-01
    provides: Domain commands, repository methods, service methods, EntryType and enriched history models
provides:
  - PATCH/DELETE category endpoints
  - PATCH material endpoint
  - POST stock-in endpoint
  - GET enriched material history endpoint
  - Frontend request/history types for the new endpoints
  - Regenerated ts-rs bindings in frontend/src/types/generated.ts
affects: [31-settings-editing-stock-in, 32-enriched-history]

# Tech tracking
tech-stack:
  added: []
  patterns: [axum-multi-method-routes, tenant-scoped-handler-delegation, ts-rs-generated-dto-bindings]

key-files:
  created: []
  modified:
    - src/modules/inventory/api/routes.rs
    - frontend/src/types/inventory.ts
    - frontend/src/types/generated.ts

key-decisions:
  - "New edit and stock-in endpoints reuse existing AuthenticatedUser plus TenantContext extraction pattern"
  - "Enriched history gets a dedicated /history/enriched endpoint instead of changing the existing history contract"
  - "Frontend keeps a manual EntryType union in inventory.ts while generated.ts stays backend-derived"

requirements-completed: []

# Metrics
completed: 2026-05-01
---

# Phase 30: Backend API Foundation Summary

**Inventory API endpoints and TypeScript bindings for category editing, material editing, stock-in, and enriched history**

## Accomplishments
- Added `PATCH /api/v1/inventory/categories/{id}` and `DELETE /api/v1/inventory/categories/{id}` handlers wired to `InventoryService`
- Added `PATCH /api/v1/inventory/materials/{id}` and `POST /api/v1/inventory/materials/{id}/stock-in`
- Added `GET /api/v1/inventory/materials/{id}/history/enriched` returning `entry_type`, `user_name`, and `category_name`
- Added request/response DTOs with `ts-rs` exports for the new handlers
- Added matching frontend inventory request/history types and regenerated `frontend/src/types/generated.ts`

## Files Modified
- `src/modules/inventory/api/routes.rs`
- `frontend/src/types/inventory.ts`
- `frontend/src/types/generated.ts`

## Verification
- `cargo check`
- `cargo test`

## Deviations from Plan
- Used `cargo test` for ts-rs generation because this repo's current `ts-rs` version no longer supports the planned `--features ts-rs/export` flag

## Issues Encountered
- The original Wave 2 executor returned without producing commits or a summary, so this plan was completed inline from the orchestrator

## Next Phase Readiness
- Phase 31 can now consume the new inventory edit, stock-in, and enriched history endpoints
- Generated bindings now include `UpdateCategoryRequest`, `UpdateMaterialRequest`, `StockInRequest`, and `EnrichedStockHistoryResponse`

## Self-Check: PASSED

- New handlers are registered in the router: FOUND
- Generated types include new inventory DTOs: FOUND
- `cargo check` passes: YES
- `cargo test` passes: YES
