# Requirements: Schreinerei SaaS

**Defined:** 2026-04-29
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.

## v1.1 Requirements (Organization-Based Tenancy)

### Keycloak Configuration

- [ ] **KC-01**: Enable Organizations feature in schreinerei realm
- [ ] **KC-02**: Configure organization scope for schreinerei_pwa client

### Organization Migration

- [ ] **ORG-01**: Create Keycloak organizations for existing tenants
- [ ] **ORG-02**: Migrate existing users as organization members
- [ ] **ORG-03**: Add keycloak_organization_id column to tenants table

### Backend Changes

- [ ] **BE-01**: Update JWT Claims struct to use organization claim
- [ ] **BE-02**: Update AuthenticatedUser extractor for organization claim
- [ ] **BE-03**: Add organization ID extraction from JWT organization claim

### Frontend Changes

- [ ] **FE-01**: Update OAuth2 scope to include organization
- [ ] **FE-02**: Update token parsing for organization claim structure

## v1.2 Requirements (Future)

### Self-Service

- **SS-01**: Public website with organization registration
- **SS-02**: Self-service organization creation flow
- **SS-03**: Organization admin dashboard
- **SS-04**: Member invitation via email

### Extended Features

- **EXT-01**: Organization identity provider support
- **EXT-02**: Multi-organization user support

## Out of Scope

| Feature | Reason |
|---------|--------|
| Multi-org user support | Single org per user for v1.1 |
| Public website/landing page | Deferred to v1.2 |
| Self-service registration | Deferred to v1.2 |
| Organization identity providers | Not needed for pilot |
| Stripe integration | Deferred to v1.2+ |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| KC-01 | Phase 6 | Pending |
| KC-02 | Phase 6 | Pending |
| ORG-01 | Phase 6 | Pending |
| ORG-02 | Phase 6 | Pending |
| ORG-03 | Phase 6 | Pending |
| BE-01 | Phase 6 | Pending |
| BE-02 | Phase 6 | Pending |
| BE-03 | Phase 6 | Pending |
| FE-01 | Phase 6 | Pending |
| FE-02 | Phase 6 | Pending |

**Coverage:**
- v1.1 requirements: 10 total
- Mapped to phases: 10
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-29*
*Last updated: 2026-04-29 after v1.1 milestone start*
