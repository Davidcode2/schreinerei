# FIN-12

Status: Missing
Fit: Medium-high later
Priority: Later
Decision: Keep phased

Current state: There is no DATEV export or integration surface today.
Evidence: `src/modules/*`, `.planning/FEATURES.md`

Implementation:
1. Start with export contracts, not deep integration.
2. Build stable invoice/accounting mappings first.
3. Add CSV or DATEV-style export before direct integration.
4. Revisit API integration only once billing data stabilizes.
