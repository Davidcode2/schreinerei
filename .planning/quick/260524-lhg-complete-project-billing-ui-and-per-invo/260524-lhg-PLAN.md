---
phase: quick-260524-lhg
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - frontend/src/lib/api/hooks/useIam.ts
  - frontend/src/pages/settings/SettingsPage.tsx
  - frontend/src/pages/settings/BillingSettingsSection.tsx
  - frontend/src/pages/settings/SettingsPage.test.tsx
  - frontend/src/types/sites.ts
  - frontend/src/lib/api/hooks/useSites.ts
  - frontend/src/pages/sites/AddSiteDialog.tsx
  - frontend/src/pages/sites/AddSiteDialog.test.tsx
  - frontend/src/pages/sites/ProjectPlanningSheet.tsx
  - frontend/src/pages/sites/ProjectPlanningSheet.test.tsx
  - frontend/src/pages/sites/CreateInvoiceDialog.tsx
  - frontend/src/pages/sites/SiteDetailPage.tsx
  - frontend/src/pages/sites/SiteDetailPage.test.tsx
  - frontend/src/test/mocks/handlers.ts
  - src/modules/billing/api/routes.rs
  - frontend/src/types/generated.ts
autonomous: true
requirements:
  - PROJ-17
  - FIN-10
must_haves:
  truths:
    - "Admins can set a tenant default hourly rate and then create or edit a project with explicit billing mode/rate fields instead of backend-only behavior."
    - "A project detail page shows the saved billing setup and lets an admin create an invoice with a one-off pricing override that does not mutate the project."
    - "Invoice draft creation, returned billing data, and PDF generation all use the same resolved per-invoice pricing values."
  artifacts:
    - path: "frontend/src/pages/sites/AddSiteDialog.tsx"
      provides: "Project creation UI with billing mode and hourly/fixed price inputs"
    - path: "frontend/src/pages/sites/ProjectPlanningSheet.tsx"
      provides: "Project edit UI for persistent billing defaults"
    - path: "frontend/src/pages/sites/CreateInvoiceDialog.tsx"
      provides: "Admin-only per-invoice override flow before invoice creation"
    - path: "src/modules/billing/api/routes.rs"
      provides: "Per-invoice pricing override request handling without persisting project changes"
  key_links:
    - from: "frontend/src/pages/settings/BillingSettingsSection.tsx"
      to: "frontend/src/pages/sites/AddSiteDialog.tsx"
      via: "default hourly rate informs project billing form defaults"
      pattern: "default_hourly_rate_cents|hourly_rate_cents"
    - from: "frontend/src/pages/sites/CreateInvoiceDialog.tsx"
      to: "src/modules/billing/api/routes.rs"
      via: "invoice create payload carries sender and pricing override fields"
      pattern: "invoice_pricing_mode|hourly_rate_cents|fixed_price_cents"
    - from: "src/modules/billing/api/routes.rs"
      to: "frontend/src/pages/sites/SiteDetailPage.tsx"
      via: "draft response invalidation/refetch keeps invoice list and pricing feedback in sync"
      pattern: "ProjectInvoiceDraftResponse|useCreateSiteInvoice"
---

<objective>
Complete the billing user flow from tenant default hourly rate through project billing defaults to admin invoice creation with optional one-off pricing overrides.

Purpose: remove the current gap where pricing semantics exist in the backend but the user flow is still split across hidden defaults, incomplete project forms, and invoice creation without override controls.
Output: one coherent admin flow covering settings, project create/edit/detail, and invoice creation override behavior.
</objective>

<execution_context>
@$HOME/.config/opencode/get-shit-done/workflows/execute-plan.md
@$HOME/.config/opencode/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/STATE.md
@.planning/quick/260524-k4d-add-project-hourly-rate-billing-defaults/260524-k4d-SUMMARY.md
@frontend/src/pages/settings/SettingsPage.tsx
@frontend/src/pages/sites/AddSiteDialog.tsx
@frontend/src/pages/sites/ProjectPlanningSheet.tsx
@frontend/src/pages/sites/SiteDetailPage.tsx
@frontend/src/lib/api/hooks/useSites.ts
@frontend/src/types/sites.ts
@src/modules/billing/api/routes.rs
@src/modules/iam/api/routes.rs

<interfaces>
From frontend/src/types/sites.ts:
```ts
export interface Site {
  budget_amount_cents: number | null
  billing_reference: string | null
  billing_notes: string | null
  quote_reference: string | null
}

export interface CreateSiteRequest {
  budget_amount_cents?: number | null
  billing_reference?: string | null
  billing_notes?: string | null
  quote_reference?: string | null
}

export interface UpdateSiteRequest {
  budget_amount_cents?: number | null
  clear_budget_amount?: boolean
  clear_billing_reference?: boolean
  clear_billing_notes?: boolean
  clear_quote_reference?: boolean
}
```

From src/modules/sites/api/routes.rs:
```rust
pub struct SiteResponse {
    pub invoice_pricing_mode: Option<String>,
    pub hourly_rate_cents: Option<i64>,
    pub fixed_price_cents: Option<i64>,
}

pub struct UpdateSiteRequest {
    pub invoice_pricing_mode: Option<String>,
    pub hourly_rate_cents: Option<i64>,
    pub fixed_price_cents: Option<i64>,
    pub clear_invoice_pricing_mode: Option<bool>,
    pub clear_hourly_rate_cents: Option<bool>,
    pub clear_fixed_price_cents: Option<bool>,
}
```

From src/modules/billing/api/routes.rs:
```rust
pub struct CreateProjectInvoiceRequest {
    pub sender_name: Option<String>,
    pub sender_address: Option<String>,
}

pub struct ProjectInvoiceDraftResponse {
    pub billing: SiteInvoiceBillingResponse,
    pub total_amount_cents: Option<i64>,
    pub line_items: Vec<ProjectInvoiceLineItemResponse>,
}
```
</interfaces>
</context>

<tasks>

<task type="auto" tdd="true">
  <name>task 1: finish the billing defaults UI across settings and project create/edit flows</name>
  <files>frontend/src/lib/api/hooks/useIam.ts, frontend/src/pages/settings/SettingsPage.tsx, frontend/src/pages/settings/BillingSettingsSection.tsx, frontend/src/pages/settings/SettingsPage.test.tsx, frontend/src/types/sites.ts, frontend/src/pages/sites/AddSiteDialog.tsx, frontend/src/pages/sites/AddSiteDialog.test.tsx, frontend/src/pages/sites/ProjectPlanningSheet.tsx, frontend/src/pages/sites/ProjectPlanningSheet.test.tsx, frontend/src/test/mocks/handlers.ts</files>
  <behavior>
    - Test 1: admins can view and update the tenant default hourly rate from settings, preserving the already-uncommitted settings work.
    - Test 2: project create and project planning forms expose invoice pricing mode plus hourly/fixed amount inputs and submit the expected site payload fields.
    - Test 3: clearing project billing inputs sends the explicit clear flags so project defaults can be removed instead of silently sticking.
  </behavior>
  <action>Finish the UI side of the billing-default slice that already has partial settings-page work in the working tree. Extend the frontend site contracts to include `invoice_pricing_mode`, `hourly_rate_cents`, `fixed_price_cents`, and matching clear flags; then add those fields to both `AddSiteDialog` and `ProjectPlanningSheet` with conditional form controls (`hourly_rate` shows hourly amount, `fixed_price` shows fixed amount, blank/none clears both). Reuse the tenant billing settings query as a sensible default/hint for new projects, but do not change the existing backend rule that project edits never rewrite the tenant default. Keep the forms coherent with the current budget/reference section instead of creating a separate billing screen.</action>
  <verify>
    <automated>npm --prefix frontend run test:run -- src/pages/settings/SettingsPage.test.tsx src/pages/sites/AddSiteDialog.test.tsx src/pages/sites/ProjectPlanningSheet.test.tsx && npm --prefix frontend exec tsc --noEmit</automated>
  </verify>
  <done>Admins can manage tenant defaults and persistent project billing defaults from the existing settings/create/edit surfaces, and the frontend submits the complete pricing-mode payload instead of budget-only metadata.</done>
</task>

<task type="auto" tdd="true">
  <name>task 2: add per-invoice pricing override support to the billing create contract</name>
  <files>src/modules/billing/api/routes.rs, frontend/src/lib/api/hooks/useSites.ts, frontend/src/types/generated.ts, frontend/src/test/mocks/handlers.ts</files>
  <behavior>
    - Test 1: invoice creation without overrides still uses the saved project billing defaults and current sender fields.
    - Test 2: invoice creation can override pricing mode/hourly rate/fixed price for a single invoice draft without persisting any project change.
    - Test 3: invalid override combinations or negative monetary values fail validation before invoice/PDF generation.
  </behavior>
  <action>Extend `CreateProjectInvoiceRequest` with optional one-off pricing fields mirroring the project billing defaults (`invoice_pricing_mode`, `hourly_rate_cents`, `fixed_price_cents`) and apply them only inside invoice draft assembly. Resolve effective billing data in one place before line items/PDF generation so the draft response, stored snapshot, and rendered PDF all use the same values. Keep the existing admin gate and tenant-scoped site lookup, regenerate `frontend/src/types/generated.ts` with `cargo export-types`, and update `useCreateSiteInvoice` so it accepts a typed payload instead of always posting `{}`. Do not write override values back to `sites` or tenant settings.</action>
  <verify>
    <automated>cargo test --lib billing::api::routes && cargo export-types && cargo fmt --check</automated>
  </verify>
  <done>Invoice creation accepts optional sender/pricing overrides, rejects invalid override payloads, and returns a draft whose billing metadata and totals match the effective one-off values.</done>
</task>

<task type="auto" tdd="true">
  <name>task 3: wire the site detail invoice dialog so admins can review and override billing before creation</name>
  <files>frontend/src/pages/sites/CreateInvoiceDialog.tsx, frontend/src/pages/sites/SiteDetailPage.tsx, frontend/src/pages/sites/SiteDetailPage.test.tsx, frontend/src/lib/api/hooks/useSites.ts, frontend/src/test/mocks/handlers.ts</files>
  <behavior>
    - Test 1: opening invoice creation on the site detail page shows project billing defaults prefilled in the dialog.
    - Test 2: submitting the dialog sends sender/pricing overrides to the billing endpoint and refreshes the invoice list on success.
    - Test 3: changing override values in the dialog does not alter the saved project billing panel until the user separately edits the project planning sheet.
  </behavior>
  <action>Create an admin-only `CreateInvoiceDialog` launched from `SiteDetailPage` instead of direct one-click creation. Prefill the dialog from the current `site` billing fields, show the resolved billing mode summary, allow optional sender fields plus one-off pricing overrides, and submit through the updated mutation from task 2. Keep the rest of the detail page intact: existing billing metadata remains the persisted project truth, the invoice list still refetches after creation, and PDF download behavior for existing invoices stays unchanged.</action>
  <verify>
    <automated>npm --prefix frontend run test:run -- src/pages/sites/SiteDetailPage.test.tsx && npm --prefix frontend exec tsc --noEmit</automated>
  </verify>
  <done>The site detail page gives admins a complete invoice-creation flow with reviewable pricing overrides and sender fields, while saved project billing defaults remain separate from one-off invoice decisions.</done>
</task>

</tasks>

<threat_model>
## Trust Boundaries

| Boundary | Description |
|----------|-------------|
| settings UI→tenant billing settings API | Admin-entered default hourly rates cross into tenant-scoped persistence |
| project forms→site API | Billing mode and monetary defaults cross from browser input into project persistence |
| invoice dialog→billing create API | One-off invoice overrides cross into draft/PDF generation without mutating the project |

## STRIDE Threat Register

| Threat ID | Category | Component | Disposition | Mitigation Plan |
|-----------|----------|-----------|-------------|-----------------|
| T-quick-lhg-01 | T | `src/modules/billing/api/routes.rs` | mitigate | Validate override pricing mode combinations and reject negative hourly/fixed values before building invoice line items. |
| T-quick-lhg-02 | I | `src/modules/billing/api/routes.rs` | mitigate | Keep invoice creation on the existing tenant-scoped summary lookup so overrides never bypass tenant isolation. |
| T-quick-lhg-03 | E | `frontend/src/pages/sites/SiteDetailPage.tsx` | mitigate | Preserve the existing admin-only invoice creation path; non-admin users must not see or submit override controls. |
| T-quick-lhg-04 | R | `frontend/src/pages/sites/CreateInvoiceDialog.tsx` | mitigate | Surface the effective billing mode and amount in the dialog and rely on the returned draft snapshot so the created invoice remains auditable. |
</threat_model>

<verification>
- `npm --prefix frontend run test:run -- src/pages/settings/SettingsPage.test.tsx src/pages/sites/AddSiteDialog.test.tsx src/pages/sites/ProjectPlanningSheet.test.tsx src/pages/sites/SiteDetailPage.test.tsx`
- `npm --prefix frontend exec tsc --noEmit`
- `cargo test --lib billing::api::routes`
- `cargo export-types`
- `cargo fmt --check`
</verification>

<success_criteria>
- The already-uncommitted billing settings work lands as part of the same shipped flow instead of staying isolated on `/settings`.
- Project creation, project editing, and project detail all expose the saved billing defaults needed for invoice generation.
- Admin invoice creation supports one-off sender/pricing overrides that affect only the created invoice draft/PDF, not the project record.
</success_criteria>

<output>
After completion, create `.planning/quick/260524-lhg-complete-project-billing-ui-and-per-invo/260524-lhg-SUMMARY.md`
</output>
