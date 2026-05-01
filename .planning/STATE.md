---
gsd_state_version: 1.0
milestone: v1.9
milestone_name: Inventory Features
status: verifying
stopped_at: Phase 31 gap closure complete
last_updated: "2026-05-01T18:15:30.000Z"
last_activity: 2026-05-01
progress:
  total_phases: 4
  completed_phases: 4
  total_plans: 11
  completed_plans: 11
  percent: 100
---

# State: Schreinerei — v1.9 Inventory Features

**Updated:** 2026-05-01

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-01)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.
**Current focus:** v1.9 gap closure complete — ready for verification/transition

## Current Position

Phase: 33 (type-safety-coverage) — COMPLETE
Plan: 3 of 3
Status: Phase 31 gap closure complete — verification passed
Last activity: 2026-05-01

Progress: [██████████] 100%

## Performance Metrics

**Velocity:**

- Total plans completed: 11
- Completed this session: Phase 31 gap closure (1/1 plan)

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 30. Backend API Foundation | 2/2 | - | - |
| 31. Settings, Editing & Stock-In | 4/4 | - | - |
| 32. Enriched History | 2/2 | - | - |
| 33. Type Safety & Coverage | 3/3 | - | - |
| Phase 31 P01 | 10m | 3 tasks | 6 files |
| Phase 31 P02 | 8m | 3 tasks | 5 files |
| Phase 31 P03 | 7m | 3 tasks | 4 files |
| Phase 32 P01 | 3m | 2 tasks | 2 files |
| Phase 32 P02 | 4m | 2 tasks | 3 files |
| Phase 33 P01 | 12m | 2 tasks | 4 files |
| Phase 33 P02 | 14m | 2 tasks | 3 files |
| Phase 33 P03 | 40m | 2 tasks | 5 files |
| Phase 31 P04 | 2m | 2 tasks | 3 files |

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
- [Phase 33]: Inventory keeps stable local type names through a thin facade while `generated.ts` becomes the only DTO source of truth. — Frontend/backend drift is removed without churn in UI imports.
- [Phase 33]: Route-layer tests now pin inventory PATCH semantics and enriched-history conversion fields directly. — Backend validation covers the exact contract the frontend consumes.
- [Phase 33]: Inventory Playwright checks verify persisted API state after UI actions and start fresh local servers. — Browser coverage now exercises the current workspace instead of stale shared processes.
- [Phase 31]: Known category-delete backend conflicts are translated to the fixed German row copy before rendering. — The settings UI no longer leaks backend FK error strings to users.
- [Phase 31]: Inventory MSW handlers now use wildcard `/api/v1` route patterns. — Settings and overview regressions cover the app's absolute API base in Vitest.

### Pending Todos

None.

### Blockers/Concerns

- Metadata-only history audit model is still unresolved and intentionally deferred
- Metadata-only history audit model is still unresolved and intentionally deferred
- Real PostgreSQL integration coverage remains intentionally deferred to v2

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| History model | `location_changed` / `min_quantity_changed` audit entries need a separate model, not `stock_entries` | Deferred | v1.9 / Phase 30 |
| Offline | Photo queue replay | Backlog | v1.8 |
| Testing | Integration tests with real PostgreSQL | v2.0 | v1.5 |

## Session Continuity

Last session: 2026-05-01T18:15:30.000Z
Stopped at: Phase 31 gap closure complete
Resume file: None
