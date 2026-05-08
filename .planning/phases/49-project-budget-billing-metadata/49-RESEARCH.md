# Phase 49: Project Budget & Billing Metadata - Research

**Researched:** 2026-05-08 [VERIFIED: system date]
**Domain:** Project aggregate metadata on the existing `sites`/project model [VERIFIED: .planning/ROADMAP.md][VERIFIED: src/modules/sites/domain/site.rs]
**Confidence:** HIGH for current code locations and extension path; MEDIUM for exact UX wording and budget-vs-actual presentation semantics [VERIFIED: codebase inspection][ASSUMED]

## User Constraints

- Extend the existing site/project aggregate with lightweight fields only; do not introduce a billing engine. [VERIFIED: user request]
- Decide the cleanest approach for budget amount, billing reference/notes, and lightweight quote reference handling. [VERIFIED: user request]
- Show budget-vs-actual on project detail by reusing the new project aggregate model from the prior aggregate slice instead of creating a separate finance subsystem. [VERIFIED: user request][VERIFIED: .planning/phases/48-project-costing-aggregates/48-RESEARCH.md]
- Do research only; do not write implementation code in this slice. [VERIFIED: user request]

## Summary

The cleanest implementation is to keep **persisted budget/billing metadata on the existing `Site` aggregate** and keep **derived actual usage on the existing `/api/v1/sites/:id/summary` aggregate endpoint**. That matches the current architecture: `SiteResponse` is already the source for persisted project fields, while `SiteProjectSummaryResponse` is already the source for derived labor/material actuals on project detail. [VERIFIED: src/modules/sites/domain/site.rs][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx]

The key constraint is that the repo currently has **no pricing or currency model** for labor or material actuals, and the new aggregate summary only exposes hours and material-withdrawal usage. A true monetary “actual cost” comparison would require rates or price snapshots that do not exist yet, so this slice should show a **budget amount plus operational actuals** (hours, withdrawals, materials) instead of inventing fake financial math. [VERIFIED: grep(src,migrations) no `hourly_rate|unit_price|labor_cost|material_cost|amount_cents|currency`][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs][VERIFIED: .planning/requirement-reviews/projects/PROJ-16.md][VERIFIED: .planning/requirement-reviews/finance/FIN-10.md]

**Primary recommendation:** add `budget_amount_cents`, `billing_reference`, `billing_notes`, and `quote_reference` to `sites`; expose them on existing site DTOs; keep actuals on `/sites/:id/summary`; render a compact “Budget & Abrechnung” card on project detail that pairs stored budget metadata with the existing labor/material summary. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][ASSUMED]

## Project Constraints (from AGENTS.md)

- Every query must remain scoped to `TenantId`. [VERIFIED: AGENTS.md]
- Backend uses Rust + SQLx + PostgreSQL in the modular monolith; changes should stay in the existing `sites` bounded context for this slice. [VERIFIED: AGENTS.md]
- Frontend uses Vite + React and generated TS contracts should continue to flow through `ts-rs`. [VERIFIED: AGENTS.md][VERIFIED: src/modules/sites/api/routes.rs]
- Full-stack contract changes normally require backend + frontend verification and `cargo export-types`. [VERIFIED: AGENTS.md]
- Do not propose patterns that require shared local DB usage; migrations and tests must assume an owner-specific local database. [VERIFIED: AGENTS.md]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Store budget/billing/quote metadata | Database / Storage [VERIFIED: migrations/005_sites_schema.sql] | API / Backend [VERIFIED: src/modules/sites/infrastructure/site_repository.rs] | These are durable project fields on the `sites` row, not client-only state. [VERIFIED: src/modules/sites/domain/site.rs] |
| Validate and patch metadata | API / Backend [VERIFIED: src/modules/sites/application/site_service.rs] | Database / Storage [VERIFIED: src/modules/sites/infrastructure/site_repository.rs] | Validation and tenant/admin enforcement already live in the site service and repository. [VERIFIED: src/modules/sites/application/site_service.rs] |
| Compute actual labor/material usage | API / Backend [VERIFIED: src/modules/sites/application/site_service.rs] | Database / Storage [VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs] | The existing summary endpoint already composes repository-level aggregates. [VERIFIED: src/modules/sites/application/site_service.rs] |
| Render budget-vs-actual detail card | Browser / Client [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx] | API / Backend [VERIFIED: frontend/src/lib/api/hooks/useSites.ts] | The page already joins `/sites/:id` and `/sites/:id/summary`; this slice should extend that pattern. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/lib/api/hooks/useSites.ts] |

## Current code/schema locations to extend

1. **Schema / migration layer** — `migrations/005_sites_schema.sql` defines the `sites` table and currently only includes `estimated_days` as planning metadata; `migrations/018_project_type_on_sites.sql` shows the existing pattern for additive project metadata changes on `sites`. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/018_project_type_on_sites.sql]
2. **Domain aggregate** — `src/modules/sites/domain/site.rs` contains `Site`, `CreateSite`, and `UpdateSite`; this is the canonical backend model to extend for new lightweight project metadata. [VERIFIED: src/modules/sites/domain/site.rs]
3. **Persistence** — `src/modules/sites/infrastructure/site_repository.rs` owns all `INSERT`, `SELECT`, and `UPDATE` statements for `sites`, plus the row mapper back into `Site`. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs]
4. **REST / ts-rs contracts** — `src/modules/sites/api/routes.rs` defines `SiteResponse`, `CreateSiteRequest`, `UpdateSiteRequest`, and the existing `SiteProjectSummaryResponse`. [VERIFIED: src/modules/sites/api/routes.rs]
5. **Aggregate actuals** — `src/modules/sites/application/site_service.rs`, `src/modules/sites/infrastructure/site_repository.rs`, and `src/modules/inventory/infrastructure/material_repository.rs` already provide the labor/material summary that should power the “actual” side of the detail UI. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs]
6. **Frontend types/hooks/UI** — `frontend/src/types/sites.ts`, `frontend/src/lib/api/hooks/useSites.ts`, `frontend/src/pages/sites/SiteDetailPage.tsx`, `frontend/src/pages/sites/ProjectPlanningSheet.tsx`, and optionally `frontend/src/pages/sites/AddSiteDialog.tsx`. [VERIFIED: frontend/src/types/sites.ts][VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/sites/ProjectPlanningSheet.tsx][VERIFIED: frontend/src/pages/sites/AddSiteDialog.tsx]

## Recommended fields and API contracts

### Recommended persisted fields on `sites`

| Field | Suggested DB / domain shape | Why this is the cleanest lightweight choice |
|------|------------------------------|---------------------------------------------|
| `budget_amount_cents` | nullable `BIGINT` / `Option<i64>` [ASSUMED] | The repo has no existing money type or pricing model, so integer minor units avoid introducing a finance library or decimal/rate subsystem in this slice. [VERIFIED: Cargo.toml][VERIFIED: grep(src,migrations) no `amount_cents|currency|rust_decimal|BigDecimal`][ASSUMED] |
| `billing_reference` | nullable text / `Option<String>` [ASSUMED] | Covers invoice reference, PO/reference number, or external bookkeeping handle without committing to a billing workflow. [ASSUMED] |
| `billing_notes` | nullable text / `Option<String>` [ASSUMED] | Captures free-form billing instructions separately from the existing operational `description` field. [VERIFIED: src/modules/sites/domain/site.rs][ASSUMED] |
| `quote_reference` | nullable text / `Option<String>` [ASSUMED] | Lightweight quote tracking fits the user request better than a dedicated quote object or attachment edge. [VERIFIED: user request][ASSUMED] |

### Recommended API contract shape

- **Extend `SiteResponse`** with the four persisted fields so project detail and planning edit surfaces can read them from the same query that already returns `estimated_days`, description, dates, and project type. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx]
- **Allow the fields on `CreateSiteRequest`**, but keep the initial UI minimal by not requiring them in `AddSiteDialog`. This keeps the API complete without expanding the create dialog unless product explicitly wants that. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: frontend/src/pages/sites/AddSiteDialog.tsx][ASSUMED]
- **Add clearable PATCH semantics on `UpdateSiteRequest` for the new nullable fields**, because the current `sites` update path uses `COALESCE(...)` and cannot clear optional values once set. The existing codebase already uses two clean patterns for this problem: `Option<Option<T>>` in `UpdateTimeEntry` and explicit clear flags in inventory updates. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs]
- **Do not move persisted metadata into `/sites/:id/summary`**. `useUpdateSite` already invalidates `site` queries but not `site-summary`, while time and inventory mutations already invalidate `site-summary`. Keeping this split avoids extra cache churn and preserves the current contract boundary. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useInventory.ts]

### Recommended budget-vs-actual contract

- Keep **budget** on `GET /api/v1/sites/:id`. [VERIFIED: src/modules/sites/api/routes.rs][ASSUMED]
- Keep **actual labor/material usage** on `GET /api/v1/sites/:id/summary`. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs]
- On the UI, compose them into one “Budget & Abrechnung” section rather than inventing a new backend finance DTO in this slice. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][ASSUMED]
- Present “actual” as **booked hours + material usage summary**, not computed money, because the codebase currently has no cost/rate inputs for true monetary actuals. [VERIFIED: grep(src,migrations) no `hourly_rate|unit_price|labor_cost|material_cost|currency`][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs]

## Minimal UI surface changes

1. **ProjectPlanningSheet** should gain one compact subsection below planning fields for `Budget`, `Angebotsreferenz`, `Abrechnungsreferenz`, and `Abrechnungshinweise`; this is already the project-edit surface and keeps the create flow unchanged. [VERIFIED: frontend/src/pages/sites/ProjectPlanningSheet.tsx][ASSUMED]
2. **SiteDetailPage** should add a small read-only “Budget & Abrechnung” card or subsection in the existing “Projektdetails” card, directly next to the current metrics and planned-days display. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][ASSUMED]
3. **Budget-vs-actual presentation** should show: stored budget amount, booked total/site/workshop hours, material counts/withdrawals, quote reference, billing reference, and billing notes. This reuses current summary metrics and avoids pretending the app has money-based actuals. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: src/modules/sites/api/routes.rs][ASSUMED]
4. **Quote file handling** should stay lightweight: show the `quote_reference` text here and continue to rely on the existing project timeline document uploads for the actual quote PDF/file until a later quote/invoice document model is required. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: frontend/src/pages/sites/CreateNoteModal.tsx][ASSUMED]

## Standard Stack

### Core

| Library / System | Version | Purpose | Why Standard |
|------------------|---------|---------|--------------|
| Rust backend + `sqlx` | `sqlx 0.8` [VERIFIED: Cargo.toml] | Persist new `sites` columns and keep tenant-scoped queries in the existing repository layer. [VERIFIED: Cargo.toml][VERIFIED: src/modules/sites/infrastructure/site_repository.rs] | This is the current backend persistence stack; no new data library is justified for four lightweight fields. [VERIFIED: Cargo.toml][ASSUMED] |
| Axum + existing site routes | `axum 0.8` [VERIFIED: Cargo.toml] | Extend current site DTOs and handlers. [VERIFIED: Cargo.toml][VERIFIED: src/modules/sites/api/routes.rs] | Existing REST surface already owns project detail + summary. [VERIFIED: src/modules/sites/api/routes.rs] |
| React + TanStack Query + ts-rs generated types | `react 19.2.5`, `@tanstack/react-query 5.100.6`, `ts-rs 12` [VERIFIED: frontend/package.json][VERIFIED: Cargo.toml] | Extend site detail/planning UI and keep typed contracts synced. [VERIFIED: frontend/package.json][VERIFIED: src/modules/sites/api/routes.rs] | Existing site screens already use this stack; no new frontend state layer is needed. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][ASSUMED] |

### Supporting

| Library / System | Version | Purpose | When to Use |
|------------------|---------|---------|-------------|
| Existing project summary endpoint | current app contract [VERIFIED: src/modules/sites/api/routes.rs] | Derived actual labor/material usage. [VERIFIED: src/modules/sites/application/site_service.rs] | Use for the “actual” side of budget-vs-actual. [VERIFIED: src/modules/sites/api/routes.rs] |
| Existing site activity attachments | current app contract [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs] | Attach quote PDFs/docs without a new quote subsystem. [VERIFIED: src/modules/sites/application/site_service.rs] | Use only as a lightweight document store in this slice. [ASSUMED] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| New fields on `sites` [ASSUMED] | Separate `project_billing_metadata` table [ASSUMED] | Unnecessary table and join complexity for four nullable fields on a 1:1 aggregate. [VERIFIED: src/modules/sites/domain/site.rs][ASSUMED] |
| `quote_reference` + existing timeline docs [ASSUMED] | Dedicated `quote_attachment_id` relation [ASSUMED] | Current attachment model is activity/feed-oriented, so a dedicated aggregate-level pointer adds lifecycle edge cases early. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs][ASSUMED] |
| Budget metadata on `SiteResponse` [ASSUMED] | Put budget fields inside `SiteProjectSummaryResponse` [ASSUMED] | That would fight the current cache invalidation split between site metadata and derived actuals. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useInventory.ts] |

**Installation:** no new dependencies recommended for this slice. [VERIFIED: Cargo.toml][VERIFIED: frontend/package.json][ASSUMED]

## Architecture Patterns

### Recommended Project Structure

- Keep all backend changes inside the existing `src/modules/sites/{domain,application,infrastructure,api}` paths and the next additive SQL migration under `migrations/`. [VERIFIED: src/modules/sites/domain/site.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: migrations/005_sites_schema.sql]
- Keep frontend changes inside `frontend/src/types/sites.ts`, `frontend/src/lib/api/hooks/useSites.ts`, `frontend/src/pages/sites/ProjectPlanningSheet.tsx`, and `frontend/src/pages/sites/SiteDetailPage.tsx`. [VERIFIED: frontend/src/types/sites.ts][VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/pages/sites/ProjectPlanningSheet.tsx][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx]

### Pattern 1: Persist metadata on the site aggregate

Store the new fields directly on `Site`, `CreateSite`, `UpdateSite`, and the `sites` table so the project aggregate remains the single source of truth for lightweight planning/billing metadata. [VERIFIED: src/modules/sites/domain/site.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][ASSUMED]

### Pattern 2: Keep derived actuals on the aggregate summary endpoint

Continue using `SiteService::get_project_summary` plus the labor/material repository queries as the only derived “actual” source on project detail. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs]

### Pattern 3: Use explicit clear semantics for nullable patch fields

The codebase already demonstrates that partial updates needing “clear this value” behavior should not rely on plain `Option<T>` + `COALESCE`; use either nested options or explicit clear flags. Apply the same rule to new budget/billing/quote fields so users can remove stale metadata. [VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs]

### Anti-Patterns to Avoid

- **Do not build a billing engine here:** Phase 49 is metadata + detail presentation only; invoice-ready exports belong later. [VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md][VERIFIED: .planning/requirement-reviews/finance/FIN-10.md]
- **Do not compute fake monetary actuals:** there is no verified rate/price data to support them today. [VERIFIED: grep(src,migrations) no `hourly_rate|unit_price|labor_cost|material_cost|currency`] 
- **Do not duplicate summary logic into the frontend:** project detail already consumes a backend summary endpoint. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx]
- **Do not hide metadata inside the activity feed only:** requirement PROJ-16 explicitly calls for project-level budget/quote support, so free-text notes alone are not sufficient. [VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: .planning/requirement-reviews/projects/PROJ-16.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Project billing subsystem | New invoice/billing workflow tables and services [ASSUMED] | Four nullable fields on `sites` + existing summary endpoint [ASSUMED] | The roadmap explicitly sequences invoice-ready work later. [VERIFIED: .planning/ROADMAP.md] |
| Quote document system | Dedicated quote aggregate or storage namespace [ASSUMED] | `quote_reference` plus existing project document uploads [ASSUMED] | Site attachments already accept PDF/doc uploads and are visible on project detail. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx] |
| Money math model | Rate cards, price snapshots, or cost rollups [ASSUMED] | Show stored budget alongside existing actual usage metrics [ASSUMED] | No current pricing source exists, so custom money actuals would be fabricated. [VERIFIED: grep(src,migrations) no `hourly_rate|unit_price|currency|amount_cents`] |

## Common Pitfalls

### Pitfall 1: Using plain `Option<T>` for clearable metadata updates

**What goes wrong:** once a nullable field is set, users cannot clear it because the repository `UPDATE` uses `COALESCE`. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs]

**How to avoid:** use explicit clear semantics for the new nullable fields, following the existing `Option<Option<T>>` or clear-flag patterns already present elsewhere in the codebase. [VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: src/modules/inventory/domain/material.rs]

### Pitfall 2: Treating budget-vs-actual as monetary when no actual cost inputs exist

**What goes wrong:** the UI implies a financially accurate over/under number even though labor/material rates are absent. [VERIFIED: grep(src,migrations) no `hourly_rate|unit_price|labor_cost|material_cost|currency`]

**How to avoid:** label the “actual” side as booked hours/material usage in this slice and defer money actuals to the invoice-ready finance follow-up. [VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md][ASSUMED]

### Pitfall 3: Creating a second source of truth for quote info

**What goes wrong:** quote references live partly on the aggregate and partly only in timeline entries, making the current quote unclear. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][ASSUMED]

**How to avoid:** keep one explicit `quote_reference` field on the aggregate and treat uploaded quote PDFs as supporting documents, not as the primary reference key. [ASSUMED]

## Verification plan

1. **Backend domain/repository** — verify create/update validation for non-negative budget values and nullable clear behavior; verify repository round-trip for new columns and tenant-scoped selects/updates. [VERIFIED: src/modules/sites/domain/site.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][ASSUMED]
2. **Backend DTO contracts** — verify `SiteResponse`, `CreateSiteRequest`, and `UpdateSiteRequest` map all new fields correctly and that generated types stay in sync through `cargo export-types`. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: AGENTS.md]
3. **Frontend query composition** — verify `SiteDetailPage` renders stored metadata from `useSite` and actuals from `useSiteSummary` without adding a new endpoint. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx]
4. **Frontend edit flow** — verify `ProjectPlanningSheet` can save and clear each new field and that cache refresh updates the detail surface. [VERIFIED: frontend/src/pages/sites/ProjectPlanningSheet.tsx][VERIFIED: frontend/src/lib/api/hooks/useSites.ts][ASSUMED]
5. **Regression checks** — keep the existing aggregate-detail test pattern in `SiteDetailPage.test.tsx` and extend it for the new budget/billing UI. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.test.tsx]

**Suggested commands if/when implemented:** `cargo test`, `cargo export-types`, `cargo fmt --check`, `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`, `npm --prefix frontend run test:run -- SiteDetailPage`, and `npm --prefix frontend run build`. [VERIFIED: AGENTS.md][VERIFIED: frontend/package.json][ASSUMED]

## Out-of-scope boundaries

- No invoice generation, PDF generation, accounting export, or downstream bookkeeping sync. [VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md][VERIFIED: .planning/requirement-reviews/finance/FIN-10.md]
- No separate billing or finance bounded context in this slice. [VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md][VERIFIED: .planning/requirement-reviews/finance/FIN-10.md]
- No automatic monetary actual-cost computation from labor/material data until the app has verified pricing/rate inputs. [VERIFIED: grep(src,migrations) no `hourly_rate|unit_price|labor_cost|material_cost|currency`]
- No dedicated quote lifecycle, quote versioning, or quote-specific attachment model. [VERIFIED: user request][ASSUMED]
- No dashboard/reporting expansion beyond project detail. [VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/phases/48-project-costing-aggregates/48-RESEARCH.md]

## State of the Art

| Old / Current approach | Recommended current approach | Impact |
|------------------------|------------------------------|--------|
| `SiteDetailPage` already joins `useSite` + `useSiteSummary` and shows operational metrics only. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx] | Keep that split; add persisted metadata to `useSite` and reuse existing summary for actuals. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][ASSUMED] | Minimal API churn and cleaner cache invalidation. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useInventory.ts] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `budget_amount_cents` as nullable `BIGINT` / `Option<i64>` is the best storage shape. | Recommended fields and API contracts | If product wants decimal-string or multi-currency support now, migration/API design changes. |
| A2 | `billing_reference`, `billing_notes`, and `quote_reference` should all be nullable text fields rather than separate entities. | Recommended fields and API contracts | If the business needs structured finance/quote workflows immediately, this slice would under-model the domain. |
| A3 | Budget-vs-actual in this slice should be operational, not monetary. | Summary / UI / boundaries | If stakeholders require real money variance now, Phase 49 scope is too small and should be reframed. |
| A4 | Keeping the create dialog unchanged is the preferred minimal UX. | Minimal UI surface changes | If users expect entering budget/quote data at creation time, `AddSiteDialog` must expand. |

## Open Questions

1. **Does “budget-vs-actual” need a real money delta now, or is budget plus operational actual usage acceptable for this slice?** [VERIFIED: .planning/requirement-reviews/projects/PROJ-16.md][ASSUMED]
2. **Is `quote_reference` enough, or does product expect one quote PDF to be explicitly spotlighted from the existing timeline attachments?** [VERIFIED: src/modules/sites/api/routes.rs][ASSUMED]
3. **Should budget/billing fields be editable during project creation, or only from the existing planning sheet after creation?** [VERIFIED: frontend/src/pages/sites/AddSiteDialog.tsx][VERIFIED: frontend/src/pages/sites/ProjectPlanningSheet.tsx][ASSUMED]

## Environment Availability

Skipped: this phase is an in-repo schema/API/UI extension and does not introduce a new external runtime or service dependency beyond the existing project stack. [VERIFIED: user request][VERIFIED: Cargo.toml][VERIFIED: frontend/package.json]

## Security Domain

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | yes [VERIFIED: AGENTS.md] | Keep existing Keycloak/JWT-authenticated site routes. [VERIFIED: AGENTS.md][VERIFIED: src/modules/sites/api/routes.rs] |
| V4 Access Control | yes [VERIFIED: src/modules/sites/application/site_service.rs] | Preserve existing admin-only create/update site checks and tenant-scoped repository filters. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs] |
| V5 Input Validation | yes [VERIFIED: src/modules/sites/domain/site.rs] | Add non-negative numeric validation and trim/normalize empty nullable strings before persistence. [VERIFIED: src/modules/sites/domain/site.rs][ASSUMED] |
| V6 Cryptography | no new crypto [VERIFIED: scope + code inspection] | Reuse existing auth stack only. [VERIFIED: AGENTS.md] |

**Known threat patterns:** tenant leakage through unscoped project updates, unauthorized metadata edits by non-admin users, and stored unsafe text in billing notes/reference fields if input normalization is skipped. The first two are already mitigated by current site service and repository patterns; the third needs standard input validation and safe rendering discipline. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][ASSUMED]

## Sources

### Primary (HIGH confidence)

- `.planning/ROADMAP.md` - phase sequencing and scope boundary. [VERIFIED: .planning/ROADMAP.md]
- `.planning/REQUIREMENTS.md` - `PROJ-16` and related requirement intent. [VERIFIED: .planning/REQUIREMENTS.md]
- `.planning/requirement-reviews/projects/PROJ-16.md` - specific implementation expectations for budget/quote/budget-vs-actual. [VERIFIED: .planning/requirement-reviews/projects/PROJ-16.md]
- `.planning/requirement-reviews/projects/PROJ-17.md` and `.planning/requirement-reviews/finance/FIN-10.md` - explicit later-phase billing/invoice boundary. [VERIFIED: .planning/requirement-reviews/projects/PROJ-17.md][VERIFIED: .planning/requirement-reviews/finance/FIN-10.md]
- `src/modules/sites/domain/site.rs`, `src/modules/sites/api/routes.rs`, `src/modules/sites/application/site_service.rs`, `src/modules/sites/infrastructure/site_repository.rs` - current project aggregate model and API. [VERIFIED: src/modules/sites/domain/site.rs][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs]
- `src/modules/inventory/infrastructure/material_repository.rs` - current material actuals summary. [VERIFIED: src/modules/inventory/infrastructure/material_repository.rs]
- `frontend/src/lib/api/hooks/useSites.ts`, `frontend/src/lib/api/hooks/useInventory.ts`, `frontend/src/pages/sites/SiteDetailPage.tsx`, `frontend/src/pages/sites/ProjectPlanningSheet.tsx`, `frontend/src/pages/sites/SiteDetailPage.test.tsx` - current frontend query split, cache invalidation, and detail UI/test surface. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useInventory.ts][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/sites/ProjectPlanningSheet.tsx][VERIFIED: frontend/src/pages/sites/SiteDetailPage.test.tsx]

### Tertiary (LOW confidence / design assumptions)

- Field naming, minor-unit money storage choice, and exact UI wording are design recommendations derived from the current codebase rather than explicit product decisions. [ASSUMED]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new libraries are needed and the extension points are explicit in the current code. [VERIFIED: Cargo.toml][VERIFIED: frontend/package.json][VERIFIED: src/modules/sites/api/routes.rs]
- Architecture: HIGH - the current repo already cleanly separates persisted site metadata from derived project summary data. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: frontend/src/lib/api/hooks/useSites.ts]
- Pitfalls: HIGH - the nullable patch/update pitfalls and missing pricing model are directly visible in the current codebase. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: grep(src,migrations) no `hourly_rate|unit_price|labor_cost|material_cost|currency`]

**Research date:** 2026-05-08 [VERIFIED: system date]
**Valid until:** 2026-06-07 unless Phase 48/49 implementation materially changes the project summary contract first. [VERIFIED: .planning/STATE.md][ASSUMED]
