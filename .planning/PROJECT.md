# Schreinerei SaaS

## What This Is

Eine SaaS-Lösung für Schreinereien, die Inventar, Baustellen, Werkzeuge und Fahrzeuge in einem System verwaltet. Mitarbeiter können Material entnehmen, Baustellenzeiten buchen, Werkzeuge reservieren und den Standort von Fahrzeugen tracken. Chefs erhalten Übersichten über Verbrauch, Arbeitszeiten und offene Bestellungen.

Mobile-first PWA für Tablet und Smartphone, mit Offline-Unterstützung für Baustellen ohne Empfang.

## Core Value

Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## Current Milestone: Planning v1.2

**Next:** Self-service organization registration and public website

## Requirements

### Validated

All 37 v1 requirements + 10 v1.1 requirements validated:

**Architecture:**
- ✓ ARCH-01 to ARCH-06 — Phase 1

**Authentication:**
- ✓ AUTH-01 to AUTH-05 — Phase 1

**Inventar:**
- ✓ INVT-01 to INVT-07 — Phase 2

**Baustellen:**
- ✓ SITE-01 to SITE-08 — Phase 3

**Fuhrpark:**
- ✓ FLEET-01 to FLEET-07 — Phase 4

**PWA:**
- ✓ PWA-01 to PWA-04 — Phase 5

**Organization-Based Tenancy (v1.1):**
- ✓ KC-01, KC-02 — Keycloak Organizations enabled and scoped
- ✓ ORG-01, ORG-02, ORG-03 — Organizations created and users migrated
- ✓ BE-01, BE-02, BE-03 — Backend uses organization claim
- ✓ FE-01, FE-02 — Frontend requests organization scope

### Active

**v1.2 Requirements (Self-Service Registration):**

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

**Pilot-Kunde:** Eine Schreinerei wird als erster Kunde die Software testen. Ziel: Frühes Feedback für iterative Verbesserung.

**Tech Stack:**
- Backend: Rust, Axum 0.8, SQLx 0.8, PostgreSQL
- Frontend: Vite 6, React 18, TypeScript, Tailwind CSS 4, shadcn/ui
- Auth: Keycloak with OAuth2 PKCE
- Offline: Workbox, Dexie.js (IndexedDB)

**Known Tech Debt:**
- No rate limiting (infrastructure level)
- Event polling vs pub/sub
- No conflict resolution for offline edits

## Constraints

- **Timeline**: V1 shipped in 2 days
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
| Vite + React PWA | Einfach, schnell, Offline-Support möglich | ✓ v1 shipped |
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

---

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

*Last updated: 2026-04-29 after v1.1 milestone completion*
