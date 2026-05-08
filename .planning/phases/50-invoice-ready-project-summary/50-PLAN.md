# Phase 50 Plan

## Goal

Add a structured invoice-ready project summary and lightweight export surface without building a billing engine or PDF generator.

## Requirements

- `PROJ-17`
- `FIN-10`

## Plan

1. Add a dedicated read-only invoice-summary endpoint on the existing site API surface.
2. Compose the response from persisted project metadata plus the existing project aggregate summary.
3. Export the new DTO through ts-rs and wire a small frontend hook for it.
4. Add one admin-facing JSON export action on `SiteDetailPage`.
5. Verify the exported summary stays aligned with project metadata and actuals.

## Design Notes

- Keep `/sites/:id` for persisted metadata.
- Keep `/sites/:id/summary` for operational actuals.
- Add `/sites/:id/invoice-summary` as a dedicated composed export contract.
- Export JSON first; no PDF, tax logic, pricing math, or immutable finance snapshots in this phase.

## Verification

- backend DTO and route tests
- `cargo export-types`
- focused `SiteDetailPage` export test
- `npx tsc --noEmit`
- `cargo fmt --check`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
