# PROJ-12

Status: Missing
Fit: Strong
Priority: Now
Decision: Keep

Current state: The dashboard still only shows planned/active sites instead of all relevant projects by default.
Evidence: `frontend/src/pages/DashboardPage.tsx`, `src/modules/sites/infrastructure/site_repository.rs`

Implementation:
1. Remove the hard backend restriction to planned/active items.
2. Stop filtering to active-only in the frontend dashboard.
3. Add explicit status filters only as a user choice.
4. Keep the default overview broad and simple.
