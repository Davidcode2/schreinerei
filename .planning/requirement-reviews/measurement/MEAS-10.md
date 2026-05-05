# MEAS-10

Status: Partial foundation only
Fit: Very strong
Priority: Now
Decision: Keep

Current state: The app has camera/document capture and offline photo queue, but no dedicated measurement domain, typed dimensions, or exportable measurement records.
Evidence: `frontend/src/pages/sites/CameraUploadFlow.tsx`, `frontend/src/lib/offline/queue.ts`

Implementation:
1. Add `measurement_sessions` and typed measurement items.
2. Reuse mobile media capture where possible.
3. Build fast room/element templates with structured fields.
4. Add export after the capture flow is stable.
