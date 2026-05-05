# AST-15

Status: Strong
Fit: Excellent
Priority: Now
Decision: Keep as baseline

Current state: `/fleet` already uses a vehicle-first weekly view with tap-friendly cells and bottom-sheet confirmation.
Evidence: `frontend/src/pages/fleet/FleetPage.tsx`, `frontend/src/pages/fleet/ReservationConfirmationSheet.tsx`

Implementation:
1. Keep this flow as the canonical booking UX.
2. Extend it with project defaults instead of redesigning it.
3. Avoid adding mandatory fields in the first tap flow.
4. Preserve visible reservations during range selection.
