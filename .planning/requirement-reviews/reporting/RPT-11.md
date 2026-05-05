# RPT-11

Status: Partial
Fit: Very strong
Priority: Soon
Decision: Keep

Current state: A dashboard exists, but it is still a basic personal/operational summary rather than a full manager KPI dashboard.
Evidence: `frontend/src/pages/DashboardPage.tsx`, `src/modules/sites/infrastructure/site_repository.rs`

Implementation:
1. Split personal dashboard from manager operations dashboard.
2. Add aggregates for open/completed projects, labor time, and material consumption.
3. Add period filters and drill-down links.
4. Build on top of project/time/material rollups, not ad-hoc frontend math.
