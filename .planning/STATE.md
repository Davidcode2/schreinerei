---
gsd_state_version: 1.0
milestone: v1.11
milestone_name: Fleet Calendar on Fleet Page
status: planning
last_updated: "2026-05-01T14:27:59.685Z"
last_activity: 2026-05-01
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

# State: Schreinerei — v1.9 Inventory Features

**Updated:** 2026-05-01

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-01)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.
**Current focus:** Phase 31 — Settings, Editing & Stock-In

## Current Position

Phase: Not started (defining requirements)
Plan: —
Status: Defining requirements
Last activity: 2026-05-01 — Milestone v1.11 started

## Performance Metrics

**Velocity:**

- Total plans completed: 2
- Completed this session: Phase 30 (2/2 plans)

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 30. Backend API Foundation | 2/2 | - | - |
| 31. Settings, Editing & Stock-In | 0/? | - | - |
| 32. Enriched History | 0/? | - | - |
| 33. Type Safety & Coverage | 0/? | - | - |

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

### Pending Todos

None.

### Blockers/Concerns

- Metadata-only history audit model is still unresolved and intentionally deferred
- Phase 31 should consume the new PATCH/DELETE/stock-in/enriched-history endpoints without re-shaping their backend contracts

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| History model | `location_changed` / `min_quantity_changed` audit entries need a separate model, not `stock_entries` | Deferred | v1.9 / Phase 30 |
| Offline | Photo queue replay | Backlog | v1.8 |
| Testing | Integration tests with real PostgreSQL | v2.0 | v1.5 |

## Session Continuity

Last session: 2026-05-01
Stopped at: Phase 30 complete, Phase 31 ready to plan
Resume file: None
