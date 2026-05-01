# Phase 32: Enriched History — Research

**Date:** 2026-05-01
**Status:** Ready for UI design contract
**Phase:** 32
**Requirements:** STOCK-02, HIST-01, HIST-02, HIST-03

## Goal

Upgrade the inventory detail history feed from a plain stock-entry list to an enriched stock-movement feed that:
- uses the dedicated Phase 30 `/history/enriched` API,
- visually differentiates entry types,
- shows who performed each action,
- links withdrawal entries back to their Baustelle,
- includes stock-in entries as first-class history events.

## What Already Exists

### Backend contract from Phase 30

Phase 30 already shipped the API and DTOs this phase should consume without backend reshaping:

- `GET /api/v1/inventory/materials/{id}/history/enriched`
- `EnrichedStockHistoryResponse` / `EnrichedStockHistoryEntry`
- `entry_type` values: `withdrawn`, `adjusted`, `material_added`, `location_changed`, `min_quantity_changed`
- `user_name` resolved server-side via `COALESCE(u.name, u.email, user_id::text)`
- `site_id`, `site_name`, and `category_name` already present in the response

Important inherited decisions:

- Enriched history is stock-movement-focused for v1.9.
- `location_changed` and `min_quantity_changed` remain deferred until a separate audit model exists.
- Stock-in is a dedicated command that emits `material_added` entries.
- Frontend keeps a manual `EntryType` union in `frontend/src/types/inventory.ts`.

### Current frontend state

The inventory detail page still renders the legacy history contract:

- `frontend/src/lib/api/hooks/useInventory.ts`
  - `useMaterialHistory(id)` calls `/api/v1/inventory/materials/{id}/history`
  - no hook exists yet for `/history/enriched`
- `frontend/src/pages/inventory/InventoryDetailPage.tsx`
  - renders date, quantity, notes, and optional `site_name`
  - does **not** show `entry_type`, `user_name`, or clickable site links
  - empty state copy still says `Noch keine Entnahmen erfasst`
- `frontend/src/types/inventory.ts`
  - already contains `EntryType` and `EnrichedStockHistoryEntry`
- `frontend/src/pages/sites/ActivityFeed.tsx`
  - already demonstrates the project pattern for clickable Baustelle links via `<Link to={/sites/:id}>`

## Established Patterns to Reuse

### Query hook pattern

Stay inside `frontend/src/lib/api/hooks/useInventory.ts` and reuse the existing React Query structure:

- `useQuery({ queryKey, queryFn, enabled, staleTime })`
- material-scoped keys such as `['material-history', id]`
- no page-local fetch logic in `InventoryDetailPage`

For this phase, prefer a dedicated hook like `useEnrichedMaterialHistory(id)` over mutating page code to fetch directly.

### Inventory detail composition pattern

Phase 31 established these patterns on `InventoryDetailPage`:

- header actions live inside `PageHeader.action`
- dialogs remain page-owned
- success/error feedback stays at page level with `sonner`

Phase 32 should keep history rendering as a dedicated detail-page section and avoid pushing formatting logic into hooks.

### Link and card pattern

Reuse the existing sites activity-feed link treatment:

- clickable Baustelle only when both `site_id` and `site_name` exist
- `Link` target `/sites/${site_id}`
- plain text fallback when no site is attached

### Visual differentiation pattern

There is no existing inventory history badge system yet, so the phase should introduce a local mapping from `EntryType` to:

- German label copy
- icon or badge treatment
- color classes

Because the requirement is UI-only and the enum values already exist, no new dependency is needed.

## Likely Files for Phase 32

### Data hook and type wiring

- `frontend/src/lib/api/hooks/useInventory.ts`
- `frontend/src/lib/api/hooks/useInventory.test.tsx`
- `frontend/src/types/inventory.ts` (only if helper labels/types need local expansion)

Expected additions:

- `useEnrichedMaterialHistory(id)` calling `/api/v1/inventory/materials/${id}/history/enriched`
- query key dedicated to enriched history so the old history contract is not silently reused
- hook-level regression coverage for the enriched endpoint contract

### Detail-page rendering

- `frontend/src/pages/inventory/InventoryDetailPage.tsx`
- `frontend/src/pages/inventory/InventoryDetailPage.test.tsx`

Expected work:

- swap from legacy history hook to enriched history hook
- render badge/label per `entry_type`
- render `von {user_name}` attribution
- make Baustelle links clickable for withdrawals with `site_id`
- update empty and quantity copy so stock-in entries no longer read like withdrawals only

### Optional extraction for readability

If the history section becomes too large, extract a local presentation component such as:

- `frontend/src/pages/inventory/MaterialHistoryFeed.tsx`

This aligns with the state decision that `MaterialHistoryFeed` should stay separate from the sites `ActivityFeed`.

## Constraints and Planning Implications

1. **No backend reshaping in this phase.**
   The enriched endpoint already exists and should be consumed as-is.

2. **Do not implement deferred metadata audit items.**
   `location_changed` and `min_quantity_changed` may exist in the enum, but the phase must not force new backend events into `stock_entries`.

3. **Phase 32 is frontend-heavy and UI-sensitive.**
   A dedicated `32-UI-SPEC.md` should define badge colors, copy, spacing, and empty-state wording before planning.

4. **Tests should stay targeted.**
   Existing repo-wide TypeScript build noise should not expand scope; prefer focused Vitest + RTL coverage for history rendering and hook endpoint selection.

5. **Tenant-safe navigation already exists.**
   Linking to `/sites/:id` reuses existing authenticated app routing; no new auth work is needed.

## Recommended Plan Shape

The implementation naturally splits into two focused plans:

1. **Enriched history data contract**
   - add the enriched history hook
   - point tests at `/history/enriched`
   - keep frontend types aligned with existing DTOs

2. **Inventory detail feed upgrade**
   - replace legacy rendering with type badges, user attribution, and site links
   - add regression coverage for stock-in rows, adjusted rows, and withdrawal links

## Risks / Common Pitfalls

- Reusing `useMaterialHistory` and only changing UI copy would miss `user_name` and `entry_type` entirely.
- Hardcoding only withdrawal styling would fail `STOCK-02` because stock-in entries need distinct treatment.
- Rendering Baustelle names as plain text would miss `HIST-03` even if the data is present.
- Treating every positive quantity as stock-in in the UI without checking `entry_type` risks future mislabeling.
- Sharing the sites `ActivityFeed` component would violate the current project decision that inventory history remains its own component.

## Research Verdict

Research complete. The phase is technically straightforward and needs no new libraries, but planning should stop until a Phase 32 `UI-SPEC.md` exists because this is a frontend/UI phase and the workflow's UI safety gate applies.
