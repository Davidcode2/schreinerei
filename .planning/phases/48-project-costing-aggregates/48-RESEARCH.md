# Phase 48 Research

## Scope

- `RPT-12`
- cost-basis foundation for `PROJ-16`

## Key Findings

- Keep this as a dedicated project-detail read model, not a new accounting subsystem.
- Labor source of truth: `time_entries.site_id`.
- Material source of truth: `stock_entries.site_id` filtered to `entry_type='withdrawn'`.
- Aggregate labor and material separately, then compose them, to avoid join-multiplication bugs.
- Surface the first slice on `SiteDetailPage` only.
- The resulting aggregate model becomes the canonical basis for budget-vs-actual, invoice-ready summaries, planning, and reporting.

## Recommended Shape

- Backend:
  - labor summary query in `SiteRepository`
  - material usage summary query in `MaterialRepository`
  - small composed project summary endpoint on the existing site surface
- Frontend:
  - replace client-side total-hours math on `SiteDetailPage`
  - add a compact project metrics section
  - add a small material usage breakdown

## Guardrails

- No money fields in this phase.
- No new accounting tables.
- No broader manager dashboard work here.
- Group material usage by material and unit; do not invent a fake cross-unit total.

## Verification

- backend tests for aggregate queries and edge cases
- frontend tests for project detail metrics rendering
- invalidation checks for time and inventory mutations
