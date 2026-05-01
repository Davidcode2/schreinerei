# Roadmap: Schreinerei v1.8 Activity Feed & Site Status

**Milestone:** v1.8
**Created:** 2026-05-01
**Granularity:** coarse
**Phases:** 4
**Coverage:** 21/21 requirements mapped

## Phases

- [x] **Phase 26: Status Change Workflow** - Site status transitions with modal UI and audit trail ✓ 2026-05-01
- [x] **Phase 27: Tabbed Activity Feed** - Activity feed reorganization with tabs for notes and materials ✓ 2026-05-01
- [x] **Phase 28: Material History Tab** - Material extraction history linked to Baustellen ✓ 2026-05-01
- [x] **Phase 29: Photo Upload & Attachments** - Photo capture, storage infrastructure, and activity attachments (completed 2026-05-01)

## Phase Details

### Phase 26: Status Change Workflow

**Goal:** Users can change site status through a controlled workflow with validation and audit trail.

**Depends on:** Phase 25 (v1.7 Active Project Context)

**Requirements:** STAT-01, STAT-02, STAT-03, STAT-04, STAT-05

**Success Criteria** (what must be TRUE):
1. User can open status change modal by tapping status chip on site detail page
2. User can select valid status transitions (geplant → aktiv → abgeschlossen) with backend validation
3. "Aktiv" button renamed to "Auswählen" avoiding confusion with status name
4. Each status change creates activity entry visible in feed for audit trail
5. Concurrent status changes are rejected with clear error message (optimistic locking)

**Plans:** Complete (3/3 plans executed)

**Status:** ✅ Complete

---

### Phase 27: Tabbed Activity Feed

**Goal:** Users can view and add notes in an organized activity feed with clear navigation.

**Depends on:** Phase 26

**Requirements:** FEED-01, FEED-02, FEED-03, FEED-04

**Success Criteria** (what must be TRUE):
1. Activity feed displays two tabs: "Notizen/Dokumente" and "Material"
2. User can add notes via button that opens text entry modal
3. Each entry shows timestamp and content preview for quick scanning
4. Feed loads additional entries on scroll with cursor-based pagination

**Plans:** Complete (2/2 plans executed)

**Status:** ✅ Complete

---

### Phase 28: Material History Tab

**Goal:** Users can view material extraction history linked to construction sites with full context.

**Depends on:** Phase 27

**Requirements:** HIST-01, HIST-02, HIST-03, HIST-04, HIST-05

**Success Criteria** (what must be TRUE):
1. Materials tab shows all stock entries linked to the Baustelle
2. User can click Baustelle name to navigate to site detail page
3. Material entry displays category name alongside material name
4. Material history shows which user extracted the material
5. Material history loads efficiently with eager loading (no N+1 queries)

**Plans:** 3 plans

Plans:
- [x] 28-01-PLAN.md — Add backend site-scoped material history endpoint with eager-loaded category/user/site context
- [x] 28-02-PLAN.md — Add frontend site history hook + typed contracts for enriched material history rows
- [x] 28-03-PLAN.md — Implement Material tab UI rendering and Baustelle navigation link with tests

---

### Phase 29: Photo Upload & Attachments

**Goal:** Users can capture and attach photos to activities with secure storage and offline support.

**Depends on:** Phase 28

**Requirements:** FILE-01, FILE-02, FILE-03, FILE-04, FILE-05, FILE-06, FILE-07

**Success Criteria** (what must be TRUE):
1. User can upload photos via camera capture or gallery selection
2. Image previews appear in activity feed after upload
3. Photos are stored securely with UUID filenames (no user-provided names)
4. Thumbnails are generated automatically on upload for fast loading
5. Offline photo capture is queued and syncs when connection restored
6. File access is authorized by tenant_id preventing cross-tenant leakage

**Plans:** 6/6 plans complete

Plans:
- [ ] 29-01-PLAN.md — Add tenant-scoped attachment schema and secure attachment read endpoints
- [ ] 29-02-PLAN.md — Implement multipart photo upload pipeline with UUID filenames and thumbnail generation
- [ ] 29-03-PLAN.md — Wire frontend camera/gallery upload flow and activity feed image previews
- [ ] 29-04-PLAN.md — Add offline photo queueing and reconnect sync processing
- [ ] 29-05-PLAN.md — Close multipart field mismatch and wire functional photo icon entrypoint
- [ ] 29-06-PLAN.md — Gap closure: remove duplicate photo activity creation and fix authenticated attachment preview loading

---

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 26. Status Change Workflow | 3/3 | ✅ Complete | 2026-05-01 |
| 27. Tabbed Activity Feed | 2/2 | ✅ Complete | 2026-05-01 |
| 28. Material History Tab | 3/3 | ✅ Complete | 2026-05-01 |
| 29. Photo Upload & Attachments | 6/6 | Complete   | 2026-05-01 |

## Coverage Map

| Requirement | Phase | Status |
|-------------|-------|--------|
| STAT-01 | Phase 26 | ✅ Complete |
| STAT-02 | Phase 26 | ✅ Complete |
| STAT-03 | Phase 26 | ✅ Complete |
| STAT-04 | Phase 26 | ✅ Complete |
| STAT-05 | Phase 26 | ✅ Complete |
| FEED-01 | Phase 27 | ✅ Complete |
| FEED-02 | Phase 27 | ✅ Complete |
| FEED-03 | Phase 27 | ✅ Complete |
| FEED-04 | Phase 27 | ✅ Complete |
| HIST-01 | Phase 28 | ✅ Verified |
| HIST-02 | Phase 28 | ✅ Verified |
| HIST-03 | Phase 28 | ✅ Verified |
| HIST-04 | Phase 28 | ✅ Verified |
| HIST-05 | Phase 28 | ✅ Verified |
| FILE-01 | Phase 29 | ✅ Verified |
| FILE-02 | Phase 29 | ✅ Verified |
| FILE-03 | Phase 29 | ✅ Verified |
| FILE-04 | Phase 29 | ✅ Verified |
| FILE-05 | Phase 29 | ✅ Verified |
| FILE-06 | Phase 29 | ✅ Verified |
| FILE-07 | Phase 29 | ✅ Verified |

---

## Backlog

### Phase 999.1: Offline Photo Queue Replay

**Goal:** Test and validate offline photo queue replay after reconnect
**Source phase:** 29
**Deferred at:** 2026-05-01 during Phase 29 UAT
**Note:** Offline functionality is largely unimplemented; offline queue replay for photos cannot be tested until broader offline support exists.

---

*Roadmap created: 2026-05-01*
*Last updated: 2026-05-01 after Phase 28+29 verification*
*Next: `/gsd-complete-milestone`*
