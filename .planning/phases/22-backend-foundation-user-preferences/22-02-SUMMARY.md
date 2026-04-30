---
phase: 22-backend-foundation-user-preferences
plan: 02
subsystem: inventory
tags: [material-withdrawal, site-linking, backend, api]
dependency_graph:
  requires: []
  provides: [site_id in material withdrawals]
  affects: [frontend material withdrawal flow]
tech-stack:
  added: []
  patterns: [optional site_id parameter, FK to sites table]
key-files:
  created: []
  modified:
    - src/modules/inventory/domain/material.rs
    - src/modules/inventory/infrastructure/material_repository.rs
    - src/modules/inventory/application/inventory_service.rs
    - src/modules/inventory/api/routes.rs
    - frontend/src/types/generated.ts
decisions: []
metrics:
  duration: 173s
  completed_date: 2026-04-30T15:21:13Z
  task_count: 4
  file_count: 5
---

# Phase 22 Plan 02: Add site_id to Material Withdrawal Summary

## One-Liner

Added optional site_id parameter to material withdrawal flow, enabling deductions to be linked to a Baustelle for the active project context feature.

## Objective Met

Material withdrawals can now be associated with a construction site via the optional `site_id` parameter. The withdrawal command, repository, service, and API all support this new field, and TypeScript types have been regenerated.

## Completed Tasks

| Task | Name | Commit | Files |
| ---- | ---- | ------ | ----- |
| 1 | Add site_id to WithdrawMaterial command | 27396a2 | src/modules/inventory/domain/material.rs, src/modules/inventory/api/routes.rs |
| 2 | Update withdraw_stock to store site_id | 1e01ebe | src/modules/inventory/infrastructure/material_repository.rs |
| 3 | Update InventoryService to pass site_id | 624ac7d | src/modules/inventory/application/inventory_service.rs |
| 4 | Update API to accept site_id in WithdrawRequest | 9c087cd | src/modules/inventory/api/routes.rs |

## Verification

All acceptance criteria met:

- [x] WithdrawMaterial command has `site_id: Option<SiteId>` field
- [x] withdraw_stock repository method accepts and stores site_id
- [x] InventoryService passes site_id from command to repository
- [x] WithdrawRequest API DTO includes `site_id: Option<String>`
- [x] TypeScript type generated: `{ quantity: number, notes: string | null, site_id: string | null }`
- [x] All 176 tests pass (171 unit + 5 integration)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Tasks 1-4 were tightly coupled**

- **Found during:** Task 1 verification
- **Issue:** The plan's sequential task structure caused compilation failures. Task 1 (domain) modified WithdrawMaterial, which broke Task 4's code (API routes). Task 2 (repository) modified withdraw_stock signature, which broke Task 3's code (service).
- **Fix:** Applied minimal fixes to downstream files to maintain compilation, then completed full implementation in subsequent tasks. Committed each task individually with proper task boundaries.
- **Files modified:** All planned files
- **Commits:** 27396a2, 1e01ebe, 624ac7d, 9c087cd

## Key Changes

### Domain Layer (material.rs)
- Added `site_id: Option<SiteId>` to `WithdrawMaterial` command
- Updated 3 test cases to include `site_id: None`

### Infrastructure Layer (material_repository.rs)
- Added `SiteId` to imports
- Added `site_id: Option<SiteId>` parameter to `withdraw_stock` method
- Updated INSERT statement to include `site_id` column in `stock_entries`

### Application Layer (inventory_service.rs)
- Pass `withdraw.site_id` to repository's `withdraw_stock` call

### API Layer (routes.rs)
- Added `SiteId` to imports
- Added `site_id: Option<String>` to `WithdrawRequest` DTO
- Parse and validate site_id UUID in handler
- Pass parsed `site_id` to `WithdrawMaterial` command

## TypeScript Types

Generated `WithdrawRequest` type now includes:
```typescript
export type WithdrawRequest = { quantity: number, notes: string | null, site_id: string | null, };
```

## Threat Model Compliance

| Threat ID | Disposition | Status |
|-----------|-------------|--------|
| T-22-02-01 | accept | Site_id stored without tenant validation (Phase 23 frontend will only send valid site_ids) |
| T-22-02-02 | accept | FK constraint ensures valid site_id; ON DELETE SET NULL handles site deletion |

## Known Stubs

None. All functionality is complete and wired.

## Self-Check: PASSED

- [x] All created files exist
- [x] All commits exist in git log
- [x] TypeScript types generated correctly
