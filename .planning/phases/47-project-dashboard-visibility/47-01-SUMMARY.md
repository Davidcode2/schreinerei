# 47-01 Summary

## Outcome

The manager dashboard now shows planned, active, and completed projects by default, with explicit status filters instead of hidden active-only behavior.

## Changes

- Expanded the backend dashboard query to include `completed` projects while still excluding `archived`
- Preserved ordering as `active`, then `planned`, then `completed`
- Replaced the frontend active-only derived list with explicit local filter state and `visibleSites`
- Added compact status filters: `Alle`, `Geplant`, `Aktiv`, `Abgeschlossen`
- Updated dashboard empty-state and project CTA wording to stay project-aware
- Updated dashboard tests to cover default all-status rendering and filter behavior

## Verification

- `python -c "from pathlib import Path; s=Path('src/modules/sites/infrastructure/site_repository.rs').read_text(); assert \"s.status IN ('planned', 'active', 'completed')\" in s; assert \"WHEN 'completed' THEN 3\" in s; assert \"s.status IN ('planned', 'active')\" not in s" && cargo test dashboard_site_includes_project_type --lib`
- `npm --prefix frontend run test -- DashboardPage.test.tsx`
- `npm --prefix frontend run build`

## Notes

- This phase intentionally stops at visibility and explicit filtering; it does not expand into full reporting or archived-project management.
