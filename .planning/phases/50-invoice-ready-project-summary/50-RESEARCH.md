# Phase 50: Invoice-Ready Project Summary - Research

**Researched:** 2026-05-08 [VERIFIED: system date]
**Domain:** Structured invoice-ready project summary/export surface on the existing sites module [VERIFIED: .planning/ROADMAP.md][VERIFIED: src/modules/sites/api/routes.rs]
**Confidence:** HIGH for reuse locations and API boundary; MEDIUM for exact export payload wording and future snapshot evolution [VERIFIED: codebase inspection][ASSUMED]

## User Constraints

- Reuse the current site aggregate, the Phase 48 project summary endpoint, and the Phase 49 budget/billing metadata. [VERIFIED: user request][VERIFIED: .planning/phases/48-project-costing-aggregates/48-RESEARCH.md][VERIFIED: .planning/phases/49-project-budget-billing-metadata/49-RESEARCH.md]
- Expose a structured invoice-ready summary or export surface without PDF generation or a billing engine. [VERIFIED: user request][VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md][VERIFIED: .planning/requirement-reviews/finance/FIN-10.md]
- Do research only; do not write implementation code. [VERIFIED: user request]

## Project Constraints (from AGENTS.md)

- Keep all queries tenant-scoped via `TenantId`. [VERIFIED: AGENTS.md]
- Stay on the existing Rust + SQLx + REST + React + ts-rs stack. [VERIFIED: AGENTS.md]
- Prefer changes inside the existing `sites` bounded context unless a new finance context is explicitly required. [VERIFIED: AGENTS.md][ASSUMED]

## Summary

The cleanest Phase 50 shape is a **new read-only invoice-summary endpoint on the existing site API surface** that composes three already-existing sources: persisted project metadata from `Site`, derived labor/material actuals from `SiteService::get_project_summary`, and optional quote/billing references already stored on `sites`. [VERIFIED: src/modules/sites/domain/site.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: migrations/022_site_budget_billing_metadata.sql]

Do **not** extend the existing `/api/v1/sites/:id/summary` response for this. That endpoint is already the narrow operational-actuals contract from Phase 48, while `GET /api/v1/sites/:id` remains the persisted metadata contract from Phase 49. A dedicated invoice-summary/export DTO keeps those boundaries intact and gives finance/export work a stable contract to grow into later. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: .planning/phases/49-project-budget-billing-metadata/49-RESEARCH.md]

**Primary recommendation:** add `GET /api/v1/sites/:id/invoice-summary` returning a versioned JSON document that merges current site metadata plus current labor/material aggregates, then expose it through a minimal admin-facing JSON export action on `SiteDetailPage`. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: .planning/requirement-reviews/finance/FIN-12.md][ASSUMED]

## 1. Current code/schema locations to reuse

1. `src/modules/sites/domain/site.rs` — canonical persisted project aggregate, including `budget_amount_cents`, `billing_reference`, `billing_notes`, and `quote_reference`. [VERIFIED: src/modules/sites/domain/site.rs]
2. `migrations/022_site_budget_billing_metadata.sql` — existing schema location for billing/budget metadata on `sites`; no new billing table exists today. [VERIFIED: migrations/022_site_budget_billing_metadata.sql]
3. `src/modules/sites/application/site_service.rs::get_project_summary` — existing composition point for project actuals. [VERIFIED: src/modules/sites/application/site_service.rs]
4. `src/modules/sites/infrastructure/site_repository.rs::get_project_labor_summary` — canonical labor aggregate query from `time_entries.site_id`. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs]
5. `src/modules/inventory/infrastructure/material_repository.rs::get_project_material_summary` — canonical material aggregate query from `stock_entries` filtered to `entry_type='withdrawn'`. [VERIFIED: src/modules/inventory/infrastructure/material_repository.rs][VERIFIED: .planning/phases/48-project-costing-aggregates/48-RESEARCH.md]
6. `src/modules/sites/api/routes.rs` — current DTO/export pattern via `#[ts(export)]`, plus existing `GET /api/v1/sites/:id` and `GET /api/v1/sites/:id/summary` handlers to mirror. [VERIFIED: src/modules/sites/api/routes.rs]
7. `frontend/src/lib/api/hooks/useSites.ts`, `frontend/src/types/sites.ts`, and `frontend/src/pages/sites/SiteDetailPage.tsx` — current frontend composition point and minimal place to hang an export button. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/types/sites.ts][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx]
8. `frontend/src/pages/sites/MediaViewer.tsx` + `frontend/src/lib/api/client.ts::getBlob` — existing protected-download pattern reusable for a JSON export file. [VERIFIED: frontend/src/pages/sites/MediaViewer.tsx][VERIFIED: frontend/src/lib/api/client.ts]

## 2. Recommended backend/API shape

### Recommended endpoint

- `GET /api/v1/sites/:id/invoice-summary` [ASSUMED]
- Read-only, tenant-scoped, same site existence check pattern as `get_site`/`get_site_summary`. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs]
- Build the response by:
  1. loading `Site` from `get_site`,
  2. loading operational actuals from `get_project_summary`,
  3. shaping one export DTO in `routes.rs`. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/api/routes.rs]

### Recommended DTO shape

```ts
type SiteInvoiceSummaryResponse = {
  export_version: "v1"
  generated_at: string
  project: {
    id: string
    name: string
    project_type: string
    customer_name: string
    location: string | null
    status: string
    start_date: string | null
    end_date: string | null
    estimated_days: number | null
  }
  billing: {
    budget_amount_cents: bigint | null
    quote_reference: string | null
    billing_reference: string | null
    billing_notes: string | null
  }
  labor: {
    total_hours: number
    entry_count: bigint
    site_hours: number
    workshop_hours: number
    last_work_date: string | null
  }
  materials: {
    distinct_material_count: bigint
    withdrawal_count: bigint
    lines: Array<{
      material_id: string
      material_name: string
      category_name: string
      unit: string
      total_withdrawn: number
      withdrawal_count: bigint
      last_withdrawn_at: string
    }>
  }
}
```

This is the cleanest shape because it **reuses existing field names and substructures** instead of inventing finance-specific semantics the app cannot yet support. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: frontend/src/types/generated.ts]

### Why a new endpoint is cleaner than changing `/summary`

- `/sites/:id` already owns persisted project metadata. [VERIFIED: src/modules/sites/api/routes.rs]
- `/sites/:id/summary` already owns operational aggregates only. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs]
- `useUpdateSite` invalidates `site`, while time/material mutations invalidate `site-summary`; merging them would blur current cache boundaries. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts]
- A dedicated invoice-summary DTO can later become the payload stored in immutable snapshots without breaking the current detail page contracts. [VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md][ASSUMED]

### Important guardrail

Do **not** add computed monetary actuals, tax logic, invoice numbers, or line-item pricing. The codebase still has no verified labor rate or material price model, so the invoice-ready surface should remain a **structured operational + metadata export**, not a billing calculator. [VERIFIED: .planning/phases/49-project-budget-billing-metadata/49-RESEARCH.md][VERIFIED: grep(src,migrations) no `hourly_rate|unit_price|labor_cost|material_cost|currency`]

## 3. Minimal UI or export surface

Recommended minimal surface: **one admin-facing action on `SiteDetailPage` labeled like “Projektübersicht exportieren” that downloads the invoice-summary payload as JSON**. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/sites/MediaViewer.tsx][ASSUMED]

Why JSON first:

- It preserves nested material lines and summary structure cleanly. [VERIFIED: src/modules/sites/api/routes.rs]
- It matches the repo’s existing DTO-first + ts-rs contract style. [VERIFIED: AGENTS.md][VERIFIED: src/modules/sites/api/routes.rs]
- FIN-12 explicitly recommends starting with export contracts before deeper finance integration. [VERIFIED: .planning/requirement-reviews/finance/FIN-12.md]

Avoid a new full screen, billing dashboard, or PDF preview in this slice. A small button or menu action is enough. [VERIFIED: user request][VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md]

## 4. Verification plan

1. **Backend contract test** — verify `GET /api/v1/sites/:id/invoice-summary` returns site metadata + labor summary + material summary from the existing sources without dropping tenant scoping. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs]
2. **Parity test** — verify invoice-summary fields match `GET /sites/:id` for billing metadata and `GET /sites/:id/summary` for operational actuals. [VERIFIED: src/modules/sites/api/routes.rs]
3. **Generated contract check** — run `cargo export-types` if the DTO is added and ensure frontend types stay aligned. [VERIFIED: AGENTS.md]
4. **Frontend export test** — verify the detail page action requests the new endpoint and downloads a JSON blob/file using the existing blob-download pattern. [VERIFIED: frontend/src/pages/sites/MediaViewer.tsx][VERIFIED: frontend/src/lib/api/client.ts]
5. **Regression check** — keep the existing `SiteDetailPage` aggregate rendering tests green and add a focused export-action test instead of building a second detail surface. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.test.tsx]

## 5. Strict out-of-scope boundaries

- No PDF invoice generation. [VERIFIED: user request][VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md]
- No billing engine, tax engine, or invoice status workflow. [VERIFIED: user request][VERIFIED: .planning/requirement-reviews/finance/FIN-10.md]
- No DATEV integration or external accounting sync yet. [VERIFIED: .planning/requirement-reviews/finance/FIN-12.md]
- No monetary actual-cost computation until labor/material pricing inputs exist. [VERIFIED: .planning/phases/49-project-budget-billing-metadata/49-RESEARCH.md]
- No immutable invoice snapshot persistence in this slice unless the scope is explicitly widened; keep this phase as a live read/export contract first. [VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md][ASSUMED]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | A dedicated `GET /sites/:id/invoice-summary` endpoint is cleaner than extending `/summary`. | Recommended backend/API shape | If product strongly prefers a single endpoint, cache and contract boundaries would need redesign. |
| A2 | JSON download is the best minimal export surface for this slice. | Minimal UI or export surface | If finance stakeholders require CSV immediately, the export format would change. |
| A3 | Immutable snapshot persistence should wait for a later finance-oriented slice. | Out-of-scope boundaries | If completion-time auditability is required now, a live read endpoint is insufficient. |
