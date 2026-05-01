---
phase: 29-photo-upload-attachments
plan: 06
subsystem: api
tags: [rust, sqlx, postgres, react, vitest, auth]
requires:
  - phase: 29-photo-upload-attachments
    provides: attachment upload/read pipeline and activity feed photo rendering
provides:
  - Upload endpoint stores attachment bytes without creating a hidden photo activity row
  - Protected attachment previews render through authenticated blob fetches
  - Regression tests for upload-photo invariants and authenticated preview path
affects: [sites, attachments, activity-feed, tenant-auth]
tech-stack:
  added: []
  patterns: [upload-first attachment persistence with explicit activity creation, authenticated binary preview loading]
key-files:
  created: [migrations/013_make_attachment_activity_optional.sql]
  modified:
    - src/modules/sites/application/site_service.rs
    - src/modules/sites/domain/activity.rs
    - src/modules/sites/infrastructure/site_repository.rs
    - frontend/src/lib/api/client.ts
    - frontend/src/pages/sites/ActivityFeed.tsx
    - frontend/src/pages/sites/ActivityFeed.test.tsx
key-decisions:
  - "Option A approved: make site_activity_attachments.activity_id nullable so upload no longer creates implicit activity rows"
  - "Protected /api/v1/attachments URLs must be loaded via authenticated apiClient.getBlob + object URLs"
patterns-established:
  - "Attachment upload endpoint performs storage only; business event creation remains in explicit createActivity path"
  - "Protected binary media is rendered via authenticated fetch, never direct unauthenticated <img src>"
requirements-completed: [FILE-01, FILE-02, FILE-07]
duration: 31min
completed: 2026-05-01
---

# Phase 29 Plan 06: Gap Closure Summary

**Photo uploads now persist attachment bytes without hidden activity side-effects, and attachment previews load through authenticated blob fetches to prevent browser 401s.**

## Performance

- **Duration:** 31 min
- **Started:** 2026-05-01T13:00:00Z
- **Completed:** 2026-05-01T13:31:00Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Removed duplicate activity creation by decoupling upload attachment persistence from activity row creation.
- Added migration to allow attachment records before explicit activity linkage (`activity_id` nullable).
- Implemented authenticated preview loading for protected attachment URLs in the activity feed.

## Task Commits

1. **Task 1: Remove duplicate activity creation from attachment upload path** - `6916733` (fix)
2. **Task 2: Render protected activity images through authenticated fetch path** - `a20456d` (feat)

## Files Created/Modified
- `migrations/013_make_attachment_activity_optional.sql` - Drops NOT NULL on `site_activity_attachments.activity_id`.
- `src/modules/sites/application/site_service.rs` - Upload flow no longer creates/updates activity rows; preserves response contract.
- `src/modules/sites/domain/activity.rs` - Attachment model supports optional activity association.
- `src/modules/sites/infrastructure/site_repository.rs` - Persists/loads nullable `activity_id`.
- `frontend/src/lib/api/client.ts` - Adds authenticated `getBlob` helper.
- `frontend/src/pages/sites/ActivityFeed.tsx` - Loads protected attachment previews via blob/object URL with cleanup.
- `frontend/src/pages/sites/ActivityFeed.test.tsx` - Verifies authenticated fetch path and object URL lifecycle.

## Decisions Made
- Applied user-approved **Option A** architectural adjustment to make `activity_id` nullable and remove implicit upload-side activity creation.
- Kept tenant-scoped authorization unchanged by leaving protected attachment endpoints and tenant lookup behavior intact.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 4 - Architectural] Optional attachment-to-activity linkage required by schema**
- **Found during:** task 1
- **Issue:** Existing schema required non-null `activity_id`, blocking removal of upload-side hidden activity creation.
- **Fix:** Added migration making `activity_id` nullable and updated domain/repository mappings to support `Option<ActivityId>`.
- **Files modified:** `migrations/013_make_attachment_activity_optional.sql`, `src/modules/sites/domain/activity.rs`, `src/modules/sites/infrastructure/site_repository.rs`
- **Verification:** `cargo test upload_photo -- --nocapture`
- **Committed in:** `6916733`

---

**Total deviations:** 1 (1 architectural decision approved by user)
**Impact on plan:** Required to satisfy duplicate-activity fix safely; no extra scope beyond blocker resolution.

## Authentication Gates

None.

## Issues Encountered
- Vitest mock hoisting issue in `ActivityFeed.test.tsx` (`Cannot access 'getBlobMock' before initialization`) fixed by mocking `apiClient.getBlob` inside module factory and deriving typed mock after import.

## Known Stubs

None.

## Next Phase Readiness
- Plan 29-06 blockers are resolved in code and tests.
- Phase 29 is ready for verification/signoff against human runtime flow.

## Self-Check: PASSED
