# Phase 47 Research — Project Dashboard Visibility

**Date:** 2026-05-07
**Requirement:** `PROJ-12`
**Goal:** Let managers see relevant projects regardless of status and apply filters explicitly instead of relying on hidden defaults.

## Discovery Level

Level 0 — existing codebase patterns only. No new library or external integration is needed.

## Current State

### Backend

- `src/modules/sites/infrastructure/site_repository.rs`
  - `get_dashboard_sites(tenant_id)` currently restricts rows with `WHERE s.tenant_id = $1 AND s.status IN ('planned', 'active')`
  - Ordering currently prioritizes `active`, then `planned`, then `start_date ASC NULLS LAST`
- `src/modules/sites/application/site_service.rs`
  - `SiteService::get_dashboard` is a thin pass-through to `site_repo.get_dashboard_sites(ctx.tenant_id)`

### Frontend

- `frontend/src/lib/api/hooks/useSites.ts`
  - `useDashboardSites()` fetches `/api/v1/dashboard/sites` with no query params
  - `useSites(query?: ListSitesQuery)` already shows the established pattern for optional `?status=...` params
- `frontend/src/pages/DashboardPage.tsx`
  - Loads `sites` via `useDashboardSites()`
  - Derives `activeSites = sites?.filter((s) => s.status === "active") || []`
  - Stats card title is `Aktive Projekte` and uses `activeSites.length`
  - Main dashboard card title is `Aktive Projekte`
  - Empty state is `Keine aktiven Projekte`
  - Card grid renders only `activeSites.map(...)`
- `frontend/src/components/dashboard/SiteCard.tsx`
  - Already renders generic `DashboardSite` data and a `StatusBadge status={site.status}`
  - No active-only assumption except the CTA text `Aktive Baustelle` / `Als aktiv setzen`

### Test / Mock Surface

- `frontend/src/pages/DashboardPage.test.tsx` currently verifies only project-aware wording and active-project section presence
- `frontend/src/test/mocks/handlers.ts` returns all `mockData.sites` for `/dashboard/sites`, so frontend tests can cover completed/planned visibility without backend test harness changes
- `PROJ-12` review already identifies the intended implementation path in `.planning/requirement-reviews/projects/PROJ-12.md`

## Recommended Implementation Shape

1. **Broaden backend dashboard dataset by default**
   - Remove the hard SQL restriction to `planned` + `active`
   - Keep tenant scoping intact
   - Preserve stable ordering, but include `completed` projects in a deliberate later position so readability stays intact

2. **Add explicit status filtering at the API/UI boundary**
   - Follow the existing optional-query-param pattern used by `useSites(query)`
   - Add an optional dashboard status filter contract rather than hidden frontend filtering
   - Default behavior should remain “all relevant dashboard projects” with no filter applied

3. **Update dashboard rendering to be filter-driven, not hidden-default-driven**
   - Remove the hardcoded frontend `status === "active"` filter as the source of truth
   - Replace active-only labels with explicit filter labels/copy
   - Keep the page fast by filtering the already-loaded dataset in memory if the API continues returning a compact dashboard list, or push filtering into the API if the dashboard contract grows query support

4. **Protect readability**
   - Keep completed projects visually distinguishable with the existing `StatusBadge`
   - Preserve compact card layout and current two-column grid
   - Avoid turning Phase 47 into a reporting dashboard; scope is visibility + explicit filter control only

## Existing Patterns to Reuse

- Optional list filtering pattern from `useSites(query)` in `frontend/src/lib/api/hooks/useSites.ts`
- Existing `StatusBadge` rendering already supports status differentiation in cards
- Existing tenant-scoped repository query pattern in `site_repository.rs`
- Dashboard invalidation pattern already wired through `queryKey: ["dashboard-sites"]`

## Risks / Pitfalls

1. **Hidden split filtering**
   - Fixing only the backend query or only the frontend `activeSites` filter would leave the other hidden constraint in place.

2. **Readability regression**
   - Simply dumping every project into the current card section without an explicit filter control or ordering could make the overview noisier.

3. **Contract drift**
   - If dashboard filtering uses a new query param, update frontend hook types/tests and backend route parsing together.

4. **Out-of-scope expansion**
   - Do not turn this phase into full historic reporting (`RPT-10` to `RPT-12`) or archived-project management.

## Suggested Verification Targets

- Backend unit/integration coverage around dashboard query behavior for `planned`, `active`, and `completed`
- Frontend dashboard tests proving:
  - planned + active + completed projects render by default
  - explicit filter control narrows the visible set
  - the page still renders concise cards and labels without active-only copy

## Planning Notes

- This is likely a 2-plan frontend+backend phase:
  1. Expand dashboard data contract and optional filter plumbing
  2. Implement explicit dashboard filter UX and regression tests
- `PROJ-12` is the only mapped requirement, but both backend and frontend must change to satisfy it end-to-end.
