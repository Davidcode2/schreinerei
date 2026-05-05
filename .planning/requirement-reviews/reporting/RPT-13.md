# RPT-13

Status: Missing
Fit: Good later
Priority: Later
Decision: Keep deferred

Current state: Search is still simple list filtering. There is no global search index, document text corpus, or AI retrieval layer.
Evidence: `frontend/src/pages/sites/*`, `frontend/src/pages/inventory/*`, `.planning/FEATURES.md`

Implementation:
1. Finish structured filters and reporting first.
2. Add global non-AI search across projects, materials, and docs.
3. Build searchable attachment text extraction later.
4. Layer AI retrieval on top only with citations and tenant-safe access.
