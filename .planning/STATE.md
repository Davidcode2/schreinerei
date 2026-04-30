# Project State: Schreinerei SaaS

## Project Reference

**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.

**Current Focus:** v1.5 Testing & Quality Foundation — Frontend test infrastructure complete

## Current Position

**Phase:** 14 — Frontend Test Infrastructure
**Status:** Complete
**Progress:** ██████████ 100%

```
Phase 12 [██████████] Complete ✓
Phase 13 [██████████] Complete ✓
Phase 14 [██████████] Complete ✓
Phase 15 [░░░░░░░░░░] Not started
Phase 16 [░░░░░░░░░░] Not started
Phase 17 [░░░░░░░░░░] Not started
```

## Performance Metrics

| Metric | Value |
|--------|-------|
| Milestone | v1.5 |
| Phases Complete | 3/6 |
| Requirements Complete | 10/21 |
| Days in Milestone | 0 |
| Tests Added | 116 (backend) + 28 (frontend) |

## Accumulated Context

### Decisions
- Phase numbering continues from Phase 12 (v1.4 ended at Phase 11)
- ts-rs chosen for type generation (prevents frontend-backend drift)
- Domain tests inline in domain files (zero mocking, fast feedback)
- E2E tests verify data persistence, not just UI presence
- Tests use helper functions for test fixtures to reduce boilerplate
- State machines tested with exhaustive transition tables
- Frontend must run on port 5175 for local testing (Keycloak config)
- QA Playbook documents validation procedures for future agents
- Vitest over Jest for frontend testing (native Vite integration)
- MSW for API mocking at network level (no axios mocking)
- Test factories generate consistent data with unique IDs

### Active Patterns
- Backend: Hexagonal architecture (domain → application → infrastructure)
- Frontend: React + Vite PWA with offline support
- Auth: Keycloak with organization claim for multi-tenancy
- Testing: Rust built-in tests, Vitest, Playwright

### Blockers
- None

## Session Continuity

**Last Session:** 2026-04-30 — Phase 14 complete
**Next Action:** `/gsd-plan-phase 15`

### Quick Context
- Project: Schreinerei SaaS (multi-tenant construction site management)
- Stack: Rust/Axum/SQLx + Vite/React/PWA + Keycloak
- LOC: ~8,900 Rust + ~8,000 TypeScript
- Previous milestones: v1.0-v1.4 shipped (37+ requirements complete)

### Key Files
| File | Purpose |
|------|---------|
| `.planning/PROJECT.md` | Project vision and context |
| `.planning/REQUIREMENTS.md` | v1.5 requirements (21 total) |
| `.planning/ROADMAP.md` | Phase structure (Phase 12-17) |
| `.planning/STATE.md` | This file |

---

*State initialized: 2026-04-30*
*Phase 12 completed: 2026-04-30*
*Phase 13 completed: 2026-04-30*
*Phase 14 completed: 2026-04-30*
