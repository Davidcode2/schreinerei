---
phase: 30-backend-api-foundation
status: issues_found
review_depth: standard
files_reviewed: 11
findings:
  critical: 0
  warning: 2
  info: 1
  total: 3
reviewed_at: 2026-05-01
---

# Phase 30 Code Review

## Summary

Reviewed the Phase 30 inventory backend/API changes from both plan summaries. I found two behavioral issues in the newly added category/history flows, plus one misleading test that gives false confidence around delete-category semantics.

## Findings

### WR-001 Deleting a category also deletes historical material audit/order data

- Severity: Warning
- Files: `src/modules/inventory/infrastructure/material_repository.rs:147-205`, `src/modules/inventory/infrastructure/material_repository.rs:707-723`, `migrations/002_inventory_schema.sql:22-59`, `migrations/004_order_requests.sql:1-5`

`delete_category` first hard-deletes every soft-deleted material in the category (`DELETE FROM materials ... deleted_at IS NOT NULL`) before removing the category itself. In this schema, both `stock_entries.material_id` and `order_requests.material_id` are `ON DELETE CASCADE`, so category deletion now wipes the full audit trail and request history for every previously deleted material in that category.

Why this matters: the new delete endpoint is no longer just category cleanup. It becomes a destructive history purge for inventory movements and order requests, which is a surprising side effect for an admin action labeled as category deletion.

Suggested fix: do not hard-delete soft-deleted materials as part of category deletion. Either block category deletion while any material row exists, or introduce a category archival/reassignment flow that preserves historical material rows.

### WR-002 Enriched history never records location/min-quantity changes

- Severity: Warning
- Files: `src/modules/inventory/application/inventory_service.rs:265-308`, `src/modules/inventory/infrastructure/material_repository.rs:449-593`, `src/modules/inventory/infrastructure/material_repository.rs:601-633`, `src/modules/inventory/domain/stock_entry.rs:14-20`

Phase 30 added `EntryType::LocationChanged` and `EntryType::MinQuantityChanged`, and `update_material` emits domain events for those changes, but the enriched history endpoint reads exclusively from `stock_entries`. The repository only inserts stock entries for `withdrawn`, `adjusted`, and `material_added`, so location edits and min-quantity edits never appear in `/history/enriched`.

Why this matters: the new API contract implies a richer material history feed, but an important class of inventory edits is silently missing from that feed. Users will see incomplete history even though the backend already knows those changes happened.

Suggested fix: either persist non-stock metadata changes into `stock_entries` with the new entry types, or narrow the history contract so it only claims to expose stock movements. Right now the code and data model disagree.

### IN-001 The new delete-category test asserts SQL that production no longer runs

- Severity: Info
- Files: `src/modules/inventory/infrastructure/material_repository.rs:1211-1218`

The unit test still only asserts a hand-written SQL snippet, not the actual repository logic. It cannot catch the destructive delete path above and documents behavior indirectly instead of exercising the real query/repository flow.

Why this matters: this gives false confidence around the exact code path that now controls whether categories can be removed.

Suggested fix: make the test exercise the real query builder / repository behavior, or remove it until there is a database-backed test that can assert the intended semantics.

## Reviewed Files

- `migrations/014_entry_type_stock_entries.sql`
- `src/modules/inventory/domain/stock_entry.rs`
- `src/modules/inventory/domain/category.rs`
- `src/modules/inventory/domain/material.rs`
- `src/modules/inventory/domain/events.rs`
- `src/common/events.rs`
- `src/modules/inventory/infrastructure/material_repository.rs`
- `src/modules/inventory/application/inventory_service.rs`
- `src/modules/inventory/api/routes.rs`
- `frontend/src/types/inventory.ts`
- `frontend/src/types/generated.ts`
