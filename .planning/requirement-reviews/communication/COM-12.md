# COM-12

Status: Partial foundation only
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Feed entries already carry timestamp, author, and media, but there is no claims-relevant evidence flag, retention policy, or export bundle.
Evidence: `src/modules/sites/domain/activity.rs`, `frontend/src/pages/sites/MediaViewer.tsx`

Implementation:
1. Add `claims_relevant` tagging on project entries.
2. Restrict deletion or require audit reason once flagged.
3. Add evidence export with note, media, author, and timestamps.
4. Keep the workflow fast enough to be used during work.
