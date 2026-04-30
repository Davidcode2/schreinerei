# 23-02 Summary

- Added active-site toggle actions to overview site cards:
  - `frontend/src/components/sites/SiteCard.tsx`
  - `frontend/src/pages/sites/SitesListPage.tsx`
- Added active-site toggle actions to dashboard site cards:
  - `frontend/src/components/dashboard/SiteCard.tsx`
  - `frontend/src/pages/DashboardPage.tsx`
- Toggle behavior now sets/clears `active_site_id` through preferences mutation and renders a single active marker based on preference state.

Verification:
- Included in `npm run build` check in `frontend/`.
- Build is blocked by unrelated pre-existing TypeScript/test environment errors.
