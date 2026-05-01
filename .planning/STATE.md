---
gsd_state_version: 1.0
milestone: v1.9
milestone_name: Inventory Features
status: ready_to_plan
stopped_at: Completed 30-01-PLAN.md
last_updated: "2026-05-01T13:14:28.619Z"
last_activity: 2026-05-01
progress:
  total_phases: 4
  completed_phases: 1
  total_plans: 2
  completed_plans: 1
  percent: 25
---

# State: Schreinerei — v1.9 Inventory Features

**Updated:** 2026-05-01

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-01)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.
**Current focus:** Phase 30 — backend-api-foundation

## Current Position

Phase: 31
Plan: Not started
Status: Ready to plan
Last activity: 2026-05-01

Progress: [█████░░░░░] 50%

## Performance Metrics

**Velocity:**

- Total plans completed: 2
- This milestone: v1.9 just started

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 30. Backend API Foundation | 0/2 | - | - |
| 31. Settings, Editing & Stock-In | 0/? | - | - |
| 32. Enriched History | 0/? | - | - |
| 33. Type Safety & Coverage | 0/? | - | - |
| 30 | 2 | - | - |

*Updated after each plan completion*
| Phase 30-backend-api-foundation P01 | 9min | 4 tasks | 8 files |

## Accumulated Context

### Decisions

Recent decisions affecting current work:

- v1.9 scope: Category editing, material editing, stock-in, enriched history
- Settings wheel icon → dedicated `/settings/inventory` route for category management
- StockIn is a separate domain command (not reusing AdjustStock) — available to all users, uses notes instead of reason
- entry_type migration: three-step nullable → backfill → NOT NULL
- Category delete uses FK constraint check with Conflict error (no soft-delete)
- MaterialHistoryFeed is a separate component (not shared with sites ActivityFeed)
- [Phase ?]: ---

phase: 30-backend-api-foundation
plan: 01
subsystem: api
tags: [entry-type, domain-commands, stock-in, enriched-history, migration]

# Dependency graph

requires:

  - phase: v1.8
    provides: Existing inventory module with stock_entries, materials, categories
provides:

  - EntryType enum with 5 variants and Display/FromStr
  - EnrichedStockEntry with user_name, entry_type, category_name
  - UpdateCategory command with validation
  - UpdateMaterial command with clear_location support
  - StockIn command with positive quantity validation
  - MaterialAdded, LocationChanged, MinQuantityChanged event payloads
  - Repository methods: update_category, delete_category, update_material, stock_in, list_enriched_stock_entries
  - Service methods: update_category, delete_category, update_material, stock_in, list_enriched_history
  - Migration 014: entry_type column with backfill

affects: [31-settings-editing-stock-in, 32-enriched-history]

# Tech tracking

tech-stack:
  added: [sqlx-VARCHAR-enum-for-EntryType]
  patterns: [PATCH-semantics-with-COALESCE, clear_location-boolean-flag, entry_type-enriched-history]

key-files:
  created:

    - migrations/014_entry_type_stock_entries.sql
  modified:

    - src/modules/inventory/domain/stock_entry.rs
    - src/modules/inventory/domain/category.rs
    - src/modules/inventory/domain/material.rs
    - src/modules/inventory/domain/events.rs
    - src/common/events.rs
    - src/modules/inventory/infrastructure/material_repository.rs
    - src/modules/inventory/application/inventory_service.rs

key-decisions:

  - "StockIn is separate from AdjustStock — available to all users, uses notes field instead of reason"
  - "UpdateMaterial uses clear_location boolean flag for explicit NULL setting, avoiding Option<Option<String>> JSON confusion"
  - "Category delete uses FK constraint check returning Conflict error, not soft-delete"
  - "entry_type migration uses 3-step nullable→backfill→NOT NULL approach for zero-downtime"
  - "EnrichedStockEntry resolves user_name via COALESCE(u.name, u.email, user_id::text)"

patterns-established:

  - "PATCH semantics with COALESCE for partial updates where None = don't change"
  - "EntryType enum maps to VARCHAR via sqlx with snake_case rename"
  - "Domain commands validate() returning Result<(), String> before repo delegation"
  - "Service layer emits domain events after successful repo operations"

requirements-completed: []

# Metrics

duration: 9min
completed: 2026-05-01
---

# Phase 30: Backend API Foundation Summary

**EntryType enum, domain commands (UpdateCategory, UpdateMaterial, StockIn), enriched history, and migration for Phase 31-32 API endpoints**

## Performance

- **Duration:** 9 min
- **Started:** 2026-05-01T13:03:18Z
- **Completed:** 2026-05-01T13:12:35Z
- **Tasks:** 4
- **Files modified:** 8

## Accomplishments

- Created migration 014 with entry_type VARCHAR(20) column, backfill logic, NOT NULL constraint, and index
- Added EntryType enum with 5 variants (Withdrawn, Adjusted, MaterialAdded, LocationChanged, MinQuantityChanged) with Display/FromStr
- Added EnrichedStockEntry struct with user_name, entry_type, category_name for rich history display
- Implemented UpdateCategory command with PATCH semantics (name/description optional, validation for non-empty name)
- Implemented UpdateMaterial command with clear_location boolean flag to handle explicit NULL setting
- Implemented StockIn command with positive quantity validation, separate from AdjustStock per STATE.md decision
- Added MaterialAdded, LocationChanged, MinQuantityChanged event payloads to domain events
- Added EventType variants (MaterialAdded, LocationChanged, MinQuantityChanged) to common events
- Implemented repository methods: update_category, delete_category (with FK conflict check), update_material, stock_in, list_enriched_stock_entries
- Implemented service methods with admin-only guards for category/material operations, all-users access for stock_in
- Updated all stock_entries INSERT statements to include entry_type column

## Task Commits

Each task was committed atomically:

1. **task 1: Add entry_type migration and EntryType domain enum** - `29b0ace` (feat)
2. **task 2: Add UpdateCategory, UpdateMaterial commands and category delete check** - `8e29c1f` (feat)
3. **task 3: Add repository methods for category CRUD, material update, stock-in, and enriched history** - `1c0d234` (feat)
4. **task 4: Add service methods wiring domain commands to repository** - `a6bad06` (feat)

## Files Created/Modified

- `migrations/014_entry_type_stock_entries.sql` - Migration adding entry_type column with backfill and index
- `src/modules/inventory/domain/stock_entry.rs` - EntryType enum, EnrichedStockEntry struct, tests
- `src/modules/inventory/domain/category.rs` - UpdateCategory command with validation and tests
- `src/modules/inventory/domain/material.rs` - UpdateMaterial, StockIn commands with validation and tests
- `src/modules/inventory/domain/events.rs` - MaterialAddedPayload, LocationChangedPayload, MinQuantityChangedPayload
- `src/common/events.rs` - EventType variants: MaterialAdded, LocationChanged, MinQuantityChanged
- `src/modules/inventory/infrastructure/material_repository.rs` - Repository methods: update_category, delete_category, update_material, stock_in, list_enriched_stock_entries, EnrichedStockEntryRow
- `src/modules/inventory/application/inventory_service.rs` - Service methods: update_category, delete_category, update_material, stock_in, list_enriched_history

## Decisions Made

- StockIn uses `notes: Option<String>` instead of AdjustStock's `reason: String` — available to all users, not admin-only
- UpdateMaterial uses `clear_location: Option<bool>` flag to handle explicit NULL setting, avoiding confusing `Option<Option<String>>` JSON
- Category delete performs FK constraint check (COUNT on materials) and returns Conflict if referenced
- EnrichedStockEntry resolves user_name via `COALESCE(u.name, u.email, se.user_id::text)` for robustness
- entry_type backfill uses heuristics: withdrawals with site_id → "withdrawn", other negatives → "adjusted", rest → "adjusted"

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Backend domain foundation complete with all commands, validation, repository, and service methods
- Ready for Plan 02 (API endpoints) which will expose these operations via REST routes
- Migration 014 needs to be run against the database before deploying
- All 215 library tests pass, cargo check passes with zero errors

---
*Phase: 30-backend-api-foundation*
*Completed: 2026-05-01*

## Self-Check: PASSED

- All 8 key files exist on disk: FOUND
- All 4 task commits present in git log: 29b0ace, 8e29c1f, 1c0d234, a6bad06
- All 215 library tests pass (0 failed)
- cargo check passes with 0 errors
- Migration 014 exists with entry_type column
- All domain commands have validate() methods (5 found)
- All stock_entries INSERT statements include entry_type column (3 inserts verified)
- EnrichedStockEntry has entry_type, user_name, category_name fields
- EventType enum has MaterialAdded, LocationChanged, MinQuantityChanged variants

### Pending Todos

None yet.

### Blockers/Concerns

- entry_type migration needs careful SQL review (nullable → backfill → NOT NULL)
- React Query cache invalidation must cover both ["categories"] and ["materials"] keys

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| Offline | Photo queue replay | Backlog | v1.8 |
| Testing | Integration tests with real PostgreSQL | v2.0 | v1.5 |

## Session Continuity

Last session: 2026-05-01T13:14:28.612Z
Stopped at: Completed 30-01-PLAN.md
Resume file: None
