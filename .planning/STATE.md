# Project State

**Project:** Schreinerei SaaS
**Current Milestone:** v1.1 Organization-Based Tenancy
**Status:** Phase 6 Complete ✓

---

## Current Position

Phase: 6 — Organization-Based Tenancy
Plan: —
Status: Complete ✓
Last activity: 2026-04-29 — Phase 6 complete

---

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-29)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.
**Current focus:** Phase 6 complete — Manual Keycloak setup required

---

## Accumulated Context

### Decisions Summary

Full decision log in PROJECT.md. Key decisions from v1.0:
- Rust Backend with Axum 0.8, SQLx 0.8
- DDD layering: domain/application/infrastructure/api
- Multi-tenant via TenantId in all queries
- Keycloak OAuth2 PKCE for SPA auth
- IndexedDB (Dexie.js) for offline storage

New decisions from Phase 6:
- Organization claim replaces tenant_id attribute in JWT
- Keycloak operations (create orgs, add members) are manual
- User.tenant_id field name unchanged (internal representation)

### Blockers/Concerns

- None currently

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
| 2026-04-29 | v1.1 Started | Organization-Based Tenancy migration |
| 2026-04-29 | Phase 6 Complete | Database + Backend + Frontend ready for org-based tenancy |

---

## Next Action

**Manual Keycloak Setup Required**

Before the organization-based tenancy works end-to-end, complete these steps in Keycloak:

1. Add `organization` scope to `schreinerei-pwa` client
2. Create organizations for existing tenants
3. Add users as organization members
4. Run database migration: `sqlx migrate run`
5. Test login — verify token contains `organization` claim

See: `docs/keycloak-organizations-setup.md` for detailed instructions.

---

After Keycloak setup: `/gsd-verify-phase 6` to verify E2E functionality.
