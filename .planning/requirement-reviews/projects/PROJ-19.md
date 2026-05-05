# PROJ-19

Status: Partial
Fit: Strong
Priority: Now
Decision: Keep

Current state: Camera upload and document upload already exist, but they are still split into separate entry flows rather than one unified composer.
Evidence: `frontend/src/pages/sites/CameraUploadFlow.tsx`, `frontend/src/pages/sites/CreateNoteModal.tsx`

Implementation:
1. Create one unified entry composer.
2. Keep a fast camera shortcut into the same model.
3. Support note + multi-image + multi-document in one submission.
4. Preserve mobile speed over configurability.
