# Roadmap: Schreinerei SaaS

**Project:** Schreinerei SaaS
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.
**Timeline:** 1-2 Wochen

---

## Milestones

- ✅ **v1.0 MVP** — Phases 1-5 (shipped 2026-04-29)
- 🚧 **v1.1 Organization-Based Tenancy** — Phase 6 (in progress)

---

## Phases

<details>
<summary>✅ v1.0 MVP (Phases 1-5) — SHIPPED 2026-04-29</summary>

- [x] Phase 1: Auth & IAM Foundation (2/2 plans) — completed 2026-04-28
- [x] Phase 2: Inventar Management (2/2 plans) — completed 2026-04-28
- [x] Phase 3: Baustellen Management (2/2 plans) — completed 2026-04-28
- [x] Phase 4: Fuhrpark & Werkzeuge (2/2 plans) — completed 2026-04-28
- [x] Phase 5: PWA & Mobile (4/4 plans) — completed 2026-04-29

</details>

---

## Phase 6: Organization-Based Tenancy

**Goal:** Migrate from attribute-based to Keycloak Organizations for multi-tenant isolation

**Requirements:** KC-01, KC-02, ORG-01, ORG-02, ORG-03, BE-01, BE-02, BE-03, FE-01, FE-02

**Plans:** 3/3 plans

**Duration Estimate:** 1 Woche

**Status:** Ready to Execute

### Plans

**Wave 1**
- [ ] 06-01-PLAN.md — Keycloak Organizations Setup (KC-01, KC-02, ORG-01, ORG-02)

**Wave 2 *(blocked on Wave 1 completion)***
- [ ] 06-02-PLAN.md — Backend JWT Migration (ORG-03, BE-01, BE-02, BE-03)

**Wave 3 *(blocked on Wave 2 completion)***
- [ ] 06-03-PLAN.md — Frontend OAuth2 Scope Update (FE-01, FE-02)

### Cross-cutting Constraints

- All plans depend on Keycloak Organizations feature being enabled
- Backend changes require organizations to exist before code deployment
- Frontend changes require backend to accept organization claim

### Success Criteria

1. Organizations feature is enabled in Keycloak realm
2. Existing tenants are migrated to Keycloak organizations
3. JWT tokens contain `organization` claim instead of `tenant_id` attribute
4. Backend correctly extracts TenantId from organization claim
5. Frontend requests `organization` scope and handles new token structure
6. All existing functionality works with organization-based tenancy

### Key Decisions

- Use single organization per user (no multi-org support in v1.1)
- Organization ID is the same as tenant ID (UUID)
- Self-service organization creation deferred to v1.2

### Dependencies

- Keycloak 26.x or later
- Existing v1.0 infrastructure

---

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Auth & IAM | v1.0 | 2/2 | Complete | 2026-04-28 |
| 2. Inventar | v1.0 | 2/2 | Complete | 2026-04-28 |
| 3. Baustellen | v1.0 | 2/2 | Complete | 2026-04-28 |
| 4. Fuhrpark | v1.0 | 2/2 | Complete | 2026-04-28 |
| 5. PWA & Mobile | v1.0 | 4/4 | Complete | 2026-04-29 |
| 6. Org Tenancy | v1.1 | 0/3 | Planned | - |

---

## Summary

| Phase | Goal | Duration | Requirements |
|-------|------|----------|--------------|
| 6 | Organization-Based Tenancy | 1 Woche | 10 |

**Total for v1.1:** 1 Woche

---

*Last updated: 2026-04-29 after Phase 6 planning*
