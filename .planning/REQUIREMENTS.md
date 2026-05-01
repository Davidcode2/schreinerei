# Requirements: Schreinerei v1.8 Activity Feed & Site Status

**Defined:** 2026-05-01
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.

## v1.8 Requirements

### Site Status Management

- [ ] **STAT-01**: User can change site status via modal (geplant → aktiv → abgeschlossen)
- [ ] **STAT-02**: Chip tap opens status change modal with valid transitions
- [ ] **STAT-03**: "Aktiv" button renamed to "Auswählen" (avoid confusion with status)
- [ ] **STAT-04**: Status change creates activity entry (audit trail)
- [ ] **STAT-05**: Concurrent status changes handled with optimistic locking

### Activity Feed

- [ ] **FEED-01**: Activity feed has two tabs (Notizen/Dokumente | Material)
- [ ] **FEED-02**: Add note button opens modal for text entry
- [ ] **FEED-03**: Each entry shows timestamp and content preview
- [ ] **FEED-04**: Activity feed entries are paginated with cursor-based loading

### File Attachments

- [ ] **FILE-01**: User can upload photos via camera or gallery
- [ ] **FILE-02**: Image preview shown in activity feed
- [x] **FILE-03**: Photos stored with UUID filenames (security)
- [ ] **FILE-04**: Image thumbnails generated on upload
- [ ] **FILE-05**: Offline photo capture queued for sync
- [ ] **FILE-06**: Sync queue processes uploads when online
- [x] **FILE-07**: File access authorized by tenant_id

### Material History

- [ ] **HIST-01**: Materials tab shows stock entries linked to Baustelle
- [ ] **HIST-02**: User can click Baustelle name to navigate to detail page
- [ ] **HIST-03**: Material entry shows category name
- [ ] **HIST-04**: Material history shows user who extracted material
- [ ] **HIST-05**: Material history query uses eager loading (no N+1)

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Document Attachments

- **DOC-01**: User can upload PDF documents
- **DOC-02**: Document preview shown in activity feed
- **DOC-03**: Document download link in feed

### Real-time Updates

- **REAL-01**: Activity feed updates via WebSocket
- **REAL-02**: Material extractions appear in real-time

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Document uploads (PDF) | Start with photos only; documents add preview complexity |
| Rich text editor for notes | Plain text is faster, simpler, offline-friendly |
| Real-time WebSocket sync | Polling sufficient for MVP team sizes |
| Video uploads | Storage/bandwidth costs, defer to v2+ |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

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
| FILE-03 | Phase 29 | Complete |
| FILE-04 | Phase 29 | Pending |
| FILE-05 | Phase 29 | Pending |
| FILE-06 | Phase 29 | Pending |
| FILE-07 | Phase 29 | Complete |

**Coverage:**
- v1.8 requirements: 21 total
- Mapped to phases: 21 ✓
- Unmapped: 0 ✓

---
*Requirements defined: 2026-05-01*
*Last updated: 2026-05-01 after initial definition*
