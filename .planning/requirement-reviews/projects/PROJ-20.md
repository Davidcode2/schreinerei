# PROJ-20

Status: Mostly met
Fit: Strong
Priority: Now
Decision: Keep as polish

Current state: Feed entries already show timestamps and previews for attachments, but consistency and exactness can still be polished.
Evidence: `frontend/src/pages/sites/ActivityFeed.tsx`, `frontend/src/pages/sites/MediaViewer.tsx`

Implementation:
1. Keep previews for all attachment-backed entries.
2. Show exact timestamp where useful in addition to relative time.
3. Make rendering consistent across entry types.
4. Avoid redesigning the feed for this requirement.
