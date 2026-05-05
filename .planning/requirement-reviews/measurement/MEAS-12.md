# MEAS-12

Status: Missing
Fit: Moderate
Priority: Later
Decision: Keep phased

Current state: DXF is not accepted as a measurement input type, and there is no DXF parser or room-layout model.
Evidence: `src/modules/sites/application/site_service.rs`, `.planning/FEATURES.md`

Implementation:
1. Start by accepting DXF as a linked artifact.
2. Add optional metadata extraction asynchronously.
3. Avoid deep semantic layout editing early.
4. Treat DXF handoff as glue, not CAD replacement.
