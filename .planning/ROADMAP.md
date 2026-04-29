# Roadmap: Schreinerei SaaS

**Project:** Schreinerei SaaS
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.
**Timeline:** 1 day

---

## Milestones

- ✅ **v1.0 MVP** — Phases 1-5 (shipped 2026-04-29)
- ✅ **v1.1 Organization-Based Tenancy** — Phase 6 (shipped 2026-04-29)
- 🚧 **v1.2 Frontend Polish** — Phase 7 (in progress)

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

<details>
<summary>✅ v1.1 Organization-Based Tenancy (Phase 6) — SHIPPED 2026-04-29</summary>

- [x] Phase 6: Organization-Based Tenancy (3/3 plans) — completed 2026-04-29

</details>

---

## Phase 7: Frontend Polish

**Goal:** Fix non-functional buttons and connect UI to backend APIs

**Requirements:** INVT-08, INVT-09, SITE-09, FLEET-08, FLEET-09, USER-01, USER-02, ERR-01, ERR-02

**Plans:** 3 plans

**Duration:** 1 day

### Plans

**Wave 1**
- [ ] 07-01 — Add Material & Site Dialogs (INVT-08, INVT-09, SITE-09)

**Wave 2**
- [ ] 07-02 — Add Fleet Dialogs (FLEET-08, FLEET-09)

**Wave 3**
- [ ] 07-03 — User Management & Error Handling (USER-01, USER-02, ERR-01, ERR-02)

### Success Criteria

1. User can add material via dialog with name, quantity, unit, location
2. User can create site via dialog with name, customer, location
3. User can add vehicle via dialog with name, plate, status
4. User can add tool via dialog with name, status
5. Admin can invite user via email dialog
6. Settings displays real users from API (not mock data)
7. QR scanner shows friendly error when camera denied
8. QR scanner provides retry button after error

---

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Auth & IAM | v1.0 | 2/2 | Complete | 2026-04-28 |
| 2. Inventar | v1.0 | 2/2 | Complete | 2026-04-28 |
| 3. Baustellen | v1.0 | 2/2 | Complete | 2026-04-28 |
| 4. Fuhrpark | v1.0 | 2/2 | Complete | 2026-04-28 |
| 5. PWA & Mobile | v1.0 | 4/4 | Complete | 2026-04-29 |
| 6. Org Tenancy | v1.1 | 3/3 | Complete | 2026-04-29 |
| 7. Frontend Polish | v1.2 | 0/3 | Planned | - |

---

## Summary

| Phase | Goal | Duration | Requirements |
|-------|------|----------|--------------|
| 7 | Frontend Polish | 1 day | 9 |

**Total for v1.2:** 1 day

---

*Last updated: 2026-04-29 after v1.2 milestone start*
