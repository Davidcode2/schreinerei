# Schreinerei SaaS

## What This Is

Eine SaaS-Losung fur Schreinereien, die Inventar, Baustellen, Werkzeuge und Fahrzeuge in einem System verwaltet. Mitarbeiter konnen Material entnehmen, Baustellenzeiten buchen, Werkzeuge reservieren und den Standort von Fahrzeugen tracken. Chefs erhalten Ubersichten uber Verbrauch, Arbeitszeiten und offene Bestellungen.

Mobile-first PWA fur Tablet und Smartphone, mit Offline-Unterstutzung fur Baustellen ohne Empfang.

## Core Value

Mitarbeiter finden alles schnell, Chefs haben den Uberblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## Latest Shipped Milestone: v1.12 Architecture Guardrails

**Outcome:** This branch now includes shipped architecture guardrails on top of the merged `v1.9` to `v1.11` work, extending the shipped phase set through `41-43`.

## Next Milestone

Not planned yet. The next planning cycle should use `.planning/FEATURES.md` and `.planning/REQUIREMENTS.md` as the source of truth for backlog shaping. Next logical workflow is `/gsd-new-milestone`.

## Current State

**v1.12 shipped on 2026-05-04.**

Latest shipped additions:

- ✅ `projects` now exists as a transition-safe architectural boundary over the current `sites` module
- ✅ API routes now receive tenant/user request context directly instead of rebuilding it manually from auth
- ✅ Mobile-first delivery is now codified in `.planning/MOBILE-FIRST-CHECKLIST.md`
- ✅ `/fleet` is now the primary reservation surface with an embedded calendar
- ✅ Reservation creation uses explicit two-tap date-range selection with a bottom confirmation sheet
- ✅ Baustelle activity stream supports separate camera/document flows and mixed attachment entries
- ✅ Fullscreen media viewer supports deep links, download, share, and creator metadata
- ✅ Inventory is fully manageable in-app with category management, stock-in, editing, and enriched history

Previously shipped foundation still working:

- ✅ Baustelle status workflow (geplant → aktiv → abgeschlossen) with modal
- ✅ Tabbed activity feed (Notizen/Dokumente + Material)
- ✅ Material extraction history with Baustelle links and category display
- ✅ Photo uploads with multipart pipeline, UUID storage, server-side thumbnails
- ✅ Authenticated blob fetch for image rendering (no unauthenticated URLs)
- ✅ Offline photo capture queue with reconnect sync (deferred runtime test)

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
- ✅ E2E test coverage

## Requirements

### Validated

All v1.x requirements validated.

**v1.11 Fleet Calendar on Fleet Page (13 requirements):**
- ✓ FCAL-01 to FCAL-03 — Embedded fleet calendar became the primary booking surface on `/fleet`
- ✓ FSEL-01 to FSEL-04 — Two-tap date-range selection with same-day and sorted reverse-order handling
- ✓ FCONF-01 to FCONF-06 — Bottom-sheet confirmation, cancel/reset, optional times, visible reservations, and stable resource colors

**v1.12 Architecture Guardrails (3 requirements):**
- ✓ ARCH-01 — `projects` architectural boundary introduced without breaking runtime routes
- ✓ ARCH-02 — Request-scoped tenant context extraction replaced manual route-level conversion
- ✓ ARCH-04 — Mobile-first engineering checklist codified from existing runtime baseline

**v1.10 Baustelle Activity Stream Features (20 requirements):**
- ✓ CAM-01 to CAM-03 — Separate camera upload flow with optional note
- ✓ DOC-01 to DOC-05 — Mixed note + attachment document entries with PDF/image support
- ✓ VIEW-01 to VIEW-09 — Fullscreen media viewer with deep links, metadata, download, and share
- ✓ ENTRY-01 to ENTRY-03 — Creator-only entry deletion with confirmation and cleanup

**v1.9 Inventory Features (12 requirements):**
- ✓ CATS-01 to CATS-03 — Category settings entrypoint, editing, and guarded delete flow
- ✓ EDIT-01 to EDIT-03 — Material location/minimum-quantity editing and direct stock correction
- ✓ STOCK-01, STOCK-02 — Stock-in dialog plus visible history entries for stock additions
- ✓ HIST-01 to HIST-03 — Enriched history badges, user attribution, and clickable Baustelle links
- ✓ VIEW-01 — Category display on the inventory overview

**v1.8 Activity Feed & Site Status (21 requirements):**
- ✓ STAT-01 to STAT-05 — Status change workflow with modal, validation, and audit trail
- ✓ FEED-01 to FEED-04 — Tabbed activity feed with note creation and pagination
- ✓ HIST-01 to HIST-05 — Material history with category, extractor, and site links
- ✓ FILE-01 to FILE-07 — Photo uploads with secure storage and offline queue

### Active

No active milestone requirements yet.

Use `/gsd-new-milestone` to define the next milestone scope.

### Product Backlog Captured

- A structured comparison of shipped functionality vs the broader product note now lives in `.planning/FEATURES.md`.
- The note-derived backlog, including architecture and approach guidance, now lives in `.planning/REQUIREMENTS.md` under `Product Backlog from 2026-05 Note`.

### Deferred

- Metadata-only inventory history entries for `location_changed` and `min_quantity_changed`.
  Concern: writing non-stock edits into `stock_entries` mixes stock movement audit data with metadata changes. If this feature returns later, it should use a separate audit model instead of overloading `stock_entries`.
- Offline photo queue replay runtime coverage.
- Timezone-sensitive local-day handling in the fleet calendar remains worth monitoring.
- Global uniqueness of resource color accents is not guaranteed; current behavior only guarantees stability and determinism.

### Future (v2.0+)

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

- RFID Hardware-Integration — Hardware nicht vorhanden, spater erganzen
- CAD/CNC Integration (DXF, Bsolid) — Nicht kritisch fur Pilot, spater implementieren
- Native Mobile App — PWA first, spater React Native/Capacitor moglich
- Offentliche Website/Landing Page — Fokus auf App, Website spater
- Rich Text Editor fur Notizen (Plain Text genug fur current scope)
- Echtzeit-WebSocket Sync (Polling reicht fur MVP-TeamgroBen)
- Video-Uploads (Speicher/Kosten, aufgeschoben)

## Context

**Pilot-Kunde:** Eine Schreinerei wird als erster Kunde die Software testen.

**Tech Stack:**
- Backend: Rust, Axum 0.8, SQLx 0.8, PostgreSQL
- Frontend: Vite 6, React 18, TypeScript, Tailwind CSS 4, shadcn/ui
- Auth: Keycloak with OAuth2 PKCE
- Offline: Workbox, Dexie.js (IndexedDB)
- Testing: Vitest, MSW, Playwright

**Known Tech Debt:**
- No integration tests with real database yet
- Offline photo queue replay still needs runtime verification
- Fleet day-key derivation still relies on ISO date splitting
- Resource color strategy is deterministic but not globally uniqueness-checked
