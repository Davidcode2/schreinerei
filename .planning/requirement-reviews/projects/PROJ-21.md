# PROJ-21

Status: Missing
Fit: Low near-term, higher later
Priority: Later
Decision: Keep deferred

Current state: Project/site creation is manual only. There is no customer-driven or email-driven intake flow.
Evidence: `frontend/src/pages/sites/AddSiteDialog.tsx`, `.planning/FEATURES.md`

Implementation:
1. Stabilize internal project creation and execution flows first.
2. Add customer/contact model only when needed.
3. Treat email/customer intake as an adapter into the same project pipeline.
4. Avoid making intake a new workflow silo.
