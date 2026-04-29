# Phase 6: Organization-Based Tenancy - Research

**Phase:** 6 - Organization-Based Tenancy
**Researched:** 2026-04-29
**Goal:** Migrate from attribute-based to Keycloak Organizations for multi-tenant isolation

---

## Executive Summary

Keycloak Organizations is a feature available in Keycloak 26+ that provides first-class multi-tenancy support. This research covers the technical approach for migrating from custom `tenant_id` attribute to native Keycloak Organizations.

**Recommendation:** Proceed with migration. Keycloak Organizations provides better isolation, standard APIs, and future-proof multi-tenant architecture.

---

## Keycloak Organizations Overview

### What is Organizations?

Keycloak Organizations is an extension that adds first-class organization support to Keycloak. It enables:
- Organization-scoped authentication
- Organization membership management
- Organization-specific identity providers
- Organization claims in tokens

### Prerequisites

- Keycloak 26.x or later (verify current version)
- Realm admin access for configuration
- Existing users and tenants in database

### Key Concepts

| Concept | Description |
|---------|-------------|
| Organization | A tenant entity with unique ID |
| Organization Membership | User's association with an organization |
| Organization Claim | JWT claim containing organization ID |
| Organization Scope | OAuth2 scope that enables organization claim |

---

## Technical Architecture

### Current State

```
User JWT Token:
├── sub: user-uuid
├── email: user@example.com
├── tenant_id: "org-uuid" (custom attribute mapper)
└── realm_access: { roles: [...] }

Backend Processing:
JWT Claims → tenant_id attribute → TenantId parsing → Request context
```

### Target State

```
User JWT Token:
├── sub: user-uuid
├── email: user@example.com
├── organization: "org-uuid" (native claim)
└── realm_access: { roles: [...] }

Backend Processing:
JWT Claims → organization claim → TenantId parsing → Request context
```

### Claim Structure Change

**Before (attribute-based):**
```json
{
  "sub": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "tenant_id": "123e4567-e89b-12d3-a456-426614174000",
  "realm_access": { "roles": ["admin"] }
}
```

**After (organization claim):**
```json
{
  "sub": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "organization": "123e4567-e89b-12d3-a456-426614174000",
  "realm_access": { "roles": ["admin"] }
}
```

---

## Implementation Research

### Step 1: Enable Organizations Feature

**Via Admin Console:**
1. Navigate to Realm Settings → Organizations
2. Enable "Organizations" feature
3. Configure organization settings

**Via REST API:**
```bash
# Enable organizations feature
PUT /admin/realms/{realm}
{
  "organizationsEnabled": true
}
```

### Step 2: Create Organizations

**Via REST API:**
```bash
POST /admin/realms/{realm}/organizations
{
  "name": "Tenant Name",
  "alias": "tenant-slug"
}
```

**Response includes organization ID:**
```json
{
  "id": "org-uuid",
  "name": "Tenant Name",
  "alias": "tenant-slug"
}
```

### Step 3: Add Users to Organizations

**Via REST API:**
```bash
POST /admin/realms/{realm}/organizations/{org-id}/members/{user-id}
```

### Step 4: Configure Organization Scope

Add `organization` to the client's optional scopes or include it in the default scope.

---

## Database Migration

### Schema Change

```sql
-- Add keycloak organization ID column
ALTER TABLE tenants ADD COLUMN keycloak_organization_id UUID;

-- Update existing tenants with organization IDs
-- (will be done via migration script)

-- Optional: Add unique constraint
ALTER TABLE tenants ADD CONSTRAINT uq_tenants_keycloak_org_id 
  UNIQUE (keycloak_organization_id);
```

### Migration Script Approach

```rust
// Pseudocode for migration
async fn migrate_tenants_to_organizations(pool: &PgPool) -> Result<()> {
    // 1. Get all tenants
    let tenants = sqlx::query!("SELECT id, name, slug FROM tenants")
        .fetch_all(pool)
        .await?;
    
    // 2. For each tenant, create Keycloak organization
    for tenant in tenants {
        let org_id = create_keycloak_organization(&tenant).await?;
        
        // 3. Store organization ID in database
        sqlx::query!(
            "UPDATE tenants SET keycloak_organization_id = $1 WHERE id = $2",
            org_id, tenant.id
        ).execute(pool).await?;
        
        // 4. Add users to organization
        let users = get_users_for_tenant(pool, tenant.id).await?;
        for user in users {
            add_user_to_organization(&user.keycloak_user_id, &org_id).await?;
        }
    }
    
    Ok(())
}
```

---

## Backend Code Changes

### JWT Claims Struct

**Current (src/auth/jwt.rs):**
```rust
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub preferred_username: Option<String>,
    pub tenant_id: String,  // ← CHANGE THIS
    pub realm_access: RealmAccess,
    pub exp: usize,
    pub iat: usize,
}
```

**Updated:**
```rust
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub preferred_username: Option<String>,
    pub organization: String,  // ← RENAMED
    pub realm_access: RealmAccess,
    pub exp: usize,
    pub iat: usize,
}
```

### AuthenticatedUser Extractor

**Current (src/auth/extractor.rs):**
```rust
let tenant_id = Uuid::parse_str(&claims.tenant_id)
    .map(TenantId)
    .map_err(|e| AppError::Auth(format!("Invalid tenant ID in token: {}", e)))?;
```

**Updated:**
```rust
let tenant_id = Uuid::parse_str(&claims.organization)
    .map(TenantId)
    .map_err(|e| AppError::Auth(format!("Invalid organization ID in token: {}", e)))?;
```

---

## Frontend Code Changes

### OAuth2 Scope Update

**Current (frontend/src/lib/auth/keycloak.ts):**
```typescript
scope: 'openid profile email'
```

**Updated:**
```typescript
scope: 'openid profile email organization'
```

### Token Payload Type

**Current (frontend/src/types/user.ts):**
```typescript
export interface KeycloakTokenPayload {
  sub: string
  email: string
  preferred_username: string
  tenant_id: string  // ← CHANGE THIS
  realm_access: { roles: string[] }
}
```

**Updated:**
```typescript
export interface KeycloakTokenPayload {
  sub: string
  email: string
  preferred_username: string
  organization: string  // ← RENAMED
  realm_access: { roles: string[] }
}
```

### User Extraction

**Current (frontend/src/lib/auth/keycloak.ts):**
```typescript
tenant_id: payload.tenant_id,
```

**Updated:**
```typescript
tenant_id: payload.organization,  // Map organization claim to tenant_id
```

---

## Common Pitfalls

1. **Organization ID Format**: Keycloak may generate its own organization IDs. Ensure we use our existing tenant UUIDs as organization IDs for consistency.

2. **User Already in Organization**: Handle case where user might already be a member.

3. **Token Refresh**: After organization membership changes, users need new tokens to see updated claims.

4. **Migration Order**: 
   - Create organizations first
   - Then add users
   - Then update code
   - Otherwise, tokens won't have organization claim

5. **Rollback**: Keep migration reversible - organizations can be deleted, column can be dropped.

---

## Testing Strategy

1. **Unit Tests**: Update existing JWT validation tests for organization claim
2. **Integration Tests**: Verify tenant isolation still works with organization claim
3. **E2E Tests**: Full login flow with organization scope

---

## Rollback Plan

If migration fails:
1. Revert backend code changes
2. Revert frontend code changes
3. Delete created organizations via API
4. Drop keycloak_organization_id column
5. Users continue with tenant_id attribute

---

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| Keycloak | 26.x+ | Organizations feature |
| reqwest | (existing) | HTTP client for Keycloak API |
| serde_json | (existing) | JSON parsing for API responses |

---

## Open Questions

- [ ] Verify Keycloak version in cluster supports Organizations
- [ ] Confirm organization ID can be set to custom UUID (not auto-generated)
- [ ] Determine if migration script should be in Rust or standalone script

---

## Recommended Approach

1. **Wave 1**: Keycloak configuration (enable feature, create organizations, migrate users)
2. **Wave 2**: Backend JWT changes (Claims struct, extractor)
3. **Wave 3**: Frontend OAuth2 changes (scope, token parsing)

This order ensures:
- Organizations exist before code expects them
- Backend is ready before frontend requests new scope
- Users get new tokens with organization claim automatically

---

*Research completed: 2026-04-29*
