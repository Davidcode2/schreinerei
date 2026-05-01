# Domain Pitfalls

**Domain:** Carpentry SaaS — Inventory v1.9
**Researched:** 2026-05-01

## Critical Pitfalls

Mistakes that cause rewrites or major issues.

### Pitfall 1: Category Delete Breaks Foreign Key Constraint

**What goes wrong:** Deleting a category that has materials referencing it causes a PostgreSQL FK constraint violation (`categories.id → materials.category_id ON DELETE RESTRICT`). The `ON DELETE RESTRICT` in migration 002 explicitly prevents this.
**Why it happens:** Developers might add a DELETE endpoint without checking for material dependencies.
**Consequences:** 500 error at runtime, user sees nothing, no graceful handling.
**Prevention:** In `delete_category` service method, check `COUNT(*) FROM materials WHERE category_id = ? AND deleted_at IS NULL` before deleting. Return `AppError::Conflict` with a clear message like "Cannot delete category: 5 materials are assigned to it. Reassign them first." This mirrors the existing `delete_material` pattern that checks for pending order requests.
**Detection:** E2E test that tries to delete a category with materials assigned.

### Pitfall 2: Stock Entry Type Migration Backfill

**What goes wrong:** Adding `entry_type` column with `NOT NULL` and no default will fail if existing rows can't be inferred. Setting a wrong default (e.g., all entries become "withdrawal") corrupts history display.
**Why it happens:** The `stock_entries` table has no type column today. All entries are either withdrawals (negative `quantity_change`) or adjustments (positive `quantity_change` from admin).
**Consequences:** History page shows all entries as the same type, or migration fails on large tables.
**Prevention:** Use a three-step migration: (1) Create ENUM type, (2) Add nullable `entry_type` column, (3) Backfill with `UPDATE stock_entries SET entry_type = 'withdrawal' WHERE quantity_change < 0; UPDATE stock_entries SET entry_type = 'adjustment' WHERE quantity_change > 0;`, (4) Alter column to `NOT NULL DEFAULT 'withdrawal'`. **Note:** `location_change` entries won't exist yet — those come from the new `UpdateMaterial` feature and will have no `stock_entries` row (they're metadata-only). Actually, location changes should NOT create stock_entries at all — they should create `domain_events` only. Reconsider whether location changes need a `stock_entries` row.
**Detection:** Manual migration test on staging database.

### Pitfall 3: Stock-In Skips Negative Quantity Validation

**What goes wrong:** Creating a `StockIn` command that reuses `AdjustStock` without proper validation allows negative stock-in quantities, which would incorrectly create "addition" entries that actually reduce stock.
**Why it happens:** Copy-pasting `AdjustStock` validation which allows negative `quantity_change` (for corrections).
**Consequences:** Data corruption — stock levels go wrong, history shows incorrect types.
**Prevention:** `StockIn::validate()` must enforce `quantity > 0`. This is different from `AdjustStock` which allows negative quantities. Use a separate domain command, not a shared one.
**Detection:** Unit test: `StockIn { quantity: -1 }` must fail validation.

## Moderate Pitfalls

### Pitfall 4: Material PATCH Endpoint Accepts Unintended Fields

**What goes wrong:** Creating a `PATCH /api/v1/inventory/materials/{id}` that accepts ALL material fields in the body, including `quantity`, `category_id`, `name`, etc. Users could change fields that shouldn't be editable through this endpoint.
**Prevention:** Create a separate `UpdateMaterialRequest` DTO with only the fields that should be editable: `location: Option<String>`, `min_quantity: Option<i32>`. All other fields are ignored. This follows the existing pattern where `WithdrawRequest` and `AdjustStockRequest` are separate DTOs.

### Pitfall 5: Category Name Uniqueness Constraint on Update

**What goes wrong:** The database has a `UNIQUE(tenant_id, name)` constraint on `categories`. When updating a category name, the update will fail if another category in the same tenant already has that name — but the error message from PostgreSQL is generic ("duplicate key value violates unique constraint").
**Prevention:** Check uniqueness before updating in the service layer, returning a friendly `AppError::Validation("Category with this name already exists")`. This mirrors the existing error handling in `create_category`.

### Pitfall 6: React Query Cache Not Invalidated After Settings Changes

**What goes wrong:** After updating or deleting a category on the settings page, the inventory list page still shows the old category name because `useCategories()` has a 30-second stale time and the mutation's `onSuccess` doesn't invalidate all related queries.
**Prevention:** Every mutation that changes categories must invalidate `["categories"]` AND `["materials"]` (since material responses include `category_id` but the list may need re-fetching). Follow the existing pattern in `useCreateCategory` which invalidates `["categories"]`.

### Pitfall 7: MaterialResponse Missing category_name Causes Frontend Duplication

**What goes wrong:** If the backend returns `category_id` but not `category_name` in material listing, the frontend must make N+1 API calls (one per material) to resolve category names, or maintain a separate category map.
**Prevention:** Add `category_name` to `MaterialResponse` by JOINing `categories` in the material listing query. This is a common pattern already used in `SiteStockHistoryRow` which JOINs `categories c ON m.category_id = c.id`.

## Minor Pitfalls

### Pitfall 8: ts-rs Type Drift

**What goes wrong:** New backend DTOs are added but `cargo test --features ts-rs/export` isn't run, so `generated.ts` is out of date.
**Prevention:** Add `#[ts(export, export_to = "frontend/src/types/generated.ts")]` to every new DTO. Run ts-rs generation as part of CI or before each commit. Make it fail if generated types differ from committed.

### Pitfall 9: Enum String Matching in PostgreSQL and Rust

**What goes wrong:** The `stock_entry_type` enum values in PostgreSQL (`'withdrawal'`, `'addition'`, `'adjustment'`, `'location_change'`) must exactly match what Rust's `FromStr` impl produces. A typo or casing difference breaks serialization.
**Prevention:** Use Rust's `Display` impl to produce the exact strings that match the PostgreSQL ENUM values. Test with `SELECT * FROM stock_entries WHERE entry_type = 'withdrawal'` after inserting from Rust.

### Pitfall 10: History Entry without Quantity Change

**What goes wrong:** Trying to create a `stock_entries` row for a location change with `quantity_change = 0` and `quantity_after = same_value`. This inflates the audit table with entries that aren't stock changes.
**Prevention:** Location changes should be tracked via `domain_events` only (as `MaterialLocationChanged`), NOT as `stock_entries`. Only actual stock changes (withdrawal, addition, adjustment) belong in `stock_entries`. The history feed on the frontend merges data from both `stock_entries` and `domain_events` for that material.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Category CRUD API | FK violation on delete with assigned materials | Check material count before delete, return Conflict |
| Stock-in domain command | Negative quantity in stock-in | Strict `validate()`: quantity must be > 0 |
| entry_type migration | Data loss or NULL constraint failure | Three-step migration with backfill |
| Material PATCH | Accepting unintended fields | Separate DTO with only editable fields |
| Settings page routing | Orphaned route without nav link | Add card in SettingsPage linking to /settings/inventory |
| History enrichment | N+1 queries for user names | JOIN users table in history query |
| ts-rs generation | Frontend/backend type drift | Run generation after DTO changes, CI check |
| Category name update | Unique constraint violation | Pre-check uniqueness, friendly error |

## Sources

- Codebase analysis: `migrations/002_inventory_schema.sql` — FK constraints, unique constraints
- Codebase analysis: `inventory/domain/material.rs` — validation patterns
- Codebase analysis: `inventory/application/inventory_service.rs` — `delete_material` constraint check pattern
- Codebase analysis: `inventory/infrastructure/material_repository.rs` — transaction patterns, error handling
- Codebase analysis: `common/events.rs` — EventType enum structure