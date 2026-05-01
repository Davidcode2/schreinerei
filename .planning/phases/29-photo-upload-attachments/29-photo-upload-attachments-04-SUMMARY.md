---
phase: 29-photo-upload-attachments
plan: 04
subsystem: ui
tags: [offline, dexie, queue, photo-upload, sync]
requires:
  - phase: 29-03
    provides: Frontend upload flow and attachment URL contracts
provides:
  - Offline queue payload support for photo uploads
  - Reconnect-driven processing for queued photo actions
  - UI fallback to enqueue photos when offline
affects: [activity-feed, offline-sync, pending-actions]
tech-stack:
  added: []
  patterns: [serialized-file queue payloads, strict payload validation before replay]
key-files:
  created:
    - frontend/src/lib/offline/queue.test.ts
    - frontend/src/lib/offline/sync.test.ts
    - frontend/src/pages/sites/CreateNoteModal.test.tsx
    - frontend/src/components/offline/PendingActionsBadge.test.tsx
  modified:
    - frontend/src/lib/offline/db.ts
    - frontend/src/lib/offline/queue.ts
    - frontend/src/pages/sites/CreateNoteModal.tsx
    - frontend/src/components/offline/PendingActionsBadge.tsx
key-decisions:
  - "Store queued photo files as data URLs plus MIME metadata so actions survive reloads and can be replayed deterministically."
  - "Validate queued photo payload shape before upload replay to drop malformed entries safely (T-29-10 mitigation)."
patterns-established:
  - "Offline photo flow: enqueue via queuePhotoUploadAction when navigator is offline, then auto-sync on reconnect."
requirements-completed: [FILE-05, FILE-06]
duration: 20 min
completed: 2026-05-01
---

# Phase 29 Plan 04: Offline photo upload queue and auto-sync Summary

**Offline photo actions now persist full upload payloads locally and replay automatically on reconnect with queue-safe validation and retry behavior.**

## Performance

- **Duration:** 20 min
- **Started:** 2026-05-01T12:04:00Z
- **Completed:** 2026-05-01T12:09:00Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Extended pending-action schema and queue typing to support dedicated `photo_upload` items.
- Added replay handler that uploads queued binary payloads first, then creates activity with returned `photo_url`.
- Wired CreateNoteModal to enqueue photo actions while offline and close successfully with user feedback.
- Added regression tests for queue persistence, reconnect processing, and pending badge visibility.

## task Commits

1. **task 1: Extend offline schema and queue payloads for photo uploads** - `f140916` (test)
2. **task 1: Extend offline schema and queue payloads for photo uploads** - `0f2c92d` (feat)
3. **task 2: Trigger automatic sync and UI feedback when connection returns** - `4606eac` (test)
4. **task 2: Trigger automatic sync and UI feedback when connection returns** - `c4adcc5` (feat)

## Files Created/Modified
- `frontend/src/lib/offline/db.ts` - PendingAction type and Dexie schema version bump for photo queue payloads.
- `frontend/src/lib/offline/queue.ts` - `photo_upload` handler, payload validation, file serialization/reconstruction, queue helper.
- `frontend/src/pages/sites/CreateNoteModal.tsx` - Offline branch enqueues photo upload action instead of failing upload.
- `frontend/src/components/offline/PendingActionsBadge.tsx` - Polite live-region feedback for pending synchronization count.
- `frontend/src/lib/offline/queue.test.ts` - Queue payload persistence and reload replay tests.
- `frontend/src/lib/offline/sync.test.ts` - Reconnect-triggered sync processing tests.
- `frontend/src/pages/sites/CreateNoteModal.test.tsx` - Offline modal enqueue behavior test.
- `frontend/src/components/offline/PendingActionsBadge.test.tsx` - Pending-count badge rendering test.

## Decisions Made
- Used data URL serialization for queued image blobs to ensure payload durability across app restarts.
- Kept retry cap semantics unchanged and reused existing queue retry/error paths for photo uploads.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
Phase 29 is ready for verification; offline photo capture now degrades gracefully and syncs on reconnect.

## Self-Check: PASSED
