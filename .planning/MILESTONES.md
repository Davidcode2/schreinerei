# Milestones

---

## v1.12 Architecture Guardrails

**Shipped:** 2026-05-04
**Phases:** 3 | **Plans:** 3

### Summary

Hardened the architecture with a transition-safe `projects` boundary, direct request-scoped tenant context extraction in the API layer, and a codified mobile-first engineering checklist.

### Accomplishments

1. Added a `projects` architectural alias over the current `sites` bounded context
2. Refactored IAM, Inventory, Fleet, and Sites API routes to receive `TenantContext` directly from extraction
3. Extended IAM bootstrap helpers to work from request context
4. Captured the mobile-first standard in `.planning/MOBILE-FIRST-CHECKLIST.md`

### Stats

- **Timeline:** 1 day (2026-05-04)
- **Requirements:** 3/3 verified
- **Verification:** `cargo test` PASS (242 unit tests + integration/doc targets)

### Key Decisions

- Introduce `projects` as an alias first instead of a risky runtime rename
- Treat request-scoped tenant context as an API-edge extractor concern
- Enforce mobile-first via a lightweight checklist because the runtime baseline already exists

### Archives

- `.planning/milestones/v1.12-ROADMAP.md`
- `.planning/milestones/v1.12-REQUIREMENTS.md`

---

## v1.11 Fleet Calendar on Fleet Page

**Shipped:** 2026-05-01
**Phases:** 3 | **Plans:** 5

### Summary

Moved fleet booking onto `/fleet` with an embedded calendar, explicit two-tap range selection, bottom-sheet confirmation, stable resource colors, and one canonical reservation entry path.

### Accomplishments

1. Embedded the reservation calendar directly into `FleetPage`
2. Replaced first-tap booking with two-tap date-range selection
3. Added bottom-sheet reservation confirmation with optional time entry
4. Preserved visible reservations while selecting new ranges
5. Removed the standalone `/fleet/calendar` route in favor of `/fleet`

### Stats

- **Timeline:** 1 day (2026-05-01)
- **Requirements:** 13/13 verified

### Key Decisions

- `/fleet` is the primary booking surface
- Range selection completes on the second tap, not the first
- Confirmation appears in a bottom sheet so the calendar stays visible
- Resource colors are deterministic frontend-derived accents

### Archives

- `.planning/milestones/v1.11-ROADMAP.md`
- `.planning/milestones/v1.11-REQUIREMENTS.md`
- `.planning/v1.11-MILESTONE-AUDIT.md`

---

## v1.10 Baustelle Activity Stream Features

**Shipped:** 2026-05-01
**Phases:** 4 | **Plans:** 9

### Summary

Completed the Baustelle activity stream overhaul with separate camera/document flows, attachment-backed entries, a fullscreen media viewer, and creator-only entry deletion.

### Accomplishments

1. Dedicated camera upload flow with native picker, preview, and optional note
2. Document composer supports note-only, attachment-only, and mixed note + image/PDF entries
3. Fullscreen media viewer with slug URLs, metadata, download, share, and close/back routing
4. Protected image/PDF preview handling with viewer deep links
5. Creator-only entry deletion with confirmation dialog and attachment cleanup

### Stats

- **Timeline:** 1 day (2026-05-01)
- **Requirements:** 20/20 verified

### Key Decisions

- Keep camera upload separate from document composition
- Preserve upload-first, activity-second orchestration
- Resolve creator display names and delete permissions on the backend
- Use route-backed viewer overlays with shareable deep links

### Archives

- `.planning/milestones/v1.10-ROADMAP.md`
- `.planning/milestones/v1.10-REQUIREMENTS.md`

---

## v1.9 Inventory Features

**Shipped:** 2026-05-01
**Phases:** 4 | **Plans:** 13

### Summary

Made the inventory workflow fully manageable from the app itself with category management, direct material editing, stock-in, enriched movement history, and tighter type/test coverage around the shipped flows.

### Accomplishments

1. Category settings page with edit/delete safeguards and dedicated `/settings/inventory` entrypoint
2. Material edit and target stock correction flow from inventory details
3. Dedicated stock-in dialog with notes and persisted history visibility
4. Enriched inventory history with badges, user attribution, and Baustelle links
5. Generated DTO alignment plus API-backed browser coverage for the new inventory flows

### Stats

- **Timeline:** 1 day (2026-05-01)
- **Requirements:** 12/12 accepted
- **Known deferred items at close:** 9 (see `STATE.md` Deferred Items)

### Key Decisions

- Inventory settings live on a dedicated authenticated route instead of a generic settings fallback
- Stock-in is a separate command from stock adjustment because it models a different business action
- Category deletion remains blocked while referenced inventory history must be preserved
- Milestone close accepted the remaining Phase 33 history color/assertion hardening gap through explicit manual verification

### Archives

- `.planning/milestones/v1.9-ROADMAP.md`
- `.planning/milestones/v1.9-REQUIREMENTS.md`
- `.planning/v1.9-MILESTONE-AUDIT.md`

---

## v1.8 Activity Feed & Site Status

**Shipped:** 2026-05-01
**Phases:** 4 | **Plans:** 11

### Summary

Brought Baustellen to life with status tracking, tabbed activity feeds, linked material history, and photo uploads with offline support.

### Accomplishments

1. Site status transitions via modal with backend validation and optimistic locking
2. Tabbed activity feed (Notizen/Dokumente + Material) with note creation
3. Material history tab showing live extraction data with category, extractor, and site links
4. Photo uploads with multipart pipeline, UUID storage, and authenticated blob rendering
5. Offline photo queue with data-URL persistence and reconnect sync
6. Decoupled photo upload from activity creation with camera-first modal entry

### Stats

- **Timeline:** 1 day (2026-05-01)
- **LOC:** ~12,133 Rust + ~11,619 TypeScript
- **Requirements:** 21/21 verified (2 deferred runtime tests)

### Key Decisions

- Status change modal with valid transition buttons only
- ActivityFeed displays status changes with arrow icon
- Attachment API uses opaque UUID routes — no internal storage keys in public paths
- Make activity_id nullable — upload stores bytes only, modal createActivity is the business event
- Authenticated blob fetch for image rendering (no unauthenticated URLs)
- Offline photo queue deferred to backlog (Phase 999.1)

### Known Deferred Items

1 offline test (photo queue replay on reconnect — broader offline support needed first)

### Archives

- `.planning/milestones/v1.8-ROADMAP.md`
- `.planning/milestones/v1.8-REQUIREMENTS.md`

---

## v1.6 User Experience & Missing Functionality

**Shipped:** 2026-04-30
**Phases:** 4 | **Plans:** 13

### Summary

Completed missing CRUD functionality, reservation workflow, and comprehensive E2E test coverage. App is now fully functional with user-friendly features.

### Accomplishments

1. Validation bugs fixed (hours > 0, inline error messages)
2. Delete operations on all entities with confirmation dialogs and soft delete
3. Edit operations for time entries and reservations
4. Reservation status workflow (pending → confirmed → active → completed/cancelled)
5. Calendar click-to-create for quick reservation creation
6. Conflict details showing which reservation overlaps
7. 4 new E2E tests for all new functionality

### Stats

- **Timeline:** 1 day (2026-04-30)
- **LOC:** ~12,290 Rust + ~9,200 TypeScript
- **Tests:** 116 backend + 28 frontend + 22 E2E
- **Requirements:** 19/19 complete

### Key Decisions

- Soft delete for all entities (offline sync compatibility)
- Client-side validation mirrors backend rules
- Calendar defaults to 8am-5pm for new reservations
- Only owner or admin can modify time entries

### Archives

- `.planning/milestones/v1.6-ROADMAP.md`
- `.planning/milestones/v1.6-REQUIREMENTS.md`

---

## v1.5 Testing & Quality Foundation

**Shipped:** 2026-04-30
**Phases:** 6 | **Plans:** 10

### Summary

Established comprehensive testing strategy with backend domain tests, frontend test infrastructure, ts-rs type generation, E2E data assertions, and feature audit documenting 24 issues for future roadmap.

### Accomplishments

1. Backend domain tests (116 tests, zero dependencies, inline in domain files)
2. QA Playbook for future agent efficiency
3. Frontend test infrastructure with Vitest + MSW + Testing Library
4. TypeScript types auto-generated from Rust DTOs (49 types, type drift prevented)
5. E2E tests now verify data persistence through API calls
6. Feature audit documented 24 issues in ISSUE-BACKLOG.md

### Stats

- **Timeline:** 2 days (2026-04-28 → 2026-04-30)
- **LOC:** ~12,290 Rust + ~8,991 TypeScript
- **Tests:** 116 backend + 28 frontend + 6 E2E
- **DTOs with ts-rs:** 49
- **Issues documented:** 24
- **Requirements:** 21/21 complete

### Key Decisions

- Tests inline in domain files for zero friction
- ts-rs v12 instead of v10 (v10 lacks export functionality)
- Vitest over Jest for frontend testing (native Vite integration)
- MSW for API mocking at network level (no axios mocking)

### Archives

- `.planning/milestones/v1.5-ROADMAP.md`
- `.planning/milestones/v1.5-REQUIREMENTS.md`

---

## v1.4 Core Feature Fixes

**Shipped:** 2026-04-30
**Phases:** 1 | **Plans:** 3

### Summary

Fixed FK constraint violations in core features by resolving Keycloak user IDs to local database user IDs. Also fixed WorkType enum mismatch and nullable user_name in reservations.

### Accomplishments

1. User ID resolution: Keycloak ID → local users.id before all FK inserts
2. Auto-provisioning users on first FK reference
3. WorkType enum aligned with frontend (site, workshop, travel, other)
4. Calendar accepts both RFC3339 and YYYY-MM-DD date formats
5. Nullable user_name in reservation details for users without names

### Stats

- **Timeline:** 1 day (2026-04-30)
- **LOC:** ~8,900 Rust + ~8,000 TypeScript
- **Requirements:** 4/4 complete

### Key Decisions

- find_or_create_by_keycloak_id() pattern for user resolution
- email added to TenantContext for user provisioning
- pool access added to all services for UserRepository access

### Archives

- `.planning/milestones/v1.4-ROADMAP.md`
- `.planning/milestones/v1.4-REQUIREMENTS.md`

---

## v1.3 Bug Fixes

**Shipped:** 2026-04-29
**Phases:** 3 | **Plans:** 9

### Summary

Comprehensive bug fixes after E2E testing with Playwright. Fixed authentication issues, UI bugs, and API integration problems.

### Accomplishments

1. Token exchange with retry logic (no more double-exchange failures)
2. Fleet "Neu" button dropdown menu
3. User management with real API data
4. Email invite dialog for admins
5. Sync status with toast notifications
6. 18 E2E tests created for regression prevention

### Stats

- **Timeline:** 1 day (2026-04-29)
- **Requirements:** 8/8 complete

### Archives

- Phase 8-10 summaries in `.planning/phases/`

---

## v1.2 Frontend Polish

**Shipped:** 2026-04-29
**Phases:** 1 | **Plans:** 3

### Summary

Fixed non-functional buttons and connected UI to backend APIs. Added dialogs for creating materials, sites, vehicles, tools.

### Accomplishments

1. Material dialog with form submission
2. Site dialog with customer selection
3. Vehicle/Tool dialogs with status selection
4. User invitation via email dialog
5. QR scanner error handling with retry

### Stats

- **Timeline:** 1 day (2026-04-29)
- **Requirements:** 9/9 complete

### Archives

- Phase 7 summaries in `.planning/phases/`

---

## v1.1 Organization-Based Tenancy

**Shipped:** 2026-04-29
**Phases:** 1 | **Plans:** 3

### Summary

Migrated from attribute-based multi-tenancy to Keycloak Organizations for native multi-tenant isolation. Updated backend and frontend to use the organization claim from JWT tokens.

### Accomplishments

1. Database migration for keycloak_organization_id column
2. Comprehensive Keycloak setup documentation with quick start guide
3. Backend JWT Claims struct updated to use organization field
4. Frontend OAuth2 scope updated to request organization claim

### Stats

- **Timeline:** 1 day (2026-04-29)
- **LOC:** ~9,000 Rust + ~57,000 TypeScript
- **Requirements:** 10/10 complete

### Key Decisions

- Keycloak operations (create orgs, add members) are manual — user handles in Admin Console
- Organization claim mapped to tenant_id internally — minimal codebase changes
- keycloak_organization_id column is nullable — populated after organization creation

### Archives

- `.planning/milestones/v1-ROADMAP.md`
- `.planning/milestones/v1-REQUIREMENTS.md`

---

## v1.0 MVP

**Shipped:** 2026-04-29
**Phases:** 5 | **Plans:** 12

### Summary

Complete SaaS application for Schreinereien with multi-tenant authentication, inventory management, construction site tracking, fleet management, and mobile-first PWA with offline support.

### Accomplishments

1. Multi-tenant Rust backend with JWT auth and DDD architecture
2. Inventory module with stock tracking, QR codes, and domain events
3. Sites module with time tracking and activity feed
4. Fleet module with reservation system and calendar
5. Mobile-first PWA with offline support and QR scanner

### Stats

- **Timeline:** 2 days (2026-04-28 → 2026-04-29)
- **LOC:** ~10,850 Rust + ~6,700 TypeScript
- **Requirements:** 37/37 complete

### Tech Debt

- No rate limiting (infrastructure level)
- Event polling vs pub/sub
- No conflict resolution for offline edits

### Archives

- `.planning/milestones/v1-ROADMAP.md`
- `.planning/milestones/v1-REQUIREMENTS.md`
- `.planning/milestones/v1-MILESTONE-AUDIT.md`

---
