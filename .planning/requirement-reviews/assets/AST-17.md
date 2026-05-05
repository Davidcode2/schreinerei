# AST-17

Status: Missing foundation
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Reservations are already independent from location, but there is no location-signal model or adapter boundary.
Evidence: `src/modules/fleet/domain/reservation.rs`, `src/modules/fleet/domain/{vehicle,tool}.rs`

Implementation:
1. Define a `LocationSignal` or `AssetObservation` port.
2. Store raw observations separately from current derived location.
3. Attach observations to asset identity, not reservation rows.
4. Expose freshness and confidence in read models.
