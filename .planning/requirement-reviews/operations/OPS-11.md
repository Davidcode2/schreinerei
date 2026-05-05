# OPS-11

Status: Missing
Fit: Strong
Priority: Later
Decision: Keep phased

Current state: Inventory can cover consumables, but manufactured parts and nesting outputs do not exist as structured models.
Evidence: `src/modules/inventory/*`, `.planning/seeds/SEED-002-cad-cnc-integration.md`

Implementation:
1. Start with packlist items backed by inventory references and custom lines.
2. Add manufactured-part linkage only after manufacturing artifacts exist.
3. Keep item source types explicit.
4. Postpone deep CAD/CNC automation.
