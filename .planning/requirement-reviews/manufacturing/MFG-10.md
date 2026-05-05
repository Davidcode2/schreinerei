# MFG-10

Status: Missing
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Attachment handling exists for site feed media, but there is no manufacturing artifact registry and current storage is too limited for DXF/CNC workflows.
Evidence: `src/modules/sites/domain/activity.rs`, `migrations/012_site_activity_attachments.sql`

Implementation:
1. Add a project/manufacturing artifact registry.
2. Support typed artifacts like DXF, BSolid export, part-list PDF, nesting sheet.
3. Move large files to object storage.
4. Ship upload/list/download/reference before any parsing.
