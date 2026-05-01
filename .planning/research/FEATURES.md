# Feature Landscape

**Domain:** Carpentry SaaS — Inventory v1.9
**Researched:** 2026-05-01

## Table Stakes

Features users expect. Missing = product feels incomplete for inventory management.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Category CRUD (create, read, update, delete) | Admins need to organize materials; current system only has create + list | Low | Domain command + service + repo + route pattern already exists for categories. Add update/delete following `DeleteMaterial` constraint-check pattern. |
| Edit material location | Materials move between storage locations; read-only location is a gap | Low | New `UpdateMaterial` command with partial update (PATCH). Only `location` and `min_quantity` fields. |
| Edit material minimum quantity | Business needs change; min_quantity needs adjustment over time | Low | Same `UpdateMaterial` command. Both fields in one endpoint, one dialog. |
| Set available quantity to arbitrary number | Correction/adjustment already exists but is admin-only; stock-in should be available to all employees | Med | New `StockIn` command, new endpoint `POST /materials/{id}/stock-in`, reuses `stock_entries` table. |
| "Material einlagern" (stock-in) action | Incoming deliveries need to be recorded; currently only deductions exist | Med | New `StockIn` command, new frontend dialog, new event type `MaterialAdded`. |
| Category string on inventory overview | Users need to see which category a material belongs to without clicking in | Low | Add `category_name` to `MaterialResponse` via JOIN in existing query. |

## Differentiators

Features that set product apart. Not expected, but valued.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Color-coded history entries | Green for stock-in, red for withdrawal, blue for adjustments, amber for location changes | Med | Requires `entry_type` enum in `stock_entries` + frontend rendering with distinct visual badges. |
| Clickable Baustelle links in history | Navigate from material withdrawal history directly to the Baustelle where material was used | Low | `site_id` already present in `stock_entries`. Frontend `Link` to `/sites/{id}`. |
| User attribution in history ("von Max Mustermann") | Accountability — who did what, when | Low | Join `users` table in history query, add `user_name` / `extracted_by` field. Pattern already used in `SiteStockHistoryRow`. |

## Anti-Features

Features to explicitly NOT build.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Bulk stock-in (batch import) | v1.9 is about single-item actions; bulk import is a v2+ concern | Single-item stock-in dialog. Room for batch in future. |
| Category hierarchy (nested categories) | Schreinerei categories are flat (Platten, Kanten, Lacke, Schrauben). Nesting adds complexity without value. | Flat category list with CRUD. |
| Real-time history via WebSocket | Polling is fine for MVP team sizes (5-20 users per org). Real-time adds infra complexity. | React Query stale/refetch with 30s stale time (existing pattern). |
| Soft-delete for categories | Categories with materials can't be deleted (FK constraint). Soft-delete adds query complexity for no user value when block-with-error exists. | Block delete with clear error message. Re-categorize materials first. |
| Audit log for category changes | Domain events already capture `CategoryUpdated`/`CategoryDeleted`. A separate audit table is redundant. | Use `domain_events` table which already logs all mutations. |

## Feature Dependencies

```
Category CRUD → InventorySettingsPage (frontend needs category API)
UpdateMaterial → MaterialEditDialog (frontend needs material PATCH API)
StockIn → StockInDialog (frontend needs stock-in API)
entry_type migration → Enriched history response → MaterialHistoryFeed
MaterialResponse + category_name JOIN → Category display on inventory overview
Enriched history → User attribution (JOIN users table)
Enriched history → Clickable Baustelle links (site_id → Link)
```

## MVP Recommendation

Prioritize:
1. Stock-in backend + frontend (most impactful: closes the "only deduct" gap)
2. Material edit backend + frontend (location and min_quantity)
3. Category CRUD backend + frontend (enables full inventory management)

Defer:
- Color-coded history: Requires migration, can ship in Phase 3 while stock-in works with existing history format
- User attribution in history: Nice-to-have, requires JOIN complexity

## Sources

- Project requirements from `.planning/PROJECT.md` — v1.9 active requirements
- Codebase analysis: existing create-only category endpoints, missing update/delete
- Codebase analysis: existing `AdjustStock` command (admin-only), gap for employee stock-in
- Codebase analysis: existing `MaterialResponse` lacks `category_name` field