# LOC-10

Status: Missing foundation
Fit: Medium now, high later
Priority: Soon for groundwork
Decision: Keep

Current state: UI strings are hardcoded German and date formatting is directly locale-bound. There is no i18n layer.
Evidence: `frontend/src/pages/*`, `frontend/src/components/*`

Implementation:
1. Add translation/message infrastructure now.
2. Replace hardcoded strings gradually.
3. Move locale-specific formatting behind helpers.
4. Keep backend enums/status codes language-neutral.
