---
phase: 29-photo-upload-attachments
plan: 03
subsystem: ui
tags: [react-query, vitest, photo-upload, activity-feed]
requires:
  - phase: 29-02
    provides: backend photo upload endpoint and attachment URLs
provides:
  - typed frontend upload mutation for site photo attachments
  - photo creation flow in activity modal using camera/gallery file input
  - regression coverage for upload wiring and preview rendering
affects: [sites-ui, activity-feed, api-hooks]
tech-stack:
  added: []
  patterns: [two-step upload-then-create-activity flow, FormData transport via api client]
key-files:
  created: [frontend/src/lib/api/hooks/useSites.test.tsx]
  modified:
    - frontend/src/lib/api/client.ts
    - frontend/src/types/sites.ts
    - frontend/src/lib/api/hooks/useSites.ts
    - frontend/src/pages/sites/CreateNoteModal.tsx
    - frontend/src/pages/sites/ActivityFeed.tsx
    - frontend/src/pages/sites/ActivityFeed.test.tsx
key-decisions:
  - "Frontend photo flow uploads file first and reuses backend-provided photo_url for activity creation."
  - "ApiClient now detects FormData and skips JSON headers/serialization to preserve multipart requests."
patterns-established:
  - "Upload hooks should return typed attachment contracts consumed by feature UIs."
  - "Activity feed preview tests must cover both image-present and image-absent states."
requirements-completed: [FILE-01, FILE-02]
duration: 6 min
completed: 2026-05-01
---

# Phase 29 Plan 03: Photo Upload Attachments Summary

**Site activity creation now supports camera/gallery photo uploads with typed attachment responses and feed preview regression coverage.**

## Performance

- **Duration:** 6 min
- **Started:** 2026-05-01T11:57:47Z
- **Completed:** 2026-05-01T12:00:00Z
- **Tasks:** 3/3
- **Files modified:** 7

## Accomplishments
- Added `UploadPhotoAttachmentResponse` and a dedicated `useUploadSitePhoto` mutation for `/attachments/photo`.
- Implemented note/photo switch in `CreateNoteModal` with `accept="image/*"` + `capture="environment"`, upload-first workflow, and existing note path preserved.
- Added regression tests for upload wiring, upload error propagation, and activity feed photo preview present/absent rendering.

## task Commits

1. **Task 1: Add typed upload hook and attachment response contracts** - `797c442` (feat)
2. **Task 2: Implement camera/gallery picker flow in activity creation UI** - `12bfcdc` (feat)
3. **Task 3: Add regression tests for upload and preview states** - `54e5fdb` (test)

## Files Created/Modified
- `frontend/src/lib/api/hooks/useSites.test.tsx` - new hook tests for multipart upload contract and error propagation.
- `frontend/src/lib/api/client.ts` - FormData-aware request/POST handling for multipart uploads.
- `frontend/src/types/sites.ts` - typed upload response contract.
- `frontend/src/lib/api/hooks/useSites.ts` - `useUploadSitePhoto` mutation and typed return.
- `frontend/src/pages/sites/CreateNoteModal.tsx` - note/photo mode UI and upload-then-create activity flow.
- `frontend/src/pages/sites/ActivityFeed.tsx` - photo preview image rendering alt text alignment.
- `frontend/src/pages/sites/ActivityFeed.test.tsx` - preview image present/absent regression tests.

## Decisions Made
- Use backend-returned `photo_url` directly for activity creation to avoid frontend URL assembly and keep contract ownership server-side.
- Keep upload mutation separate from `useCreateActivity` to preserve existing activity invalidation behavior.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added multipart-safe API client behavior**
- **Found during:** Task 1 (upload hook implementation)
- **Issue:** Existing `apiClient.post` always JSON-stringified payloads and forced JSON content-type, which breaks FormData uploads.
- **Fix:** Added FormData detection in `request` and `post` so multipart requests are sent without JSON serialization/header override.
- **Files modified:** `frontend/src/lib/api/client.ts`
- **Verification:** `npm run test:run -- useSites`
- **Committed in:** `797c442`

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary for correctness of upload endpoint integration; no scope creep beyond upload transport support.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Photo upload-to-activity frontend path is implemented and covered by targeted tests.
- Ready for Plan 29-04.

## Self-Check: PASSED
