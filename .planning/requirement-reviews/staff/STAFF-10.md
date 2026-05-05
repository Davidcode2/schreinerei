# STAFF-10

Status: Missing
Fit: Good
Priority: Soon
Decision: Keep as narrow foundation

Current state: The app has actual time bookings, but no employee work-time model for planning capacity.
Evidence: `src/modules/sites/domain/time_entry.rs`, `src/modules/iam/domain/user.rs`

Implementation:
1. Add `user_work_time_models` with effective dates.
2. Store baseline capacity, not payroll logic.
3. Expose manager-only CRUD first.
4. Keep this separate from actual time entries.
