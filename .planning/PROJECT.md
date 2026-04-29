# Schreinerei SaaS

## What This Is

Eine SaaS-Lösung für Schreinereien, die Inventar, Baustellen, Werkzeuge und Fahrzeuge in einem System verwaltet. Mitarbeiter können Material entnehmen, Baustellenzeiten buchen, Werkzeuge reservieren und den Standort von Fahrzeugen tracken. Chefs erhalten Übersichten über Verbrauch, Arbeitszeiten und offene Bestellungen.

Mobile-first PWA für Tablet und Smartphone, mit Offline-Unterstützung für Baustellen ohne Empfang.

## Core Value

Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## Current Milestone: v1.1 Organization-Based Tenancy

**Goal:** Migrate from attribute-based to Keycloak Organizations for multi-tenant isolation

**Target features:**
- Enable Keycloak Organizations in schreinerei realm
- Migrate existing tenants to Keycloak Organizations
- Update JWT claims to use `organization` claim instead of `tenant_id` attribute
- Update Rust backend to extract TenantId from organization claim
- Update frontend to request `organization` scope
- Organization self-service management (create org, invite members)

## Requirements

### Validated

All 37 v1 requirements validated:

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

### Active

**v1.1 Requirements (Organization-Based Tenancy):**

- ORG-01: Enable Organizations feature in Keycloak realm
- ORG-02: Create organizations for existing tenants
- ORG-03: Migrate users to organizations as members
- ORG-04: Update JWT claims to use organization claim
- ORG-05: Update Rust backend for organization claim extraction
- ORG-06: Update frontend OAuth2 scope to include organization
- ORG-07: Organization self-service creation (public website)
- ORG-08: Organization member invitation flow

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

*Last updated: 2026-04-29 after v1.0 MVP milestone*
