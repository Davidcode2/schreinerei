# PROJ-18

Status: Partial to strong
Fit: Very strong
Priority: Now
Decision: Keep

Current state: The Baustelle feed already handles notes, photos, documents, timestamps, and material history, but it is not yet treated as the canonical project memory.
Evidence: `frontend/src/pages/sites/ActivityFeed.tsx`, `src/modules/sites/domain/activity.rs`

Implementation:
1. Reframe the feed as the project timeline.
2. Add structured observation and follow-up entry types.
3. Keep material/time history reachable from the same project context.
4. Avoid parallel communication channels inside the app.
