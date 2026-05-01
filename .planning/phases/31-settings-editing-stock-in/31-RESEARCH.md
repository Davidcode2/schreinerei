# Phase 31: Settings, Editing & Stock-In — Research

**Date:** 2026-05-01
**Status:** Ready for planning
**Phase:** 31
**Requirements:** CATS-01, CATS-02, CATS-03, EDIT-01, EDIT-02, EDIT-03, STOCK-01, VIEW-01

## Goal

Expose the Phase 30 inventory edit APIs through existing React inventory screens so users can:
- manage categories from a dedicated inventory settings route,
- edit material location and minimum stock,
- correct stock to an arbitrary quantity,
- record stock-in with amount and notes,
- see category names directly on the inventory overview.

## What Already Exists

### Backend contract from Phase 30

Phase 30 already shipped the backend surface this phase must consume without reshaping:

- `PATCH /api/v1/inventory/categories/{id}`
- `DELETE /api/v1/inventory/categories/{id}`
- `PATCH /api/v1/inventory/materials/{id}`
- `POST /api/v1/inventory/materials/{id}/stock-in`
- `GET /api/v1/inventory/materials/{id}/history/enriched`

Important inherited decisions from prior work:

- Stock-in is a separate action from adjust-stock.
- Material updates use `clear_location` for explicit location removal.
- Category deletion is blocked when any material row exists; frontend must surface the conflict error clearly.
- Enriched history remains stock-movement-focused; metadata-only history entries are deferred and must not be added here.

### Existing frontend surfaces

Current inventory UI patterns already in code:

- `frontend/src/pages/inventory/InventoryListPage.tsx`
  - owns the inventory overview header and action button area
  - already loads `useCategories()` and `useMaterials()`
- `frontend/src/components/inventory/MaterialCard.tsx`
  - renders material list cards; good place to display category name
- `frontend/src/pages/inventory/InventoryDetailPage.tsx`
  - already contains a details card, actions card, and withdraw dialog
  - natural place for edit + stock-in entrypoints
- `frontend/src/lib/api/hooks/useInventory.ts`
  - holds inventory React Query hooks and invalidation patterns
- `frontend/src/pages/settings/SettingsPage.tsx`
  - existing settings landing page using card sections and `PageHeader`
- `frontend/src/App.tsx`
  - route registration point; currently only `/settings` exists, no nested inventory settings route yet

## Established Patterns to Reuse

### React Query mutation pattern

Use the same structure already present in `useCreateCategory`, `useWithdrawMaterial`, and `useDeleteMaterial`:

- mutation in `frontend/src/lib/api/hooks/useInventory.ts`
- invalidate stable query keys on success (`["categories"]`, `["materials"]`, `["material"]`, `["low-stock"]`)
- surface user feedback with `toast.success(...)` / `toast.error(...)` in the page/dialog layer

### Dialog pattern

Inventory already uses page-local dialog state:

- `InventoryDetailPage` controls `WithdrawDialog`
- `InventoryListPage` controls `AddMaterialDialog`

Phase 31 should follow the same pattern for:

- material edit dialog
- stock-in dialog
- category edit / delete confirmation flows if implemented inline on inventory settings page

### Route and page composition pattern

Use `PageHeader` + card sections for new inventory settings UI, matching:

- `frontend/src/pages/settings/SettingsPage.tsx`
- `frontend/src/pages/inventory/InventoryListPage.tsx`

Add explicit route registration in `frontend/src/App.tsx`; do not rely on hidden navigation.

## Likely Files for Phase 31

### Hook and type wiring

- `frontend/src/lib/api/hooks/useInventory.ts`
- `frontend/src/types/inventory.ts` (only if local helper types are still missing)

Expected additions:

- `useUpdateCategory`
- `useDeleteCategory`
- `useUpdateMaterial`
- `useStockInMaterial` (or equivalent naming aligned with existing hooks)
- query invalidation for category/material detail refresh

### Inventory overview + navigation

- `frontend/src/pages/inventory/InventoryListPage.tsx`
- `frontend/src/components/inventory/MaterialCard.tsx`
- `frontend/src/App.tsx`
- likely a new inventory settings page under `frontend/src/pages/settings/` or `frontend/src/pages/inventory/`

Expected work:

- add gear/settings entrypoint from inventory page to `/settings/inventory`
- show `material.category_name` or derived category label on overview cards

### Inventory detail actions

- `frontend/src/pages/inventory/InventoryDetailPage.tsx`
- new dialog components for edit + stock-in

Expected work:

- edit material location
- edit minimum quantity
- support arbitrary quantity correction via update flow chosen in UI design
- dedicated stock-in dialog with amount and optional notes

## Constraints and Planning Implications

1. **No backend reshaping in this phase.**
   Phase 31 should consume the new API contracts from Phase 30 as-is.

2. **Delete conflict UX is required.**
   `CATS-02` is not just delete behavior; it requires clear messaging when deletion is blocked by existing materials.

3. **Do not add metadata history entries.**
   `location_changed` / `min_quantity_changed` audit events are deferred. This phase must not plan UI work that depends on them appearing in history.

4. **This is a frontend-heavy phase.**
   A UI design contract is recommended before planning because the workflow's UI safety gate will likely require `UI-SPEC.md` for this phase.

5. **No new libraries are needed.**
   Existing stack (React, React Router, React Query, shadcn/ui, sonner) is sufficient.

## Recommended Plan Shape

The implementation naturally splits into three slices:

1. **Inventory settings + route**
   - route registration
   - inventory page gear entrypoint
   - category list/edit/delete UX with conflict handling

2. **Material detail editing + stock-in**
   - new hooks
   - edit dialog for location/min quantity/arbitrary stock correction
   - stock-in dialog with notes
   - detail page action wiring

3. **Overview display + regression coverage**
   - category name on overview cards
   - targeted frontend tests for new dialogs/routes/conflict messaging

## Risks / Common Pitfalls

- Forgetting to invalidate both list and detail queries after material mutations will leave stale quantities on screen.
- Treating delete failures as generic errors will miss the requirement for explicit FK/conflict messaging.
- Reusing withdraw UX for stock-in would violate the product decision that stock-in is a distinct action.
- Building the inventory settings page inside the generic settings page without a dedicated route would miss `CATS-03`.
- Relying on the old material history endpoint for this phase is unnecessary; enriched history is for Phase 32.

## Research Verdict

Research complete. This phase is ready for planning once the UI gate is satisfied.
