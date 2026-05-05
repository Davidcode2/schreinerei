# OPS-10

Status: Missing
Fit: Strong
Priority: Later than current core backlog
Decision: Keep

Current state: There is no packlist model or montage-ready trigger. Current site status flow is too simple for execution-readiness logic.
Evidence: `src/modules/sites/domain/site.rs`, `.planning/FEATURES.md`

Implementation:
1. Add a montage-ready or execution-readiness concept.
2. Model packlists as snapshot records, not only live derived queries.
3. Generate initial checklists from project, inventory, and fleet context.
4. Allow manual additions before any deeper automation.
