# INV-12

Status: Partial
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Expiry-aware stock exists as dated quantity buckets, but not as full lot/batch records with richer metadata.
Evidence: `migrations/017_material_expiry_tracking.sql`, `src/modules/inventory/infrastructure/material_repository.rs`

Implementation:
1. Replace date-only buckets with true lot/batch records.
2. Link stock-in/goods receipt to created lots.
3. Use FEFO withdrawal for expiring items.
4. Define a migration path for legacy quantity.
