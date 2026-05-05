# ARCH-01

Status: Partial
Fit: Strong
Priority: Now
Decision: Keep

Current state: The repo already follows a modular-monolith shape with `iam`, `inventory`, `sites`, and `fleet`, but `projects`, `manufacturing`, and `billing` are not explicit bounded contexts yet.
Evidence: `src/modules.rs`, `src/modules/*`, `.planning/FEATURES.md`

Implementation:
1. Evolve `sites` into a broader `projects` context before adding more scope.
2. Keep hexagonal boundaries per module: domain, application, infrastructure.
3. Add new contexts only behind explicit ports/events, not cross-module repo access.
4. Add architecture checks for boundary violations.
