# ARCH-05

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Attachment features exist, but files are still stored in PostgreSQL blobs rather than S3-compatible or cluster-local object storage.
Evidence: `migrations/012_site_activity_attachments.sql`, `src/modules/sites/infrastructure/site_repository.rs`

Implementation:
1. Add an `ObjectStorage` port plus storage config.
2. Move attachment bytes out of Postgres; keep only metadata in DB.
3. Preserve authenticated blob access through backend routes.
4. Migrate existing files and thumbnails safely.
