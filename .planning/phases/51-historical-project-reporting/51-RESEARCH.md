# Phase 51 Research

## Scope

- `RPT-10`

## Key Findings

- Reuse the existing project aggregate metadata and the canonical labor/material aggregate model from earlier `v1.14` phases.
- Keep reporting on the projects/sites surface, not on the dashboard.
- Default the historical reporting scope to completed and archived projects.
- Derive `cost_basis` from existing metadata and actuals; do not persist it.
- Keep the first reporting surface read-only and manager-focused.

## Recommended Shape

- Backend:
  - add a dedicated read model endpoint under `/api/v1/sites/history-report`
  - derive rows from persisted site metadata plus separate labor/material aggregate subqueries
  - support filters for customer, project type, worker, date range, duration, and cost basis
- Frontend:
  - add a small `SiteHistoryReportPage`
  - expose it from the projects area (`/sites/history`)
  - keep it read-only and link each row to the existing project detail page

## Guardrails

- No dashboard rewrite
- No billing engine or accounting export
- No fake monetary actuals
- No edit actions on the reporting page

## Verification

- backend report query tests
- frontend report page tests
- `cargo export-types`
- `npx tsc --noEmit`
- `cargo fmt --check`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
