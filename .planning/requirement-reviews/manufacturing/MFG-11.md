# MFG-11

Status: Missing
Fit: Useful but not first
Priority: Later
Decision: Keep phased

Current state: There is no part-list model, no nesting model, and no manufacturing-linkage structure in the current codebase.
Evidence: `.planning/seeds/SEED-002-cad-cnc-integration.md`, `src/modules/*`

Implementation:
1. Link uploaded part lists and nesting outputs to projects first.
2. Track artifact relationships like `derived_from` and revision.
3. Add read-only structured metadata before any generator logic.
4. Postpone nesting optimization.
