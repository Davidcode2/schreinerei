# PROJ-15

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Assignment backend exists and fleet reservations can point to sites, but there is no coherent project planning view yet.
Evidence: `migrations/005_sites_schema.sql`, `src/modules/fleet/domain/reservation.rs`

Implementation:
1. Add worker assignment UI on project detail.
2. Add project filter/view to fleet planning surfaces.
3. Show workers, reservations, and project timing together.
4. Build on existing calendars instead of inventing a second planner.
