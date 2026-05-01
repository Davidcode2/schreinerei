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
- [ ] **FILE-03**: Photos stored with UUID filenames (security)
- [ ] **FILE-04**: Image thumbnails generated on upload
- [ ] **FILE-05**: Offline photo capture queued for sync
- [ ] **FILE-06**: Sync queue processes uploads when online
- [ ] **FILE-07**: File access authorized by tenant_id

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
| STAT-01 | — | Pending |
| STAT-02 | — | Pending |
| STAT-03 | — | Pending |
| STAT-04 | — | Pending |
| STAT-05 | — | Pending |
| FEED-01 | — | Pending |
| FEED-02 | — | Pending |
| FEED-03 | — | Pending |
| FEED-04 | — | Pending |
| FILE-01 | — | Pending |
| FILE-02 | — | Pending |
| FILE-03 | — | Pending |
| FILE-04 | — | Pending |
| FILE-05 | — | Pending |
| FILE-06 | — | Pending |
| FILE-07 | — | Pending |
| HIST-01 | — | Pending |
| HIST-02 | — | Pending |
| HIST-03 | — | Pending |
| HIST-04 | — | Pending |
| HIST-05 | — | Pending |

**Coverage:**
- v1.8 requirements: 21 total
- Mapped to phases: 0
- Unmapped: 21 ⚠️

---
*Requirements defined: 2026-05-01*
*Last updated: 2026-05-01 after initial definition*
