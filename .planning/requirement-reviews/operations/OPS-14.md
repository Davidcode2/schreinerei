# OPS-14

Status: Missing
Fit: Good later
Priority: Later
Decision: Keep deferred

Current state: Some status data exists in inventory/fleet, but there is no workshop map layer and no charging/telemetry inputs.
Evidence: `src/modules/inventory/*`, `src/modules/fleet/*`

Implementation:
1. Treat status indicators as an enhancement on top of OPS-13.
2. Start with existing app-native signals like low stock and maintenance.
3. Add hardware-backed charging/telemetry later via adapters.
4. Keep graceful fallback when no live data exists.
