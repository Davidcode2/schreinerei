# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-01)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.
**Current focus:** v1.7 shipped — ready for next milestone planning

## Current Position

Phase: 25 of 25 (Deduction History Site Name End-to-End Wiring)
Plan: Complete
Status: v1.7 shipped and archived
Last activity: 2026-05-01 — Archived v1.7 milestone

Progress: [██████████] 100% (v1.7 milestone complete and archived)

## Performance Metrics

**Velocity:**
- Total plans completed: 11 (v1.7 milestone)
- Average duration: ~5 minutes
- Total execution time: ~55 minutes

**By Phase:**

| Phase | Plans | Duration |
|-------|-------|----------|
| 22. Backend Foundation | 4 | ~12 min |
| 23. Frontend UI & Auto-Assignment | 4 | ~20 min |
| 24. Verification & Revalidation | 2 | ~18 min |
| 25. DEDU-03 Frontend Wiring | 1 | ~2 min |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.

Recent v1.7 decisions:
- Active Baustelle is user-scoped (not global)
- Deterministic hash-based colors (no manual selection)
- JSONB storage for preferences (flexible schema evolution)
- site_id is optional in WithdrawMaterial (backward compatible)
- FK-safe tenant-local user mapping for preferences API

### Pending Todos

None.

### Blockers/Concerns

None.

## Deferred Items

Items deferred to v2.0:

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| Offline | Active preference in IndexedDB for offline access | v2.0 scope | 2026-04-30 |
| Offline | Preference sync when connectivity restored | v2.0 scope | 2026-04-30 |
| Offline | Stale active project handling during offline | v2.0 scope | 2026-04-30 |
| Dashboard | Filter by active Baustelle | v2.0 scope | 2026-04-30 |
| Dashboard | Active Baustelle statistics | v2.0 scope | 2026-04-30 |

## Session Continuity

Last session: 2026-05-01
Stopped at: v1.7 archived, ready for next milestone
Resume file: None
