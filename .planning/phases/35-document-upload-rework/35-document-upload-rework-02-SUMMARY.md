---
phase: 35-document-upload-rework
plan: 02
subsystem: ui
tags: [react, vite, vitest, modal, uploads, forms]
requires:
  - phase: 35-document-upload-rework
    provides: attachment-aware backend DTOs and generic upload endpoint
provides:
  - Multi-file document composer UI with note plus attachment validation
  - Generic frontend attachment upload hook and upload-first submit orchestration
  - Attachment-aware local site types for the modal and feed components
affects: [document-modal, activity-feed, camera-upload, frontend-types]
tech-stack:
  added: []
  patterns: [upload attachments first then create activity once, aggregated invalid file feedback, per-file removal UI]
key-files:
  created: []
  modified:
    - frontend/src/pages/sites/CreateNoteModal.tsx
    - frontend/src/pages/sites/CreateNoteModal.test.tsx
    - frontend/src/lib/api/hooks/useSites.ts
    - frontend/src/lib/api/hooks/useSites.test.tsx
    - frontend/src/types/sites.ts
key-decisions:
  - "Kept camera uploads on the existing photo-specific hook while the document modal moved to the generic attachment endpoint."
  - "Submitted all selected files with Promise.all before issuing exactly one create-activity mutation."
patterns-established:
  - "Document composer validity is content OR attachments, never content-only required."
  - "Invalid file batches keep valid files selected and surface one aggregated inline error."
requirements-completed: [DOC-01, DOC-02, DOC-03, DOC-04, DOC-05]
duration: 10 min
completed: 2026-05-01
---

# Phase 35 Plan 02: Document Upload Rework Summary

**The document modal now behaves like a real composer: users can add optional note text, attach multiple images or PDFs, remove individual files, and submit everything through one upload-first workflow.**

## Performance

- **Duration:** 10 min
- **Started:** 2026-05-01T13:51:35Z
- **Completed:** 2026-05-01T14:01:29Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Replaced the old note/photo toggle UI with a dedicated document composer matching the approved copy and file-selection flow.
- Added attachment-aware frontend types and generic upload hook coverage for PDF and image uploads.
- Wired modal submit to upload selected files first and then create exactly one activity payload with returned attachment IDs.

## Task Commits

Each task was committed atomically:

1. **task 1: replace note/photo toggle with note + multi-attachment composer** - `3c5b347` (test), `034b033` (feat)
2. **task 2: wire generic attachment upload and create-activity submission flow** - `3272e03` (test), `4cd8c3d` (feat)

_Note: TDD tasks used separate red/green commits._

## Files Created/Modified
- `frontend/src/pages/sites/CreateNoteModal.tsx` - renders the note + attachment composer and handles upload-first submit orchestration.
- `frontend/src/pages/sites/CreateNoteModal.test.tsx` - covers attachment selection, invalid-file handling, removal, and submit sequencing.
- `frontend/src/lib/api/hooks/useSites.ts` - adds the generic attachment upload mutation while preserving the camera upload hook.
- `frontend/src/lib/api/hooks/useSites.test.tsx` - verifies `attachment` FormData usage and attachment ID payload submission.
- `frontend/src/types/sites.ts` - adds attachment-aware local activity and upload types.

## Decisions Made
- Preserved the dedicated camera hook and endpoint so Phase 34 camera behavior stays isolated from the new document composer flow.
- Used `Promise.all` for attachment uploads because each selected file is independent and the create request only depends on the returned attachment IDs.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- The activity feed can now consume real attachment arrays from the modal workflow.
- The generic upload hook is available for any later document viewer or edit flows.

## Self-Check: PASSED
