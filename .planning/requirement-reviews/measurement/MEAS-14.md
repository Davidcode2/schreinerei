# MEAS-14

Status: Missing
Fit: Extremely strong
Priority: Now
Decision: Keep

Current state: There are no measurement plausibility checks yet. Existing validation is generic, not dimension-aware.
Evidence: `src/modules/sites/application/site_service.rs`, `.planning/FEATURES.md`

Implementation:
1. Define a typed measurement schema first.
2. Add explainable rule-based checks like total-vs-part mismatch and missing required fields.
3. Separate warnings from blocking errors.
4. Allow override with reason when field reality differs.
