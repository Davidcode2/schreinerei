# AST-12

Status: Missing
Fit: Very strong
Priority: Now
Decision: Keep

Current state: There is no reminder system, so nothing persists visibly until maintenance is resolved.
Evidence: `.planning/ROADMAP.md`, `src/modules/fleet/*`

Implementation:
1. Persist reminders as open records.
2. Show them in fleet overview and asset detail until explicitly resolved.
3. Do not allow silent dismissal.
4. Add severity for due vs overdue.
