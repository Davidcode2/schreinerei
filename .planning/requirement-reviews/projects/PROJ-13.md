# PROJ-13

Status: Partial
Fit: Very strong
Priority: Now
Decision: Keep

Current state: Material withdrawals can link to a site, but linkage is optional, which weakens billing and analytics.
Evidence: `src/modules/inventory/domain/material.rs`, `frontend/src/pages/inventory/WithdrawDialog.tsx`

Implementation:
1. Require project linkage for real material consumption.
2. Default the project from active context for speed.
3. Keep disposal and stock correction outside this rule.
4. Use internal workshop projects instead of null linkage.
