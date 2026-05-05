# OPS-13

Status: Missing
Fit: Good
Priority: Low to medium
Decision: Keep

Current state: Tools, vehicles, and materials only have free-text locations. There is no normalized workshop map or zone model.
Evidence: `src/modules/fleet/domain/{tool,vehicle}.rs`, `src/modules/inventory/domain/material.rs`

Implementation:
1. Add workshop layout and canonical zones/slots.
2. Normalize existing free-text locations onto those zones.
3. Build a simple placement editor and static map view.
4. Keep free-text as fallback during migration.
