# Phase 25 Research — Deduction History Site Name End-to-End Wiring

**Date:** 2026-05-01
**Phase:** 25
**Requirement focus:** DEDU-03

## Goal of Research

Identify the minimum frontend wiring needed to consume and render backend `site_name` from material stock history endpoint `/api/v1/inventory/materials/{id}/history`.

## Current State (Evidence)

- Backend producer exists and already returns history rows with `site_id` + `site_name` (`src/modules/inventory/api/routes.rs`, Phase 22-04 summary).
- Generated frontend type exists (`frontend/src/types/generated.ts` includes `StockEntryResponse`).
- Frontend has no hook/API consumer for material history in inventory hooks (`frontend/src/lib/api/hooks/useInventory.ts`).
- Inventory detail page has no history section (`frontend/src/pages/inventory/InventoryDetailPage.tsx`).
- Milestone audit explicitly flags this integration gap (`.planning/v1.7-MILESTONE-AUDIT.md`).

## Existing Patterns to Reuse

1. **React Query data hooks pattern**
   - `useMaterial`, `useMaterials`, `useReservations` style in `frontend/src/lib/api/hooks/useInventory.ts` and `useFleet.ts`.
   - Use stable query keys and `enabled: !!id` guards for id-based fetches.

2. **Conditional site label rendering pattern**
   - `frontend/src/pages/fleet/ReservationsList.tsx` renders site name only when present (`reservation.site_name && ...`).

3. **Inventory page structure pattern**
   - `InventoryDetailPage.tsx` uses card-based sections and existing loading/error handling for primary material data.

4. **Frontend tests with MSW pattern**
   - Component tests use `@/test/utils`, `server.use(...)`, and `http.get/post` overrides (example: `AddMaterialDialog.test.tsx`).

## Recommended Implementation Shape

1. Add a typed inventory history row interface in `frontend/src/types/inventory.ts` (or explicitly consume generated `StockEntryResponse` with a local alias).
2. Add `useMaterialHistory(id: string)` hook in `frontend/src/lib/api/hooks/useInventory.ts`:
   - Query key: `['material-history', id]`
   - GET path: `/api/v1/inventory/materials/${id}/history`
   - `enabled: !!id`
3. Update `InventoryDetailPage.tsx` to call `useMaterialHistory(material.id)` and render a dedicated “Historie” section:
   - Show withdrawal entries (negative `quantity_change`) with timestamp, amount, optional notes.
   - Render `site_name` badge/text when present.
   - Render fallback text for missing history.
4. Add/extend frontend test coverage for history rendering path and `site_name` visibility.

## Common Pitfalls

- Forgetting query invalidation after withdrawal: history list can appear stale after successful deduction.
- Showing `site_name` label unconditionally causing empty visual artifacts for null values.
- Coupling history rendering to generated file edits (`generated.ts` must not be manually modified).

## Verification Strategy

- Unit/component test: mocked history response with one entry with `site_name` and one with `null`.
- Assert UI renders site name text only for the linked entry.
- Ensure page still renders when history endpoint returns empty array.

## Scope Boundaries

- In scope: frontend consumer + rendering + tests for DEDU-03.
- Out of scope: backend route/model changes (already shipped in Phase 22-04).
