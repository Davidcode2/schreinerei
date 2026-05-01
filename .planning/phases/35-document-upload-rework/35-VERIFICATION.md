---
phase: 35-document-upload-rework
verified: 2026-05-01T16:06:30Z
status: passed
score: 5/5 must-haves verified
overrides_applied: 0
---

# Phase 35: Document Upload Rework Verification Report

**Phase Goal:** Users can create one document activity with optional note text plus multiple image/PDF attachments, and those entries render correctly in the Baustelle activity stream.
**Verified:** 2026-05-01T16:06:30Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Users can submit note-only, attachment-only, and note+attachment document entries | ✓ VERIFIED | `src/modules/sites/domain/activity.rs` validates note activities with `content OR attachment_ids`; `CreateNoteModal.tsx` enables submit when trimmed content or selected files exist; `CreateNoteModal.test.tsx` covers note text and attachment-only composition flows |
| 2 | Users can upload image and PDF attachments through a generic document endpoint | ✓ VERIFIED | `src/modules/sites/api/routes.rs` exposes `/api/v1/sites/{id}/attachments` with multipart field `attachment`; `src/modules/sites/application/site_service.rs` allowlists `image/jpeg`, `image/png`, `image/webp`, and `application/pdf`; `useSites.test.tsx` verifies the `attachment` FormData key |
| 3 | Uploaded attachments are linked to one activity without hidden extra rows | ✓ VERIFIED | `src/modules/sites/infrastructure/site_repository.rs` adds `link_activity_attachments`; `src/modules/sites/application/site_service.rs` uploads first, then calls `create_activity` once and links returned attachment IDs inside tenant/site scope |
| 4 | Activity responses expose attachment metadata arrays for frontend consumers | ✓ VERIFIED | `ActivityResponse` in `src/modules/sites/api/routes.rs` contains `attachments`; `frontend/src/types/generated.ts` exports `ActivityResponse`, `SiteActivityAttachmentResponse`, and `UploadSiteAttachmentResponse` |
| 5 | The activity feed renders document entries with image previews, PDF cards, and fallback copy | ✓ VERIFIED | `frontend/src/pages/sites/ActivityFeed.tsx` renders `Dokument hinzugefügt`, attachment grids, `PDF` cards, and `Vorschau nicht verfügbar`; `ActivityFeed.test.tsx` covers image preview, PDF rendering, and fallback states |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| Backend tests and ts-rs export | `cargo test` | 199 Rust tests passed; ts-rs export tests regenerated frontend types | ✓ PASS |
| Frontend document flow regression suite | `npm run test:run -- useSites CreateNoteModal ActivityFeed CameraUploadFlow` | 21/21 tests passed | ✓ PASS |
| Schema drift check | `gsd-sdk query verify.schema-drift 31` | `drift_detected: false` | ✓ PASS |

### Requirements Coverage

| Requirement | Status | Evidence |
|-------------|--------|----------|
| DOC-01 | ✓ SATISFIED | Document composer submits note + attachments in one create flow |
| DOC-02 | ✓ SATISFIED | PDF uploads accepted and rendered as labeled PDF cards |
| DOC-03 | ✓ SATISFIED | Image uploads accepted and rendered as authenticated previews |
| DOC-04 | ✓ SATISFIED | Attachment-only document entries are valid in domain + UI validation |
| DOC-05 | ✓ SATISFIED | MIME validation covers images and PDFs in backend and frontend |

### Human Verification Required

None.

### Gaps Summary

No gaps found. Phase 35 backend contracts, modal workflow, and feed rendering all match the planned document upload behavior, and the existing camera-specific path still has passing frontend regression coverage.

---

_Verified: 2026-05-01T16:06:30Z_
_Verifier: OpenCode inline executor_
