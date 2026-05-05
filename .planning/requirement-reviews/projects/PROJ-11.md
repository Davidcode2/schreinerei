# PROJ-11

Status: Partial
Fit: Strong
Priority: Now
Decision: Keep

Current state: Location and date fields exist, but project scheduling, worker assignment UI, and project-centric operational notes are still weak.
Evidence: `migrations/005_sites_schema.sql`, `frontend/src/pages/sites/*`

Implementation:
1. Strengthen planning fields on the current site/project aggregate.
2. Surface worker assignment management in UI.
3. Separate planning notes from feed notes where useful.
4. Keep the project detail screen as the main execution surface.
