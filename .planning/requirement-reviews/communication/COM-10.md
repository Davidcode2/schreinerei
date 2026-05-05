# COM-10

Status: Partial foundation only
Fit: Strong
Priority: Soon
Decision: Keep

Current state: The project feed already supports notes, photos, documents, timestamps, and protected media, but not audio capture.
Evidence: `frontend/src/pages/sites/ActivityFeed.tsx`, `src/modules/sites/domain/activity.rs`

Implementation:
1. Add audio/voice-note as another feed attachment type.
2. Reuse the existing activity-entry model instead of a separate subsystem.
3. Add simple record/upload/playback flows.
4. Keep authenticated media access and mobile-first UX.
