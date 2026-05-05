# AST-13

Status: Strong
Fit: Excellent
Priority: Now
Decision: Keep

Current state: Tool and vehicle reservations already exist with overlap checks, calendar visibility, and holder context.
Evidence: `src/modules/fleet/application/fleet_service.rs`, `frontend/src/pages/fleet/CalendarView.tsx`

Implementation:
1. Keep explicit reservations as the source of truth.
2. Add a clear derived `current_holder` read field.
3. Keep holder context visible in calendar, detail, and QR flows.
4. Harden linked-context validation.
