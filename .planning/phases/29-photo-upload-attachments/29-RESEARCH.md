# Phase 29 Research: Photo Upload & Attachments

**Phase:** 29 — Photo Upload & Attachments  
**Date:** 2026-05-01  
**Scope:** FILE-01..FILE-07

## What exists already

- Activity feed already supports `activity_type: photo` and displays `photo_url` in `ActivityFeed.tsx`.
- Backend `CreateActivity`/`ActivityResponse` already carries optional `photo_url`.
- Tenant isolation is enforced in services/repositories via `TenantContext` and tenant-scoped SQL patterns.
- Offline queue exists (Dexie + `pendingActions`) and already syncs `activity` actions when coming back online.

## Gaps to close for this phase

1. No binary upload endpoint (currently only URL string can be posted).
2. No storage layout that binds file ownership to tenant/site.
3. No thumbnail generation pipeline.
4. No secure file-serving endpoint with tenant authorization checks.
5. Offline queue does not persist photo blobs + metadata for deferred upload.
6. Frontend has no camera/gallery picker UX for activity creation.

## Recommended architecture

### Storage model

- Add `site_activity_attachments` table (tenant-scoped):
  - `id UUID PK`
  - `tenant_id UUID NOT NULL`
  - `activity_id UUID NOT NULL REFERENCES site_activities(id)`
  - `site_id UUID NOT NULL`
  - `storage_key TEXT NOT NULL` (UUID-based filename)
  - `thumbnail_key TEXT NOT NULL`
  - `mime_type TEXT NOT NULL`
  - `size_bytes BIGINT NOT NULL`
  - `created_at TIMESTAMPTZ NOT NULL`
- Keep `site_activities.photo_url` for backward compatibility, but populate it from generated secure URL shape.

### Upload API split

1. `POST /api/v1/sites/{id}/attachments/photo` (multipart/form-data)
   - Validates tenant/site scope from auth context.
   - Stores original image under UUID key.
   - Generates thumbnail under separate UUID key.
   - Returns attachment payload (`attachment_id`, `photo_url`, `thumbnail_url`).
2. `GET /api/v1/attachments/{attachment_id}` and `/thumbnail`
   - Resolves attachment by `attachment_id + tenant_id`.
   - Streams bytes only when tenant matches.

### Thumbnail generation

- Use Rust `image` crate for deterministic thumbnail generation server-side.
- Standardize thumbnail size (e.g., max 320px width, preserve aspect ratio).

### Offline flow

- Extend Dexie queue entry for photo uploads to store:
  - `siteId`, `caption/content`, `blob`, `mimeType`, local preview URL reference.
- Sync engine uploads binary first, then creates activity record with returned attachment URL.
- Retry behavior stays consistent with existing `MAX_RETRIES` logic.

## Security considerations (must enforce)

- UUID filenames only; never trust or persist client filename (`FILE-03`).
- Enforce MIME allowlist (jpeg/png/webp) and file size cap at API boundary.
- Attachment retrieval must include tenant-scoped lookup (`FILE-07`).
- Parameterized SQL only; no path concatenation from user input.

## Existing code touchpoints

- Backend routes: `src/modules/sites/api/routes.rs`
- Backend activity domain/service/repo:
  - `src/modules/sites/domain/activity.rs`
  - `src/modules/sites/application/site_service.rs`
  - `src/modules/sites/infrastructure/site_repository.rs`
- Frontend activity UI/types:
  - `frontend/src/pages/sites/ActivityFeed.tsx`
  - `frontend/src/types/sites.ts`
  - `frontend/src/lib/api/hooks/useSites.ts`
- Offline queue:
  - `frontend/src/lib/offline/db.ts`
  - `frontend/src/lib/offline/queue.ts`
  - `frontend/src/lib/offline/sync.ts`

## Risks / pitfalls

- Large blob handling in IndexedDB: must avoid unbounded growth (prune after successful sync).
- Thumbnail CPU load on API server: cap resolution + size before processing.
- Broken previews if feed still reads legacy `photo_url` only: migrate response shape carefully.

## Recommendation for planning

- Split into 4 plans:
  1. Storage + secure backend contracts
  2. Upload + thumbnail pipeline
  3. Frontend capture/gallery + preview integration
  4. Offline queueing + deferred sync pipeline

- Keep plan dependencies mostly sequential for backend correctness, with frontend plan starting once upload contract exists.
