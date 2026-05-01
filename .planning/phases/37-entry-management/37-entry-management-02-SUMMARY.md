---
phase: 37-entry-management
plan: "02"
subsystem: ui
tags: [react, tanstack-query, activity-feed, confirmation-dialog, toast]
requires:
  - phase: 37-entry-management
    provides: can_delete activity contract and creator-only delete endpoint
provides:
  - typed activity delete mutation
  - confirmation-backed feed delete affordance
  - toast-driven success and failure feedback for activity deletion
affects: [activity-stream, feed-ux, destructive-actions]
tech-stack:
  added: []
  patterns: [backend-driven permission rendering, feed-level delete confirmation flow]
key-files:
  created: []
  modified: [frontend/src/types/sites.ts, frontend/src/lib/api/hooks/useSites.ts, frontend/src/lib/api/hooks/useSites.test.tsx, frontend/src/pages/sites/ActivityFeed.tsx, frontend/src/pages/sites/ActivityFeed.test.tsx]
key-decisions:
  - "Render delete affordances solely from activity.can_delete so UI logic never guesses ownership from auth state."
  - "Keep delete orchestration in ActivityFeed so one confirmation dialog can manage feed state and toast feedback cleanly."
patterns-established:
  - "Feed delete actions use a site-scoped React Query mutation that invalidates only ['activities', siteId]."
  - "Destructive activity actions always pass through DeleteConfirmDialog before a mutation runs."
requirements-completed: [ENTRY-01, ENTRY-02, ENTRY-03]
duration: 15min
completed: 2026-05-01
---

# Phase 37 Plan 02: Entry Management Summary

**The activity feed now renders API-approved delete actions, confirms destructive clicks, and refreshes the site timeline after successful entry removal.**

## Performance

- **Duration:** 15 min
- **Started:** 2026-05-01T16:20:00Z
- **Completed:** 2026-05-01T16:35:05Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Extended the frontend `Activity` contract with `can_delete` and added a site-scoped `useDeleteActivity` mutation.
- Added per-entry delete affordances only for backend-approved activities in the feed.
- Reused the shared confirmation dialog and surfaced success/failure with toast feedback while preserving existing media-viewer links.

## Task Commits

1. **task 1: add an activity delete mutation and typed feed contract** - `fbc883f` (feat)
2. **task 2: render creator-only delete affordances with blocking confirmation** - `73cd022` (feat)

## Files Created/Modified
- `frontend/src/types/sites.ts` - adds the local `can_delete` activity field.
- `frontend/src/lib/api/hooks/useSites.ts` - adds `useDeleteActivity` and scoped activity query invalidation.
- `frontend/src/lib/api/hooks/useSites.test.tsx` - verifies the delete hook request shape, invalidation, and error propagation.
- `frontend/src/pages/sites/ActivityFeed.tsx` - renders delete buttons, confirmation state, and toast-backed mutation handling.
- `frontend/src/pages/sites/ActivityFeed.test.tsx` - covers delete affordance visibility, confirmation flow, and failure feedback without regressing viewer links.

## Decisions Made
- Kept the delete button inside the feed card header so the existing media tiles stay unchanged and route behavior remains intact.
- Used toast messages from the existing `sonner` integration instead of introducing a parallel feedback mechanism.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Corrected the frontend test command to run from the actual workspace**
- **Found during:** task 1 (add an activity delete mutation and typed feed contract)
- **Issue:** the plan’s root-level `npm test` command fails because the JavaScript workspace is under `frontend/`, not the repository root.
- **Fix:** ran Vitest from `frontend/` with the same test targets.
- **Files modified:** none
- **Verification:** `npm test -- --run src/lib/api/hooks/useSites.test.tsx`
- **Committed in:** `fbc883f` (part of task commit)

**2. [Rule 3 - Blocking] Switched the `sonner` test stub to a partial mock**
- **Found during:** task 2 (render creator-only delete affordances with blocking confirmation)
- **Issue:** a full mock removed `Toaster`, breaking the shared render wrapper before any feed assertions could run.
- **Fix:** preserved the real `Toaster` export and mocked only `toast.success` / `toast.error`.
- **Files modified:** frontend/src/pages/sites/ActivityFeed.test.tsx
- **Verification:** `npm test -- --run src/pages/sites/ActivityFeed.test.tsx`
- **Committed in:** `73cd022` (part of task commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes were test-environment corrections required to verify the intended UI behavior. No scope creep.

## Issues Encountered

- The frontend verification steps in the plan assumed a root workspace, but this repository splits Rust and React into separate package roots.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 37 is user-complete: creator-only deletes are enforced server-side and exposed cleanly in the feed.
- Verification can now evaluate the full milestone end-to-end without any remaining planned entry-management gaps.

## Self-Check: PASSED
