# Milestones

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
