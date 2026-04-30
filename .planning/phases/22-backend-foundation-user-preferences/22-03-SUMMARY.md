---
phase: 22-backend-foundation-user-preferences
plan: 03
subsystem: iam
tags: [api, preferences, endpoints, ts-rs]
dependency_graph:
  requires:
    - 22-01 (UserPreferencesService and domain types)
  provides:
    - GET /api/v1/preferences endpoint
    - PATCH /api/v1/preferences endpoint
    - PreferencesResponse DTO
    - UpdatePreferencesRequest DTO
  affects:
    - Frontend preferences API calls
tech-stack:
  added:
    - axum routing with GET/PATCH on same path
  patterns:
    - DTO pattern with ts-rs type generation
    - TenantContext for tenant-scoped operations
key-files:
  created: []
  modified:
    - src/modules/iam/api/routes.rs
    - frontend/src/types/generated.ts
decisions:
  - Use SiteId::parse() instead of Uuid::parse_str() for cleaner code
  - Use existing TenantContext pattern for consistency
metrics:
  duration: ~5 minutes
  completed_date: 2026-04-30
---

# Phase 22 Plan 03: Preferences API Endpoints Summary

## One-liner

Added GET/PATCH /api/v1/preferences endpoints to expose user preferences service via HTTP with TypeScript type generation.

## Completed Tasks

| Task | Name | Commit | Status |
|------|------|--------|--------|
| 1 | Add preferences routes to IAM router | 60b7644 | ✅ |

## Key Changes

### API Endpoints

**GET /api/v1/preferences**
- Returns current user's validated preferences
- Auto-clears invalid active_site_id (deleted/archived sites)
- Returns `PreferencesResponse` with `active_site_id: Option<String>`

**PATCH /api/v1/preferences**
- Updates user's active site preference
- Accepts `UpdatePreferencesRequest` with `active_site_id: Option<String>`
- Validates site exists and belongs to tenant
- Returns updated `PreferencesResponse`

### DTOs

```rust
pub struct PreferencesResponse {
    pub active_site_id: Option<String>,
}

pub struct UpdatePreferencesRequest {
    pub active_site_id: Option<String>,
}
```

Both DTOs have `#[ts(export)]` for automatic TypeScript generation.

## Deviations from Plan

None - plan executed exactly as written.

## Threat Mitigations Applied

| Threat ID | Category | Mitigation |
|-----------|----------|------------|
| T-22-03-01 | Elevation of Privilege | Service layer validates site belongs to user's tenant before assignment |
| T-22-03-02 | Information Disclosure | User only sees their own preferences; tenant_id enforced at service layer |
| T-22-03-03 | Denial of Service | Parse UUID with error handling, return validation error on malformed input |

## Verification

- [x] `cargo check` passes
- [x] Route `/api/v1/preferences` registered for GET and PATCH
- [x] PreferencesResponse and UpdatePreferencesRequest DTOs defined
- [x] ts-rs export annotations present for TypeScript generation

## Self-Check: PASSED

- Created files: N/A (modifications only)
- Commits: 60b7644 exists
