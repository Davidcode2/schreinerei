# AST-10

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Vehicles and tools are separate aggregates. Large workshop machines are not modeled as first-class assets.
Evidence: `migrations/006_fleet_schema.sql`, `src/modules/fleet/domain/{vehicle,tool}.rs`

Implementation:
1. Introduce a core `Asset` identity and asset kind.
2. Keep type-specific metadata in separate detail records.
3. Add `machine` as a first-class asset kind.
4. Make reservations and maintenance depend on asset capabilities, not table names.
