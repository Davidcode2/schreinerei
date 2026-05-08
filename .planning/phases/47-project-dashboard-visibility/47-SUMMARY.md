# Phase 47 Summary

Completed: 2026-05-07

## Outcome

- Dashboard project loading no longer hides completed work at the backend query layer.
- The dashboard now shows planned, active, and completed projects by default.
- Status filtering is explicit and user-controlled through dashboard filter buttons, with archived projects available on demand.

## Verification

- `SQLX_OFFLINE=true cargo test`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
- `cargo fmt --check`
- `npm test -- --run src/pages/sites/TimeEntryDialog.test.tsx src/pages/inventory/InventoryDetailPage.test.tsx src/pages/DashboardPage.test.tsx`
- `npx tsc --noEmit`
