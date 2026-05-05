# PROJ-10

Status: Missing as explicit model
Fit: Very strong
Priority: Now
Decision: Keep

Current state: The app still uses `site` / `Baustelle` as the main aggregate. Workshop work is mostly represented by null site linkage.
Evidence: `src/modules/sites/domain/site.rs`, `src/modules/sites/domain/time_entry.rs`

Implementation:
1. Evolve `Site` into a broader `Project` concept.
2. Add `project_type` such as `external_site` and `internal_workshop`.
3. Backfill existing Baustellen as external projects.
4. Delay hard table renames until behavior stabilizes.
