---
phase: 06-organization-tenancy
plan: 03
subsystem: frontend
tags: [oauth2, organization, scope, token]
dependency_graph:
  requires:
    - "06-02-PLAN.md (backend JWT changes)"
  provides:
    - "OAuth2 flow with organization scope"
    - "Token parsing for organization claim"
key-files:
  modified:
    - path: "frontend/src/types/user.ts"
      purpose: "Updated KeycloakTokenPayload with organization field"
    - path: "frontend/src/lib/auth/keycloak.ts"
      purpose: "Updated OAuth2 scope and token extraction"
decisions:
  - "OAuth2 scope includes 'organization' for organization claim request"
  - "KeycloakTokenPayload uses organization field instead of tenant_id"
  - "User interface tenant_id field name unchanged (internal representation)"
  - "Mapping: payload.organization → user.tenant_id"
metrics:
  duration_minutes: 2
  completed_date: 2026-04-29
  task_count: 3
  file_count: 2
---

# Phase 6 Plan 03: Frontend OAuth2 Scope Update Summary

Updated frontend OAuth2 flow to request organization scope and handle organization claim in JWT tokens.

## What Was Built

### Type Updates
- **frontend/src/types/user.ts** — Changed `KeycloakTokenPayload.tenant_id` to `organization`
- `User` interface unchanged — still uses `tenant_id` internally

### OAuth2 Flow Updates
- **frontend/src/lib/auth/keycloak.ts**:
  - Scope updated: `openid profile email organization`
  - `extractUserFromToken` maps `payload.organization` to `user.tenant_id`

## Key Decisions

1. **Scope includes organization** — Requests organization claim from Keycloak
2. **Internal field name unchanged** — User.tenant_id stays the same, minimizing codebase changes
3. **Mapping at extraction** — organization claim is mapped to tenant_id at token parsing time

## Code Changes

### Before
```typescript
// user.ts
export interface KeycloakTokenPayload {
  tenant_id: string
}

// keycloak.ts
scope: 'openid profile email',
tenant_id: payload.tenant_id,
```

### After
```typescript
// user.ts
export interface KeycloakTokenPayload {
  organization: string
}

// keycloak.ts
scope: 'openid profile email organization',
tenant_id: payload.organization,
```

## Verification

- [x] OAuth2 scope includes 'organization'
- [x] KeycloakTokenPayload has organization field
- [x] extractUserFromToken maps organization to tenant_id
- [x] `npm run build` succeeds
- [x] Commit created

## End-to-End Flow

1. User clicks login → redirected to Keycloak with `organization` scope
2. Keycloak returns JWT with `organization` claim (if user is org member)
3. Frontend parses token → extracts `organization` → maps to `tenant_id`
4. All existing code using `user.tenant_id` continues to work

---

*Completed: 2026-04-29*
