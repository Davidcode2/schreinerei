# 45-02 Summary

## Outcome

The project detail page now makes the timeline the primary context surface, and timeline history renders exact timestamps and preview tiles consistently across legacy and unified entries.

## Changes

- Renamed the notes/documents tab to `Timeline`
- Added exact German-formatted timestamps next to creator metadata on timeline cards
- Kept legacy photo rows and attachment-backed note entries on one preview-link model
- Reordered `SiteDetailPage` so `Projekt-Timeline` appears before secondary detail/planning sections
- Added timeline helper copy and preserved both the unified entry action and the fast camera shortcut
- Extended tests for timeline label, empty state, exact metadata presence, and page ordering

## Verification

- `npm --prefix frontend run test -- ActivityFeed.test.tsx SiteDetailPage.test.tsx`
- `npm --prefix frontend run build`

## Notes

- Material history remains reachable from the same project context and the dashboard visibility behavior is still deferred to Phase 47.
