# AST-11

Status: Missing
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Assets only have coarse status. There are no maintenance schedules, due dates, or reminder records.
Evidence: `src/modules/fleet/domain/{vehicle,tool}.rs`, `migrations/006_fleet_schema.sql`

Implementation:
1. Add maintenance schedules per asset.
2. Add due occurrence/reminder records with status and resolution metadata.
3. Separate maintenance planning from live asset status.
4. Surface due and overdue maintenance in overview and asset detail.
