---
phase: 22-backend-foundation-user-preferences
plan: 04
status: complete
duration: 2m
requirements: [DEDU-03]
---

# Plan 22-04: Deduction History with Baustelle Name

## Objective
Add deduction history endpoint that includes Baustelle name when a deduction is linked to a site.

## Summary

Created stock entry history API endpoint that LEFT JOINs sites table to include site names for material withdrawals.

### Changes Made

**Domain Types** (`src/modules/inventory/domain/stock_entry.rs`):
- Created `StockEntry` struct with all stock entry fields
- Created `StockEntryWithSite` struct with optional `site_name` field
- Added helper methods `is_withdrawal()` and `withdrawn_quantity()`
- Added 6 unit tests for stock entry functionality

**Repository** (`src/modules/inventory/infrastructure/material_repository.rs`):
- Added `list_stock_entries_with_site()` method
- LEFT JOINs sites table on site_id to resolve site names
- Returns `StockEntryWithSite` with populated `site_name`
- Created `StockEntryRow` struct for database mapping

**API** (`src/modules/inventory/api/routes.rs`):
- Added GET `/api/v1/inventory/materials/{id}/history` endpoint
- Created `StockEntryResponse` DTO with ts-rs export
- Response includes `site_id` and `site_name` fields
- Returns last 50 stock entries for a material

### Key Files Created/Modified

| File | Change |
|------|--------|
| `src/modules/inventory/domain/stock_entry.rs` | Created - StockEntry and StockEntryWithSite types |
| `src/modules/inventory/domain.rs` | Added `pub mod stock_entry;` |
| `src/modules/inventory/infrastructure/material_repository.rs` | Added `list_stock_entries_with_site` method |
| `src/modules/inventory/api/routes.rs` | Added history endpoint and StockEntryResponse DTO |

### Verification

- `cargo check` passes
- 6 new unit tests for StockEntry domain type
- TypeScript types auto-generated via ts-rs
- LEFT JOIN correctly resolves site names for tenant-scoped entries

### Success Criteria Met

- [x] User can GET /api/v1/inventory/materials/{id}/history
- [x] Response includes `site_name` when deduction linked to Baustelle
- [x] Response includes `null` site_name when not linked
- [x] TypeScript types generated for frontend

## Self-Check: PASSED

- All tasks executed
- Each task committed individually  
- Code compiles without errors
- Tests pass
