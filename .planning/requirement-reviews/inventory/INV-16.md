# INV-16

Status: Partial
Fit: Strong
Priority: Now
Decision: Keep

Current state: Stock-in supports quantity, notes, and optional expiry, but not proper supplier metadata, receipt headers, or line-level goods receipt.
Evidence: `frontend/src/pages/inventory/StockInDialog.tsx`, `src/modules/inventory/application/inventory_service.rs`

Implementation:
1. Add structured goods-receipt header and line models.
2. Separate supplier/receipt metadata from free-text notes.
3. Post receipt lines into stock movements and lots.
4. Keep the worker UI minimal and office fields optional.
