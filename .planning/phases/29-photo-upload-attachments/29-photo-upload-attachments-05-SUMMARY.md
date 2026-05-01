---
phase: 29-photo-upload-attachments
plan: 05
subsystem: ui
tags: [react, vitest, multipart, offline-sync]
requires:
  - phase: 29-04
    provides: offline photo queue persistence and replay flow
provides:
  - Canonical multipart `photo` field contract for online and offline photo uploads
  - Functional camera entrypoint on site detail that opens photo-first modal
  - Regression tests for payload contract and photo entry behavior
affects: [site-activities, photo-upload, offline-queue]
tech-stack:
  added: []
  patterns:
    - Shared multipart field naming across online and offline upload paths
    - Modal initial mode controlled by caller via explicit prop
key-files:
  created: []
  modified:
    - frontend/src/lib/api/hooks/useSites.ts
    - frontend/src/lib/offline/queue.ts
    - frontend/src/lib/api/hooks/useSites.test.tsx
    - frontend/src/lib/offline/queue.test.ts
    - frontend/src/pages/sites/SiteDetailPage.tsx
    - frontend/src/pages/sites/CreateNoteModal.tsx
    - frontend/src/pages/sites/CreateNoteModal.test.tsx
key-decisions:
  - "Standardize upload payload key to `photo` for both direct and replayed uploads to match backend validation contract."
  - "Drive modal default tab via `initialActivityType` so camera entrypoint opens directly in photo mode while preserving note flow defaults."
patterns-established:
  - "Photo upload tests assert FormData keys explicitly to prevent contract drift"
requirements-completed: [FILE-01, FILE-02, FILE-05, FILE-06]
duration: 2 min
completed: 2026-05-01
---

# Phase 29 Plan 05: Photo Upload Contract + Camera Entrypoint Summary

**Photo uploads now use a backend-compatible multipart `photo` field in both online and offline replay paths, and the site camera button opens the modal directly in photo mode.**

## Performance

- **Duration:** 2 min
- **Started:** 2026-05-01T10:32:19Z
- **Completed:** 2026-05-01T10:34:30Z
- **Tasks:** 2/2
- **Files modified:** 7

## Accomplishments
- Fixed multipart field mismatch (`file` → `photo`) in online upload hook and offline replay handler.
- Added regression assertions that inspect `FormData` and verify the `photo` key is present.
- Wired site detail camera button to open `CreateNoteModal` in photo mode via `initialActivityType`.

## Task Commits

1. **Task 1: Fix multipart contract mismatch for online and offline photo uploads** - `2c3827d` (fix)
2. **Task 2: Wire SiteDetail photo icon to functional photo modal path** - `1c06722` (feat)

## Files Created/Modified
- `frontend/src/lib/api/hooks/useSites.ts` - Online upload now appends file under `photo` key.
- `frontend/src/lib/offline/queue.ts` - Offline replay upload now uses the same `photo` key.
- `frontend/src/lib/api/hooks/useSites.test.tsx` - Verifies upload FormData includes `photo` and excludes `file`.
- `frontend/src/lib/offline/queue.test.ts` - Verifies replay FormData uses `photo` before activity creation.
- `frontend/src/pages/sites/SiteDetailPage.tsx` - Camera button now opens modal; note/photo entrypoint mode tracked.
- `frontend/src/pages/sites/CreateNoteModal.tsx` - Supports `initialActivityType` and open-state sync for photo-first flow.
- `frontend/src/pages/sites/CreateNoteModal.test.tsx` - Adds test proving camera-style open starts in photo mode.

## Verification Results
- `npm run test:run -- useSites offline/queue` ✅
- `npm run test:run -- CreateNoteModal` ✅
- `npm run test:run -- useSites offline/queue CreateNoteModal` ✅

## Decisions Made
- Enforced one canonical multipart field name (`photo`) across all client upload paths to satisfy backend route validation and avoid online/offline divergence.
- Used prop-driven initial modal mode instead of duplicating modal components, preserving existing submit flow and reset behavior.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
Phase 29 gap-closure upload and entrypoint defects are resolved and regression-covered.

## Self-Check: PASSED
