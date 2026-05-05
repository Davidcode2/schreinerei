# OPS-12

Status: Missing
Fit: Very strong if OPS-10 ships
Priority: High inside packlist work
Decision: Keep

Current state: There is no loading confirmation workflow, but the app already has strong mobile interaction patterns suitable for it.
Evidence: `frontend/src/pages/fleet/ReservationConfirmationSheet.tsx`, `frontend/src/pages/inventory/*`

Implementation:
1. Build loading confirmation as a tap-first checklist, not a form.
2. Default project and vehicle context automatically.
3. Persist per-item loaded/missing/skipped state.
4. Add offline-safe sync if used in the field.
