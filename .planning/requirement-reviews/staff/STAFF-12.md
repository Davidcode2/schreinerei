# STAFF-12

Status: Missing
Fit: High long-term
Priority: Later
Decision: Keep deferred

Current state: Sites already have dates and assignments, but planning data is too weak to support credible capacity calculations.
Evidence: `migrations/005_sites_schema.sql`, `frontend/src/pages/sites/AddSiteDialog.tsx`

Implementation:
1. Strengthen project planning fields and assignment usage first.
2. Define project demand rules before building availability math.
3. Build a read model combining capacity, absences, schedules, and assignments.
4. Add conflict/overbooking views only after the underlying data is reliable.
