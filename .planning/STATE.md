# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-30)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.
**Current focus:** v1.7 gap closure complete; ready for phase verification/transition

## Current Position

Phase: 25 of 25 (Deduction History Site Name End-to-End Wiring)
Plan: 1 plan completed
Status: Phase complete
Last activity: 2026-05-01 — Executed 25-01 plan for DEDU-03 frontend wiring and tests

Progress: [██████████] 100% (v1.7 milestone and gap-closure phases complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 11 (v1.7 milestone + gap closure)
- Average duration: ~5 minutes
- Total execution time: ~55 minutes

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 22. Backend Foundation | 4 | 4 | ~3 min |
| 23. Frontend UI & Auto-Assignment | 4 | 4 | ~5 min |
| 24. Verification & Revalidation | 2 | 2 | ~9 min |
| 25. DEDU-03 Frontend Wiring | 1 | 1 | ~2 min |

**Recent Trend:**
- Phase 22 completed efficiently with parallel Wave 1 execution

*Updated after each plan completion*

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- v1.7 planning: Active Baustelle is user-scoped (not global)
- v1.7 planning: Deterministic hash-based colors (no manual selection)
- 23-04: Preferences endpoints now resolve tenant-local users.id before writes (FK-safe)
- 22-01: UserPreferences stores active_site_id as Option<String>
- 22-02: site_id is optional in WithdrawMaterial (backward compatible)
- 22-03: PATCH /preferences with null clears active site
- 22-04: LEFT JOIN for site names in stock history
- 24-01: Requirement verification uses explicit evidence pointers per REQ row
- 24-02: Phase 24 revalidation verdict includes complete pass/partial/missing totals
- 25-01: Inventory detail consumes stock history and conditionally renders site_name with regression tests

### Pending Todos

No pending v1.7 todos.

### Blockers/Concerns

- None currently blocking execution.

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

Last session: 2026-05-01
Stopped at: Completed 25-01-PLAN.md
Resume file: .planning/phases/25-deduction-history-site-name-end-to-end-wiring/25-01-SUMMARY.md
