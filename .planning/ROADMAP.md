# Roadmap: Schreinerei v1.10

## Overview

Enhance the Baustelle activity stream with separate camera/document upload flows, mixed note+attachment entries, a fullscreen media viewer with share links, and creator-only entry deletion. Builds on v1.8's photo upload infrastructure (multipart pipeline, UUID storage, authenticated blob fetch).

## Phases

**Phase Numbering:**
- Integer phases (30-33): Planned milestone work for v1.10
- Prior phases (1-29): See v1.0–v1.8 milestones in MILESTONES.md

- [x] **Phase 30: Camera Upload Flow** - Separate camera button from document modal with optional note (completed 2026-05-01)
- [ ] **Phase 31: Document Upload Rework** - Support note AND attachments in a single entry (all combinations)
- [ ] **Phase 32: Media Viewer** - Fullscreen viewer with slug URLs, metadata, download, and share
- [ ] **Phase 33: Entry Management** - Delete own entries with confirmation

## Phase Details

### Phase 30: Camera Upload Flow
**Goal**: Users can capture or select photos directly from the activity stream without opening the document modal
**Depends on**: Nothing new (builds on v1.8 photo upload infrastructure)
**Requirements**: CAM-01, CAM-02, CAM-03
**Success Criteria** (what must be TRUE):
  1. Camera button on activity stream opens native camera/gallery picker (not the document modal)
  2. User can attach an optional text note before submitting a camera upload
  3. Selected photo automatically attaches to the activity entry and appears in the feed
**Plans**: 1 plan
- [x] 30-01-PLAN.md — Create CameraUploadFlow component and wire into SiteDetailPage

### Phase 31: Document Upload Rework
**Goal**: Users can create activity entries combining notes and file attachments in any combination
**Depends on**: Nothing new (reworks existing document modal)
**Requirements**: DOC-01, DOC-02, DOC-03, DOC-04, DOC-05
**Success Criteria** (what must be TRUE):
  1. User can create an entry with both a note AND attachments in a single submission
  2. User can upload PDF files as attachments with or without a note
  3. User can upload image files as attachments with or without a note
  4. User can create an entry with attachments only (no note text required)
  5. Modal validates and accepts both PDF and image file types with clear error messages
**Plans**: TBD

### Phase 32: Media Viewer
**Goal**: Users can view, download, and share media attachments in a fullscreen viewer with direct links
**Depends on**: Phase 30 (image previews from camera uploads), Phase 31 (document previews from upload modal)
**Requirements**: VIEW-01, VIEW-02, VIEW-03, VIEW-04, VIEW-05, VIEW-06, VIEW-07, VIEW-08, VIEW-09
**Success Criteria** (what must be TRUE):
  1. Clicking an image or document preview in the feed opens a fullscreen modal
  2. Fullscreen modal displays media left/center with note text to the right, plus creator name and timestamp
  3. User can download the media file and copy a direct link to the entry
  4. Fullscreen modal has a unique slug-based URL for direct linking and sharing
  5. Modal fills nearly the entire screen with a close button to return to the feed
**Plans**: TBD

### Phase 33: Entry Management
**Goal**: Users can delete their own activity entries with confirmation
**Depends on**: Nothing new (operates on existing activity entries)
**Requirements**: ENTRY-01, ENTRY-02, ENTRY-03
**Success Criteria** (what must be TRUE):
  1. User can delete an activity entry they created (only the creator sees the delete option)
  2. Delete triggers a confirmation dialog before removing the entry
  3. Deletion removes the entry and all associated attachments from the feed
**Plans**: TBD

## Progress

**Execution Order:**
Phases 30 → 31 → 32 → 33 (Phase 32 depends on 30+31; Phase 33 is independent but ordered last)

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 30. Camera Upload Flow | 1/1 | Complete    | 2026-05-01 |
| 31. Document Upload Rework | 0/? | Not started | - |
| 32. Media Viewer | 0/? | Not started | - |
| 33. Entry Management | 0/? | Not started | - |