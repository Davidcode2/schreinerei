# PROJ-17

Status: Missing
Fit: High long-term
Priority: Later
Decision: Keep

Current state: There is no invoice generation or invoice-ready project snapshot flow yet.
Evidence: `src/modules/sites/*`, `.planning/FEATURES.md`

Implementation:
1. Start with invoice-ready project summaries, not a full billing engine.
2. Snapshot linked time and material usage at completion.
3. Add exportable invoice data before PDF generation.
4. Keep final billing in a dedicated finance context.
