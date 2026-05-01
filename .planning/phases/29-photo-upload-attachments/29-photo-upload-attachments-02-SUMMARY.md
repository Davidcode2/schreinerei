---
phase: 29-photo-upload-attachments
plan: 02
subsystem: api
tags: [axum, multipart, image, ts-rs, attachments]
requires:
  - phase: 29-01
    provides: tenant-scoped attachment persistence and read endpoints
provides:
  - Multipart photo upload endpoint with typed frontend DTO contract
  - UUID-based original/thumbnail storage key generation
  - Immediate activity photo_url linkage to attachment-backed routes
affects: [activity-feed, offline-photo-sync, frontend-upload-ui]
tech-stack:
  added: [image]
  patterns: [server-side MIME+size validation, tenant-bound attachment linking, opaque attachment URLs]
key-files:
  created: []
  modified:
    - Cargo.toml
    - Cargo.lock
    - src/modules/sites/api/routes.rs
    - src/modules/sites/application/site_service.rs
    - src/modules/sites/infrastructure/site_repository.rs
    - frontend/src/types/generated.ts
key-decisions:
  - "Create photo activity during upload and update it with canonical attachment URL after persistence."
  - "Use image crate thumbnail(320x320 max edge) and preserve MIME-specific output format."
patterns-established:
  - "Attachment public paths are always attachment-id based, never storage-key based."
  - "Upload boundary enforces MIME allowlist and size cap before image processing."
requirements-completed: [FILE-01, FILE-03, FILE-04, FILE-07]
duration: 27 min
completed: 2026-05-01
---

# Phase 29 Plan 02: Upload + Thumbnail Pipeline Summary

**Axum multipart upload now creates photo activities with UUID-backed attachment assets and immediate thumbnail-preview URLs.**

## Performance

- **Duration:** 27 min
- **Started:** 2026-05-01T10:00:00Z
- **Completed:** 2026-05-01T10:27:00Z
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments
- Added `POST /api/v1/sites/{id}/attachments/photo` with multipart extraction and typed TS-exported response (`attachment_id`, `photo_url`, `thumbnail_url`).
- Implemented upload pipeline validation (MIME allowlist + size limit), thumbnail generation, and server-side UUID storage-key creation for original + thumbnail.
- Linked uploaded attachments to photo activity records by updating activity `photo_url` to stable attachment routes under tenant/site constraints.

## Task Commits

1. **Task 1: Add multipart upload route and DTO contracts** - `98cebe9` (feat)
2. **Task 2: Implement UUID storage + thumbnail generation flow** - `4ddab76` (feat)
3. **Task 3: Link uploaded attachment to photo activity feed records** - `237c917` (feat)

## Files Created/Modified
- `src/modules/sites/api/routes.rs` - multipart upload endpoint and upload response DTO export.
- `src/modules/sites/application/site_service.rs` - upload orchestration, validation, thumbnail generation, URL construction.
- `src/modules/sites/infrastructure/site_repository.rs` - activity `photo_url` update method for attachment linkage.
- `Cargo.toml` / `Cargo.lock` - multipart + image processing dependencies.
- `frontend/src/types/generated.ts` - regenerated TS bindings for upload response.

## Decisions Made
- Persist attachment first-class metadata and derive public URLs from `attachment_id` routes to avoid exposing storage internals.
- Enforce validation at API/service boundary before thumbnail decode to mitigate oversized/invalid payload abuse.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Corrected plan-provided cargo test invocation syntax**
- **Found during:** task 1 verification
- **Issue:** `cargo test sites --lib upload -- --nocapture` is not a valid Cargo CLI argument order.
- **Fix:** Executed equivalent scoped verifications with valid syntax: `cargo test --lib upload -- --nocapture`, `cargo test --lib thumbnail -- --nocapture`, and `cargo test --lib activity -- --nocapture`.
- **Files modified:** None
- **Verification:** All scoped suites and full `cargo test sites --lib` passed.
- **Committed in:** N/A (verification-only deviation)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** No scope creep; deviation only adjusted command form to run required verification successfully.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Backend upload contract and activity linkage are complete and test-covered.
- Ready for frontend capture/gallery integration in follow-up plans.

## Self-Check: PASSED
