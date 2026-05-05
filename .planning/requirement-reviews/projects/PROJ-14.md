# PROJ-14

Status: Partial
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Time can be booked for sites or as workshop time, but productive workshop/CNC time is not consistently project-linked.
Evidence: `src/modules/sites/domain/time_entry.rs`, `frontend/src/pages/sites/TimeEntryDialog.tsx`

Implementation:
1. Keep work types explicit.
2. Require project linkage for productive/billable work.
3. Allow null only for true overhead/admin time.
4. Default to active project in time-booking UI.
