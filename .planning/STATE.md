# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-30)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.
**Current focus:** v1.7 Active Project Context

## Current Position

Phase: 23 of 24 (Frontend UI & Auto-Assignment)
Plan: — of —
Status: Ready to plan
Last activity: 2026-04-30 — Phase 22 complete, 4/4 plans

Progress: [████░░░░░░] 33% (v1.7 milestone)

## Performance Metrics

**Velocity:**
- Total plans completed: 4 (v1.7 milestone)
- Average duration: ~3 minutes
- Total execution time: ~12 minutes

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 22. Backend Foundation | 4 | 4 | ~3 min |
| 23. Frontend UI & Auto-Assignment | 0 | TBD | — |
| 24. Opt-Out Dialog & E2E Tests | 0 | TBD | — |

**Recent Trend:**
- Phase 22 completed efficiently with parallel Wave 1 execution

*Updated after each plan completion*

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- v1.7 planning: Active Baustelle is user-scoped (not global)
- v1.7 planning: Deterministic hash-based colors (no manual selection)
- v1.7 planning: Opt-out dialog with 5-second auto-confirm
- 22-01: UserPreferences stores active_site_id as Option<String>
- 22-02: site_id is optional in WithdrawMaterial (backward compatible)
- 22-03: PATCH /preferences with null clears active site
- 22-04: LEFT JOIN for site names in stock history

### Pending Todos

None yet.

### Blockers/Concerns

None yet.

## Deferred Items

Items acknowledged and carried forward from v1.6 close:

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| Offline | Active preference in IndexedDB for offline access | v2.0 scope | 2026-04-30 |
| Offline | Preference sync when connectivity restored | v2.0 scope | 2026-04-30 |
| Offline | Stale active project handling during offline | v2.0 scope | 2026-04-30 |
| Dashboard | Filter by active Baustelle | v2.0 scope | 2026-04-30 |
| Dashboard | Active Baustelle statistics | v2.0 scope | 2026-04-30 |

## Session Continuity

Last session: 2026-04-30
Stopped at: Phase 22 complete, ready to plan Phase 23
Resume file: None
