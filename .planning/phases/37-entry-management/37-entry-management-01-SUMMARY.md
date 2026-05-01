---
phase: 37-entry-management
plan: "01"
subsystem: api
tags: [rust, axum, sqlx, activity-feed, deletion]
requires:
  - phase: 36-media-viewer
    provides: activity feed DTOs, protected attachment urls, and viewer-safe photo_url conventions
provides:
  - creator-only activity delete endpoint
  - server-derived can_delete contract for activity DTOs
  - attachment cleanup for linked documents and legacy photo uploads
affects: [phase-37-ui, activity-stream, attachment-cleanup]
tech-stack:
  added: []
  patterns: [server-derived activity permissions, tenant-scoped destructive activity operations]
key-files:
  created: []
  modified: [src/modules/sites/api/routes.rs, src/modules/sites/application/site_service.rs, src/modules/sites/infrastructure/site_repository.rs, src/modules/sites/domain/activity.rs, frontend/src/types/generated.ts, tests/site_activity_list_activities_test.rs]
key-decisions:
  - "Expose only a boolean can_delete bit from the API so the browser never compares Keycloak subjects to tenant-local activity user ids."
  - "Treat note/photo entries as hard-deletable but keep status_change entries immutable workflow history."
patterns-established:
  - "Activity deletion always scopes lookup and delete by tenant_id + site_id + activity_id before mutating rows."
  - "Legacy photo activities clean up their protected attachment row by parsing the attachment UUID from photo_url before deleting the activity."
requirements-completed: [ENTRY-01, ENTRY-03]
duration: 25min
completed: 2026-05-01
---

# Phase 37 Plan 01: Entry Management Summary

**Creator-only activity deletion now ships as a tenant-scoped backend contract with API-owned permission bits and attachment-safe cleanup.**

## Performance

- **Duration:** 25 min
- **Started:** 2026-05-01T16:10:00Z
- **Completed:** 2026-05-01T16:35:05Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Added a server-derived `can_delete` flag to activity DTOs so the frontend can trust backend permissions directly.
- Added `DELETE /api/v1/sites/{site_id}/activities/{activity_id}` with creator-only enforcement for note/photo entries.
- Verified that deletion removes both linked document attachments and legacy photo attachments referenced through protected `photo_url` paths.

## Task Commits

1. **task 1: expose a server-derived deletable activity contract** - `248ca54` (feat)
2. **task 2: implement creator-only delete endpoint with full attachment cleanup** - `7888260` (feat)

## Files Created/Modified
- `src/modules/sites/api/routes.rs` - extends `ActivityResponse` with `can_delete` and adds the activity delete route.
- `src/modules/sites/application/site_service.rs` - derives delete permissions, validates ownership, and orchestrates attachment cleanup.
- `src/modules/sites/infrastructure/site_repository.rs` - adds scoped activity lookup/delete helpers plus attachment row deletion.
- `src/modules/sites/domain/activity.rs` - stores the backend-derived `can_delete` bit on the activity aggregate.
- `frontend/src/types/generated.ts` - refreshes the generated activity DTO with `can_delete`.
- `tests/site_activity_list_activities_test.rs` - covers owner deletes, forbidden deletes, and photo/document cleanup paths.

## Decisions Made
- Used the existing tenant-local user resolution path in `SiteService` for delete authorization instead of introducing any client-supplied identity hint.
- Kept status-change entries non-deletable to preserve workflow history even for the same creator.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated stale ts-rs verification command to the repo’s v12 export flow**
- **Found during:** task 1 (expose a server-derived deletable activity contract)
- **Issue:** `cargo test --features ts-rs/export` fails because `ts-rs` v12 no longer exposes that Cargo feature in this repository.
- **Fix:** Verified exports with `cargo test export_bindings`, which is how `#[ts(export)]` runs in the current dependency version.
- **Files modified:** none
- **Verification:** `cargo test export_bindings`
- **Committed in:** `248ca54` (part of task commit)

**2. [Rule 1 - Bug] Corrected the SQL fixture’s Keycloak-to-local-user mapping for delete ownership tests**
- **Found during:** task 2 (implement creator-only delete endpoint with full attachment cleanup)
- **Issue:** the integration fixture stored a synthetic `keycloak_user_id` that could never resolve back to the seeded local user, causing false forbidden results.
- **Fix:** aligned the fixture with production by storing the same UUID string that `TenantContext` resolves through `find_or_create_by_keycloak_id`.
- **Files modified:** tests/site_activity_list_activities_test.rs
- **Verification:** `cargo test delete_activity && cargo test delete_activity_photo_cleanup`
- **Committed in:** `7888260` (part of task commit)

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both fixes were required to verify the planned backend contract correctly. No scope creep.

## Issues Encountered

- The original plan verification command assumed a retired ts-rs feature flag, so verification had to follow the library’s current export test path.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The frontend can now trust `activity.can_delete` and call a stable delete endpoint.
- Attachment cleanup semantics are fixed for both linked documents and older photo-only upload rows.

## Self-Check: PASSED
