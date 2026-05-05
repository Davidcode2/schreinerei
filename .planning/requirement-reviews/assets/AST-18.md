# AST-18

Status: Deferred correctly
Fit: Good later, bad as baseline dependency
Priority: Later
Decision: Keep deferred

Current state: RFID is explicitly out of baseline scope, and manual/QR-based workflows already provide a workable fallback.
Evidence: `.planning/PROJECT.md`, `.planning/seeds/SEED-003-rfid-integration.md`

Implementation:
1. Keep baseline workflows fully functional without RFID.
2. Model RFID as an infrastructure adapter that emits observations.
3. Preserve manual override and offline behavior.
4. Add privacy and retention rules before rollout.
