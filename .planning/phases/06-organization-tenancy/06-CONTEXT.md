# Phase 6: Organization-Based Tenancy - Context

**Gathered:** 2026-04-29
**Status:** Ready for planning
**Source:** Requirements analysis from REQUIREMENTS.md

<domain>
## Phase Boundary

Migrate from attribute-based multi-tenancy to Keycloak Organizations for proper tenant isolation. This phase enables the Organizations feature in Keycloak, migrates existing tenant data to organizations, and updates both backend and frontend to use the new `organization` claim in JWT tokens.

**In Scope:**
- Enable Keycloak Organizations feature in schreinerei realm
- Create Keycloak organizations for existing tenants
- Migrate users as organization members
- Add `keycloak_organization_id` column to tenants table
- Update Rust backend JWT Claims to use organization claim
- Update frontend OAuth2 scope to include organization

**Out of Scope:**
- Multi-organization user support (single org per user in v1.1)
- Public website/landing page (deferred to v1.2)
- Self-service registration (deferred to v1.2)
- Organization identity providers (not needed for pilot)

</domain>

<decisions>
## Implementation Decisions

### Keycloak Configuration (KC-01, KC-02)
- **D-01**: Enable Organizations feature in the schreinerei realm via Keycloak Admin Console or REST API
- **D-02**: Configure `organization` scope for the `schreinerei-pwa` client to include organization claim in tokens

### Organization Migration (ORG-01, ORG-02, ORG-03)
- **D-03**: Create one Keycloak organization per existing tenant, using tenant UUID as organization ID
- **D-04**: Organization name = tenant name from database
- **D-05**: Migrate all existing users as members of their respective organizations
- **D-06**: Add `keycloak_organization_id` column to tenants table to store the Keycloak organization UUID (matches tenant.id)
- **D-07**: Organization membership uses the user's existing Keycloak user ID

### Backend Changes (BE-01, BE-02, BE-03)
- **D-08**: Rename `tenant_id` field in Claims struct to `organization` to match Keycloak's claim name
- **D-09**: The organization claim contains the organization ID as a string (UUID)
- **D-10**: Update AuthenticatedUser extractor to parse organization claim instead of tenant_id attribute
- **D-11**: TenantId extraction logic: parse organization claim UUID → TenantId (same wrapper, different source)
- **D-12**: Keep TenantId type unchanged - only the claim source changes

### Frontend Changes (FE-01, FE-02)
- **D-13**: Update OAuth2 scope from `openid profile email` to `openid profile email organization`
- **D-14**: Update KeycloakTokenPayload type to use `organization` field instead of `tenant_id`
- **D-15**: Update extractUserFromToken to use `organization` claim
- **D-16**: User type `tenant_id` field name stays the same (internal representation unchanged)

### the agent's Discretion
- Exact migration script implementation details
- Error handling for edge cases (user already in organization, etc.)
- Transaction boundaries for migration
- Rollback strategy if migration fails

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Current Auth Implementation
- `src/auth/jwt.rs` — JWT Claims struct with tenant_id field
- `src/auth/extractor.rs` — AuthenticatedUser extractor parsing tenant_id
- `src/common/types.rs` — TenantId type definition

### Current Frontend Auth
- `frontend/src/lib/auth/keycloak.ts` — OAuth2 PKCE flow with scope configuration
- `frontend/src/types/user.ts` — KeycloakTokenPayload type definition

### Database Schema
- `migrations/001_initial_schema.sql` — tenants and users table definitions
- `src/modules/iam/domain/tenant.rs` — Tenant aggregate definition

</canonical_refs>

<specifics>
## Specific Ideas

### Keycloak Organizations Feature
- Available in Keycloak 26.x and later
- Provides first-class organization concept
- Organization membership managed per user
- Organization claim automatically included in tokens when user is member

### Migration Approach
1. Enable Organizations feature in realm
2. For each tenant in database:
   - Create Keycloak organization with tenant UUID as ID
   - Store organization ID in tenants table
3. For each user in database:
   - Add user as member of their organization
4. Configure client scope for organization claim
5. Update backend and frontend code
6. Test with existing users

### Rollback Consideration
- Organizations can be deleted if migration fails
- keycloak_organization_id column can be dropped
- Code changes are reversible (revert to tenant_id attribute)

</specifics>

<deferred>
## Deferred Ideas

- **Self-service organization creation** — Deferred to v1.2 with public website
- **Organization invitation flow** — Deferred to v1.2
- **Multi-organization user support** — Not planned for v1.x
- **Organization identity providers** — Not needed for pilot

</deferred>

---

*Phase: 06-organization-tenancy*
*Context gathered: 2026-04-29*
