# Schreinerei SaaS

## What This Is

Eine SaaS-Lösung für Schreinereien, die Inventar, Baustellen, Werkzeuge und Fahrzeuge in einem System verwaltet. Mitarbeiter können Material entnehmen, Baustellenzeiten buchen, Werkzeuge reservieren und den Standort von Fahrzeugen tracken. Chefs erhalten Übersichten über Verbrauch, Arbeitszeiten und offene Bestellungen.

Mobile-first PWA für Tablet und Smartphone, mit Offline-Unterstützung für Baustellen ohne Empfang.

## Core Value

Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## Current Milestone: v1.10 Baustelle Activity Stream Features

**Goal:** Fix and enhance the Baustelle activity stream — separate camera/document upload flows, support mixed note+attachment entries, and add a fullscreen media viewer with share links.

**Target features:**
- Camera button opens camera/gallery picker (distinct from document modal)
- Document upload modal supports note AND attachments together (currently note OR image)
- All attachment combinations: note only, note+PDF(s), note+image(s), PDF(s) only, image(s) only
- Clickable image/document previews in feed open fullscreen viewer
- Fullscreen view modal with slug: large media, note alongside, timestamp, username, download & share
- Creator can delete their own entries

## Current State

**v1.8 shipped on 2026-05-01.** v1.9 (Inventory Features) deferred. Phase 30 complete.

v1.8 features working:
- ✅ Baustelle status workflow (geplant → aktiv → abgeschlossen) with modal
- ✅ Tabbed activity feed (Notizen/Dokumente + Material)
- ✅ Material extraction history with Baustelle links and category display
- ✅ Photo uploads with multipart pipeline, UUID storage, server-side thumbnails
- ✅ Authenticated blob fetch for image rendering (no unauthenticated URLs)
- ✅ Offline photo capture queue with reconnect sync (deferred runtime test)
- ✅ Camera-first modal entry point for photo uploads

v1.10 progress (Phases 30-31 complete):
- ✅ Camera button opens dedicated CameraUploadFlow (native picker, not document modal)
- ✅ Camera upload supports optional note
- ✅ Camera photo automatically attaches to activity entry
- ✅ Document modal supports note-only, attachment-only, and mixed note + image/PDF entries
- ✅ Activity feed renders document entries with image previews, PDF cards, and preview fallbacks

## Previous Milestones

All core features working:
- ✅ Material management with stock tracking and QR codes
- ✅ Construction site management with time tracking
- ✅ Vehicle and tool reservations with calendar
- ✅ Mobile PWA with offline support
- ✅ Organization-based multi-tenancy via Keycloak
- ✅ All frontend dialogs functional
- ✅ Full CRUD operations on all entities
- ✅ Reservation status workflow
- ✅ E2E test coverage (22 tests)

## Requirements

### Validated

All v1.x requirements validated:

**v1.8 Activity Feed & Site Status (21 requirements):**
- ✓ STAT-01 to STAT-05 — Status change workflow with modal, validation, and audit trail
- ✓ FEED-01 to FEED-04 — Tabbed activity feed with note creation and pagination
- ✓ HIST-01 to HIST-05 — Material history with category, extractor, and site links
- ✓ FILE-01 to FILE-07 — Photo uploads with secure storage and offline queue

**v1.7 Active Project Context (17 requirements):**
- ✓ PREF-01, PREF-02, PREF-03 — User preferences with validation
- ✓ DEDU-01, DEDU-02, DEDU-03 — Material deductions linked to Baustelle
- ✓ ACTV-01 to ACTV-07 — Active site UI with persistence
- ✓ AUTO-01 to AUTO-04 — Auto-assignment with override

**v1.6 User Experience & Missing Functionality (19 requirements):**
- ✓ FIX-01, FIX-02 — Validation bugs fixed
- ✓ DEL-01 to DEL-05 — Delete operations with soft delete
- ✓ EDIT-01 to EDIT-03 — Edit operations for time entries and reservations
- ✓ RESV-01 to RESV-03 — Reservation workflow complete
- ✓ UX-01, UX-02 — UX improvements
- ✓ TEST-12 to TEST-15 — E2E test coverage

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

- [x] Camera button opens camera/gallery picker on activity stream (Phase 30 ✓)
- [x] Document upload modal supports note AND attachments (all combinations)
- [ ] Clickable image/document previews in feed
- [ ] Fullscreen media viewer with slug, note, timestamp, user, download, share
- [ ] Delete own entries (creator only)

### Future (v1.9+)

**Inventory Features (deferred from v1.9):**
- Category editing via inventory settings page
- Edit inventory item location and minimum quantity
- Set available quantity to arbitrary number
- "Material einlagern" (stock in) action with modal
- Extended inventory history (material added, location changed, user attribution)
- Clickable Baustelle links in history events
- Category display on inventory overview

### Future (v2.0+)

**Integration Tests:**
- INT-01: Integration tests with real PostgreSQL for inventory module
- INT-02: Integration tests for sites module
- INT-03: Integration tests for fleet module
- INT-04: Multi-tenant isolation tests for all modules

**Inventory Features:**
- Category editing via inventory settings page
- Inventory item editing (location, minimum quantity)
- Set available quantity to arbitrary number
- "Material einlagern" (stock in) action with modal
- Extended inventory history with user attribution
- Clickable Baustelle links in history events
- Category display on inventory overview

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
- Foto-Uploads und Dokument-Anhänge — Photos done in v1.8, document support in v1.10
- Rich Text Editor für Notizen (Plain Text für v1.8)
- Echtzeit-WebSocket Sync (Polling reicht für MVP-Teamgrößen)
- Video-Uploads (Speicher/Kosten, aufgeschoben)

## Context

**Pilot-Kunde:** Eine Schreinerei wird als erster Kunde die Software testen.

**Tech Stack:**
- Backend: Rust, Axum 0.8, SQLx 0.8, PostgreSQL (~12,133 LOC)
- Frontend: Vite 6, React 18, TypeScript, Tailwind CSS 4, shadcn/ui (~11,619 LOC)
- Auth: Keycloak with OAuth2 PKCE
- Offline: Workbox, Dexie.js (IndexedDB)
- Testing: Vitest, MSW, Playwright

**Known Tech Debt:**
- 24 issues documented in ISSUE-BACKLOG.md
- Offline photo queue replay not runtime-tested (deferred to backlog Phase 999.1)
- No integration tests with real database (deferred to future milestone)
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
| Active Baustelle user-scoped | Per-user context, not global | ✓ v1.7 shipped |
| Hash-based colors | Deterministic, no user decisions | ✓ v1.7 shipped |
| JSONB preferences | Flexible schema evolution | ✓ v1.7 shipped |
| FK-safe user mapping | Tenant-local user resolution | ✓ v1.7 shipped |
| Status change modal with valid transitions | Controlled workflow, audit trail | ✓ v1.8 shipped |
| ActivityFeed tabs (Notes + Materials) | Organized information display | ✓ v1.8 shipped |
| "Auswählen" instead of "Aktiv" | Avoids confusion with status name | ✓ v1.8 shipped |
| Opaque UUID attachment routes | No internal storage keys in URLs | ✓ v1.8 shipped |
| Nullable activity_id for uploads | Decouple upload from activity creation | ✓ v1.8 shipped |
| Authenticated blob fetch for images | No unauthenticated image URLs | ✓ v1.8 shipped |

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

*Last updated: 2026-05-01 after Phase 30 completion**
