# STAFF-11

Status: Missing
Fit: Strong after STAFF-10
Priority: Later
Decision: Keep

Current state: There is no holiday/absence entity, employee calendar, or remaining-capacity calculation.
Evidence: `src/modules/sites/*`, `.planning/FEATURES.md`

Implementation:
1. Add `staff_absences` with range, type, status, and notes.
2. Build a simple manager calendar/list view.
3. Compute estimated availability from work-time models minus absences.
4. Avoid inferring absence from missing time entries.
