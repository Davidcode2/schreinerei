---
phase: 35-document-upload-rework
plan: 03
subsystem: ui
tags: [react, vite, vitest, activity-feed, previews]
requires:
  - phase: 35-document-upload-rework
    provides: attachment arrays on activity responses and composer submit flow
provides:
  - Attachment-aware activity cards for document entries
  - PDF and image document tiles with protected preview fallback rendering
  - Updated feed empty-state copy aligned with the document workflow
affects: [activity-feed, document-viewer, camera-upload]
tech-stack:
  added: []
  patterns: [authenticated image preview loading, visible fallback shell on preview failure, attachment grid rendering]
key-files:
  created: []
  modified:
    - frontend/src/pages/sites/ActivityFeed.tsx
    - frontend/src/pages/sites/ActivityFeed.test.tsx
key-decisions:
  - "Kept legacy camera photo previews visible while document entries render from attachment arrays."
  - "Used the authenticated blob fetch path only for image attachments; PDF entries render static cards with visible labeling."
patterns-established:
  - "Document feed cards render note text first, then a responsive attachment grid."
  - "Attachment preview failures degrade to a visible card shell instead of disappearing from the feed."
requirements-completed: [DOC-01, DOC-02, DOC-03, DOC-05]
duration: 3 min
completed: 2026-05-01
---

# Phase 35 Plan 03: Document Upload Rework Summary

**The activity feed now shows document entries as note-first cards with authenticated image previews, PDF tiles, and a visible fallback state when protected preview fetches fail.**

## Performance

- **Duration:** 3 min
- **Started:** 2026-05-01T14:01:29Z
- **Completed:** 2026-05-01T14:04:30Z
- **Tasks:** 1
- **Files modified:** 2

## Accomplishments
- Rendered document attachment grids directly from attachment arrays instead of relying only on `photo_url`.
- Added PDF-specific cards and explicit preview fallback copy for failed protected image fetches.
- Updated the notes/documents empty state to describe both notes and PDF/image uploads.

## Task Commits

Each task was committed atomically:

1. **task 1: render attachment-backed activity cards for images and PDFs** - `23b5e46` (test), `00657ee` (feat)

_Note: TDD tasks used separate red/green commits._

## Files Created/Modified
- `frontend/src/pages/sites/ActivityFeed.tsx` - renders document headings, responsive attachment tiles, PDF cards, and preview fallback states.
- `frontend/src/pages/sites/ActivityFeed.test.tsx` - verifies document headings, PDF rendering, and failed preview fallback behavior.

## Decisions Made
- Kept the camera photo rendering path so previous photo activities still display even while document entries move to attachment arrays.
- Treated PDF entries as explicit document cards rather than fake image previews to match the approved Phase 35 contract.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 36 can layer fullscreen media viewing on top of the attachment grid without changing the feed data contract.
- Document and camera entries now coexist in the feed with distinct visuals.

## Self-Check: PASSED
