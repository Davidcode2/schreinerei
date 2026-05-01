---
phase: 29-photo-upload-attachments
verified: 2026-05-01T10:12:14Z
status: gaps_found
score: 3/6 must-haves verified
overrides_applied: 0
gaps:
  - truth: "User can upload photos via camera capture or gallery selection"
    status: failed
    reason: "Frontend sends multipart field 'file' while backend requires field name 'photo'; upload endpoint rejects payload."
    artifacts:
      - path: "frontend/src/lib/api/hooks/useSites.ts"
        issue: "FormData uses formData.append(\"file\", file)"
      - path: "src/modules/sites/api/routes.rs"
        issue: "Multipart handler only accepts field.name() == Some(\"photo\")"
    missing:
      - "Align multipart contract (same field name in frontend and backend)."
  - truth: "Image previews appear in activity feed after upload"
    status: failed
    reason: "Upload flow is broken by multipart mismatch, so the upload→activity→preview path is not achievable."
    artifacts:
      - path: "frontend/src/pages/sites/CreateNoteModal.tsx"
        issue: "Depends on failed upload mutation for photo path"
      - path: "frontend/src/pages/sites/ActivityFeed.tsx"
        issue: "Preview render exists but relies on successful upload-created activity photo_url"
    missing:
      - "Fix upload contract so photo activities with photo_url are actually created from UI uploads."
  - truth: "Offline photo capture is queued and syncs when connection restored"
    status: failed
    reason: "Queue replay also uses multipart field 'file'; reconnect sync calls same endpoint contract and will fail similarly."
    artifacts:
      - path: "frontend/src/lib/offline/queue.ts"
        issue: "Replay path sends formData.append('file', file)"
      - path: "src/modules/sites/api/routes.rs"
        issue: "Upload endpoint requires multipart field 'photo'"
    missing:
      - "Use matching multipart field name in offline replay upload path."
      - "Re-verify queued photo actions succeed on reconnect."
---

# Phase 29: Photo Upload & Attachments Verification Report

**Phase Goal:** Users can capture and attach photos to activities with secure storage and offline support.
**Verified:** 2026-05-01T10:12:14Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | User can upload photos via camera capture or gallery selection | ✗ FAILED (BLOCKER) | `useSites.ts` appends `"file"` (line 222), backend accepts only `"photo"` (`routes.rs` lines 706-724). |
| 2 | Image previews appear in activity feed after upload | ✗ FAILED (BLOCKER) | `ActivityFeed.tsx` renders `activity.photo_url` (`img src`, lines 102-109), but upload path that should create these entries is broken by multipart mismatch. |
| 3 | Photos are stored securely with UUID filenames (no user-provided names) | ✓ VERIFIED | `site_service.rs` generates server-side UUID keys (`original_key`, `thumbnail_key`, lines 464-465); no client filename used for storage key. |
| 4 | Thumbnails are generated automatically on upload for fast loading | ✓ VERIFIED | `generate_thumbnail_bytes` in `site_service.rs` (lines 65-82) called in upload flow (line 466), thumbnail bytes persisted in repository (`site_repository.rs` lines 572-603). |
| 5 | Offline photo capture is queued and syncs when connection restored | ✗ FAILED (BLOCKER) | Offline queue is implemented (`queuePhotoUploadAction` + `syncPendingActions`), but replay upload uses `"file"` field (`queue.ts` line 49) against backend `"photo"` requirement. |
| 6 | File access is authorized by tenant_id preventing cross-tenant leakage | ✓ VERIFIED | Attachment reads use tenant derived from auth (`TenantContext::from_auth`) and repository query filters `WHERE id = $1 AND tenant_id = $2` (`site_repository.rs` lines 635-655). |

**Score:** 3/6 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `migrations/012_site_activity_attachments.sql` | Tenant-scoped attachments table and indexes | ✓ VERIFIED | Table includes `tenant_id`, FK to `site_activities`, and indexes. |
| `src/modules/sites/infrastructure/site_repository.rs` | Tenant-scoped persistence + lookup | ✓ VERIFIED | `create_activity_attachment` and `find_attachment_by_id(attachment_id, tenant_id)` implemented. |
| `src/modules/sites/api/routes.rs` | Authorized attachment read + upload endpoints | ⚠️ PARTIAL | Endpoints exist and tenant auth enforced; multipart contract incompatible with frontend (`photo` vs `file`). |
| `src/modules/sites/application/site_service.rs` | Upload orchestration with validation + thumbnails | ✓ VERIFIED | MIME/size validation, UUID keys, thumbnail generation, attachment persistence. |
| `frontend/src/lib/api/hooks/useSites.ts` | Upload mutation hook | ⚠️ HOLLOW | Hook exists and wired, but posts wrong multipart field name for backend contract. |
| `frontend/src/lib/offline/queue.ts` | Photo-upload queue handler | ⚠️ HOLLOW | Queue/replay implemented, but replay uses wrong multipart field name so sync cannot complete. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `routes.rs` | `site_repository.rs` | attachment_id + tenant_id lookup | ✓ WIRED | `get_attachment_*` calls `find_attachment_by_id(attachment_uuid, ctx.tenant_id)`. |
| `migrations/012_site_activity_attachments.sql` | `site_activities` | activity_id foreign key | ✓ WIRED | `activity_id UUID NOT NULL REFERENCES site_activities(id)` present. |
| `routes.rs` | `site_service.rs` | multipart handler invokes upload service | ✓ WIRED | `upload_site_photo_attachment` calls `service.upload_photo_attachment(...)`. |
| `site_service.rs` | `site_repository.rs` | persist attachment metadata + activity linkage | ✓ WIRED | Calls `create_activity_attachment` and `update_activity_photo_url`. |
| `CreateNoteModal.tsx` | `useSites.ts` | photo upload mutation then create activity | ✓ WIRED | Uses `uploadPhoto.mutateAsync` then `createActivity.mutateAsync`. |
| `CreateNoteModal.tsx` | `offline/queue.ts` | enqueue when offline | ✓ WIRED | Calls `queuePhotoUploadAction(...)` in offline branch. |
| `offline/sync.ts` | `offline/queue.ts` | process on online event | ✓ WIRED | `initSync` online listener calls `fullSync` → `syncPendingActions` → `processAction`. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `ActivityFeed.tsx` | `activity.photo_url` | API activities response populated by upload flow | No (upload path blocked by multipart mismatch) | ✗ DISCONNECTED |
| `get_attachment_bytes` route | `attachment.original_bytes` | `find_attachment_by_id(...tenant_id...)` DB lookup | Yes | ✓ FLOWING |
| `offline/queue.ts` replay | `uploadResponse.photo_url` | `POST /attachments/photo` | No (request shape mismatch) | ✗ DISCONNECTED |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Upload field-name contract consistency | static code check | Frontend uses `file`; backend requires `photo` | ✗ FAIL |
| Offline replay contract consistency | static code check | Queue replay uses `file`; backend requires `photo` | ✗ FAIL |
| Runtime command checks | N/A | Skipped (would require app/test runtime setup not guaranteed in verification context) | ? SKIP |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| FILE-01 | 29-02, 29-03 | User can upload photos via camera or gallery | ✗ BLOCKED | Frontend/backend multipart field mismatch blocks upload path. |
| FILE-02 | 29-03 | Image preview shown in activity feed | ✗ BLOCKED | Preview UI exists, but upload path to produce previewable entries is broken. |
| FILE-03 | 29-01, 29-02 | Photos stored with UUID filenames | ✓ SATISFIED | UUID key generation in service; storage keys internal. |
| FILE-04 | 29-02 | Image thumbnails generated on upload | ✓ SATISFIED | Thumbnail generation and persistence implemented. |
| FILE-05 | 29-04 | Offline photo capture queued | ✓ SATISFIED (artifact) / ✗ BLOCKED (outcome) | Queue persistence exists; but end-to-end goal depends on successful replay. |
| FILE-06 | 29-04 | Sync queue processes uploads when online | ✗ BLOCKED | Replayed upload payload uses wrong field name; sync retries fail. |
| FILE-07 | 29-01, 29-02 | File access authorized by tenant_id | ✓ SATISFIED | Tenant extracted from auth, repository filters by tenant_id. |

Orphaned requirements for Phase 29 from `REQUIREMENTS.md`: **None** (all FILE-01..FILE-07 are referenced by phase plans).

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `frontend/src/lib/api/hooks/useSites.ts` | 222 | Multipart field mismatch (`file` vs backend `photo`) | 🛑 Blocker | Upload endpoint rejects payload; core phase goal broken. |
| `frontend/src/lib/offline/queue.ts` | 49 | Multipart field mismatch in replay path | 🛑 Blocker | Offline sync cannot complete queued photo uploads. |

### Gaps Summary

Phase tasks were completed, but core outcome is not achieved end-to-end. Upload and offline replay are both blocked by a request-contract mismatch between frontend/offline clients and backend multipart parsing. Security and storage internals are implemented correctly, but user-facing photo upload and reconnect sync behaviors fail the roadmap contract.

---

_Verified: 2026-05-01T10:12:14Z_
_Verifier: OpenCode (gsd-verifier)_
