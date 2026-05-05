# PROJ-16

Status: Partial
Fit: Strong
Priority: Soon
Decision: Keep

Current state: Only `estimated_days` exists. Budget, quote attachment, and budget-vs-actual tracking are missing.
Evidence: `src/modules/sites/domain/site.rs`, `.planning/REQUIREMENTS.md`

Implementation:
1. Add project budget and optional quote reference fields.
2. Derive actual labor/material usage from linked bookings.
3. Show budget vs actual on project detail first.
4. Delay rich finance workflows until booking discipline is stable.
