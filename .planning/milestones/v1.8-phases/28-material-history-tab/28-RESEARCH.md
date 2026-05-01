# Phase 28: Material History Tab - Research

**Phase:** 28  
**Date:** 2026-05-01  
**Status:** Complete

## Objective

Research implementation details for showing material extraction history in the Baustellen activity feed with:
- site-linked entries
- category name
- extracting user
- eager-loaded backend queries (no N+1)

## Existing Implementation Findings

### Backend
- `GET /api/v1/inventory/materials/{id}/history` already exists in `src/modules/inventory/api/routes.rs`.
- It returns `StockEntryResponse` mapped from `StockEntryWithSite`.
- Repository method `list_stock_entries_with_site` already performs a SQL `LEFT JOIN sites` and returns `site_name`.
- Current response does **not** include material category name or extracting user display fields.

### Frontend
- `ActivityFeed.tsx` currently has a "Material" tab placeholder only.
- `SiteDetailPage.tsx` renders `ActivityFeed` and already has `siteId` context.
- `useInventory.ts` exposes `useMaterialHistory(materialId)` only, which is material-centric.
- `InventoryDetailPage.tsx` already renders stock history cards including `site_name`; this is a reusable rendering pattern.

### Data Model Signals
- `stock_entries` has `site_id`, `user_id`, `material_id`, quantity metadata.
- Existing repository query shape indicates a single SQL can enrich rows with related entities (site/category/user) to meet HIST-05.

## Required Additions to Meet Phase 28 Requirements

1. Add site-scoped history endpoint (or equivalent service path) to load stock entries for one Baustelle, not one material.
2. Extend response DTOs/types to include:
   - material name
   - category name
   - extracting user display name
3. Replace Material tab placeholder with actual fetched history list.
4. Render Baustelle link target for each entry where site_name is shown (navigate to detail route).
5. Keep query eager-loaded with explicit joins across stock_entries/materials/categories/users/sites.

## Architectural Recommendation

Implement a **site-scoped inventory history endpoint** in inventory module:

- Candidate: `GET /api/v1/inventory/sites/{site_id}/history`
- SQL: single query with joins (`stock_entries` + `materials` + `categories` + `users` + `sites`) and tenant filter.
- Frontend: new hook in `useInventory.ts`, then render results inside `ActivityFeed` material tab.

This keeps material-history concerns in inventory module and avoids overloading site activity endpoint.

## Risks and Mitigations

1. **Risk:** N+1 regressions if enrichment done in per-row lookups  
   **Mitigation:** enforce one SQL query with joins; add test asserting enriched fields present from one endpoint call.

2. **Risk:** User display name not consistently available in IAM schema  
   **Mitigation:** return fallback (`email` or `user_id`) when display name is null.

3. **Risk:** Frontend type drift across Rust/TS  
   **Mitigation:** derive DTO with `ts-rs` and regenerate `frontend/src/types/generated.ts`.

## Test Strategy

- Backend integration test for site history endpoint:
  - tenant scoping
  - includes category + user + site fields
  - ordering newest first
- Frontend component test for Material tab:
  - fetches site history
  - renders material/category/user
  - site link points to `/sites/:id`

---

*Research complete for Phase 28*
