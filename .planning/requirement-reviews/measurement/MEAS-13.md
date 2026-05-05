# MEAS-13

Status: Missing
Fit: Real but niche near-term
Priority: Later
Decision: Keep deferred

Current state: Point-cloud linkage does not exist and current attachment storage is not suitable for large scan artifacts.
Evidence: `migrations/012_site_activity_attachments.sql`, `.planning/FEATURES.md`

Implementation:
1. Start with external scan references, not native ingestion.
2. Store provider, URL, preview metadata, and notes.
3. Keep audit trail and permissions.
4. Postpone native point-cloud storage/rendering.
