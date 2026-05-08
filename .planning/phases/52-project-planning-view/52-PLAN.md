# Phase 52 Plan

## Goal

Add a project-centric planning view that shows timing, worker assignments, and linked reservations together on the project detail surface.

## Requirements

- `PROJ-15`

## Plan

1. Extend fleet read contracts with optional `site_id` filtering.
2. Include `site_id` in reservation calendar summaries so project-filtered fleet reads stay precise.
3. Add an embedded project-filtered planning section to `SiteDetailPage`.
4. Reuse existing assignment UI and project timing metadata rather than inventing a second planner.
5. Verify filtered reservation/calendar behavior and detail-page rendering.

## Design Notes

- Keep the project or site as the central unit.
- Reuse `ProjectAssignmentsSection`, `ProjectPlanningSheet`, and `CalendarView`.
- Keep `/fleet` intact; add project-filtered read behavior instead of redesigning fleet.
- No drag-and-drop scheduling, Gantt view, or staff capacity model in this phase.

## Verification

- backend DTO/filter tests for reservation summary `site_id` and `site_id` query support
- focused `SiteDetailPage` planning test
- frontend typecheck
- `cargo export-types`
- `cargo fmt --check`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
