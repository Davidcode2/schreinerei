# Requirements: Schreinerei

**Defined:** 2026-05-01
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.

## v1 Requirements

### Camera Upload (CAM)

- [x] **CAM-01**: Camera button on activity stream opens native camera/gallery picker (not document modal)
- [x] **CAM-02**: User can add an optional text note when uploading via camera flow
- [x] **CAM-03**: Camera upload automatically attaches selected image to activity entry

### Document Upload (DOC)

- [x] **DOC-01**: Document modal supports note AND attachment(s) in a single entry (currently note OR image)
- [x] **DOC-02**: User can upload one or more PDFs as attachments (with optional note)
- [x] **DOC-03**: User can upload one or more images as attachments (with optional note)
- [x] **DOC-04**: User can create an entry with attachment-only (no note text required)
- [x] **DOC-05**: Document modal accepts both PDF and image file types with appropriate validation

### Media Viewer (VIEW)

- [x] **VIEW-01**: Click on image preview in feed opens fullscreen modal showing large image
- [x] **VIEW-02**: Click on document preview in feed opens fullscreen modal showing document
- [x] **VIEW-03**: Fullscreen modal has its own slug-based URL for direct linking and sharing
- [x] **VIEW-04**: Fullscreen modal displays note text alongside the media (to the right of image)
- [x] **VIEW-05**: Fullscreen modal shows timestamp and username of entry creator
- [x] **VIEW-06**: Fullscreen modal has download button to download the media file
- [x] **VIEW-07**: Fullscreen modal has share button to copy page link to clipboard
- [x] **VIEW-08**: Fullscreen modal has close button to return to activity feed
- [x] **VIEW-09**: Modal fills almost the entire screen for optimal media viewing

### Entry Management (ENTRY)

- [x] **ENTRY-01**: User can delete their own activity entries (creator-only permission)
- [x] **ENTRY-02**: Delete action shows confirmation dialog before removing entry
- [x] **ENTRY-03**: Deletion removes entry and associated attachments from feed

## v2 Requirements

### Integration Tests

- **INT-01**: Integration tests with real PostgreSQL for inventory module
- **INT-02**: Integration tests for sites module
- **INT-03**: Integration tests for fleet module
- **INT-04**: Multi-tenant isolation tests for all modules

### Inventory Features (deferred from v1.9)

- **INVT-10**: Category editing via inventory settings page
- **INVT-11**: Edit inventory item location and minimum quantity
- **INVT-12**: Set available quantity to arbitrary number
- **INVT-13**: "Material einlagern" (stock in) action with modal
- **INVT-14**: Extended inventory history with user attribution
- **INVT-15**: Clickable Baustelle links in history events
- **INVT-16**: Category display on inventory overview

### Self-Service Registration

- **SS-01**: Public website with organization registration
- **SS-02**: Self-service organization creation flow
- **SS-03**: Organization admin dashboard
- **SS-04**: Member invitation via email
- **EXT-01**: Organization identity provider support
- **EXT-02**: Multi-organization user support

## Out of Scope

| Feature | Reason |
|---------|--------|
| RFID Hardware-Integration | Hardware nicht vorhanden, später ergänzen |
| CAD/CNC Integration (DXF, Bsolid) | Nicht kritisch für Pilot, später implementieren |
| Native Mobile App | PWA first, später React Native/Capacitor möglich |
| Öffentliche Website/Landing Page | Fokus auf App, Website später |
| Rich Text Editor für Notizen | Plain Text für jetzt, später erweitern |
| Echtzeit-WebSocket Sync | Polling reicht für MVP-Teamgrößen |
| Video-Uploads | Speicher/Kosten, aufgeschoben |
| Offline photo queue replay test | Deferred to backlog (Phase 999.1) |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CAM-01 | 30 | Complete |
| CAM-02 | 30 | Complete |
| CAM-03 | 30 | Complete |
| DOC-01 | 31 | Complete |
| DOC-02 | 31 | Complete |
| DOC-03 | 31 | Complete |
| DOC-04 | 31 | Complete |
| DOC-05 | 31 | Complete |
| VIEW-01 | 32 | Complete |
| VIEW-02 | 32 | Complete |
| VIEW-03 | 32 | Complete |
| VIEW-04 | 32 | Complete |
| VIEW-05 | 32 | Complete |
| VIEW-06 | 32 | Complete |
| VIEW-07 | 32 | Complete |
| VIEW-08 | 32 | Complete |
| VIEW-09 | 32 | Complete |
| ENTRY-01 | 33 | Complete |
| ENTRY-02 | 33 | Complete |
| ENTRY-03 | 33 | Complete |

**Coverage:**
- v1 requirements: 20 total
- Mapped to phases: 20 ✓
- Unmapped: 0 ✓

---
*Requirements defined: 2026-05-01*
*Last updated: 2026-05-01 after initial definition*