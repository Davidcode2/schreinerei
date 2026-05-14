# 45-01 Summary

## Outcome

The project timeline now has one shared mobile-first composer for note text, camera capture, uploaded images, and PDFs.

## Changes

- Added `ProjectTimelineComposer` as the single creation flow for timeline entries
- Routed `CreateNoteModal` through the shared composer in default mode
- Routed `CameraUploadFlow` through the shared composer in camera mode
- Preserved offline single-photo queue behavior for the camera shortcut
- Standardized new timeline submissions on one `activity_type: "note"` payload with linked `attachment_ids`
- Added targeted tests for note-only, attachment-only, camera mode, and wrapper behavior

## Verification

- `npm --prefix frontend run test -- ProjectTimelineComposer.test.tsx CreateNoteModal.test.tsx CameraUploadFlow.test.tsx`

## Notes

- Legacy `photo` activity rows remain readable history, but new unified entries converge on note-plus-attachments.
