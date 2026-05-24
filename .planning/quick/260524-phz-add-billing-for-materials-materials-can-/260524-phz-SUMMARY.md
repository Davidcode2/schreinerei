---
status: complete
quick_id: 260524-phz
date: 2026-05-24
code_commit: b049fe5
---

# Quick task 260524-phz Summary

## Outcome

- Added material billing defaults to inventory with a nullable `base_price_cents` and `price_markup_percentage`.
- Extended material create and edit flows so admins can manage those billing defaults directly in the existing inventory dialogs.
- Priced invoice material lines from inventory defaults and persisted the resolved line pricing into generated invoices.
- Added per-invoice material markup overrides in the invoice creation dialog so admins can adjust customer markup per material without mutating inventory defaults.

## Key Decisions

- Material pricing stays inventory-owned: base price plus default markup live on each material record.
- Per-invoice flexibility only overrides the markup percentage, not the inventory base price.
- Materials without a base price remain visible on invoices but unpriced, so missing pricing data is explicit instead of silently guessed.

## Verification

- `cargo test --lib inventory::domain::material`
- `cargo test --lib inventory::api::routes`
- `cargo test --lib billing::api::routes`
- `cargo fmt --check`
- `cargo export-types`
- `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`
- `npm --prefix frontend run test:run -- src/pages/inventory/AddMaterialDialog.test.tsx src/pages/inventory/MaterialEditDialog.test.tsx src/lib/api/hooks/useInventory.test.tsx src/lib/api/hooks/useSites.test.tsx src/pages/sites/SiteDetailPage.test.tsx`
- `npm --prefix frontend run test:run -- src/pages/inventory/InventoryDetailPage.test.tsx`
- `npm run build` in `frontend/`

All commands passed.
