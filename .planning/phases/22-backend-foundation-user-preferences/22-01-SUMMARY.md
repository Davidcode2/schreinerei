---
phase: 22-backend-foundation-user-preferences
plan: 01
subsystem: iam
tags: [backend, rust, repository, service, validation]
requires: [PREF-01, PREF-02, PREF-03]
provides: [UserPreferencesRepository, UserPreferencesService]
affects: [iam-module, sites-module]
tech-stack:
  added:
    - UserPreferencesRepository (infrastructure layer)
    - UserPreferencesService (application layer)
  patterns:
    - Repository pattern for data access
    - Service layer with validation
    - Upsert pattern for atomic updates
    - JSONB merge for partial preference updates
key-files:
  created:
    - src/modules/iam/infrastructure/user_preferences_repository.rs
    - src/modules/iam/application/user_preferences_service.rs
  modified:
    - src/modules/iam/infrastructure.rs
    - src/modules/iam/application.rs
decisions:
  - Use upsert (INSERT ON CONFLICT) for atomic preference creation and updates
  - Store preferences as JSONB for flexible schema evolution
  - Validate site ownership and status before allowing preference update
  - Auto-clear active site preference when site becomes invalid
metrics:
  duration: 1m
  completed_date: 2026-04-30
  tasks_completed: 2
  files_created: 2
  files_modified: 2
---

# Phase 22 Plan 01: User Preferences Repository and Service Summary

Created repository and service layer for user preferences with site validation logic, enabling backend storage and validation of user's active Baustelle preference.

## Completed Tasks

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Create UserPreferencesRepository | `9c918e8` | src/modules/iam/infrastructure/user_preferences_repository.rs, src/modules/iam/infrastructure.rs |
| 2 | Create UserPreferencesService with validation | `b973b4a` | src/modules/iam/application/user_preferences_service.rs, src/modules/iam/application.rs |

## Implementation Details

### Task 1: UserPreferencesRepository

Created the repository layer with three core methods:

- **`get_or_create`**: Atomically creates default preferences if not exists using INSERT ON CONFLICT DO NOTHING, then SELECTs existing record. Ensures every user has a preferences record without race conditions.

- **`update`**: Uses INSERT ON CONFLICT with JSONB merge (`preferences || $1::jsonb`) to atomically update specific preference fields while preserving others. Creates record if missing.

- **`clear_active_site`**: Convenience method that updates preferences with `{"active_site_id": null}` via JSONB merge.

Key design decisions:
- Upsert pattern ensures atomicity without separate existence checks
- JSONB storage allows flexible preference schema evolution
- All queries include tenant_id in WHERE clause for multi-tenant isolation

### Task 2: UserPreferencesService

Created the service layer with validation logic:

- **`get_preferences`**: Simple pass-through to repository's get_or_create.

- **`set_active_site`**: Validates site exists, belongs to tenant, and is not archived before updating. Returns error for invalid sites. This addresses threat T-22-01-01 (elevation of privilege) by ensuring users can only set sites within their tenant.

- **`clear_active_site`**: Clears the active site preference and returns updated record.

- **`get_validated_preferences`**: Retrieves preferences and validates the active site is still valid. Auto-clears if site is deleted, archived, or otherwise invalid. This addresses the requirement that the system clears preference automatically if Baustelle becomes invalid.

Key design decisions:
- Site validation reuses existing SiteRepository.find_site_by_id which checks deleted_at and tenant_id
- Archived status check prevents setting completed projects as active
- Auto-clear behavior runs transparently on retrieval, not on background job

## Threat Model Compliance

| Threat ID | Mitigation |
|-----------|------------|
| T-22-01-01 | Site ownership validated via tenant_id in find_site_by_id query |
| T-22-01-02 | All update queries include tenant_id in WHERE clause |
| T-22-01-03 | Users can only access their own preferences (user_id + tenant_id composite key) |

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check

- [x] Files exist: src/modules/iam/infrastructure/user_preferences_repository.rs
- [x] Files exist: src/modules/iam/application/user_preferences_service.rs
- [x] Commits exist: 9c918e8, b973b4a
- [x] cargo check passes
