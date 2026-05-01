---
phase: 36-media-viewer
plan: "02"
subsystem: ui
tags: [react, react-router, dialog, viewer, clipboard]
requires:
  - phase: 36-media-viewer
    provides: creator_name and attachment-aware viewer metadata
provides:
  - canonical viewer routes and slug helpers
  - fullscreen viewer shell with share and download actions
  - route-backed close behavior on the site detail page
affects: [phase-36-plan-03, activity-feed, deep-links]
tech-stack:
  added: []
  patterns: [route-backed overlay state, authenticated blob preview/download]
key-files:
  created: [frontend/src/pages/sites/MediaViewer.tsx, frontend/src/pages/sites/mediaViewerRoute.ts]
  modified: [frontend/src/App.tsx, frontend/src/types/sites.ts, frontend/src/pages/sites/SiteDetailPage.tsx, frontend/src/pages/sites/MediaViewer.test.tsx]
key-decisions:
  - "Keep the viewer mounted from SiteDetailPage so deep links preserve browser navigation and close back to /sites/:id."
  - "Fetch preview blobs through apiClient.getBlob for both preview and download instead of exposing direct attachment URLs."
patterns-established:
  - "Viewer routes are built through mediaViewerRoute helpers, not ad-hoc string concatenation."
  - "Protected media previews use object URLs created from authenticated blob fetches."
requirements-completed: [VIEW-03, VIEW-04, VIEW-05, VIEW-06, VIEW-07, VIEW-08, VIEW-09]
duration: 45min
completed: 2026-05-01
---

# Phase 36 Plan 02: Media Viewer Summary

**A route-backed fullscreen viewer now opens protected site media with metadata, direct-link copying, and authenticated download behavior.**

## Performance

- **Duration:** 45 min
- **Started:** 2026-05-01T18:05:00Z
- **Completed:** 2026-05-01T18:50:00Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Registered `/sites/:id/media/:activityId/:attachmentId/:slug?` and route helper utilities for canonical viewer links.
- Added a dialog-backed fullscreen viewer shell with metadata sidebar, note fallback copy, clipboard share, and protected download behavior.
- Wired SiteDetailPage to resolve viewer targets from route params and close back to the canonical site detail route.

## Task Commits

1. **task 1: add canonical viewer route and selection helpers** - `cc80097`, `d5c5a94` (test, feat)
2. **task 2: implement fullscreen viewer shell with share, download, and fallback states** - `1faa19b`, `279cbaa` (test, feat)

## Files Created/Modified
- `frontend/src/pages/sites/mediaViewerRoute.ts` - slugifies filenames, resolves targets, and extracts legacy photo attachment IDs.
- `frontend/src/pages/sites/MediaViewer.tsx` - renders the fullscreen viewer shell.
- `frontend/src/pages/sites/SiteDetailPage.tsx` - maps route params to viewer state and close navigation.
- `frontend/src/App.tsx` - registers the viewer route alongside the site detail page.
- `frontend/src/types/sites.ts` - extends local `Activity` typing with `creator_name` and `status_change` support.
- `frontend/src/pages/sites/MediaViewer.test.tsx` - covers route helpers plus viewer metadata/share/download behavior.

## Decisions Made
- Reused the site detail page as the route owner so browser back/forward works naturally for deep-linked viewer states.
- Used the same authenticated blob fetch for preview and download to preserve Phase 35's protected media invariant.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Feed tiles can now route into a real fullscreen viewer surface.
- Canonical viewer path helpers are available for all activity tile entrypoints.

## Self-Check: PASSED
