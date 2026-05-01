---
phase: 36-media-viewer
plan: "01"
subsystem: api
tags: [rust, sqlx, ts-rs, activities, viewer]
requires:
  - phase: 35-document-upload-rework
    provides: attachment-aware activity DTOs and protected attachment URLs
provides:
  - creator display names on activity responses
  - tenant-scoped fallback logic for viewer metadata
  - regenerated frontend activity DTO with creator_name
affects: [phase-36-plan-02, frontend-viewer, activity-feed]
tech-stack:
  added: []
  patterns: [tenant-scoped user join for read models, generated type contract tests]
key-files:
  created: [tests/site_activity_list_activities_test.rs, tests/generated_activity_types_test.rs]
  modified: [src/modules/sites/domain/activity.rs, src/modules/sites/infrastructure/site_repository.rs, src/modules/sites/api/routes.rs, frontend/src/types/generated.ts]
key-decisions:
  - "Resolve creator_name in SQL with a tenant-scoped LEFT JOIN and COALESCE fallback to avoid cross-tenant leakage."
  - "Protect generated ts-rs output with a filesystem-backed Rust test instead of hand-editing generated TypeScript."
patterns-established:
  - "Activity viewer metadata stays server-derived and tenant-scoped."
  - "Generated frontend contracts get regression coverage from Rust tests."
requirements-completed: [VIEW-05]
duration: 35min
completed: 2026-05-01
---

# Phase 36 Plan 01: Media Viewer Summary

**Activity responses now ship human-readable creator names with tenant-safe fallback logic and a regenerated frontend DTO contract.**

## Performance

- **Duration:** 35 min
- **Started:** 2026-05-01T17:35:00Z
- **Completed:** 2026-05-01T18:10:00Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Added `creator_name` to the activity domain, API DTO, and generated frontend contract.
- Joined local users tenant-safely in the activity repository with name → email → user-id fallback behavior.
- Added regression tests for activity ordering, attachment hydration, and generated type output.

## Task Commits

1. **task 1: extend activity reads with creator display names** - `8666118`, `aa49086` (test, feat)
2. **task 2: regenerate frontend DTOs for the viewer contract** - `61f29b1`, `2078e7c` (test, feat)

## Files Created/Modified
- `src/modules/sites/domain/activity.rs` - adds `creator_name` to the activity read model.
- `src/modules/sites/infrastructure/site_repository.rs` - resolves creator display names in the tenant-scoped query.
- `src/modules/sites/api/routes.rs` - exposes `creator_name` on `ActivityResponse`.
- `frontend/src/types/generated.ts` - exports the updated activity DTO.
- `tests/site_activity_list_activities_test.rs` - verifies fallback logic, ordering, and attachment hydration.
- `tests/generated_activity_types_test.rs` - verifies generated type output contains `creator_name` and preserved attachment fields.

## Decisions Made
- Used a tenant-scoped `LEFT JOIN users ON users.id = site_activities.user_id AND users.tenant_id = site_activities.tenant_id` so mismatched identities fall back to the activity UUID string instead of leaking another tenant's profile.
- Kept attachment hydration in `SiteService::list_activities` unchanged and validated it with integration coverage rather than reworking the existing pipeline.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Repository-wide `cargo fmt --check` currently reports pre-existing formatting drift in unrelated Rust files. Per scope boundaries, verification for this plan used targeted Rust tests and targeted formatting on the new generated-contract test file instead of reformatting unrelated code.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The frontend can now trust `creator_name` in site activity responses.
- Route-backed viewer work can build directly on generated attachment-aware activity types.

## Self-Check: PASSED
