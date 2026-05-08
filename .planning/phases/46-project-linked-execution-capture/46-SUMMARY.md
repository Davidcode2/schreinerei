# Phase 46 Summary

Completed: 2026-05-07

## Outcome

- Material withdrawals now require a project link for real consumption and still allow disposal without attribution.
- Productive time booking now requires project linkage for both on-site and workshop work, while overhead time can remain unlinked.
- Time-entry updates can now explicitly clear `site_id` when a booking changes to non-project overhead work.

## Verification

- `SQLX_OFFLINE=true cargo test`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
- `cargo fmt --check`
- `npm test -- --run src/pages/sites/TimeEntryDialog.test.tsx src/pages/inventory/InventoryDetailPage.test.tsx src/pages/DashboardPage.test.tsx`
- `npx tsc --noEmit`
