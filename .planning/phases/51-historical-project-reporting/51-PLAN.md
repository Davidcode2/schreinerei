# Phase 51 Plan

## Goal

Add a manager-focused historical project reporting surface with structured filters over completed and archived projects.

## Requirements

- `RPT-10`

## Plan

1. Add a historical project reporting read model on the existing sites/projects backend surface.
2. Derive reporting rows from persisted project metadata plus the canonical labor/material aggregates.
3. Support filters for customer, date range, worker, duration, project type, and cost basis.
4. Add a read-only manager-facing reporting page under the projects area.
5. Verify filter behavior, tenant scoping, and row-to-detail navigation.

## Design Notes

- Keep this reporting-only and read-only.
- Default scope to completed and archived projects.
- Derive `cost_basis`; do not persist it.
- Do not broaden this into a dashboard rewrite.

## Verification

- backend report query tests
- frontend reporting page tests
- `cargo export-types`
- `npx tsc --noEmit`
- `cargo fmt --check`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
