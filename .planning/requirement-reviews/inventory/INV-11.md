# INV-11

Status: Partial to strong
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Category-level expiry capability already exists through `can_expire`, but edge cases around changing category behavior need hardening.
Evidence: `src/modules/inventory/domain/category.rs`, `migrations/017_material_expiry_tracking.sql`

Implementation:
1. Treat expiry tracking as a category capability.
2. Add rules for toggling `can_expire` on live categories.
3. Clarify UI that expiry is only required for relevant categories.
4. Add migration-safe tests for legacy stock.
