# INV-17

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep with procurement split

Current state: Order requests exist, but fulfillment is still too lightweight and risks bypassing consistent stock/lot handling.
Evidence: `migrations/004_order_requests.sql`, `src/modules/inventory/domain/order_request.rs`

Implementation:
1. Keep replenishment demand in Inventory.
2. Treat procurement lifecycle as a separate bounded context or lightweight submodule.
3. Route receipt completion back through goods-receipt/stock flows.
4. Add explicit states: requested, ordered, received, cancelled.
