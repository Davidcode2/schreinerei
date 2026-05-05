# ARCH-04

Status: Strong
Fit: Very strong
Priority: Now
Decision: Keep

Current state: The product is already mobile-first and PWA-first, with mobile navigation, bottom-sheet flows, camera capture, and offline support.
Evidence: `frontend/src/App.tsx`, `frontend/src/components/layout/*`, `frontend/public/manifest.json`

Implementation:
1. Keep mobile-first as a permanent acceptance criterion.
2. Prefer bottom sheets, camera-first entry, and large tap targets.
3. Add mobile verification to every new major workflow.
4. Reject desktop-only interaction patterns in planning.
