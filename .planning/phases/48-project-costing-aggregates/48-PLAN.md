# Phase 48 Plan

## Goal

Add canonical per-project labor and material aggregates on the project detail surface.

## Requirements

- `RPT-12`
- cost-basis foundation for `PROJ-16`

## Plan

1. Add backend project aggregate read models.
2. Expose the aggregate summary through the existing site API surface.
3. Replace client-side hours math on `SiteDetailPage` with the backend summary.
4. Add material usage metrics and a compact breakdown on project detail.
5. Verify aggregate correctness, exclusion rules, and mutation-driven freshness.

## Design Notes

- Labor is aggregated from `time_entries` grouped by `site_id`.
- Material usage is aggregated from `stock_entries` grouped by `site_id`, restricted to `entry_type='withdrawn'`.
- Disposal and adjustments must not count as project material consumption.
- Use one composed summary DTO instead of multiple ad-hoc frontend fetches.

## Verification

- `cargo test` for repository and route-level aggregate behavior
- targeted frontend tests for `SiteDetailPage`
- `cargo export-types`
- `npx tsc --noEmit`
- `cargo fmt --check`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
