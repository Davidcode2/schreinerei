# Phase 54 Plan

## Goal

Add one-click PDF invoice creation from the project/site detail page by reusing the existing invoice-ready project summary data and keeping invoice creation free of mandatory extra form entry.

## Bead

- `sc-t1n7v` - Add one-click PDF invoice creation from site detail

## Requirements

- `PROJ-17`
- `FIN-10`

## Current State

- `GET /api/v1/sites/{id}/invoice-summary` already composes project metadata, billing metadata, labor actuals, and material actuals.
- `SiteDetailPage` already has an admin-only `Projektübersicht exportieren` JSON action wired through `useSiteInvoiceSummary`.
- Billing data currently lives on `sites`: `budget_amount_cents`, `billing_reference`, `billing_notes`, and `quote_reference`.
- There is no invoice persistence table, PDF generator dependency, PDF endpoint, or frontend invoice download action yet.
- `ApiClient.getBlob` already supports authenticated binary downloads and should be reused.

## Plan

1. Add a small backend invoice PDF generation component under the existing `sites` bounded context.
2. Reuse `SiteService::get_invoice_summary` as the single data source for invoice content.
3. Add `GET /api/v1/sites/{id}/invoice.pdf` returning `application/pdf` with a download-friendly `Content-Disposition` filename.
4. Add optional request support for a one-off amount override, for example `?amount_cents=...`, with validation rejecting negative or malformed values.
5. Render the PDF with practical invoice sections: project/customer header, billing references, date range/status, optional total amount, labor summary, material usage summary, and billing notes.
6. Replace or rename the admin JSON export action on `SiteDetailPage` to a one-click PDF invoice button that downloads the blob through `apiClient.getBlob`.
7. Keep the JSON invoice-summary endpoint in place as the stable data/debug contract; do not remove Phase 50 behavior unless a product decision explicitly says the JSON export button should disappear.
8. Add focused backend and frontend tests for the PDF path and site detail action.

## Design Notes

- Keep this in `sites` for the slice because all available invoice data is already site/project data; do not introduce a broad finance context only for one generated PDF.
- Prefer `printpdf` or another Rust-native PDF crate over browser/headless rendering so the backend endpoint remains deployable in the current Rust service container.
- Do not add invoice numbers, tax/VAT logic, immutable invoice snapshots, payment status, DATEV export, or accounting integrations in this phase.
- Do not require a modal or form before generation. If amount override is implemented in the first slice, keep it optional and lightweight, such as a small admin-only dialog or an inline optional amount field near the button.
- If no amount is provided and `budget_amount_cents` is missing, still generate the PDF using the available project/labor/material summary and show the amount as not specified.
- Keep all backend reads tenant-scoped through the existing `TenantContext` and `SiteService` path.

## Proposed Files

- `Cargo.toml` and `Cargo.lock` - add the chosen PDF generation crate.
- `src/modules/sites/application/site_service.rs` - add a `generate_invoice_pdf` service method that calls `get_invoice_summary`.
- `src/modules/sites/application/invoice_pdf.rs` or `src/modules/sites/invoice_pdf.rs` - isolate PDF rendering and unit-testable formatting helpers.
- `src/modules/sites/api/routes.rs` - add the PDF route, query DTO, headers, and handler test coverage.
- `frontend/src/lib/api/hooks/useSites.ts` - add a download helper or mutation for invoice PDF blob fetching.
- `frontend/src/pages/sites/SiteDetailPage.tsx` - add the admin-facing invoice PDF button and optional override UI if included.
- `frontend/src/pages/sites/SiteDetailPage.test.tsx` - verify the button requests the PDF endpoint and downloads a blob.
- `frontend/src/test/mocks/handlers.ts` - add a default invoice PDF mock if needed by existing tests.

## Acceptance Criteria

- An admin can click one button on a site detail page and receive a PDF invoice download.
- Invoice generation works without any required extra fields beyond data already stored on the site/project.
- A missing budget amount does not block PDF creation.
- Optional amount override, if used, affects only the generated PDF response and does not mutate the project.
- The backend response has `Content-Type: application/pdf` and a deterministic project-based filename.
- Invalid site IDs and missing sites keep existing validation/not-found behavior.
- Cross-tenant access remains impossible because the route uses the existing tenant-scoped site lookup.

## Verification

- Unit tests for invoice PDF formatting and amount override validation.
- Backend route test asserting PDF MIME type, non-empty bytes, and tenant-scoped not-found behavior.
- Existing invoice-summary tests stay green.
- Focused `SiteDetailPage` test for the admin PDF download action.
- `cargo export-types` only if new exported DTOs are introduced.
- `cargo fmt --check`.
- Relevant `cargo test` for sites/application/API tests.
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`.
- Frontend `npx tsc --noEmit` and the focused Vitest file for `SiteDetailPage`.

## Risks

- PDF crate APIs can produce brittle tests if assertions inspect exact bytes. Tests should assert headers, non-empty PDF bytes, and stable helper output rather than full binary equality.
- Real invoices may eventually need legal/tax requirements that this slice does not model. This phase should label the output as a pragmatic project invoice PDF, not a complete accounting subsystem.
- Material actuals currently summarize quantities, not monetary cost, so the PDF must avoid implying material pricing unless pricing data exists later.
