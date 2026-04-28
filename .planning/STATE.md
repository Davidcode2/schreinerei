# Project State

**Project:** Schreinerei SaaS
**Current Phase:** 1 — Auth & IAM Foundation
**Status:** Planned

---

## Progress

```
Phase 1: ██░░░░░░░░ 20%  Planned
Phase 2: ░░░░░░░░░░ 0%  Blocked (Phase 1)
Phase 3: ░░░░░░░░░░ 0%  Blocked (Phase 1, 2)
Phase 4: ░░░░░░░░░░ 0%  Blocked (Phase 1, 3)
Phase 5: ░░░░░░░░░░ 0%  Blocked (Phase 1-4)
```

## Current Phase: 1

**Name:** Auth & IAM Foundation
**Goal:** Multi-Tenant Authentication mit Keycloak, User-Management, Basis-API
**Status:** Planned

### Plans

| Plan | Status | Requirements |
|------|--------|--------------|
| 01-01 — Backend Foundation + Auth Middleware | Pending | AUTH-01, AUTH-02 |
| 01-02 — IAM Module + User Management API | Pending | AUTH-03, AUTH-04, AUTH-05 |

### Requirements

- [ ] AUTH-01: User kann sich via Keycloak einloggen
- [ ] AUTH-02: Multi-Tenant-Trennung ist gewährleistet
- [ ] AUTH-03: Admin kann neue User einladen
- [ ] AUTH-04: Rollen: Admin, Mitarbeiter
- [ ] AUTH-05: User kann sein Profil bearbeiten

### Success Criteria

1. User kann sich via Keycloak einloggen und sieht sein Tenant-spezifisches Dashboard
2. Admin kann neue User einladen und Rollen zuweisen
3. Multi-Tenant-Trennung ist verifiziert
4. Basis-API-Struktur ist aufgebaut
5. Deployment auf K8s funktioniert

---

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-28)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.
**Current focus:** Auth & IAM Foundation

---

## History

| Date | Event | Details |
|------|-------|---------|
| 2026-04-28 | Project Initialized | PROJECT.md, REQUIREMENTS.md, ROADMAP.md created |

---

## Next Action

Run `/gsd-execute-phase 1` to execute Phase 1 plans. Start with Plan 01-01.
