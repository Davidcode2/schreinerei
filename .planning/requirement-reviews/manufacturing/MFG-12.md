# MFG-12

Status: Planning principle only
Fit: Essential
Priority: Now
Decision: Keep

Current state: The code does not violate this yet, but the boundary must stay explicit as manufacturing scope grows.
Evidence: `.planning/FEATURES.md`, `.planning/REQUIREMENTS.md`

Implementation:
1. Treat manufacturing as glue, not an in-app CAD replacement.
2. Keep provenance, file state, and workflow orchestration inside the app.
3. Keep geometry authoring and CAM logic in external specialist tools.
4. Enforce the boundary at module design time.
