---
phase: 03-baustellen-management
plan: 02
subsystem: sites
tags: [rust, axum, sqlx, postgres, activity-feed, dashboard]

# Dependency graph
requires:
  - phase: 03-baustellen-management
    plan: 01
    provides: Sites module foundation, SiteRepository, SiteService, REST API structure
provides:
  - Activity Feed for photos and notes on construction sites
  - Dashboard endpoint showing open sites with summary metrics
affects: [frontend-sites]

# Tech tracking
tech-stack:
  added: []
  patterns: [Activity aggregate, Timeline ordering, Dashboard aggregation]

key-files:
  created:
    - src/modules/sites/domain/activity.rs
  modified:
    - src/modules/sites/domain.rs
    - src/modules/sites/infrastructure/site_repository.rs
    - src/modules/sites/application/site_service.rs
    - src/modules/sites/api/routes.rs

key-decisions:
  - "Activity types: Photo (requires photo_url), Note (requires content), StatusChange (system-generated)"
  - "Dashboard shows only planned + active sites (not completed/archived)"
  - "Dashboard aggregates assigned_users count and total_hours per site"

patterns-established:
  - "Activity validation enforces type-specific requirements"
  - "Dashboard query uses LEFT JOINs for aggregation without blocking"
  - "Activity listing ordered by created_at DESC for timeline view"

requirements-completed: [SITE-05, SITE-06, SITE-07, SITE-08]

# Metrics
duration: 10min
completed: 2026-04-28
---

# Phase 3 Plan 02: Activity Feed and Dashboard Summary

**Activity Feed with photos/notes on construction sites plus dashboard endpoint showing open sites with assigned users and total hours.**

## Performance

- **Duration:** ~10 min
- **Started:** 2026-04-28T19:10:00Z
- **Completed:** 2026-04-28T19:20:00Z
- **Tasks:** 5
- **Files modified:** 5

## Accomplishments

- Created Activity domain model with type-specific validation (Photo requires URL, Note requires content)
- Extended repository with activity and dashboard methods using tenant-scoped queries
- Added service methods for activity creation and dashboard retrieval
- Exposed REST API endpoints for activity feed and dashboard
- Dashboard aggregates assigned users and total hours per open site

## Task Commits

Each task was committed atomically:

1. **Task 1: Add site event types to common/events.rs** - Already complete (event types added in Plan 03-01)
2. **Task 2: Create Activity domain model** - `6eb0a61` (feat)
3. **Task 3: Add activity methods to repository** - `864780e` (feat)
4. **Task 4: Add activity and dashboard methods to service** - `941cabc` (feat)
5. **Task 5: Add activity and dashboard API endpoints** - `32031b6` (feat)

## Files Created/Modified

### Created
- `src/modules/sites/domain/activity.rs` - Activity aggregate, ActivityType enum, CreateActivity command

### Modified
- `src/modules/sites/domain.rs` - Export activity module
- `src/modules/sites/infrastructure/site_repository.rs` - Activity methods, DashboardSite struct, get_dashboard_sites query
- `src/modules/sites/application/site_service.rs` - create_activity, list_activities, get_dashboard methods
- `src/modules/sites/api/routes.rs` - Activity and dashboard endpoints

## Decisions Made

1. **ActivityType validation**: Photo must have photo_url, Note must have content - enforces data integrity
2. **StatusChange as system-only**: Users cannot manually create status change activities - reserved for system events
3. **Dashboard filtering**: Only shows planned + active sites - excludes completed/archived from overview
4. **Activity ordering**: created_at DESC for timeline view - newest first

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - all tasks completed successfully.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 3 complete with all 8 requirements implemented
- Sites module fully functional with CRUD, assignments, time tracking, activity feed, and dashboard
- Ready for Phase 4: Fuhrpark & Werkzeuge (Vehicle & Tool Management)
- Frontend can integrate with all Sites API endpoints

---
*Phase: 03-baustellen-management*
*Completed: 2026-04-28*
