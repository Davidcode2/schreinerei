---
phase: 31-document-upload-rework
reviewed: 2026-05-01T16:06:30Z
status: clean
depth: quick
---

# Phase 31 Code Review

## Scope

- `src/modules/sites/domain/activity.rs`
- `src/modules/sites/application/site_service.rs`
- `src/modules/sites/infrastructure/site_repository.rs`
- `src/modules/sites/api/routes.rs`
- `frontend/src/pages/sites/CreateNoteModal.tsx`
- `frontend/src/lib/api/hooks/useSites.ts`
- `frontend/src/pages/sites/ActivityFeed.tsx`

## Result

No blocking bugs or obvious security regressions found in the phase 31 source changes during executor review.

## Notes

- Tenant/site scoping is enforced when linking uploaded attachments to newly created activities.
- The legacy camera upload path is preserved separately from the new generic document upload endpoint.
- Protected image previews still flow through authenticated `apiClient.getBlob` fetches.
