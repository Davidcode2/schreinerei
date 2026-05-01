# Roadmap: Schreinerei v1.8 Activity Feed & Site Status

**Milestone:** v1.8
**Created:** 2026-05-01
**Granularity:** coarse
**Phases:** 4
**Coverage:** 21/21 requirements mapped

## Phases

- [ ] **Phase 26: Status Change Workflow** - Site status transitions with modal UI and audit trail
- [ ] **Phase 27: Tabbed Activity Feed** - Activity feed reorganization with tabs for notes and materials
- [ ] **Phase 28: Material History Tab** - Material extraction history linked to Baustellen
- [ ] **Phase 29: Photo Upload & Attachments** - Photo capture, storage infrastructure, and activity attachments

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

**Plans:** TBD

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

**Plans:** TBD

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

**Plans:** TBD

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

**Plans:** TBD

---

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 26. Status Change Workflow | 0/3 | Not started | - |
| 27. Tabbed Activity Feed | 0/2 | Not started | - |
| 28. Material History Tab | 0/3 | Not started | - |
| 29. Photo Upload & Attachments | 0/4 | Not started | - |

## Coverage Map

| Requirement | Phase | Status |
|-------------|-------|--------|
| STAT-01 | Phase 26 | Pending |
| STAT-02 | Phase 26 | Pending |
| STAT-03 | Phase 26 | Pending |
| STAT-04 | Phase 26 | Pending |
| STAT-05 | Phase 26 | Pending |
| FEED-01 | Phase 27 | Pending |
| FEED-02 | Phase 27 | Pending |
| FEED-03 | Phase 27 | Pending |
| FEED-04 | Phase 27 | Pending |
| HIST-01 | Phase 28 | Pending |
| HIST-02 | Phase 28 | Pending |
| HIST-03 | Phase 28 | Pending |
| HIST-04 | Phase 28 | Pending |
| HIST-05 | Phase 28 | Pending |
| FILE-01 | Phase 29 | Pending |
| FILE-02 | Phase 29 | Pending |
| FILE-03 | Phase 29 | Pending |
| FILE-04 | Phase 29 | Pending |
| FILE-05 | Phase 29 | Pending |
| FILE-06 | Phase 29 | Pending |
| FILE-07 | Phase 29 | Pending |

---

*Roadmap created: 2026-05-01*
*Ready for planning: `/gsd-plan-phase 26`*
