# Phase 49 Plan

## Goal

Add lightweight budget and billing metadata to the project aggregate and show it next to the existing project aggregate actuals.

## Requirements

- `PROJ-16`
- `FIN-10`

## Plan

1. Extend the `sites` schema and aggregate with budget and billing metadata fields.
2. Expose the new fields on the existing site create/update/read contracts.
3. Add explicit clear semantics for the new nullable update fields.
4. Extend `ProjectPlanningSheet` with lightweight budget and billing fields.
5. Add a read-only “Budget & Abrechnung” section to `SiteDetailPage` using site metadata plus the existing aggregate actuals.
6. Verify create/update/read contracts, clear behavior, and detail rendering.

## Design Notes

- Persist only lightweight fields in this phase:
  - `budget_amount_cents`
  - `billing_reference`
  - `billing_notes`
  - `quote_reference`
- Keep actuals on `/api/v1/sites/:id/summary`.
- Do not compute fake money actuals; pair stored budget with operational actuals only.
- Keep quote handling metadata-only; actual quote files remain timeline attachments.

## Verification

- targeted `ProjectPlanningSheet` and `SiteDetailPage` tests
- `cargo export-types`
- `npx tsc --noEmit`
- `cargo fmt --check`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
