---
phase: quick-260524-phz
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - migrations/035_material_billing_defaults.sql
  - src/modules/inventory/domain/material.rs
  - src/modules/inventory/infrastructure/material_repository.rs
  - src/modules/inventory/api/routes.rs
  - src/modules/billing/domain/invoice.rs
  - src/modules/billing/api/routes.rs
  - src/modules/sites/api/routes.rs
  - src/bin/export-types.rs
  - frontend/src/types/generated.ts
  - frontend/src/types/inventory.ts
  - frontend/src/types/sites.ts
  - frontend/src/lib/api/hooks/useInventory.ts
  - frontend/src/pages/inventory/AddMaterialDialog.tsx
  - frontend/src/pages/inventory/MaterialEditDialog.tsx
  - frontend/src/pages/inventory/AddMaterialDialog.test.tsx
  - frontend/src/pages/inventory/MaterialEditDialog.test.tsx
  - frontend/src/pages/sites/CreateInvoiceDialog.tsx
  - frontend/src/pages/sites/SiteDetailPage.test.tsx
  - frontend/src/test/mocks/handlers.ts
autonomous: true
requirements:
  - FIN-10
---

<objective>
Add material billing defaults to inventory and use them in project invoice creation so material line items can be priced by default while still allowing a one-off per-project invoice override.
</objective>

<context>
@.planning/STATE.md
@.planning/quick/260524-lhg-complete-project-billing-ui-and-per-invo/260524-lhg-SUMMARY.md
@src/modules/inventory/domain/material.rs
@src/modules/inventory/infrastructure/material_repository.rs
@src/modules/inventory/api/routes.rs
@src/modules/billing/api/routes.rs
@src/modules/billing/domain/invoice.rs
@src/modules/sites/api/routes.rs
@frontend/src/pages/inventory/AddMaterialDialog.tsx
@frontend/src/pages/inventory/MaterialEditDialog.tsx
@frontend/src/pages/sites/CreateInvoiceDialog.tsx
</context>

<tasks>
<task type="auto">
  <name>task 1: add material billing defaults to inventory persistence and API</name>
  <files>migrations/035_material_billing_defaults.sql, src/modules/inventory/domain/material.rs, src/modules/inventory/infrastructure/material_repository.rs, src/modules/inventory/api/routes.rs, src/bin/export-types.rs, frontend/src/types/generated.ts, frontend/src/types/inventory.ts</files>
  <action>Add a base price in cents plus a default markup percentage to materials, validate both as non-negative, persist them via a new migration, expose them through create/update/list/detail DTOs, and regenerate frontend types.</action>
  <verify>cargo test --lib inventory::domain::material inventory::api::routes && cargo export-types</verify>
  <done>Materials carry tenant-scoped billing defaults end-to-end across DB, domain, API, and generated types.</done>
</task>

<task type="auto">
  <name>task 2: expose billing defaults in inventory create and edit UI</name>
  <files>frontend/src/lib/api/hooks/useInventory.ts, frontend/src/pages/inventory/AddMaterialDialog.tsx, frontend/src/pages/inventory/MaterialEditDialog.tsx, frontend/src/pages/inventory/AddMaterialDialog.test.tsx, frontend/src/pages/inventory/MaterialEditDialog.test.tsx, frontend/src/test/mocks/handlers.ts</files>
  <action>Add form controls for base price and default customer markup percentage in the existing inventory dialogs, submit them through the existing hooks, and cover both create and edit flows with tests.</action>
  <verify>npm --prefix frontend run test:run -- src/pages/inventory/AddMaterialDialog.test.tsx src/pages/inventory/MaterialEditDialog.test.tsx src/lib/api/hooks/useInventory.test.tsx</verify>
  <done>Admins can create and edit material billing defaults from the inventory UI without a separate billing screen.</done>
</task>

<task type="auto">
  <name>task 3: price invoice material lines and allow per-invoice markup overrides</name>
  <files>src/modules/billing/domain/invoice.rs, src/modules/billing/api/routes.rs, src/modules/sites/api/routes.rs, frontend/src/types/generated.ts, frontend/src/types/sites.ts, frontend/src/pages/sites/CreateInvoiceDialog.tsx, frontend/src/pages/sites/SiteDetailPage.test.tsx, frontend/src/test/mocks/handlers.ts</files>
  <action>Extend project material summary and invoice line items with pricing metadata, compute material line totals from base price plus markup, preserve the resolved values in the invoice snapshot, and let the invoice dialog override the markup percentage per material before invoice creation.</action>
  <verify>cargo test --lib billing::api::routes && npm --prefix frontend run test:run -- src/pages/sites/SiteDetailPage.test.tsx</verify>
  <done>Invoice drafts and PDFs show priced material lines by default, and admins can adjust the markup per material for a single invoice without mutating inventory defaults.</done>
</task>
</tasks>

<output>
After completion, create `.planning/quick/260524-phz-add-billing-for-materials-materials-can-/260524-phz-SUMMARY.md`
</output>
