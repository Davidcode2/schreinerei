# INV-13

Status: Partial
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Expired quantities can be computed, but there is no alert queue, manager inbox, or overview-level surfacing.
Evidence: `frontend/src/pages/inventory/InventoryDetailPage.tsx`, `src/modules/inventory/*`

Implementation:
1. Add expired and expiring-soon read models.
2. Surface alerts in overview/dashboard, not only detail pages.
3. Emit persistent replenishment/disposal signals.
4. Add a disposal follow-up path for expired stock.
