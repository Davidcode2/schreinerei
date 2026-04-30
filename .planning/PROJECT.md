# Schreinerei SaaS

## What This Is

Eine SaaS-Lösung für Schreinereien, die Inventar, Baustellen, Werkzeuge und Fahrzeuge in einem System verwaltet. Mitarbeiter können Material entnehmen, Baustellenzeiten buchen, Werkzeuge reservieren und den Standort von Fahrzeugen tracken. Chefs erhalten Übersichten über Verbrauch, Arbeitszeiten und offene Bestellungen.

Mobile-first PWA für Tablet und Smartphone, mit Offline-Unterstützung für Baustellen ohne Empfang.

## Core Value

Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## Current State

**Shipped:** v1.5 Testing & Quality Foundation (2026-04-30)

**Next Milestone:** v1.6 (planning required)

See `.planning/ISSUE-BACKLOG.md` for 24 documented issues prioritized for v1.6+.

## Previous Milestones

### v1.4 Core Feature Fixes (Shipped 2026-04-30)

All core features working:
- ✅ Material management with stock tracking and QR codes
- ✅ Construction site management with time tracking
- ✅ Vehicle and tool reservations with calendar
- ✅ Mobile PWA with offline support
- ✅ Organization-based multi-tenancy via Keycloak
- ✅ All frontend dialogs functional
- ✅ E2E test coverage (18 tests)

## Requirements

### Validated

All v1.x requirements validated:

**v1.5 Testing & Quality Foundation (21 requirements):**
- ✓ TEST-01 to TEST-03 — Backend domain tests
- ✓ TEST-04 to TEST-09 — Frontend testing and ts-rs
- ✓ TEST-10, TEST-11 — E2E data assertions
- ✓ QA-01 to QA-04 — Agent QA Playbook
- ✓ AUDIT-01 to AUDIT-06 — Feature audit

**v1.0 MVP (37 requirements):**
- ✓ ARCH-01 to ARCH-06 — Phase 1
- ✓ AUTH-01 to AUTH-05 — Phase 1
- ✓ INVT-01 to INVT-07 — Phase 2
- ✓ SITE-01 to SITE-08 — Phase 3
- ✓ FLEET-01 to FLEET-07 — Phase 4
- ✓ PWA-01 to PWA-04 — Phase 5

**v1.1 Organization-Based Tenancy (10 requirements):**
- ✓ KC-01, KC-02 — Keycloak Organizations enabled
- ✓ ORG-01, ORG-02, ORG-03 — Organizations created and migrated
- ✓ BE-01, BE-02, BE-03 — Backend uses organization claim
- ✓ FE-01, FE-02 — Frontend requests organization scope

**v1.2 Frontend Polish (9 requirements):**
- ✓ INVT-08, INVT-09 — Material dialog and QR navigation
- ✓ SITE-09 — Site dialog
- ✓ FLEET-08, FLEET-09 — Vehicle/Tool dialogs
- ✓ USER-01, USER-02 — User invitation and listing
- ✓ ERR-01, ERR-02 — QR scanner error handling

**v1.3 Bug Fixes (8 requirements):**
- ✓ BUG-001 to BUG-008 — All E2E discovered bugs fixed

**v1.4 Core Feature Fixes (4 requirements):**
- ✓ CORE-01 to CORE-04 — FK constraints resolved

### Active

(None — v1.5 complete, planning v1.6)

### Future (v1.6+)

**From Issue Backlog:**
- Fix BUG-TIME-001: Hours validation in TimeEntryDialog
- Add E2E tests for update/delete operations
- Implement site delete UI
- Add time entry edit/delete functionality

**Integration Tests:**
- INT-01: Integration tests with real PostgreSQL for inventory module
- INT-02: Integration tests for sites module
- INT-03: Integration tests for fleet module
- INT-04: Multi-tenant isolation tests for all modules

**Self-Service Registration:**

- SS-01: Public website with organization registration
- SS-02: Self-service organization creation flow
- SS-03: Organization admin dashboard
- SS-04: Member invitation via email
- EXT-01: Organization identity provider support
- EXT-02: Multi-organization user support

### Out of Scope

- RFID Hardware-Integration — Hardware nicht vorhanden, später ergänzen
- CAD/CNC Integration (DXF, Bsolid) — Nicht kritisch für Pilot, später implementieren
- Native Mobile App — PWA first, später React Native/Capacitor möglich
- Öffentliche Website/Landing Page — Fokus auf App, Website später
- Datenmigration — Pilot startet bei Null

## Context

**Pilot-Kunde:** Eine Schreinerei wird als erster Kunde die Software testen.

**Tech Stack:**
- Backend: Rust, Axum 0.8, SQLx 0.8, PostgreSQL (~12,290 LOC)
- Frontend: Vite 6, React 18, TypeScript, Tailwind CSS 4, shadcn/ui (~8,991 LOC)
- Auth: Keycloak with OAuth2 PKCE
- Offline: Workbox, Dexie.js (IndexedDB)
- Testing: Vitest, MSW, Playwright

**Known Tech Debt:**
- 24 issues documented in ISSUE-BACKLOG.md
- No integration tests with real database (deferred to v1.6)
- No rate limiting (infrastructure level)
- Event polling vs pub/sub
- No conflict resolution for offline edits

## Constraints

- **Timeline**: V1.x shipped in 3 days
- **Architecture**: Hexagonal Architecture (Ports & Adapters) mit Modular Monolith und DDD Bounded Contexts
- **Multi-Tenancy**: TenantId in jeder Query
- **Deployment**: Kubernetes cluster
- **Offline**: Service Worker, IndexedDB für Baustellen ohne Empfang
- **Longevity**: 10+ Jahre Wartbarkeit ohne komplette Neuentwicklung

## Architecture

### Hexagonal Architecture (Ports & Adapters)

Jedes Modul hat drei Schichten:

- **Domain**: Reine Business-Logik, Entities, Value Objects, Domain Events. Keine Dependencies auf Frameworks oder Externes.
- **Application**: Use Cases / Services. Orchestriert Domain-Objekte. Definiert Ports (Traits) für Infrastructure.
- **Infrastructure**: Adapter für Datenbank, External APIs, Message Bus. Implementiert Ports aus Application.

### Modular Monolith mit DDD Bounded Contexts

```
src/
├── common/           # Shared Kernel (TenantId, Money, Error Types)
├── auth/             # Keycloak integration & JWT verification
└── modules/
    ├── iam/          # Identity & Access Management
    ├── inventory/    # Material management
    ├── sites/        # Construction site management
    └── fleet/        # Vehicle & tool management
```

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust Backend | Sicherheit, Performance, 10+ Jahre Wartbarkeit | ✓ Axum 0.8, SQLx 0.8 working |
| Vite + React PWA | Einfach, schnell, Offline-Support möglich | ✓ v1.4 shipped |
| Keycloak (bestehend) | Bereits im Cluster, SSO, Multi-Tenant-ready | ✓ JWT validation with JWKS caching |
| Modularer Monolith | Klare Trennung, später extrahierbar | ✓ DDD structure established |
| Multi-Tenant ab Tag 1 | Architektur mitdenken, nicht nachrüsten | ✓ TenantId in all tables |
| SQLx runtime queries | No database during build | ✓ Working |
| Domain Events (V1) | Inter-module communication | ✓ Events stored in DB |
| QR codes with tenant prefix | Uniqueness across tenants | ✓ SVG generation |
| Site status state machine | Controlled status transitions | ✓ Planned → Active → Completed → Archived |
| Unified reservations table | One table for vehicles and tools | ✓ Resource type enum |
| OAuth2 PKCE for SPA auth | Secure token flow | ✓ Keycloak integration |
| IndexedDB via Dexie | Offline data persistence | ✓ Sync queue |
| Keycloak Organizations | Native multi-tenant isolation | ✓ v1.1 organization claim |
| Manual Keycloak ops | No automation needed for pilot | ✓ User manages orgs in Admin Console |
| Organization → tenant_id mapping | Minimal codebase changes | ✓ Frontend maps at extraction |
| Keycloak ID → local user ID | FK constraint resolution | ✓ find_or_create_by_keycloak_id() |
| Tests inline in domain files | Zero friction, fast feedback | ✓ 116 backend tests |
| ts-rs v12 for type generation | Prevents frontend-backend drift | ✓ 49 DTOs exported |
| Vitest over Jest | Native Vite integration | ✓ 28 frontend tests |
| MSW for API mocking | Network-level mocking | ✓ No axios mocking |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---

*Last updated: 2026-04-30 after v1.5 milestone*
