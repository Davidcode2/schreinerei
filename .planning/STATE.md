# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-30)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.
**Current focus:** v1.7 Active Project Context

## Current Position

Phase: 23 of 24 (Frontend UI & Auto-Assignment)
Plan: 04 of 04
Status: Ready to execute
Last activity: 2026-04-30 — Plans 23-01..23-03 completed, UAT found toggle FK gap, and 23-04 fix plan created

Progress: [███████░░░] 58% (v1.7 milestone)

## Performance Metrics

**Velocity:**
- Total plans completed: 7 (v1.7 milestone)
- Average duration: ~3 minutes
- Total execution time: ~21 minutes

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 22. Backend Foundation | 4 | 4 | ~3 min |
| 23. Frontend UI & Auto-Assignment | 3 | 4 | ~3 min |
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
- 23-UAT: Active-site toggle currently hits preferences FK mismatch path in backend (500)
- 23-04 plan added to resolve user mapping before preference writes
- 22-01: UserPreferences stores active_site_id as Option<String>
- 22-02: site_id is optional in WithdrawMaterial (backward compatible)
- 22-03: PATCH /preferences with null clears active site
- 22-04: LEFT JOIN for site names in stock history

### Pending Todos

1 pending UAT test in `23-UAT.md` (Time Entry conditional prefill).

### Blockers/Concerns

- Active-site toggle can return 500 due to preferences/user FK mapping mismatch; addressed by pending 23-04 backend fix plan.

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
Stopped at: Phase 23 in progress, ready to execute 23-04 and resume UAT
Resume file: None
