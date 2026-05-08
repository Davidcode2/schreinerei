---
phase: 45-unified-project-timeline
plan: 01
subsystem: ui
tags: [react, vitest, project-timeline, attachments, mobile]
requires:
  - phase: 44-project-model-foundation
    provides: project-first site detail context and attachment-backed activity feed foundation
provides:
  - shared project timeline composer for note, image, and pdf submissions
  - thin note and camera entrypoints that reuse the same attachment-backed activity path
affects: [site-detail, activity-feed, project-timeline, media-viewer]
tech-stack:
  added: []
  patterns: [shared react composer for multiple entrypoints, attachment-first activity submission]
key-files:
  created:
    - frontend/src/pages/sites/ProjectTimelineComposer.tsx
    - frontend/src/pages/sites/ProjectTimelineComposer.test.tsx
  modified:
    - frontend/src/pages/sites/CreateNoteModal.tsx
    - frontend/src/pages/sites/CreateNoteModal.test.tsx
    - frontend/src/pages/sites/CameraUploadFlow.tsx
    - frontend/src/pages/sites/CameraUploadFlow.test.tsx
    - frontend/src/lib/api/hooks/useSites.test.tsx
key-decisions:
  - "Use one shared composer component for both note and camera entrypoints so mixed-media submission rules stay in one place."
  - "Keep unified submissions on the existing attachment-backed note activity path instead of the legacy photo activity flow."
patterns-established:
  - "Project timeline entrypoints should delegate to ProjectTimelineComposer instead of reimplementing upload state."
  - "Mixed media timeline submissions upload attachments first, then create one note activity with attachment_ids."
requirements-completed: [PROJ-19]
duration: 126 min
completed: 2026-05-07
---

# Phase 45 Plan 01: Unified Project Timeline Summary

**Shared mobile-first composer now combines notes, captured images, and uploaded PDFs into one attachment-backed project timeline entry flow.**

## Performance

- **Duration:** 126 min
- **Started:** 2026-05-07T20:18:35Z
- **Completed:** 2026-05-07T22:24:28Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Added `ProjectTimelineComposer` as the single UI contract for note-only, attachment-only, and mixed-media timeline submissions.
- Replaced separate note and camera implementations with thin wrappers that preserve `SiteDetailPage` props while sharing one composer.
- Extended regression coverage so unified composer behavior and activity-query invalidation stay protected.

## task Commits

Each task was committed atomically:

1. **task 1: build the shared project timeline composer contract** - `9496a0f` (test), `f0f497c` (feat)
2. **task 2: refit note and camera entrypoints onto the shared composer** - `a3fc0cb` (test), `2a64df1` (feat)

**Plan metadata:** pending

## Files Created/Modified
- `frontend/src/pages/sites/ProjectTimelineComposer.tsx` - shared composer with standard picker, camera picker, file validation, and note activity submission.
- `frontend/src/pages/sites/ProjectTimelineComposer.test.tsx` - regression tests for mixed-media staging and file validation.
- `frontend/src/pages/sites/CreateNoteModal.tsx` - wrapper that delegates the note entrypoint to the shared composer.
- `frontend/src/pages/sites/CreateNoteModal.test.tsx` - verifies the note entrypoint preserves its stable wrapper contract.
- `frontend/src/pages/sites/CameraUploadFlow.tsx` - wrapper that routes the camera shortcut into the shared composer in camera mode.
- `frontend/src/pages/sites/CameraUploadFlow.test.tsx` - verifies the camera entrypoint delegates to the shared composer.
- `frontend/src/lib/api/hooks/useSites.test.tsx` - confirms unified note activity creation still invalidates the site activities query.

## Decisions Made
- Used one shared composer for both entry buttons so validation, previews, and upload ordering live in a single component.
- Kept the camera shortcut fast by auto-opening the camera input from the shared composer instead of preserving a second submit path.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Installed missing frontend dependencies before running plan verification**
- **Found during:** task 1 (build the shared project timeline composer contract)
- **Issue:** `npm --prefix frontend run test -- ProjectTimelineComposer.test.tsx` failed because `frontend/node_modules` was missing and `vitest` was not installed in the workspace.
- **Fix:** Ran `npm --prefix frontend install` to restore the local frontend toolchain.
- **Files modified:** None committed
- **Verification:** `npm --prefix frontend run test -- ProjectTimelineComposer.test.tsx`
- **Committed in:** none (workspace setup only)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The fix restored the existing frontend verification toolchain and did not change shipped behavior.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- The shared composer contract is ready for detail-page timeline promotion and feed-copy polish in plan 45-02.
- Legacy photo upload hook support remains available for older callers outside these two entrypoints.

## Self-Check: PASSED
- Found summary artifact and shared composer source file.
- Verified task commits `9496a0f`, `f0f497c`, `a3fc0cb`, and `2a64df1` in git history.
