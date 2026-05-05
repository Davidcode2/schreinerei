# RPT-12

Status: Partial
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Site detail and history already show raw time and material activity, but there are no true per-project aggregates yet.
Evidence: `frontend/src/pages/sites/SiteDetailPage.tsx`, `src/modules/inventory/infrastructure/material_repository.rs`

Implementation:
1. Tighten project linkage for withdrawals and productive time.
2. Add per-project aggregate queries for material and labor.
3. Surface the analytics first on project detail.
4. Reuse the same aggregates later for dashboard/reporting views.
