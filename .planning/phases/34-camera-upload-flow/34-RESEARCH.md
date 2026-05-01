# Phase 34: Camera Upload Flow — Research

**Phase:** 30 — Camera Upload Flow  
**Date:** 2026-05-01  
**Discovery Level:** 0 (Established patterns, no new dependencies)

## Phase Boundary

Separate camera button from document modal with optional note. This phase creates a new CameraUploadFlow component that bypasses the existing CreateNoteModal when the Camera icon is clicked, allowing direct native camera/gallery picker access and optional note text.

## Standard Stack

- **Frontend:** Vite 6, React 18, TypeScript, Tailwind CSS 4, shadcn/ui
- **Backend:** Rust, Axum 0.8, SQLx 0.8 (no backend changes needed this phase)
- **Offline:** Workbox, Dexie.js (existing queue handles photo+note)
- **Testing:** Vitest, MSW for frontend tests
- **Auth:** Keycloak OAuth2 PKCE (existing)

## Architecture Patterns

### Current Photo Upload Flow (v1.8)
1. User clicks Camera button on SiteDetailPage
2. SiteDetailPage sets `noteModalActivityType="photo"` and opens CreateNoteModal
3. CreateNoteModal shows "Notiz" and "Foto" tabs
4. When "Foto" tab is active, shows `<input type="file" accept="image/*" capture="environment">`
5. On submit: if offline → `queuePhotoUploadAction()`, if online → `useUploadSitePhoto()` then `useCreateActivity()`
6. Two-step: POST `/sites/{id}/attachments/photo` (multipart) → get `photo_url` → POST `/sites/{id}/activities` with `photo_url`

### Key Files
| File | Role |
|------|------|
| `frontend/src/pages/sites/SiteDetailPage.tsx` | Wires Camera/FileText buttons to modals |
| `frontend/src/pages/sites/CreateNoteModal.tsx` | Current combined note+photo modal (v1.8 camera-first) |
| `frontend/src/pages/sites/ActivityFeed.tsx` | Renders activity cards with photo previews |
| `frontend/src/lib/api/hooks/useSites.ts` | `useCreateActivity()`, `useUploadSitePhoto()` hooks |
| `frontend/src/lib/api/client.ts` | `apiClient` with `getBlob()` for authenticated image fetch |
| `frontend/src/lib/offline/queue.ts` | `queuePhotoUploadAction()` for offline photo uploads |
| `frontend/src/types/sites.ts` | TypeScript types for Activity, CreateActivityRequest |
| `src/modules/sites/domain/activity.rs` | Backend ActivityType enum, CreateActivity validation |
| `src/modules/sites/api/routes.rs` | Backend routes for photo upload, activity creation |

### Backend API (No changes needed)
- `POST /api/v1/sites/{id}/attachments/photo` — multipart upload, returns `{attachment_id, photo_url, thumbnail_url}`
- `POST /api/v1/sites/{id}/activities` — create activity with `{activity_type, content?, photo_url?}`
- `GET /api/v1/attachments/{id}` — authenticated blob fetch
- `GET /api/v1/attachments/{id}/thumbnail` — authenticated thumbnail fetch

**Critical backend detail:** `CreateActivity` validation allows Photo type with `content` field. Currently the frontend CreateNoteModal doesn't expose this for photo uploads, but the backend supports it natively.

### Offline Queue Support
`queuePhotoUploadAction({ siteId, file, content? })` already serializes to IndexedDB with content field. The offline sync handler processes photo+content correctly.

## What Changes (Phase 34)

### Frontend Only
1. **New `CameraUploadFlow` component** — Direct camera/gallery picker with optional note field
2. **SiteDetailPage wiring** — Camera button opens CameraUploadFlow, NOT CreateNoteModal
3. **ActivityFeed display** — Already handles photo activities with content text

### No Backend Changes
The existing API routes fully support what Phase 34 needs:
- Photo upload endpoint ✅
- Activity creation with `content` + `photo_url` ✅
- Authenticated blob fetch ✅
- Offline queue with content support ✅

## Common Pitfalls

1. **Don't modify CreateNoteModal** — It will be reworked in Phase 35 (Document Upload Rework). Keep it as-is.
2. **Maintain offline support** — The new CameraUploadFlow must use the same `queuePhotoUploadAction()` for offline
3. **Don't change backend validation** — Photo+note is already supported, just not exposed in frontend
4. **Use same upload flow** — Upload photo first → get URL → create activity with URL + content

## Architectural Responsibility Map

| Layer | Change | Risk |
|-------|--------|------|
| Presentation | New CameraUploadFlow component | Low — standard React component |
| Presentation | SiteDetailPage wiring change | Low — swap import/call |
| Presentation | No changes needed in ActivityFeed | None — already handles photo+content |
| Application | No changes | None |
| Infrastructure | No changes | None |
| Domain | No changes | None |