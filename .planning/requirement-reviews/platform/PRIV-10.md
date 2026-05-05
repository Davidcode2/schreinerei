# PRIV-10

Status: Guardrail only
Fit: Very strong as policy
Priority: Now as planning rule
Decision: Keep

Current state: There is no live GPS tracking implementation today. Current location concepts are manual or reservation-based.
Evidence: `src/modules/fleet/domain/{vehicle,tool}.rs`, `.planning/PROJECT.md`

Implementation:
1. Treat GDPR review as a mandatory gate before any GPS work.
2. Separate vehicle telemetry from employee tracking in design.
3. Define consent, retention, and access rules before implementation.
4. Do not start passive tracking as a convenience feature.
