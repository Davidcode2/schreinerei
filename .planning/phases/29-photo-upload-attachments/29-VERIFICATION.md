---
phase: 29-photo-upload-attachments
verified: 2026-05-01T10:37:52Z
status: human_needed
score: 6/6 must-haves verified
overrides_applied: 0
re_verification:
  previous_status: gaps_found
  previous_score: 3/6
  gaps_closed:
    - "User can upload photos via camera capture or gallery selection"
    - "Image previews appear in activity feed after upload"
    - "Offline photo capture is queued and syncs when connection restored"
  gaps_remaining: []
  regressions: []
human_verification:
  - test: "Mobile camera/gallery upload from Site detail flow"
    expected: "Camera icon opens photo-first modal; selecting/taking a photo uploads successfully and appears in activity feed preview"
    why_human: "Requires real device/browser camera permissions and full UI interaction"
  - test: "Offline queue replay after reconnect"
    expected: "When offline, photo action is queued; after reconnect, upload + activity creation complete and pending count decreases"
    why_human: "Requires network state transitions and end-to-end runtime behavior"
---

# Phase 29: Photo Upload & Attachments Verification Report

**Phase Goal:** Users can capture and attach photos to activities with secure storage and offline support.
**Verified:** 2026-05-01T10:37:52Z
**Status:** human_needed
**Re-verification:** Yes — after gap closure

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | User can upload photos via camera capture or gallery selection | ✓ VERIFIED | `useSites.ts` now sends `formData.append("photo", file)`; backend `upload_site_photo_attachment` requires `field.name()=="photo"` (`routes.rs` 706-724). Regression test asserts `photo` key and `file` absent (`useSites.test.tsx` 33-55). |
| 2 | Image previews appear in activity feed after upload | ✓ VERIFIED | Upload flow returns `photo_url` and `CreateNoteModal.tsx` passes it into `createActivity` (lines 77-89). `ActivityFeed.tsx` renders `<img src={activity.photo_url}>` (102-109). Backend stores and serves `photo_url` via `update_activity_photo_url` + `list_activities` select (site_repository.rs 606-617, 555-559). |
| 3 | Photos are stored securely with UUID filenames (no user-provided names) | ✓ VERIFIED | Service generates `original_key` and `thumbnail_key` from UUIDs (`site_service.rs` 464-465), independent of user filename. |
| 4 | Thumbnails are generated automatically on upload for fast loading | ✓ VERIFIED | `generate_thumbnail_bytes` called during upload flow (`site_service.rs` 466) and persisted as attachment thumbnail bytes (`site_repository.rs` 579-586). |
| 5 | Offline photo capture is queued and syncs when connection restored | ✓ VERIFIED | Offline replay now uses `formData.append('photo', file)` and posts to `/attachments/photo` before creating activity (`queue.ts` 44-59). Sync wiring: `sync.ts` imports `processAction`, runs `syncPendingActions`, and listens to `online` event (lines 2, 55, 65, 108). Queue test verifies replay uses `photo` key and creates activity (`queue.test.ts` 93-109). |
| 6 | File access is authorized by tenant_id preventing cross-tenant leakage | ✓ VERIFIED | Attachment read query filters `WHERE id = $1 AND tenant_id = $2` (`site_repository.rs` 645-646), route resolves tenant from auth context before lookup (`routes.rs` 769, 775-777). |

**Score:** 6/6 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `frontend/src/lib/api/hooks/useSites.ts` | Online upload uses backend-compatible multipart key | ✓ VERIFIED | `formData.append("photo", file)` present; no `append("file", file)` usage. |
| `frontend/src/lib/offline/queue.ts` | Offline replay uses same multipart key | ✓ VERIFIED | `formData.append('photo', file)` for `photo_upload` handler. |
| `frontend/src/pages/sites/SiteDetailPage.tsx` | Camera entrypoint opens functional photo modal path | ✓ VERIFIED | Camera button `onClick={openPhotoModal}`; `openPhotoModal` sets photo mode + opens modal. |
| `src/modules/sites/api/routes.rs` | Upload endpoint enforces canonical multipart field | ✓ VERIFIED | Only accepts multipart field `photo`; explicit validation error otherwise. |
| `src/modules/sites/application/site_service.rs` | Secure storage + thumbnail generation | ✓ VERIFIED | UUID keys + thumbnail generation wired in upload flow. |
| `src/modules/sites/infrastructure/site_repository.rs` | Tenant-scoped activity + attachment persistence and reads | ✓ VERIFIED | Tenant-scoped `list_activities`, `update_activity_photo_url`, and `find_attachment_by_id`. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `CreateNoteModal.tsx` | `useSites.ts` | `useUploadSitePhoto` mutateAsync | ✓ WIRED | Imports hook, instantiates `uploadPhoto`, calls `uploadPhoto.mutateAsync(...)` in submit flow. |
| `queue.ts` | `/api/v1/sites/{siteId}/attachments/photo` | `photo_upload` replay handler | ✓ WIRED | Replay posts FormData to attachment upload endpoint then creates activity. |
| `SiteDetailPage.tsx` | `CreateNoteModal.tsx` | photo icon click opens modal in photo mode | ✓ WIRED | `openPhotoModal` sets `noteModalActivityType="photo"`, sets modal open, passes `initialActivityType` prop. |
| `offline/sync.ts` | `offline/queue.ts` | process queue on online event | ✓ WIRED | `initSync` registers online listener → `fullSync` → `syncPendingActions` → `processAction`. |
| `routes.rs` | `site_service.rs` | multipart upload handler invokes service | ✓ WIRED | `upload_site_photo_attachment` calls `service.upload_photo_attachment(...)`. |
| `routes.rs` | `site_repository.rs` | tenant-authorized attachment lookup | ✓ WIRED | `find_attachment_by_id(attachment_uuid, ctx.tenant_id)` used in attachment read routes. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `CreateNoteModal.tsx` | `photoUrl` | `uploadPhoto.mutateAsync` response `photo_url` | Yes | ✓ FLOWING |
| `site_repository.rs` + `ActivityFeed.tsx` | `activity.photo_url` | DB `site_activities.photo_url` (`list_activities`) rendered by `<img src>` | Yes | ✓ FLOWING |
| `offline/queue.ts` | `uploadResponse.photo_url` | Runtime POST `/attachments/photo` response used in activity creation | Yes | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Upload + offline replay contract and modal tests | `npm run test:run -- useSites offline/queue CreateNoteModal` | 3 files, 7 tests passed in 1.17s | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| FILE-01 | 29-02, 29-03, 29-05 | User can upload photos via camera or gallery | ✓ SATISFIED | Multipart key aligned (`photo`) across upload clients + modal file picker + passing tests. |
| FILE-02 | 29-03, 29-05 | Image preview shown in activity feed | ✓ SATISFIED | `photo_url` is persisted and rendered by `ActivityFeed` image block. |
| FILE-03 | 29-01, 29-02 | Photos stored with UUID filenames | ✓ SATISFIED | UUID-derived storage keys in service layer. |
| FILE-04 | 29-02 | Image thumbnails generated on upload | ✓ SATISFIED | Thumbnail bytes generated and persisted in upload pipeline. |
| FILE-05 | 29-04, 29-05 | Offline photo capture queued for sync | ✓ SATISFIED | `queuePhotoUploadAction` persists payload; tests verify queued payload shape. |
| FILE-06 | 29-04, 29-05 | Sync queue processes uploads when online | ✓ SATISFIED | Online listener wiring + replay handler posts `photo` and creates activity; test covers replay path. |
| FILE-07 | 29-01, 29-02 | File access authorized by tenant_id | ✓ SATISFIED | Attachment lookup requires tenant-scoped query from auth context. |

Orphaned requirements for Phase 29 from `REQUIREMENTS.md`: **None** (all FILE-01..FILE-07 are referenced by phase plans).

### Anti-Patterns Found

No blocker anti-patterns found in phase-relevant modified files. The previous multipart-contract mismatch (`file` vs `photo`) is resolved in both online and offline paths.

### Human Verification Required

### 1. Mobile camera/gallery upload from Site detail flow

**Test:** Open site detail page on a real/mobile browser, click camera icon, choose/take a photo, save.
**Expected:** Modal opens directly in photo mode; upload succeeds; activity feed shows preview image.
**Why human:** Requires device camera permission flow and visual UI confirmation.

### 2. Offline queue replay after reconnect

**Test:** Go offline, create photo activity, then restore connection.
**Expected:** Pending action is queued offline, then auto-processed on reconnect; activity appears with preview and pending count drops.
**Why human:** Requires real network transition and runtime sync behavior.

### Gaps Summary

Previously reported blockers are closed: multipart contract is now consistent (`photo`) across online and offline upload paths, and the camera entrypoint is wired to a photo-first modal flow. Automated and static verification indicates phase goal is implemented. Final sign-off depends on human runtime validation for mobile capture UX and reconnect sync behavior.

---

_Verified: 2026-05-01T10:37:52Z_
_Verifier: OpenCode (gsd-verifier)_
