# FIN-11

Status: Missing
Fit: Necessary for market fit, not immediate workflow value
Priority: Later with design now
Decision: Keep

Current state: There is no e-invoicing support or invoice data model compatible with German structured invoice formats.
Evidence: `.planning/REQUIREMENTS.md`, `.planning/FEATURES.md`

Implementation:
1. Normalize legal party, tax, and payment fields early.
2. Add immutable invoice snapshot records.
3. Reserve an export pipeline for structured invoice formats later.
4. Avoid bolting compliance fields on at the end.
