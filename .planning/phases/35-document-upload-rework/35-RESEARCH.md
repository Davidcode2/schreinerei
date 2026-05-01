# Phase 35: Document Upload Rework — Research

**Phase:** 31 — Document Upload Rework  
**Date:** 2026-05-01  
**Discovery Level:** 0 (existing stack/patterns, no new dependencies)

## Phase Boundary

Rework the existing document modal so users can create a single activity entry with any combination of:

- note only
- note + image attachment(s)
- note + PDF attachment(s)
- attachment(s) only

This phase is not just a frontend modal tweak. The current backend/frontend contracts only support either a plain note or a single uploaded photo URL, so Phase 35 requires a coordinated activity + attachment contract update.

## Standard Stack

- **Frontend:** Vite, React, TypeScript, Tailwind, shadcn/ui
- **Backend:** Rust, Axum 0.8, SQLx 0.8, PostgreSQL
- **Auth:** Keycloak tenant-scoped request context
- **Testing:** Vitest on frontend, Rust unit tests on backend
- **Established upload pattern:** upload-first, activity-second with authenticated blob fetches

## Current Implementation Reality

### Frontend constraints

| File | Current behavior | Phase 35 impact |
|------|------------------|-----------------|
| `frontend/src/pages/sites/CreateNoteModal.tsx` | Two mutually exclusive modes (`note` or `photo`) | Cannot submit note + attachment together; cannot submit PDF; cannot submit multiple files |
| `frontend/src/lib/api/hooks/useSites.ts` | `useUploadSitePhoto()` accepts one `File` and posts multipart field `photo` | No generic attachment upload hook |
| `frontend/src/types/generated.ts` / `frontend/src/types/sites.ts` | Activity DTO has `content` + single `photo_url` only | No attachment array in frontend contract |
| `frontend/src/pages/sites/ActivityFeed.tsx` | Renders one `photo_url` image preview per activity | No document cards, no multiple attachments, no PDF preview affordance |

### Backend constraints

| File | Current behavior | Phase 35 impact |
|------|------------------|-----------------|
| `src/modules/sites/domain/activity.rs` | `CreateActivity::validate()` requires note content for `note`, requires `photo_url` for `photo` | Attachment-only entries and note+attachment note entries are impossible today |
| `src/modules/sites/api/routes.rs` | `POST /api/v1/sites/{id}/attachments/photo` only reads multipart field `photo` | PDF uploads are impossible today |
| `src/modules/sites/application/site_service.rs` | Upload validation allowlist is only `image/jpeg`, `image/png`, `image/webp` | Must be generalized for PDF attachments |
| `src/modules/sites/infrastructure/site_repository.rs` | `site_activities` persists only one `photo_url` field; `list_activities()` returns no attachment join data | Feed cannot render multiple attachments from DB |

### Attachment table constraint (critical)

`migrations/012_site_activity_attachments.sql` still contains:

```sql
UNIQUE (tenant_id, activity_id)
```

This means the existing schema only allows **one attachment per activity**. Phase 35 requirement DOC-02/DOC-03 says “one or more PDFs/images”, so a migration is required to remove this uniqueness constraint before multiple attachments can work.

## Established Patterns To Reuse

1. **Upload-first, activity-second**  
   Phase 29/30 already proved the right sequencing: upload binary first, receive attachment URLs/IDs, then create the activity entry.

2. **Authenticated attachment reads**  
   `ActivityFeed.tsx` loads protected `/api/v1/attachments/{id}` URLs through `apiClient.getBlob()` and object URLs. Future image/document previews should keep this pattern.

3. **Tenant-scoped attachment persistence**  
   `site_repository.rs` already stores attachments with `tenant_id`, `site_id`, nullable `activity_id`, and `mime_type`. That makes the existing table a good base for document attachments after the one-to-many constraint is fixed.

4. **No hidden activity creation inside upload**  
   Phase 29 gap closure removed implicit activity creation from upload routes. Keep that invariant: uploads store bytes only, explicit create-activity call creates the business event.

## Recommended Architecture

### 1. Generalize attachments instead of extending `photo_url`

Do **not** try to encode Phase 35 as more clever usage of `photo_url`. The feed now needs a real attachment collection, not a single URL string.

Recommended direction:

- keep camera flow (`activity_type: photo`) working as-is for Phase 34
- extend activity read/create contracts so document-modal entries can carry **attachment IDs / attachment metadata arrays**
- let the feed render attachments from that array instead of from one `photo_url`

### 2. Add multi-attachment linking to activity creation

The current flow can already upload unattached binaries because `site_activity_attachments.activity_id` is nullable. Phase 35 should use that intentionally:

1. user uploads one or more files
2. backend stores unattached rows
3. create-activity request includes uploaded attachment IDs plus optional note content
4. backend creates the activity row
5. backend links each uploaded attachment row to the new activity ID

This avoids hidden upload-side activity creation and preserves the Phase 29 contract.

### 3. Relax note validation to allow “content OR attachments”

Current `CreateActivity` validation is too strict for DOC-01..DOC-04.

Required invariant for document-modal entries:

- valid if `trimmed content` exists
- valid if `attachment_ids.len() > 0`
- invalid only if **both** are missing

### 4. Generalize upload endpoint beyond images

Current route/service only support image MIME types and thumbnail generation. Phase 35 needs at least:

- `image/jpeg`
- `image/png`
- `image/webp`
- `application/pdf`

Implication:

- image attachments can keep thumbnail generation
- PDF attachments need storage without image thumbnail generation
- upload response should expose enough metadata for the UI to distinguish image vs PDF

### 5. Expand activity read DTOs

Phase 35 and Phase 36 both need richer feed data. Returning only `photo_url` is no longer enough.

Minimum contract expansion needed for planning:

- activity-level attachment list
- attachment id
- mime type
- original/read URL
- thumbnail URL when available

## Likely File Impact

### Backend

- `migrations/*` — remove one-attachment uniqueness and possibly add attachment metadata columns if needed
- `src/modules/sites/domain/activity.rs` — extend command/entity validation for attachment-backed entries
- `src/modules/sites/application/site_service.rs` — generic upload validation/linking flow
- `src/modules/sites/infrastructure/site_repository.rs` — batch-link attachments to activity and read activity attachment collections
- `src/modules/sites/api/routes.rs` — generic attachment upload/create activity DTO updates

### Frontend

- `frontend/src/pages/sites/CreateNoteModal.tsx` — modal becomes note + attachments composer, not note/photo toggle
- `frontend/src/pages/sites/CreateNoteModal.test.tsx` — cover all valid combinations and validation failures
- `frontend/src/lib/api/hooks/useSites.ts` — add generic upload hook / extend create-activity request
- `frontend/src/lib/api/hooks/useSites.test.tsx` — verify multipart field names and response handling
- `frontend/src/types/generated.ts` — regenerated DTOs after backend contract changes
- `frontend/src/types/sites.ts` — local site activity attachment types
- `frontend/src/pages/sites/ActivityFeed.tsx` — render attachment previews/cards instead of only one `photo_url`

## Common Pitfalls

1. **Do not keep the current one-file-per-activity uniqueness constraint** — it directly violates DOC-02 and DOC-03.
2. **Do not overload the camera flow into this modal again** — Phase 34 intentionally split camera UX from document UX.
3. **Do not lose authenticated blob fetch behavior** — PDFs/images should not regress to unauthenticated URLs.
4. **Do not make attachment-only entries depend on fake note text** — DOC-04 requires no note text.
5. **Do not keep feed rendering tied only to `photo_url`** — multiple attachments require a first-class attachment array.

## Architectural Responsibility Map

| Layer | Expected change | Risk |
|-------|------------------|------|
| Presentation | Rebuild document modal into mixed note+attachments composer | Medium |
| Presentation | Update activity feed to render attachment collections | Medium |
| Application | Generalize upload + create activity orchestration | Medium |
| Infrastructure | Remove one-to-one attachment constraint; add attachment linking/queries | High |
| Domain | Update validation rules for attachment-backed note entries | Medium |

## Planning Guidance

This phase should be planned as a coordinated backend+frontend change, not as “just modal UI”. The highest-risk dependency is the backend contract shift from single `photo_url` to real attachment collections. Once that contract exists, frontend modal and feed work can proceed deterministically.
