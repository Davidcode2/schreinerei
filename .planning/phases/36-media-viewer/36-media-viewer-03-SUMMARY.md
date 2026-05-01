---
phase: 36-media-viewer
plan: "03"
subsystem: ui
tags: [react, activity-feed, links, accessibility, viewer]
requires:
  - phase: 36-media-viewer
    provides: canonical viewer routes and fullscreen viewer shell
provides:
  - clickable image and pdf feed tiles
  - clickable legacy photo fallback tiles
  - accessible media-entry labels for viewer routes
affects: [phase-36-verification, activity-stream]
tech-stack:
  added: []
  patterns: [link-wrapped media cards, viewer-route reuse from feed]
key-files:
  created: []
  modified: [frontend/src/pages/sites/ActivityFeed.tsx, frontend/src/pages/sites/ActivityFeed.test.tsx]
key-decisions:
  - "Wrap the full tile card in a Link so preview, filename, and fallback state share one keyboard-focusable hit area."
patterns-established:
  - "Feed media entrypoints always use buildMediaViewerPath via mediaViewerRoute helpers."
requirements-completed: [VIEW-01, VIEW-02, VIEW-03]
duration: 20min
completed: 2026-05-01
---

# Phase 36 Plan 03: Media Viewer Summary

**Activity feed image, PDF, and legacy photo tiles now deep-link into the fullscreen viewer through accessible canonical routes.**

## Performance

- **Duration:** 20 min
- **Started:** 2026-05-01T18:50:00Z
- **Completed:** 2026-05-01T19:10:00Z
- **Tasks:** 1
- **Files modified:** 2

## Accomplishments
- Wrapped image and PDF tiles in canonical media-viewer links with accessible `Medium öffnen:` labels.
- Parsed legacy photo attachment IDs from protected `photo_url` values so older camera activities open in the same viewer flow.
- Preserved the visible fallback tile state when preview loading fails while still keeping the tile clickable.

## Task Commits

1. **task 1: make feed media tiles route into the fullscreen viewer** - `88addb4`, `85c36a4` (test, feat)

## Files Created/Modified
- `frontend/src/pages/sites/ActivityFeed.tsx` - links viewer-capable tiles to canonical viewer routes.
- `frontend/src/pages/sites/ActivityFeed.test.tsx` - verifies image, PDF, legacy-photo, and fallback-link behavior.

## Decisions Made
- Kept status-change activities non-interactive and limited route generation to viewer-capable attachments/photos only.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 36 user-facing viewer flow is complete end-to-end from feed tile to deep link.
- Phase 37 can build entry-management affordances on top of the stabilized activity stream UI.

## Self-Check: PASSED
