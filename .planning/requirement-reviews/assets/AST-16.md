# AST-16

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Resource colors are deterministic and stable, but not guaranteed unique across all vehicles.
Evidence: `frontend/src/pages/fleet/resourceCalendarColor.ts`, `.planning/PROJECT.md`

Implementation:
1. Persist vehicle display colors in backend state.
2. Backfill colors automatically for existing vehicles.
3. Enforce tenant-scoped uniqueness.
4. Keep status indication separate from identity color.
