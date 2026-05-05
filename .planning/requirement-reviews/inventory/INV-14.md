# INV-14

Status: Missing
Fit: Very strong
Priority: Now
Decision: Keep

Current state: There is low-stock handling, but no explicit one-tap "last package taken" action.
Evidence: `frontend/src/pages/inventory/WithdrawDialog.tsx`, `src/modules/inventory/domain/events.rs`

Implementation:
1. Add a `mark_last_package_taken` domain command.
2. Offer it inside withdrawal and QR-driven flows.
3. Deduplicate repeated signals until restock.
4. Feed it into manager replenishment state.
