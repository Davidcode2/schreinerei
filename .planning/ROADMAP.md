# Roadmap: Schreinerei SaaS

**Project:** Schreinerei SaaS
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.
**Timeline:** 1-2 Wochen

---

## Milestones

- ✅ **v1.0 MVP** — Phases 1-5 (shipped 2026-04-29)
- ✅ **v1.1 Organization-Based Tenancy** — Phase 6 (code complete, manual Keycloak setup pending)

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

## Phase 6: Organization-Based Tenancy ✓

**Goal:** Migrate from attribute-based to Keycloak Organizations for multi-tenant isolation

**Requirements:** KC-01, KC-02, ORG-01, ORG-02, ORG-03, BE-01, BE-02, BE-03, FE-01, FE-02

**Plans:** 3/3 plans complete

**Duration:** 1 day

**Status:** Code Complete ✓ (Manual Keycloak setup pending)

### Plans

**Wave 1**
- [x] 06-01 — Database Migration + Documentation (KC-01, KC-02, ORG-01, ORG-02)

**Wave 2**
- [x] 06-02 — Backend JWT Migration (ORG-03, BE-01, BE-02, BE-03)

**Wave 3**
- [x] 06-03 — Frontend OAuth2 Scope Update (FE-01, FE-02)

### Success Criteria

1. ✓ Database migration for organization ID created
2. ✓ Backend Claims struct uses `organization` field
3. ✓ Backend extractor parses organization claim
4. ✓ Frontend OAuth2 scope includes `organization`
5. ✓ Frontend token parsing uses organization claim
6. ⏳ Organizations created in Keycloak (manual)
7. ⏳ Users added as organization members (manual)
8. ⏳ End-to-end flow verified (pending Keycloak setup)

### Key Decisions

- Keycloak operations (create orgs, add members) are manual
- Organization ID maps to tenant ID (same UUID)
- User.tenant_id field name unchanged (internal representation)

### Dependencies

- Keycloak 26.x or later ✓
- Manual Keycloak setup (see docs/keycloak-setup.md)

---

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Auth & IAM | v1.0 | 2/2 | Complete | 2026-04-28 |
| 2. Inventar | v1.0 | 2/2 | Complete | 2026-04-28 |
| 3. Baustellen | v1.0 | 2/2 | Complete | 2026-04-28 |
| 4. Fuhrpark | v1.0 | 2/2 | Complete | 2026-04-28 |
| 5. PWA & Mobile | v1.0 | 4/4 | Complete | 2026-04-29 |
| 6. Org Tenancy | v1.1 | 3/3 | Code Complete | 2026-04-29 |

---

## Summary

| Phase | Goal | Duration | Requirements |
|-------|------|----------|--------------|
| 6 | Organization-Based Tenancy | 1 day | 10 |

**Total for v1.1:** 1 day (code)

---

*Last updated: 2026-04-29 after Phase 6 completion*
