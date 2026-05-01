---
phase: 29-photo-upload-attachments
plan: 01
subsystem: api
tags: [attachments, tenant-isolation, postgres, axum]
requires:
  - phase: 28-material-history-tab
    provides: tenant-scoped site/activity patterns reused for attachment auth
provides:
  - tenant-scoped attachment schema for site activities
  - repository contracts for attachment persistence and lookup
  - authenticated read endpoints for original/thumbnail bytes
affects: [29-02-upload-pipeline, activity-feed-photo-preview]
tech-stack:
  added: []
  patterns: [tenant_id-bound attachment queries, opaque attachment-id public routes]
key-files:
  created: [migrations/012_site_activity_attachments.sql]
  modified: [src/modules/sites/domain/activity.rs, src/modules/sites/infrastructure/site_repository.rs, src/modules/sites/api/routes.rs]
key-decisions:
  - "Attachment API uses opaque attachment UUID routes; storage keys remain internal only."
  - "Attachment reads return NotFound when tenant-scoped lookup fails to avoid cross-tenant disclosure."
patterns-established:
  - "Repository methods that access attachment records always take tenant_id."
requirements-completed: [FILE-03, FILE-07]
duration: 2 min
completed: 2026-05-01
---

# Phase 29 Plan 01: Attachment contract and tenant-authorized read path Summary

**Tenant-scoped attachment records and byte read endpoints now enforce attachment_id + tenant_id authorization without exposing storage keys.**

## Performance

- **Duration:** 2 min
- **Started:** 2026-05-01T09:41:00Z
- **Completed:** 2026-05-01T09:42:53Z
- **Tasks:** 2/2
- **Files modified:** 4

## Accomplishments
- Added migration `012_site_activity_attachments.sql` with tenant ownership, FK link to `site_activities`, and attachment indexes.
- Added `SiteActivityAttachment` domain and repository create/find methods with mandatory tenant scoping.
- Added `GET /api/v1/attachments/{attachment_id}` and `/thumbnail` routes resolving tenant from JWT context only.

## Task Commits

1. **Task 1: Define attachment persistence contracts and migration** - `5659389` (feat)
2. **Task 2: Add tenant-authorized attachment retrieval endpoints** - `cb32ee1` (feat)

## Files Created/Modified
- `migrations/012_site_activity_attachments.sql` - New attachment table contract and indexes.
- `src/modules/sites/domain/activity.rs` - Attachment metadata model aligned with tenant ownership.
- `src/modules/sites/infrastructure/site_repository.rs` - Tenant-scoped attachment insert/lookup contracts.
- `src/modules/sites/api/routes.rs` - Authorized original/thumbnail read endpoints.

## Decisions Made
- Used opaque attachment UUID endpoints instead of storage-key paths to prevent key leakage.
- Kept `photo_url` compatibility on activity rows while moving canonical attachment tracking into dedicated rows.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 29-01 now provides the schema and authorization boundary needed by 29-02 upload + thumbnail generation.
- No blockers identified.

## Self-Check: PASSED
