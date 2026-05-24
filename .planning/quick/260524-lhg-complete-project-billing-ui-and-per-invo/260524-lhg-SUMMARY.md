---
status: complete
quick_id: 260524-lhg
date: 2026-05-24
code_commit: 542a08d
---

# Quick task 260524-lhg Summary

## Outcome

- Added admin billing settings UI so the tenant-wide default hourly rate can be viewed and updated from `/settings`.
- Extended project create and edit forms with persistent billing defaults:
  - invoice pricing mode
  - hourly rate
  - fixed price
- Added a dedicated invoice creation dialog on the project detail page so admins can override pricing per invoice without mutating the saved project.
- Extended the billing create contract to accept one-off pricing overrides and resolve effective billing data in one place before draft, snapshot, and PDF generation.
- Aligned site and invoice frontend types, mocks, hooks, and tests with the billing model.

## Key Decisions

- Tenant default hourly rate is only a convenience for new project creation and remains separate from later project edits.
- Project billing defaults stay on the project aggregate and are visible in the project detail page.
- Per-invoice overrides affect only the created invoice draft/PDF and do not write back to the project.

## Verification

- `cargo test --lib`
- `cargo fmt --check`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
- `cargo export-types`
- `npm run test -- useSites.test.tsx SettingsPage.test.tsx AddSiteDialog.test.tsx ProjectPlanningSheet.test.tsx SiteDetailPage.test.tsx`
- `npm run build`

All commands passed.
