# Milestones

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

- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`

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
