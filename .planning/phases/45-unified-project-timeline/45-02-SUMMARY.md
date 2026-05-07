---
phase: 45-unified-project-timeline
plan: 02
subsystem: ui
tags: [react, vitest, project-timeline, activity-feed, site-detail]
requires:
  - phase: 45-unified-project-timeline
    provides: shared project timeline composer and unified note/camera entrypoints
provides:
  - project detail layout that treats the timeline as the canonical project memory
  - consistent absolute-plus-relative timestamp rendering across timeline cards
affects: [site-detail, activity-feed, material-history, media-viewer]
tech-stack:
  added: []
  patterns: [timeline-first detail page layout, shared timestamp presentation for project timeline cards]
key-files:
  created: []
  modified:
    - frontend/src/pages/sites/SiteDetailPage.tsx
    - frontend/src/pages/sites/SiteDetailPage.test.tsx
    - frontend/src/pages/sites/ActivityFeed.tsx
    - frontend/src/pages/sites/ActivityFeed.test.tsx
key-decisions:
  - "Promote the timeline above secondary management cards on the detail page so the project memory is visible first."
  - "Show both exact and relative timestamps on each timeline card using the existing created_at field."
patterns-established:
  - "Project detail pages should surface the project timeline before planning and secondary management panels."
  - "Timeline cards should render exact plus relative timestamps without changing protected media viewer links."
requirements-completed: [PROJ-18, PROJ-20]
duration: 9 min
completed: 2026-05-07
---

# Phase 45 Plan 02: Unified Project Timeline Summary

**Project detail pages now foreground one canonical Projekt-Timeline surface with consistent timestamps and preserved protected media previews.**

## Performance

- **Duration:** 9 min
- **Started:** 2026-05-07T22:24:28Z
- **Completed:** 2026-05-07T22:33:06Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Moved the timeline card ahead of secondary management sections and rewrote the surrounding copy so it reads as the canonical project memory.
- Normalized timeline cards to show exact and relative timestamps together for note, photo, and status entries.
- Preserved the material history tab and protected media-viewer links while renaming the timeline surface consistently.

## task Commits

Each task was committed atomically:

1. **task 1: make the project timeline the primary context surface on detail pages** - `0603f82` (test), `89a7bb2` (feat), `d3a49c0` (test)
2. **task 2: normalize timeline timestamps and preview rendering across entry types** - `85403ee` (test), `fb1984a` (feat)

**Plan metadata:** pending

## Files Created/Modified
- `frontend/src/pages/sites/SiteDetailPage.tsx` - promotes the timeline card and canonical copy above secondary detail cards.
- `frontend/src/pages/sites/SiteDetailPage.test.tsx` - verifies timeline-first copy, composer access, and retained material history reachability.
- `frontend/src/pages/sites/ActivityFeed.tsx` - adds exact timestamp rendering and timeline-specific tab and empty-state copy.
- `frontend/src/pages/sites/ActivityFeed.test.tsx` - verifies timestamp consistency, preserved viewer routing, and project timeline copy.

## Decisions Made
- Promoted the timeline before planning and detail sections so workers see the shared project memory first on the page.
- Combined exact and relative timestamps on timeline cards rather than replacing one with the other.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 45 now delivers both the unified composer and timeline-first detail-page presentation.
- No additional blockers remain inside this phase.

## Self-Check: PASSED
- Found summary artifact plus the updated `SiteDetailPage.tsx` and `ActivityFeed.tsx` files.
- Verified task commits `0603f82`, `89a7bb2`, `d3a49c0`, `85403ee`, and `fb1984a` in git history.
