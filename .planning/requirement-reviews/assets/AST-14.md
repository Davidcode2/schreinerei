# AST-14

Status: Partial
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Reservations can link to `site_id`, but not to a broader `project_id`, and there is no explicit reservation purpose field.
Evidence: `src/modules/fleet/domain/reservation.rs`, `frontend/src/pages/fleet/ReservationDialog.tsx`

Implementation:
1. Add `project_id` as the long-term reservation context.
2. Add a short `purpose` field separate from free-form notes.
3. Prefill date range and purpose from project context.
4. Validate project ownership tenant-safely in the application layer.
