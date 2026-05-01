# Project Research Summary

**Project:** Schreinerei SaaS — Inventory v1.9
**Domain:** Carpentry SaaS — inventory management features
**Researched:** 2026-05-01
**Confidence:** HIGH

## Executive Summary

This is a mature Rust/Ajax/React SaaS for carpentry workshops, extending an already-shipped inventory module (v1.8) with category CRUD, material editing, stock-in, and enriched history. The v1.9 milestone is an incremental extension — no new modules, no new dependencies, no new infrastructure. Every feature has a direct precedent in the existing codebase: category CRUD follows the `UpdateSite`/`DeleteMaterial` patterns, material update uses PATCH + `Option<T>` partial updates, and stock-in mirrors the existing withdrawal flow with a positive `quantity_change`.

The recommended approach is four sequential phases: (1) backend foundation — domain commands, service methods, migration, and API endpoints; (2) frontend settings and actions — InventorySettingsPage, MaterialEditDialog, StockInDialog; (3) enriched history — MaterialHistoryFeed with color-coded entry types and clickable Baustelle links; (4) type generation and tests. All work stays within the existing `inventory` module and follows established hexagonal architecture patterns.

The primary risk is the `entry_type` migration on `stock_entries` — adding a NOT NULL enum column to existing rows requires careful backfill (withdrawals vs. adjustments). Secondary risks include FK constraint violations on category delete and React Query cache invalidation gaps. All risks have clear mitigations documented in the pitfall research.

## Key Findings

### Recommended Stack

No new technology needed. All patterns are validated in production v1.8 code. The only infrastructure change is one PostgreSQL migration adding a `stock_entry_type` ENUM column — well within existing migration patterns.

**Core technologies:**
- Rust/Axum/SQLx: Backend — already in use, hexagonal pattern proven over 29 phases
- React 18/Vite 6/TypeScript: Frontend — already in use, TanStack Query + shadcn/ui established
- PostgreSQL 15+: Database — `stock_entry_type` ENUM follows existing `Unit`/`SiteStatus` enum pattern
- ts-rs: Type generation — 49 DTOs already exported, add new DTOs to existing pipeline

### Expected Features

**Must have (table stakes):**
- Category CRUD (edit, delete) — admins need full category lifecycle management
- Edit material location and minimum quantity — single PATCH endpoint, single edit dialog
- Stock-in action ("Material einlagern") — closes the "only deductions exist" gap, available to all users
- History entry types (`entry_type` enum) — enables color-coded, typed history display
- User attribution in history ("von Max Mustermann") — accountability
- Category name on inventory overview — avoids N+1 queries

**Should have (differentiators):**
- Color-coded history entries — green/red/blue/amber badges per entry type
- Clickable Baustelle links in history — navigate from withdrawal to the construction site
- Inventory settings page — dedicated `/settings/inventory` route for category management

**Defer (v2+):**
- Batch stock-in / barcode scanning / reorder automation — single-item actions for v1.9
- Category hierarchy (nested categories) — flat categories match carpentry domain
- Real-time WebSocket history — polling sufficient for 5–20 users per org
- Soft-delete for categories — block-with-error is simpler and more explicit
- Separate audit table for categories — `domain_events` already captures all mutations

### Architecture Approach

All v1.9 features extend the existing `inventory` module. No new modules or bounded contexts. Follow the established domain → application → infrastructure → API hexagonal pattern. Key architectural decision: stock-in should use a separate domain command (`StockIn`) and repository method (`add_stock`) rather than reusing `AdjustStock`, because stock-in is available to all users (not admin-only) and uses `notes` instead of `reason`.

**Major components:**
1. `inventory::domain::material` — `StockIn` and `UpdateMaterial` commands with validation
2. `inventory::domain::category` — `UpdateCategory` and `DeleteCategory` commands with FK constraint check
3. `inventory::domain::stock_entry` — `StockEntryType` enum for history type discrimination
4. `frontend::MaterialHistoryFeed` — new component (not shared with sites `ActivityFeed`) for enriched history
5. `frontend::InventorySettingsPage` — new route `/settings/inventory` with `CategoryManager`
6. `frontend::StockInDialog` — mirrors existing `WithdrawDialog` pattern for positive quantity

### Critical Pitfalls

1. **Category delete FK violation** — Check material count before delete, return `AppError::Conflict` with message. Mirrors existing `delete_material` pattern.
2. **`entry_type` migration backfill** — Three-step migration: create ENUM, add nullable column, backfill from `quantity_change` sign, then set NOT NULL. No location_change rows in `stock_entries` — those go in `domain_events` only.
3. **Stock-in negative quantity** — `StockIn::validate()` must enforce `quantity > 0`. Separate domain command, not shared with `AdjustStock`.
4. **PATCH accepting unintended fields** — Create a separate `UpdateMaterialRequest` DTO with only `location` and `min_quantity` as `Option<T>` fields.
5. **React Query cache invalidation** — Category mutations must invalidate both `["categories"]` and `["materials"]` query keys.

## Implications for Roadmap

Based on the dependency graph from FEATURES.md and the build order from ARCHITECTURE.md, the natural phase structure is:

### Phase 1: Backend Foundation — Domain Commands + Migration + API
**Rationale:** All frontend features depend on API endpoints existing. Build the data layer and API first so frontend can consume it.
**Delivers:** Category CRUD endpoints, material PATCH endpoint, stock-in endpoint, `entry_type` migration, new event types (MaterialAdded, MaterialLocationChanged, CategoryUpdated, CategoryDeleted), enriched history response with `user_name`, `category_name`, `entry_type`.
**Addresses:** All 6 table-stakes features at the backend level
**Avoids:** Pitfall 1 (FK check on category delete), Pitfall 2 (migration backfill), Pitfall 3 (negative quantity validation), Pitfall 4 (PATCH field whitelist), Pitfall 7 (N+1 category_name query)

### Phase 2: Frontend — Settings Page + Material Edit + Stock-In Dialog
**Rationale:** Depends on Phase 1 APIs. These are the primary user-facing action surfaces.
**Delivers:** InventorySettingsPage with CategoryManager, MaterialEditDialog for location/min_quantity, StockInDialog mirroring WithdrawDialog pattern, category name display on inventory overview.
**Uses:** React 18, TanStack Query mutations, shadcn/ui Dialog components
**Implements:** Frontend halves of table-stakes features (category CRUD, material edit, stock-in)
**Avoids:** Pitfall 6 (React Query cache invalidation — handled in mutation `onSuccess`)

### Phase 3: Frontend — Enriched History Feed
**Rationale:** Depends on Phase 1's enriched history API (entry_type, user_name, site links). Visual differentiation is the differentiator value.
**Delivers:** MaterialHistoryFeed component with color-coded entry type badges, clickable Baustelle links (React Router `Link`), user attribution ("von {user_name}") in history entries.
**Implements:** All 3 differentiator features
**Avoids:** Anti-pattern 4 (sharing ActivityFeed component directly — build MaterialHistoryFeed with same visual pattern but different data shape)

### Phase 4: Type Generation + Tests
**Rationale:** Depends on all DTOs being finalized in Phase 1. Run ts-rs export, verify generated types, add backend validation tests, add E2E tests.
**Delivers:** ts-rs export for all new DTOs, unit tests for StockIn/UpdateMaterial/UpdateCategory/DeleteCategory validation, E2E tests for settings page, stock-in dialog, history enrichment.
**Avoids:** Pitfall 8 (ts-rs type drift), Pitfall 9 (enum string mismatch between Rust and PostgreSQL)

### Phase Ordering Rationale

- Phases 1→2→3 are strictly sequential: each depends on the previous phase's API or data
- Phase 4 can partially overlap Phase 3 (backend tests can run once Phase 1 is done) but ts-rs generation must wait until all DTOs are finalized
- Phase 1 is the critical path — everything else is blocked without it
- Grouping backend-first follows the established pattern in v1.0–v1.8 and prevents blocked frontend work

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 1:** The `entry_type` migration strategy needs careful SQL review. The three-step nullable → backfill → NOT NULL pattern should be planned explicitly.
- **Phase 3:** MaterialHistoryFeed component design — how to merge `stock_entries` and `domain_events` data for a unified timeline. The data shapes differ and need a frontend merge strategy.

Phases with standard patterns (skip research-phase):
- **Phase 2:** Settings page, edit dialog, and stock-in dialog all follow established patterns (UpdateSite, WithdrawDialog, SettingsPage card linking). No novel patterns.
- **Phase 4:** ts-rs generation and test patterns are well-established from v1.5.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Zero new dependencies. All technologies proven across 29 phases. |
| Features | HIGH | Every feature has a direct precedent in existing code. No novel UX patterns. |
| Architecture | HIGH | All codebase-verified. Hexagonal pattern established. Component boundaries clear. |
| Pitfalls | HIGH | All pitfalls derived from actual codebase analysis (migrations, FK constraints, repository patterns). |

**Overall confidence:** HIGH

### Gaps to Address

- **Location change history display:** Research is clear that location changes should use `domain_events` only, NOT `stock_entries`. The MaterialHistoryFeed will need to merge data from two sources. The exact UI merge strategy (unified timeline vs. separate sections) should be decided during Phase 3 planning.
- **`extracted_by` vs `user_name` field naming:** The existing `SiteStockHistoryRow` uses `extracted_by` for the user who withdrew material. The new stock-in operation needs an equivalent field. Naming convention should be consistent — decide during Phase 1 planning whether to use `performed_by` as a generic field or keep `extracted_by`/`added_by` as separate fields.
- **Offline conflict resolution for stock-in:** The existing offline sync handles withdrawals but hasn't been runtime-tested. Adding stock-in introduces a new mutation type that needs sync handling. Defer offline-specific testing to the backlog (Phase 999.1 already covers this).

## Sources

### Primary (HIGH confidence)
- Codebase analysis: `src/modules/inventory/` — all layers (domain, application, infrastructure, API) examined and patterns verified
- Codebase analysis: `migrations/002_inventory_schema.sql` — FK constraints, unique constraints, ENUM patterns
- Codebase analysis: `frontend/src/pages/inventory/` and `frontend/src/pages/sites/ActivityFeed.tsx` — UI patterns established
- Codebase analysis: `Cargo.toml` and `frontend/package.json` — all dependencies verified, no new packages needed

### Secondary (MEDIUM confidence)
- Codebase analysis: `src/common/events.rs` — EventType enum patterns for new event types
- Codebase analysis: `inventory/application/inventory_service.rs` — service method patterns (delete_material constraint check, adjust_stock admin-only pattern)
- Codebase analysis: `frontend/src/lib/api/hooks/useInventory.ts` — React Query mutation patterns for cache invalidation

---
*Research completed: 2026-05-01*
*Ready for roadmap: yes*