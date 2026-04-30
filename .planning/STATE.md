# Project State

**Project:** Schreinerei SaaS
**Current Milestone:** v1.4 Core Feature Fixes
**Status:** Not started

---

## Current Position

Phase: 11 — Core Feature Fixes
Plan: Not started
Status: Planned - 3 plans created in 2 waves
Last activity: 2026-04-29 — Phase 11 planned for FK constraint fixes

---

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-29)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.
**Current focus:** E2E testing infrastructure complete, config bugs discovered

---

## Accumulated Context

### Decisions Summary

Full decision log in PROJECT.md. Key decisions:
- Rust Backend with Axum 0.8, SQLx 0.8
- DDD layering: domain/application/infrastructure/api
- Multi-tenant via TenantId in all queries
- Keycloak OAuth2 PKCE for SPA auth
- IndexedDB (Dexie.js) for offline storage
- Keycloak Organizations for tenant isolation (v1.1)

### Roadmap Evolution

- Phase 8 added: Fix all frontend-accessible functionalities
- Phase 8 complete: Backend and frontend bugs resolved
- Phase 9 added: Comprehensive frontend testing with playwright-cli
- Phase 9 complete: 18 E2E tests, 2 bugs discovered
- Phase 10 added: Bug fixes for 8 bugs discovered in testing
- Phase 10 complete: All auth and UI bugs fixed
- Phase 11 added: Fix core features with FK constraint violations

### Pending Todos

Investigate and fix FK constraint violations in Phase 11:
- CORE-01: stock_entries_user_id_fkey violation on material withdrawal
- CORE-02: reservations_user_id_fkey violation on reservation creation
- CORE-03: Time entries POST returns 400
- CORE-04: Fleet calendar GET returns 400

### Blockers/Concerns

User ID foreign key constraints failing - likely the user_id being passed doesn't exist in users table or is a Keycloak ID that needs mapping.

---

## History

| Date | Event | Details |
|------|-------|---------|
| 2026-04-28 | Project Initialized | PROJECT.md, REQUIREMENTS.md, ROADMAP.md created |
| 2026-04-28 | Phase 1 Complete | Auth & IAM Foundation finished |
| 2026-04-28 | Phase 2 Complete | Inventory Management finished |
| 2026-04-28 | Phase 3 Complete | Baustellen Management finished |
| 2026-04-28 | Phase 4 Complete | Fuhrpark & Werkzeuge finished |
| 2026-04-29 | Phase 5 Complete | PWA & Mobile finished |
| 2026-04-29 | v1.0 Complete | MVP shipped |
| 2026-04-29 | v1.1 Complete | Organization-Based Tenancy shipped |
| 2026-04-29 | v1.2 Started | Frontend Polish milestone |
| 2026-04-29 | Phase 7 Planned | 3 plans created in 3 waves |
| 2026-04-29 | Phase 7 Complete | Material/Site/Fleet dialogs, User management, QR scanner |
| 2026-04-29 | Phase 8 Added | Bug fixes phase for end-to-end functionality |
| 2026-04-29 | Phase 8 Planned | 2 plans created for backend and frontend bug fixes |
| 2026-04-29 | Phase 8 Complete | Dashboard API, VehicleType, logout, category creation, reservation dates fixed |
| 2026-04-29 | Phase 9 Added | Comprehensive frontend testing with playwright-cli |
| 2026-04-29 | Phase 9 Complete | 18 E2E tests created, 2 config bugs discovered |
| 2026-04-29 | Phase 10 Added | Bug fixes for 8 bugs discovered in testing |
| 2026-04-29 | Phase 10 Planned | 4 plans created in 2 waves |
| 2026-04-29 | Phase 10 Complete | All 8 bugs fixed |
| 2026-04-29 | Phase 11 Added | Core feature FK constraint fixes |
| 2026-04-29 | Phase 11 Planned | 3 plans created for user ID resolution |

---

## Next Action

Plan Phase 11 to fix core feature FK constraint violations:

1. Run `/gsd-plan-phase 11` to create plans for fixing core features
2. Investigate user_id FK violations in stock_entries and reservations tables
3. Check if user_id from JWT is being stored correctly

---

*Last updated: 2026-04-29 after Phase 11 added*
