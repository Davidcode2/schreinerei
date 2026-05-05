# ARCH-03

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Current-state tables and some audit/history exist, but mutation coverage is inconsistent across modules and metadata-only audit is still deferred.
Evidence: `migrations/003_domain_events.sql`, `src/common/events.rs`, `.planning/PROJECT.md`

Implementation:
1. Standardize one audit contract for all state mutations.
2. Write state change and audit append in one transaction.
3. Cover edits, deletes, assignments, and attachment lifecycle.
4. Build separate read projections for user-facing history.
