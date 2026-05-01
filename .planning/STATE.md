---
gsd_state_version: 1.0
milestone: v1.9
milestone_name: Inventory Features
status: ready_to_plan
stopped_at: Phase 32 verified and completed
last_updated: "2026-05-01T15:30:20.054Z"
last_activity: 2026-05-01 -- Phase 32 verified and completed
progress:
  total_phases: 4
  completed_phases: 3
  total_plans: 7
  completed_plans: 7
  percent: 75
---

# State: Schreinerei — v1.9 Inventory Features

**Updated:** 2026-05-01

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-01)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.
**Current focus:** Phase 33 — type-safety-&-coverage

## Current Position

Phase: 33
Plan: Not started
Status: Ready to plan
Last activity: 2026-05-01

Progress: [███████░░░] 75%

## Performance Metrics

**Velocity:**

- Total plans completed: 7
- Completed this session: Phase 32 (2/2 plans)

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 30. Backend API Foundation | 2/2 | - | - |
| 31. Settings, Editing & Stock-In | 3/3 | - | - |
| 32. Enriched History | 2/2 | - | - |
| 33. Type Safety & Coverage | 0/? | - | - |
| Phase 31 P01 | 10m | 3 tasks | 6 files |
| Phase 31 P02 | 8m | 3 tasks | 5 files |
| Phase 31 P03 | 7m | 3 tasks | 4 files |
| Phase 32 P01 | 3m | 2 tasks | 2 files |
| Phase 32 P02 | 4m | 2 tasks | 3 files |

## Accumulated Context

### Decisions

Recent decisions affecting current work:

- v1.9 scope: Category editing, material editing, stock-in, enriched history
- Settings wheel icon → dedicated `/settings/inventory` route for category management
- StockIn is a separate domain command (not reusing AdjustStock) — available to all users, uses notes instead of reason
- entry_type migration: three-step nullable → backfill → NOT NULL
- Category delete preserves history: return Conflict while any material row exists, including soft-deleted rows
- Enriched history stays stock-movement-focused for now
- `location_changed` and `min_quantity_changed` history entries are deferred until a separate audit model exists
- MaterialHistoryFeed is a separate component (not shared with sites ActivityFeed)
- [Phase 31]: Shared inventory mutations now flow through dedicated React Query hooks. — Later dialogs can reuse exact backend contracts without page-local fetch logic.
- [Phase 31]: Inventory settings lives at /settings/inventory under the authenticated route tree. — The gear entrypoint leads to a dedicated management surface instead of a generic settings fallback.
- [Phase 31]: Blocked category deletes remain inline on the affected row. — Conflict feedback is explicit and survives dialog closure.
- [Phase 31]: Detail-page stock correction accepts a target quantity and converts it to an adjust delta. — The frontend stays aligned with the existing backend adjust endpoint.
- [Phase 32]: Enriched inventory history uses a dedicated React Query cache key and hook. — Legacy history payloads stay isolated from the richer UI contract.
- [Phase 32]: Inventory detail history renders through a dedicated MaterialHistoryFeed with isolated ErrorState retry handling. — The page keeps stock-movement UX separate from the sites activity feed.

### Pending Todos

None.

### Blockers/Concerns

- Metadata-only history audit model is still unresolved and intentionally deferred
- Phase 33 should keep enriched history coverage aligned with the shipped Phase 32 UI contract

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| History model | `location_changed` / `min_quantity_changed` audit entries need a separate model, not `stock_entries` | Deferred | v1.9 / Phase 30 |
| Offline | Photo queue replay | Backlog | v1.8 |
| Testing | Integration tests with real PostgreSQL | v2.0 | v1.5 |

## Session Continuity

Last session: 2026-05-01T15:30:20.047Z
Stopped at: Phase 32 verified and completed
Resume file: None
