# Phase 32: Media Viewer — Research

**Phase:** 32 — Media Viewer  
**Date:** 2026-05-01  
**Discovery Level:** 0 (existing stack/patterns, no new dependencies)

## Phase Boundary

Add a fullscreen media viewer on top of the existing Baustelle activity feed so users can:

- open image previews from camera uploads in a large viewer
- open document attachments from note/document entries in the same viewer flow
- deep-link directly to a viewer state with a shareable URL
- see note text, creator identity, timestamp, download action, and copy-link action beside the media

This phase builds on the existing upload-first attachment pipeline from Phases 29–31. It should not change upload behavior, attachment storage, or unauthenticated access rules.

## Standard Stack

- **Frontend:** Vite, React 18, TypeScript, React Router, Tailwind, shadcn/ui
- **Backend:** Rust, Axum 0.8, SQLx 0.8, PostgreSQL
- **Auth:** Keycloak tenant-scoped request context
- **Testing:** Vitest frontend tests, Rust tests where DTO/service logic changes
- **Established binary-content pattern:** authenticated blob fetch via `apiClient.getBlob()` and object URLs

## Current Implementation Reality

### Frontend constraints

| File | Current behavior | Phase 32 impact |
|------|------------------|-----------------|
| `frontend/src/pages/sites/ActivityFeed.tsx` | Renders attachment tiles and legacy photo preview tiles, but nothing is clickable and no viewer state exists | Must become the entrypoint that opens viewer for both camera photos and document attachments |
| `frontend/src/pages/sites/SiteDetailPage.tsx` | Owns activity feed and modal state for note/camera/status dialogs only | Needs viewer state + route synchronization |
| `frontend/src/App.tsx` | Only site detail route is `/sites/:id` | Needs a shareable viewer route layered onto the existing site detail page |
| `frontend/src/types/sites.ts` | `Activity` contains `user_id`, `content`, `photo_url`, `attachments`, `created_at` | Missing creator display name and viewer-specific attachment identifiers/slugs |
| `frontend/src/pages/settings/UserManagementSection.tsx` | Existing clipboard copy pattern with toast feedback | Can be reused for “copy share link” UX |

### Backend constraints

| File | Current behavior | Phase 32 impact |
|------|------------------|-----------------|
| `src/modules/sites/api/routes.rs` | `ActivityResponse` returns `user_id`, but no creator display name and no viewer slug | DTO must grow to satisfy VIEW-03 and VIEW-05 |
| `src/modules/sites/application/site_service.rs` | Lists activities with hydrated attachments only | Needs to keep attachment hydration while exposing viewer metadata without breaking tenant isolation |
| `src/modules/sites/infrastructure/site_repository.rs` | `list_activities()` selects only from `site_activities`; no join to users, no slug column | Needs richer query or additional lookup for creator name and direct-link identifier |
| `GET /api/v1/attachments/{attachment_id}` | Streams protected bytes with tenant-scoped lookup | Download should reuse this path, not create public URLs |

## Gaps Against Phase Requirements

1. **VIEW-01 / VIEW-02:** Feed tiles are not interactive; no fullscreen viewer exists.
2. **VIEW-03:** There is no slug/direct-link field or route for a viewer state.
3. **VIEW-04:** Viewer sidebar layout does not exist.
4. **VIEW-05:** Activity responses do not expose creator display names.
5. **VIEW-06:** No dedicated download affordance exists in the feed/viewer.
6. **VIEW-07:** No clipboard share-link flow exists for activities/media.
7. **VIEW-08 / VIEW-09:** No large modal/page shell with route-aware close behavior exists.

## Established Patterns To Reuse

1. **Authenticated blob fetch for protected media**  
   `ActivityFeed.tsx` already resolves protected image attachments through `apiClient.getBlob()` and object URLs. The viewer should reuse this instead of falling back to raw `<img src="/api/v1/attachments/...">`.

2. **Upload-first, activity-second attachment model**  
   Phase 29/31 intentionally separate storage from business events. Phase 32 should remain read-only on top of those contracts.

3. **Dedicated camera vs document flows**  
   Phase 30 split camera UX from document composer UX. Viewer work must not reintroduce upload-mode coupling.

4. **React Router page ownership**  
   `App.tsx` owns route definitions; `SiteDetailPage.tsx` owns local feature dialogs. A route-backed viewer should keep this boundary clear instead of burying navigation logic inside tile components.

5. **Clipboard toast pattern**  
   `navigator.clipboard.writeText(...)` with success/error toast already exists in settings UI and can be mirrored for share links.

## Recommended Architecture

### 1. Use a route-backed viewer state, not purely local modal state

Recommended route shape:

- `/sites/:id/media/:activityId/:attachmentId`

Why:

- gives a stable shareable URL without introducing a brand-new top-level page
- keeps the viewer scoped to a specific Baustelle detail context
- works for both image attachments and legacy camera-photo activities once each viewerable item has a concrete attachment identity

For legacy photo activities created before attachment arrays existed, the frontend can continue deriving a synthetic display tile for feed rendering, but the direct-link route needs a real stable identifier. Research implication: the backend should expose a canonical viewer target for every viewerable item, not force the frontend to invent one from `photo_url`.

### 2. Prefer attachment-backed viewer targets over raw `photo_url`

Document entries already have `attachments[]` with `attachment_id`, `mime_type`, URLs, and thumbnails. The viewer contract should normalize camera-photo and document-attachment items to the same conceptual model:

- `activity` metadata
- `viewer item` identifier / slug
- file metadata (`filename`, `mime_type`, protected URL, thumbnail URL if image)

That avoids bifurcated viewer logic where photos and documents have different routing rules.

### 3. Expand activity read DTOs with creator and viewer-link metadata

Minimum additional contract needed for planning:

- `creator_name` (or equivalent display field)
- per-viewer-item stable link token/slug
- enough metadata to determine download filename and viewer title

The repository currently stores only `user_id` in `site_activities`, so the read path likely needs a tenant-scoped join to local users for display name resolution.

### 4. Model close behavior as navigation, not boolean state only

Close button behavior should return the user to `/sites/:id` by navigation, so deep-linked viewers can be opened directly and closed predictably. This is safer than a purely local dialog flag because it preserves browser back/forward semantics.

### 5. Download should reuse protected attachment endpoints

Do not create public share/download URLs. The viewer’s download button should fetch the protected attachment bytes (authenticated) and trigger a browser download with the original filename. Sharing copies the app URL, not a direct blob-storage URL.

## Likely File Impact

### Frontend

- `frontend/src/App.tsx` — add media-viewer route that still renders inside the authenticated site flow
- `frontend/src/pages/sites/SiteDetailPage.tsx` — derive viewer-open state from route params and wire close navigation
- `frontend/src/pages/sites/ActivityFeed.tsx` — make image/PDF tiles clickable and route into viewer
- `frontend/src/pages/sites/ActivityFeed.test.tsx` — cover tile click behavior and viewer-link generation
- `frontend/src/types/sites.ts` — add creator display name and viewer-link metadata to local types
- **Likely new file:** `frontend/src/pages/sites/MediaViewer.tsx` (or similarly named component) — fullscreen shell, metadata sidebar, download/share/close actions
- **Likely new test:** `frontend/src/pages/sites/MediaViewer.test.tsx`

### Backend

- `src/modules/sites/api/routes.rs` — extend activity response contracts
- `src/modules/sites/application/site_service.rs` — preserve hydration while shaping richer viewer data
- `src/modules/sites/infrastructure/site_repository.rs` — join creator display info and/or persist/read viewer slug metadata
- `frontend/src/types/generated.ts` — regenerated after DTO changes

## Common Pitfalls

1. **Do not share unauthenticated blob URLs** — sharing should copy the app route, not storage access paths.
2. **Do not implement viewer state as local-only modal state** — VIEW-03 requires direct linking.
3. **Do not keep creator display as raw `user_id`** — VIEW-05 requires a human-readable user identity.
4. **Do not special-case PDFs into a separate page flow** — requirement says one fullscreen viewer experience for media attachments.
5. **Do not break Phase 30 legacy photo rendering** — camera-upload activities must still open correctly.
6. **Do not regress tenant scoping** — all activity/media lookups must remain tenant-bound.

## Architectural Responsibility Map

| Layer | Expected change | Risk |
|-------|------------------|------|
| Presentation | Clickable feed tiles + fullscreen viewer shell + route-driven close/share/download actions | Medium |
| Presentation | Route/state synchronization between site detail page and viewer | Medium |
| Application | Shape richer activity read model for viewer metadata | Medium |
| Infrastructure | Join creator identity and expose stable viewerable media identifiers | High |
| Domain | Likely none or minimal unless slug becomes a first-class domain field | Low |

## Planning Guidance

This phase should be planned as one backend contract foundation plan plus one or two frontend integration plans. The critical dependency is exposing a stable viewer contract for both document attachments and legacy camera-photo activities. Once the DTO carries creator metadata and stable viewer targets, the frontend route/viewer wiring becomes deterministic.
