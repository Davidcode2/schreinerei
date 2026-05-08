# Phase 46: Project-Linked Execution Capture - Research

**Researched:** 2026-05-07  
**Domain:** Project-attributed material withdrawals and productive time booking in the existing Rust/React modular monolith [VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/REQUIREMENTS.md]  
**Confidence:** HIGH

<user_constraints>
## User Constraints

- No phase-specific `46-CONTEXT.md` exists, so this research is constrained by the canonical planning sources only (`FEATURES.md`, `PROJECT.md`, `REQUIREMENTS.md`, `ROADMAP.md`, `STATE.md`, and the per-requirement reviews). [VERIFIED: gsd-sdk init.phase-op][VERIFIED: codebase read]
- Keep the workflow project-centric and mobile-first; reduce office follow-up instead of adding admin-heavy capture steps. [VERIFIED: .planning/PROJECT.md][VERIFIED: .planning/FEATURES.md][VERIFIED: .planning/REQUIREMENT-REVIEW-SUMMARY.md]
- Phase 46 must address `PROJ-13` and `PROJ-14` only; dashboard visibility is Phase 47. [VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/REQUIREMENTS.md]
- Deferred and out-of-scope topics remain deferred here: RFID, GPS tracking, customer intake, deep CAD/CNC automation, and finance integrations beyond attribution groundwork. [VERIFIED: .planning/PROJECT.md][VERIFIED: .planning/REQUIREMENT-REVIEW-SUMMARY.md]
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROJ-13 | Every material consumption booking is linked to a project to support billing and analytics. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse existing `stock_entries.site_id`, enforce linkage in backend validation for non-disposal withdrawals, and default the picker from `active_site_id`. [VERIFIED: migrations/011_user_preferences.sql][VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.tsx] |
| PROJ-14 | Workers can book time to a project for both on-site work and workshop preparation / CNC work. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse existing `time_entries.site_id`, require linkage for productive `site`/`workshop` bookings, keep overhead/null cases explicit, and default from `active_site_id`. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Keep functions short, classes short, names descriptive, and responsibilities single-purpose. [VERIFIED: AGENTS.md]
- Every query and mutation must remain tenant-scoped through `TenantId`/request context. [VERIFIED: AGENTS.md]
- Stay inside the modular-monolith boundaries (`common`, `auth`, `modules/*`) rather than creating cross-cutting shadow subsystems. [VERIFIED: AGENTS.md]
- Prefer the modern Rust module file structure for any new module work; do not introduce new `mod.rs`-style modules. [VERIFIED: AGENTS.md]
- Use `ts-rs` for request/response DTO contract changes and regenerate frontend types with `cargo export-types` when DTOs change. [VERIFIED: AGENTS.md]
- Before any runtime verification, use a dedicated worktree-local PostgreSQL database/container; never run tests or migrations against a shared local database. [VERIFIED: AGENTS.md]
- Full-stack changes must verify both backend and frontend quality gates before commit. [VERIFIED: AGENTS.md]
- Use `bd` for task tracking during execution; do not introduce markdown TODO workflows. [VERIFIED: AGENTS.md]

## Summary

Phase 46 does **not** need a new execution-capture subsystem. The backend already stores project linkage on both stock movements (`stock_entries.site_id`) and time bookings (`time_entries.site_id`), and the frontend already carries an active project preference (`user_preferences.preferences.active_site_id`) that is used to prefill multiple flows. [VERIFIED: migrations/011_user_preferences.sql][VERIFIED: migrations/005_sites_schema.sql][VERIFIED: src/modules/iam/domain/user_preferences.rs][VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.tsx][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx]

The real gap is **rule enforcement consistency**. Material withdrawals still accept an optional `site_id`, and time booking still treats project selection as optional for `site` work while forcing `null` for `workshop` work. That leaves the exact requirement gap the product reviews identified: productive work can still be captured without project attribution, which weakens future billing and reporting. [VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: frontend/src/pages/inventory/WithdrawDialog.tsx][VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: .planning/requirement-reviews/projects/PROJ-13.md][VERIFIED: .planning/requirement-reviews/projects/PROJ-14.md]

The best plan is therefore a **hardening-and-defaulting phase**: enforce project linkage on the server for real consumption/productive time, preserve explicit exceptions (`disposal`, `adjust_stock`, and true overhead/admin time), and keep the UI fast by defaulting from `active_site_id` whenever that context already exists. [VERIFIED: .planning/ROADMAP.md][VERIFIED: src/modules/inventory/application/inventory_service.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/iam/application/user_preferences_service.rs]

**Primary recommendation:** Reuse the existing `site_id` columns and `active_site_id` preference, add backend validation for attribution rules, and only change UI copy/selection logic enough to make compliant capture the fastest path. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/011_user_preferences.sql][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: frontend/src/pages/inventory/WithdrawDialog.tsx]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Enforce project linkage for material consumption | API / Backend | Database / Storage | The rule must be authoritative server-side because the current API accepts optional `site_id` and writes audit rows directly into `stock_entries`. [VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs] |
| Persist attributable material history | Database / Storage | API / Backend | `stock_entries.site_id` already exists and is the durable reporting/billing link. [VERIFIED: migrations/011_user_preferences.sql][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs] |
| Enforce project linkage for productive time | API / Backend | Database / Storage | `time_entries.site_id` is already the persistence hook, but the business rule is currently too loose and must be validated before insert/update. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: src/modules/sites/application/site_service.rs] |
| Prefill project in booking dialogs | Browser / Client | API / Backend | The fastest-path UX comes from reusing `preferences.active_site_id` in the dialogs, while the backend still validates the submitted project. [VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.tsx][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: src/modules/iam/application/user_preferences_service.rs] |
| Downstream analytics/billing readiness | Database / Storage | API / Backend | Future reporting phases can aggregate by existing `site_id` links; this phase should improve data discipline rather than invent a new reporting store. [VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs] |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Axum | `0.8` pinned in repo; latest stable `0.8.9` published 2026-04-14. [VERIFIED: Cargo.toml][VERIFIED: crates.io API] | HTTP handlers, JSON extraction, route composition. [CITED: https://github.com/tokio-rs/axum/blob/main/axum/src/docs/extract.md] | Existing inventory and time-entry routes already follow Axum extractor patterns, so Phase 46 should extend those handlers instead of introducing a new transport abstraction. [VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: src/modules/sites/api/routes.rs] |
| SQLx | `0.8` pinned in repo; latest stable `0.8.6` published 2025-10-15 (`0.9.0-alpha.1` exists but is pre-release). [VERIFIED: Cargo.toml][VERIFIED: crates.io API] | PostgreSQL queries and transactions. [CITED: https://github.com/launchbadge/sqlx/blob/main/README.md] | Material withdrawal already uses a transaction with row locking, which is exactly the right place to keep attribution writes atomic. [VERIFIED: src/modules/inventory/infrastructure/material_repository.rs][CITED: https://context7.com/launchbadge/sqlx/llms.txt] |
| React | `19.2.5` pinned in repo; latest `19.2.6` published 2026-05-06. [VERIFIED: frontend/package.json][VERIFIED: npm registry] | Mobile-first dialog state and selection UX. [VERIFIED: frontend/src/pages/inventory/WithdrawDialog.tsx][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx] | Existing dialogs are already controlled React components; this phase only needs targeted form-state changes, not a new form framework. [VERIFIED: frontend/src/pages/inventory/WithdrawDialog.tsx][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx] |
| `@tanstack/react-query` | `5.100.6` pinned in repo; latest `5.100.9` published 2026-05-03. [VERIFIED: frontend/package.json][VERIFIED: npm registry] | Mutation orchestration and cache invalidation. [CITED: https://github.com/tanstack/query/blob/main/docs/framework/react/guides/invalidations-from-mutations.md] | The current hooks already invalidate time-entry, site, inventory, and dashboard queries after mutations; Phase 46 should preserve that pattern. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][CITED: https://github.com/tanstack/query/blob/main/docs/framework/react/guides/invalidations-from-mutations.md] |
| `ts-rs` | `12` pinned in repo; latest stable `12.0.1` published 2026-01-31. [VERIFIED: Cargo.toml][VERIFIED: crates.io API] | Rust-to-TypeScript DTO generation. [VERIFIED: AGENTS.md] | If request DTOs change from optional to required semantics, the generated frontend contract must change from the Rust source of truth. [VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: AGENTS.md] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Vitest | `4.1.5` pinned and latest published 2026-05-05. [VERIFIED: frontend/package.json][VERIFIED: npm registry] | Fast dialog and request-payload regression tests. [VERIFIED: frontend/src/pages/sites/TimeEntryDialog.test.tsx][VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.test.tsx] | Use for UX/defaulting rules, especially payload shaping and required-field behavior. [VERIFIED: frontend/src/pages/sites/TimeEntryDialog.test.tsx][VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.test.tsx] |
| MSW | `2.14.2` pinned in repo. [VERIFIED: frontend/package.json] | API-layer mocking for frontend request assertions. [VERIFIED: frontend/src/test/mocks/handlers.ts] | Use when verifying that dialogs send `site_id`/`notes`/`work_type` combinations correctly without a running backend. [VERIFIED: frontend/src/test/mocks/handlers.ts] |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Reusing `stock_entries.site_id` and `time_entries.site_id` | New “execution capture” tables or a separate billing-prep subsystem | Do **not** do this in Phase 46; the schema already has attribution columns, so a new subsystem adds migration and sync risk without unlocking the requirement. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/011_user_preferences.sql] |
| Existing controlled dialogs | A new form stack such as React Hook Form + Zod | [ASSUMED] This could reduce field boilerplate later, but it expands scope and is not required to enforce attribution rules in the current narrow dialogs. |

**Installation:**
```bash
# No new npm packages are recommended for Phase 46.
```

## Architecture Patterns

### System Architecture Diagram

```text
User opens material/time dialog
        |
        v
Frontend loads active project preference (`/api/v1/preferences`)
        |
        +--> Prefill selected project in dialog when known
        |
        v
User submits capture payload
        |
        v
Axum handler parses JSON + request context
        |
        v
Application service validates attribution rule
  - material withdrawal: require project unless disposal/correction path
  - time entry: require project for productive work types
        |
        v
SQLx transaction / write
  - update current quantity or insert time entry
  - persist `site_id` attribution in audit/current-state table
        |
        v
Publish domain event + return DTO
        |
        v
TanStack Query invalidates affected queries
        |
        v
Project detail, inventory history, dashboard, and future reporting read consistent attributed data
```

### Recommended Project Structure
```text
src/
├── modules/
│   ├── inventory/
│   │   ├── domain/            # Withdrawal rule object / validation helper
│   │   ├── application/       # Server-side enforcement before repository write
│   │   ├── infrastructure/    # Atomic stock write + attributed stock_entries row
│   │   └── api/               # Request DTO and handler parsing
│   └── sites/
│       ├── domain/            # Productive-time attribution rule
│       ├── application/       # Time-entry create/update enforcement
│       ├── infrastructure/    # time_entries persistence
│       └── api/               # Request DTO and handler parsing
frontend/src/
├── pages/inventory/           # Withdraw dialog + inventory detail defaults
├── pages/sites/               # Time booking dialog + project-specific entry points
├── lib/api/hooks/             # React Query mutation invalidation
└── test/mocks/                # MSW payload assertions
```

### Pattern 1: Server-Enforced Attribution, Client-Assisted Defaulting
**What:** The browser should prefill the project from `active_site_id`, but the backend must reject non-exempt productive captures without a project. [VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.tsx][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: src/modules/sites/domain/time_entry.rs]

**When to use:** Use on every create/update path where a user can record real consumption or productive time. [VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/REQUIREMENTS.md]

**Example:**
```rust
// Source: src/modules/inventory/application/inventory_service.rs + src/modules/sites/application/site_service.rs [VERIFIED: codebase grep]
if is_real_material_consumption && create.site_id.is_none() {
    return Err(AppError::Validation("Project link is required".to_string()));
}

if is_productive_time_entry(create.work_type) && create.site_id.is_none() {
    return Err(AppError::Validation("Project link is required for productive work".to_string()));
}
```

### Pattern 2: Keep Persistence Atomic at the Repository Boundary
**What:** Material quantity change and attributed stock-history insertion should remain inside one SQLx transaction. [VERIFIED: src/modules/inventory/infrastructure/material_repository.rs][CITED: https://context7.com/launchbadge/sqlx/llms.txt]

**When to use:** Use for withdrawals/disposal/correction paths so quantity and attribution cannot diverge. [VERIFIED: src/modules/inventory/infrastructure/material_repository.rs]

**Example:**
```rust
// Source: https://context7.com/launchbadge/sqlx/llms.txt
let mut tx = pool.begin().await?;
sqlx::query("INSERT INTO stock_entries (...) VALUES (...)")
    .execute(&mut *tx)
    .await?;
tx.commit().await?;
```

### Pattern 3: Invalidate Read Models After Capture Mutations
**What:** Keep mutation hooks responsible for invalidating all affected read surfaces. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][CITED: https://github.com/tanstack/query/blob/main/docs/framework/react/guides/invalidations-from-mutations.md]

**When to use:** After material withdrawals, time entry create/update/delete, and any mutation that changes dashboard totals or project detail state. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: src/modules/sites/infrastructure/site_repository.rs]

**Example:**
```tsx
// Source: https://github.com/tanstack/query/blob/main/docs/framework/react/guides/invalidations-from-mutations.md
const mutation = useMutation({
  mutationFn: saveCapture,
  onSuccess: async () => {
    await Promise.all([
      queryClient.invalidateQueries({ queryKey: ['time-entries'] }),
      queryClient.invalidateQueries({ queryKey: ['dashboard-sites'] }),
    ])
  },
})
```

### Anti-Patterns to Avoid
- **UI-only enforcement:** A required select in the dialog is insufficient because the current API contracts still accept `null` `site_id` values. [VERIFIED: frontend/src/pages/inventory/WithdrawDialog.tsx][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: src/modules/sites/api/routes.rs]
- **Using `work_type === "workshop"` to force `site_id = null`:** That directly contradicts `PROJ-14`, which explicitly requires project-linked workshop preparation time. [VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: .planning/REQUIREMENTS.md]
- **Treating stock correction as consumption:** Corrections already have a separate admin-only `adjust_stock` command and should stay outside the attribution rule. [VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: src/modules/inventory/application/inventory_service.rs]
- **Adding a new reporting table now:** Reporting depends on disciplined writes, not a second source of truth. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/011_user_preferences.sql]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Project memory for common capture flows | A second frontend “active project” store | Existing `user_preferences.active_site_id` read through `/api/v1/preferences` | The preference is already validated server-side and reused across projects/fleet/time flows. [VERIFIED: src/modules/iam/application/user_preferences_service.rs][VERIFIED: frontend/src/pages/sites/SitesListPage.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx] |
| Attributed material audit | A new material-consumption table | Existing `stock_entries.site_id` and `entry_type` | The reporting link is already persisted with the stock movement. [VERIFIED: migrations/011_user_preferences.sql][VERIFIED: migrations/014_entry_type_stock_entries.sql][VERIFIED: src/modules/inventory/infrastructure/material_repository.rs] |
| Attributed time audit | A shadow project-labor table | Existing `time_entries.site_id` | Dashboard totals already aggregate from `time_entries` by `site_id`. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: src/modules/sites/infrastructure/site_repository.rs] |
| Cache refresh choreography | Manual ad hoc refetch state flags | TanStack Query invalidation in mutation hooks | The project already uses this pattern; it keeps read models consistent without bespoke glue. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][CITED: https://github.com/tanstack/query/blob/main/docs/framework/react/guides/invalidations-from-mutations.md] |
| JSON parsing and request-body plumbing | Custom body parsing | Axum `Json` extractors + typed DTOs | This is the framework-standard pattern already used in the repo. [VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: src/modules/sites/api/routes.rs][CITED: https://github.com/tokio-rs/axum/blob/main/axum/src/docs/extract.md] |

**Key insight:** Phase 46 is mostly a data-discipline phase, not a capability-discovery phase; the necessary storage hooks and UX defaulting primitives already exist. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/011_user_preferences.sql][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.tsx]

## Common Pitfalls

### Pitfall 1: Enforcing the rule only in the dialog
**What goes wrong:** Alternate clients, future batch tools, or even existing API calls can still submit `null` `site_id` and create unattributed productive records. [VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: src/modules/sites/api/routes.rs]

**Why it happens:** Both request DTOs currently allow optional `site_id`, and the domain validators only check quantity/date/hours. [VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: src/modules/sites/domain/time_entry.rs]

**How to avoid:** Add explicit service/domain rules keyed off exemption type (`disposal`, `adjust_stock`, overhead/admin work) and keep the UI as a convenience layer only. [VERIFIED: src/modules/inventory/application/inventory_service.rs][VERIFIED: src/modules/sites/application/site_service.rs]

**Warning signs:** Successful API writes still appear in history or `time_entries` with `site_id = null` for normal work. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/011_user_preferences.sql]

### Pitfall 2: Leaving workshop productivity outside the project model
**What goes wrong:** Workshop preparation/CNC time remains invisible in project cost and duration reporting even after the unified project model shipped. [VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: .planning/ROADMAP.md]

**Why it happens:** The current dialog forcibly nulls `site_id` unless `work_type === "site"`. [VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx]

**How to avoid:** Treat `workshop` as a productive project-linked work type and use internal workshop projects instead of `null` linkage. [VERIFIED: .planning/requirement-reviews/projects/PROJ-14.md][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.test.tsx]

**Warning signs:** Internal workshop projects exist in `sites`, but newly created workshop time rows still have `site_id = null`. [VERIFIED: frontend/src/pages/sites/TimeEntryDialog.test.tsx][VERIFIED: migrations/005_sites_schema.sql]

### Pitfall 3: Forgetting edit-path consistency
**What goes wrong:** New creates comply, but edited records can still drift out of compliance or remain impossible to clear correctly for true overhead/admin cases. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs]

**Why it happens:** `UpdateTimeEntry` uses `Option<Option<SiteId>>`, but the current SQL update uses `COALESCE($1, site_id)`, which cannot intentionally clear `site_id` to `NULL`. [VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs]

**How to avoid:** Give update paths the same explicit rule matrix as create paths and use a `CASE`-style SQL update for `site_id`, just as notes already do. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs]

**Warning signs:** Editing a time entry from project-linked to overhead appears to succeed in UI but the persisted `site_id` never changes. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs]

### Pitfall 4: Downstream analytics expecting project-aware events that do not exist yet
**What goes wrong:** Future billing/reporting consumers read domain events and discover the stock/time payloads do not always carry enough project attribution. [VERIFIED: src/modules/inventory/domain/events.rs][VERIFIED: src/modules/sites/domain/events.rs]

**Why it happens:** `StockWithdrawnPayload` lacks `site_id`, and `TimeEntryCreatedPayload` uses a tenant-level aggregate id with optional `site_id`. [VERIFIED: src/modules/inventory/domain/events.rs][VERIFIED: src/modules/sites/domain/events.rs]

**How to avoid:** Decide during planning whether Phase 46 consumers will read tables directly or whether event payloads must be enriched now. [VERIFIED: src/modules/inventory/domain/events.rs][VERIFIED: src/modules/sites/domain/events.rs][ASSUMED]

**Warning signs:** A later reporting phase needs extra joins or cannot distinguish project-linked vs unattributed events from the event stream alone. [VERIFIED: src/modules/inventory/domain/events.rs][VERIFIED: src/modules/sites/domain/events.rs]

## Code Examples

Verified patterns from official sources:

### Axum typed JSON request extraction
```rust
// Source: https://github.com/tokio-rs/axum/blob/main/axum/src/docs/extract.md
use axum::extract::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateCapture {
    site_id: Option<String>,
    notes: Option<String>,
}

async fn create_capture(Json(payload): Json<CreateCapture>) {
    // validate + delegate to service
}
```

### SQLx transaction boundary for atomic capture writes
```rust
// Source: https://context7.com/launchbadge/sqlx/llms.txt
let mut tx = pool.begin().await?;

sqlx::query("UPDATE materials SET quantity = $1 WHERE id = $2")
    .bind(new_quantity)
    .bind(material_id)
    .execute(&mut *tx)
    .await?;

sqlx::query("INSERT INTO stock_entries (...) VALUES (...)")
    .execute(&mut *tx)
    .await?;

tx.commit().await?;
```

### TanStack Query invalidation after mutation success
```tsx
// Source: https://github.com/tanstack/query/blob/main/docs/framework/react/guides/invalidations-from-mutations.md
const queryClient = useQueryClient()

const mutation = useMutation({
  mutationFn: saveTimeEntry,
  onSuccess: async () => {
    await Promise.all([
      queryClient.invalidateQueries({ queryKey: ['time-entries'] }),
      queryClient.invalidateQueries({ queryKey: ['my-time-entries'] }),
      queryClient.invalidateQueries({ queryKey: ['dashboard-sites'] }),
    ])
  },
})
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Optional Baustelle linkage on withdrawals and nullable workshop time | Unified project model plus active-project defaults, with Phase 46 still needing enforcement hardening | Unified project model shipped in Phase 44; timeline context shipped in Phase 45; enforcement is pending in Phase 46. [VERIFIED: .planning/ROADMAP.md] | The product direction is now “capture once, attribute by default,” so optional linkage is the last major mismatch. [VERIFIED: .planning/PROJECT.md][VERIFIED: .planning/ROADMAP.md] |
| Separate “Baustelle vs workshop” mental model | One `sites` aggregate now represents both external and internal work through `project_type` | Phase 44. [VERIFIED: .planning/ROADMAP.md][VERIFIED: frontend/src/types/sites.ts] | Workshop preparation can now be project-linked without inventing a second entity. [VERIFIED: .planning/requirement-reviews/projects/PROJ-14.md][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.test.tsx] |
| Project choice re-entered across flows | `active_site_id` preference reused across sites, inventory, and fleet UIs | Active project context groundwork shipped before Phase 46 and is observable in current screens. [VERIFIED: .planning/ROADMAP.md][VERIFIED: frontend/src/pages/sites/SitesListPage.tsx][VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx] | Phase 46 can reduce friction without extra state architecture. [VERIFIED: frontend/src/pages/sites/SitesListPage.tsx][VERIFIED: frontend/src/pages/inventory/InventoryDetailPage.tsx] |

**Deprecated/outdated:**
- Treating “workshop” as synonymous with “no project” is now outdated because Phase 44 introduced internal workshop projects explicitly to avoid null linkage for internal productive work. [VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/requirement-reviews/projects/PROJ-13.md][VERIFIED: .planning/requirement-reviews/projects/PROJ-14.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | A new form library is unnecessary for this narrow phase. [ASSUMED] | Standard Stack → Alternatives Considered | Low — scope may expand slightly later, but Phase 46 can still ship without it. |
| A2 | Phase 46 can defer event-payload enrichment if near-term reporting reads tables directly instead of the event stream. [ASSUMED] | Common Pitfalls → Pitfall 4 | Medium — later analytics work may need a follow-up migration or event versioning. |
| A3 | `travel` and `other` can remain nullable in Phase 46 if the milestone only enforces project linkage for on-site and workshop productivity. [ASSUMED] | Open Questions | Medium — cost attribution could stay incomplete for project-related travel. |
| A4 | Historical unattributed rows may be left unchanged if the business accepts forward-only attribution from Phase 46 onward. [ASSUMED] | Open Questions | High — back-office reporting may still have legacy gaps. |

## Open Questions

1. **Should `travel` also require project linkage?**
   - What we know: `PROJ-14` explicitly names on-site work and workshop preparation, and the current enum also includes `travel` and `other`. [VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: src/common/types.rs]
   - What's unclear: Whether project-related travel is considered productive/billable in this tenant’s operating model. [ASSUMED]
   - Recommendation: Plan Phase 46 around `site` + `workshop` as the hard requirement and log travel attribution as a milestone-level decision if the product owner wants tighter cost tracking now. [ASSUMED]

2. **Do historical `NULL site_id` records need cleanup?**
   - What we know: Both `time_entries.site_id` and `stock_entries.site_id` have historically allowed `NULL`, and prior flows created nullable records. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/011_user_preferences.sql][VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: src/modules/sites/domain/time_entry.rs]
   - What's unclear: Whether Phase 46 reporting starts from new data only or must rehabilitate older unattributed rows. [ASSUMED]
   - Recommendation: Decide during planning whether to include a one-off backfill/admin cleanup slice or explicitly scope enforcement to new writes only. [ASSUMED]

3. **Should event payloads become project-aware now?**
   - What we know: Stock and time creation already emit domain events, but the payloads are not fully optimized for future project-cost consumers. [VERIFIED: src/modules/inventory/domain/events.rs][VERIFIED: src/modules/sites/domain/events.rs]
   - What's unclear: Whether downstream reporting phases will consume tables, events, or both. [ASSUMED]
   - Recommendation: If Phase 46 is supposed to be the stable data contract for later billing/reporting, add `site_id` to event payloads now; otherwise defer cleanly and document the dependency. [ASSUMED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | Frontend dialog/test work | ✓ [VERIFIED: local shell] | `v24.11.1` [VERIFIED: local shell] | — |
| npm | Package scripts, Vitest, Vite | ✓ [VERIFIED: local shell] | `11.6.2` [VERIFIED: local shell] | — |
| Cargo | Backend compile/tests/ts-rs generation | ✓ [VERIFIED: local shell] | `1.95.0` [VERIFIED: local shell] | — |
| rustc | Backend build | ✓ [VERIFIED: local shell] | `1.95.0` [VERIFIED: local shell] | — |
| `psql` client | Local DB verification and data inspection | ✓ [VERIFIED: local shell] | `18.3` [VERIFIED: local shell] | — |
| Docker | Dedicated per-worktree PostgreSQL container per project rules | ✓ [VERIFIED: local shell] | `29.4.2` [VERIFIED: local shell] | No fallback; AGENTS requires isolated local DBs. [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:**
- None detected at the tooling level. [VERIFIED: local shell]

**Missing dependencies with fallback:**
- None. [VERIFIED: local shell]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Existing Keycloak auth remains upstream of this phase; Phase 46 should not replace it. [VERIFIED: AGENTS.md] |
| V3 Session Management | no | Existing OAuth2 PKCE / auth session behavior is out of scope here. [VERIFIED: .planning/PROJECT.md] |
| V4 Access Control | yes | Validate submitted `site_id` against tenant-scoped project lookup before create/update. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/iam/application/user_preferences_service.rs] |
| V5 Input Validation | yes | Validate quantities, hours, dates, enum parsing, and new attribution rules in Rust service/domain logic. [VERIFIED: src/modules/inventory/domain/material.rs][VERIFIED: src/modules/sites/domain/time_entry.rs][VERIFIED: src/common/types.rs] |
| V6 Cryptography | no | No new cryptographic behavior is introduced in this phase. [VERIFIED: phase scope] |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Cross-tenant project ID submission on withdrawal/time booking | Elevation of privilege / Tampering | Reuse tenant-scoped project lookup (`get_site` / `find_site_by_id`) before persistence. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/iam/application/user_preferences_service.rs] |
| Intentional omission of `site_id` to hide billable/productive work | Repudiation / Tampering | Enforce backend attribution rules for non-exempt writes, not just UI form requirements. [VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: src/modules/sites/api/routes.rs] |
| Invalid enum or malformed date input in time booking | Tampering | Continue parsing `work_type` through `FromStr` and `work_date` through `NaiveDate::parse_from_str`. [VERIFIED: src/common/types.rs][VERIFIED: src/modules/sites/api/routes.rs] |
| Race conditions during stock withdrawal | Tampering | Keep quantity change and history insert inside the existing SQLx transaction with `FOR UPDATE`. [VERIFIED: src/modules/inventory/infrastructure/material_repository.rs][CITED: https://context7.com/launchbadge/sqlx/llms.txt] |

## Sources

### Primary (HIGH confidence)
- `Cargo.toml` — backend stack versions and pinned dependencies. [VERIFIED: Cargo.toml]
- `frontend/package.json` — frontend stack versions and test tooling. [VERIFIED: frontend/package.json]
- `migrations/005_sites_schema.sql` — existing `time_entries.site_id` schema and indexes. [VERIFIED: migrations/005_sites_schema.sql]
- `migrations/011_user_preferences.sql` — `user_preferences` and `stock_entries.site_id`. [VERIFIED: migrations/011_user_preferences.sql]
- `src/modules/inventory/*` — withdrawal API/service/repository behavior and stock event payloads. [VERIFIED: codebase grep]
- `src/modules/sites/*` — time-entry API/service/repository behavior, dashboard aggregation, and time event payloads. [VERIFIED: codebase grep]
- `src/modules/iam/*` — active project preference validation behavior. [VERIFIED: codebase grep]
- `frontend/src/pages/inventory/InventoryDetailPage.tsx` and `WithdrawDialog.tsx` — current withdrawal defaulting and optional project UX. [VERIFIED: codebase read]
- `frontend/src/pages/sites/TimeEntryDialog.tsx` — current time-booking defaulting and `workshop => null site_id` behavior. [VERIFIED: codebase read]
- `https://github.com/tokio-rs/axum/blob/main/axum/src/docs/extract.md` — official Axum extractor patterns. [CITED: https://github.com/tokio-rs/axum/blob/main/axum/src/docs/extract.md]
- `https://github.com/launchbadge/sqlx/blob/main/README.md` and Context7 SQLx docs — official transaction/query patterns. [CITED: https://github.com/launchbadge/sqlx/blob/main/README.md][CITED: https://context7.com/launchbadge/sqlx/llms.txt]
- `https://github.com/tanstack/query/blob/main/docs/framework/react/guides/invalidations-from-mutations.md` — official mutation invalidation pattern. [CITED: https://github.com/tanstack/query/blob/main/docs/framework/react/guides/invalidations-from-mutations.md]
- npm registry lookups for `react`, `@tanstack/react-query`, `vite`, `vitest`, and `tailwindcss`. [VERIFIED: npm registry]
- crates.io API lookups for `axum`, `sqlx`, and `ts-rs`. [VERIFIED: crates.io API]

### Secondary (MEDIUM confidence)
- None. All non-assumed technical claims above were verified against the codebase, registry metadata, or official docs. [VERIFIED: research log]

### Tertiary (LOW confidence)
- None beyond the explicit assumptions captured in the Assumptions Log. [VERIFIED: research log]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Existing dependencies are directly observable in the repo and current versions were checked against npm/crates registries. [VERIFIED: Cargo.toml][VERIFIED: frontend/package.json][VERIFIED: npm registry][VERIFIED: crates.io API]
- Architecture: HIGH - The needed persistence columns, request flows, and preference-default patterns are already implemented in the codebase. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/011_user_preferences.sql][VERIFIED: codebase read]
- Pitfalls: HIGH - The biggest risks are visible in current optional DTOs, null-for-workshop UI logic, and update-path SQL. [VERIFIED: src/modules/inventory/api/routes.rs][VERIFIED: frontend/src/pages/sites/TimeEntryDialog.tsx][VERIFIED: src/modules/sites/infrastructure/site_repository.rs]

**Research date:** 2026-05-07  
**Valid until:** 2026-06-06
