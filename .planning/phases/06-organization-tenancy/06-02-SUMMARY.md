---
phase: 06-organization-tenancy
plan: 02
subsystem: auth
tags: [jwt, claims, organization, tenant]
dependency_graph:
  requires:
    - "06-01-PLAN.md (database migration)"
  provides:
    - "JWT Claims with organization field"
    - "AuthenticatedUser extraction from organization claim"
  affects:
    - "06-03-PLAN.md (frontend needs matching changes)"
key-files:
  modified:
    - path: "src/auth/jwt.rs"
      purpose: "Updated Claims struct with organization field"
    - path: "src/auth/extractor.rs"
      purpose: "Updated to parse organization claim"
decisions:
  - "Renamed tenant_id field to organization in Claims struct"
  - "TenantId wrapper type unchanged - same type, different source"
  - "Error messages updated to reference organization"
metrics:
  duration_minutes: 2
  completed_date: 2026-04-29
  task_count: 2
  file_count: 2
---

# Phase 6 Plan 02: Backend JWT Migration Summary

Updated Rust backend to extract TenantId from Keycloak organization claim instead of tenant_id attribute.

## What Was Built

### JWT Claims Update
- **src/auth/jwt.rs** — Changed `tenant_id` field to `organization` in Claims struct
- Updated field documentation to reference Keycloak Organizations feature

### Extractor Update
- **src/auth/extractor.rs** — Updated `from_claims` to parse `claims.organization`
- Error messages updated to reference organization ID

## Key Decisions

1. **Field renamed, type unchanged** — `Claims.organization` replaces `Claims.tenant_id`, but `TenantId` wrapper type remains the same
2. **Same extraction logic** — UUID parsing and wrapping in TenantId is identical, just different claim source
3. **Backward compatible internally** — All code using TenantId continues to work unchanged

## Code Changes

### Before
```rust
pub struct Claims {
    // ...
    pub tenant_id: String,
}

let tenant_id = Uuid::parse_str(&claims.tenant_id)
    .map(TenantId)
    .map_err(|e| AppError::Auth(format!("Invalid tenant ID in token: {}", e)))?;
```

### After
```rust
pub struct Claims {
    // ...
    pub organization: String,
}

let tenant_id = Uuid::parse_str(&claims.organization)
    .map(TenantId)
    .map_err(|e| AppError::Auth(format!("Invalid organization ID in token: {}", e)))?;
```

## Verification

- [x] Claims struct uses `organization` field
- [x] Extractor parses `claims.organization`
- [x] `cargo build` succeeds
- [x] Commit created

## Next Steps

Plan 06-03 will update the frontend OAuth2 scope and token parsing to match.

---

*Completed: 2026-04-29*
