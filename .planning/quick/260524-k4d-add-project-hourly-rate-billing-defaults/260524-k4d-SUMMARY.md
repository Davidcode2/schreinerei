---
status: complete
quick_id: 260524-k4d
date: 2026-05-24
code_commit: 8c07380
---

# Quick task 260524-k4d Summary

## Outcome

- Added project-level invoice pricing defaults on `sites`: `invoice_pricing_mode`, `hourly_rate_cents`, and `fixed_price_cents`.
- Added tenant-level `default_hourly_rate_cents` used to prefill new projects when no hourly rate is provided at create time.
- Kept tenant default updates create-only: explicit hourly rates on project creation refresh the tenant default, later project edits do not.
- Extended invoice draft generation, persisted snapshots, generated TypeScript contracts, and Typst PDF output to support:
  - unpriced legacy invoices
  - hourly-rate labor pricing
  - fixed-price project invoices
- Kept materials unpriced while still showing booked hours on invoices via the existing labor summary.

## Key Backend Decisions

- Pricing policy remains on the billing path, not in project summary aggregation.
- Fixed price is stored on the project and rendered as a dedicated invoice line item.
- Hourly mode keeps the existing site/workshop labor split, but applies the same project hourly rate to both labor lines.

## Verification

- `cargo test --lib`
- `cargo test --test billing_repository_test`
- `cargo export-types`
- `cargo fmt --check`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`

All commands passed.
