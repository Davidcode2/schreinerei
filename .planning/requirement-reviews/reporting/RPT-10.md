# RPT-10

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Site lists support only basic status/text filtering. Cost, duration, worker, and project-type reporting filters do not exist.
Evidence: `frontend/src/pages/sites/SitesPage.tsx`, `src/modules/sites/infrastructure/site_repository.rs`

Implementation:
1. Add missing data foundations: project type, cost basis, stricter project linkage.
2. Build a reporting query/read model for historical projects.
3. Add structured filters for customer, date range, worker, duration, and cost.
4. Keep reporting manager-focused and read-only.
