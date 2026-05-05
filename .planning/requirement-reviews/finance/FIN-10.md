# FIN-10

Status: Missing
Fit: Strong
Priority: Soon
Decision: Keep

Current state: The app has customer, time, and material raw data, but no invoice-ready project snapshot, billing fields, or PDF invoice generation.
Evidence: `src/modules/sites/domain/site.rs`, `src/modules/sites/domain/time_entry.rs`, `src/modules/inventory/domain/material.rs`

Implementation:
1. Strengthen project-linked material and time booking first.
2. Add billing metadata to the project aggregate.
3. Build invoice-ready snapshot/export before PDF generation.
4. Keep billing in a dedicated finance context.
