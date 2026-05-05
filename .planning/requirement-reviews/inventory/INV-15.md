# INV-15

Status: Partial
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Low-stock events exist, but there is no persistent replenishment queue or real notification workflow.
Evidence: `src/modules/inventory/domain/order_request.rs`, `src/common/events.rs`

Implementation:
1. Create a replenishment signal/read model.
2. Generate signals from minimum breach and last-package actions.
3. Add manager UI for open, acknowledged, ordered, and resolved states.
4. Connect signals to stock-in/goods-receipt closure.
