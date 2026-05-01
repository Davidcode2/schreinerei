---
phase: 35-document-upload-rework
plan: 01
subsystem: api
tags: [rust, axum, sqlx, postgres, ts-rs, attachments]
requires:
  - phase: 34-camera-upload-flow
    provides: separate camera upload flow that must remain backward-compatible
  - phase: 29-photo-upload-attachments
    provides: upload-first attachment persistence and authenticated blob fetches
provides:
  - Multi-attachment activity contracts for note-only, attachment-only, and mixed document entries
  - Generic image/PDF upload endpoint with tenant-scoped attachment linking
  - Attachment-aware activity read DTOs for downstream frontend feed and modal work
affects: [sites, activity-feed, document-modal, camera-upload, frontend-types]
tech-stack:
  added: []
  patterns: [upload-first then create-activity, tenant-scoped attachment linking, attachment array DTOs]
key-files:
  created: [migrations/014_document_attachments_multi_file.sql]
  modified:
    - src/modules/sites/domain/activity.rs
    - src/modules/sites/application/site_service.rs
    - src/modules/sites/infrastructure/site_repository.rs
    - src/modules/sites/api/routes.rs
    - frontend/src/types/generated.ts
key-decisions:
  - "Kept /attachments/photo response contract for the Phase 34 camera flow while adding a new generic /attachments endpoint for document uploads."
  - "Hydrate activity attachments through tenant-scoped repository queries instead of overloading photo_url for multi-file entries."
patterns-established:
  - "Document activities are valid when trimmed content or at least one uploaded attachment ID is present."
  - "PDF uploads skip thumbnail generation but still reuse the authenticated attachment URL pipeline."
requirements-completed: [DOC-01, DOC-02, DOC-03, DOC-04, DOC-05]
duration: 12 min
completed: 2026-05-01
---

# Phase 35 Plan 01: Document Upload Rework Summary

**Attachment-aware site activities now support mixed notes, images, and PDFs through tenant-scoped upload/link/read contracts without regressing the camera upload flow.**

## Performance

- **Duration:** 12 min
- **Started:** 2026-05-01T13:39:37Z
- **Completed:** 2026-05-01T13:51:35Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Added a migration and domain validation rules for multi-attachment document activities.
- Introduced generic attachment upload/read DTOs and regenerated frontend types for attachment arrays.
- Linked uploaded attachments to created activities within tenant/site scope and returned attachment metadata on activity reads.

## Task Commits

Each task was committed atomically:

1. **task 1: establish multi-attachment activity contracts and schema** - `e4c5781` (test), `d67a3be` (feat)
2. **task 2: implement generic upload, attachment linking, and activity reads** - `eeda8aa` (test), `03b92ed` (feat)

_Note: TDD tasks used separate red/green commits._

## Files Created/Modified
- `migrations/014_document_attachments_multi_file.sql` - removes the one-attachment-per-activity constraint and stores original filenames.
- `src/modules/sites/domain/activity.rs` - adds attachment metadata, attachment IDs, and note-or-attachment validation.
- `src/modules/sites/application/site_service.rs` - handles PDF-safe uploads, conditional thumbnails, attachment linking, and attachment hydration.
- `src/modules/sites/infrastructure/site_repository.rs` - persists filename metadata, links multiple attachments, and loads attachment collections per activity.
- `src/modules/sites/api/routes.rs` - exposes generic upload contracts and attachment-aware activity DTOs while preserving the camera endpoint.
- `frontend/src/types/generated.ts` - exports attachment-aware activity and upload request/response types.

## Decisions Made
- Preserved the legacy photo upload route and response so Phase 34 camera UX keeps working while new document uploads move to `/api/v1/sites/{id}/attachments`.
- Used attachment arrays on activities as the canonical document contract instead of stretching `photo_url` beyond single-photo semantics.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Replaced the invalid ts-rs export command with the repository's real export path**
- **Found during:** task 1 (establish multi-attachment activity contracts and schema)
- **Issue:** `cargo test --features ts-rs/export` fails because the pinned `ts-rs` crate version has no `export` feature.
- **Fix:** Used the documented `#[ts(export)]` behavior for this version and ran plain `cargo test` to regenerate `frontend/src/types/generated.ts`.
- **Files modified:** `frontend/src/types/generated.ts`
- **Verification:** `cargo test`
- **Committed in:** `d67a3be`

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** No scope creep; the fix was required to regenerate real TypeScript bindings in this repository.

## Issues Encountered
- The generated type file refreshed additional pre-existing exports when `cargo test` re-ran all ts-rs binding tests; the attachment contract changes remained the only intentional API changes in this plan.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Backend contracts are ready for the document composer and feed UI plans in Wave 2.
- Camera uploads remain backward-compatible while the frontend migrates to the generic attachment endpoint.

## Self-Check: PASSED
