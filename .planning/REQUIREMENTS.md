# Requirements: Schreinerei SaaS

**Defined:** 2026-05-01
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.

## v1.9 Requirements

### Category Settings (CATS)

- [ ] **CATS-01**: User can edit category name via inventory settings page
- [ ] **CATS-02**: User can delete category (blocked if materials reference it, with clear error)
- [ ] **CATS-03**: User can navigate to inventory settings page via settings wheel icon on inventory page

### Material Editing (EDIT)

- [ ] **EDIT-01**: User can edit inventory item location via edit icon in details section
- [ ] **EDIT-02**: User can edit minimum quantity via modal (same modal as location edit)
- [ ] **EDIT-03**: User can set available quantity to an arbitrary number (stock correction)

### Stock-In (STOCK)

- [ ] **STOCK-01**: User can record material stock-in ("Material einlagern") with amount and notes
- [ ] **STOCK-02**: Stock-in events appear in inventory history with "MaterialAdded" entry type

### Enriched History (HIST)

- [ ] **HIST-01**: Inventory history shows color-coded event types (green for stock-in, red for withdrawal, blue for location/quantity changes)
- [ ] **HIST-02**: History entries display the user who performed the action
- [ ] **HIST-03**: Baustelle name in withdrawal history is clickable and navigates to baustelle detail page

### Overview Display (VIEW)

- [ ] **VIEW-01**: Inventory overview shows category name on each material entry

## v2 Requirements

Deferred to future release.

### Integration Tests

- **INT-01**: Integration tests with real PostgreSQL for inventory module
- **INT-02**: Integration tests for sites module
- **INT-03**: Integration tests for fleet module
- **INT-04**: Multi-tenant isolation tests for all modules

### Self-Service Registration

- **SS-01**: Public website with organization registration
- **SS-02**: Self-service organization creation flow
- **SS-03**: Organization admin dashboard
- **SS-04**: Member invitation via email

### Offline Enhancements

- **OFFL-01**: Offline stock-in queue with reconnect sync
- **OFFL-02**: Conflict resolution for concurrent edits

## Out of Scope

| Feature | Reason |
|---------|--------|
| Bulk stock-in / batch import | Single-item actions sufficient for v1.9, batch later |
| Category hierarchy (nested) | Flat categories match carpentry domain |
| Real-time WebSocket history | Polling sufficient for 5-20 users per org |
| Soft-delete for categories | Block-with-error is simpler, FK constraint prevents accidental delete |
| Separate audit table for categories | domain_events already captures all mutations |
| RFID Hardware Integration | Hardware not available, later |
| CAD/CNC Integration | Not critical for pilot, later |
| Barcode scanning | v2+ concern |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CATS-01 | Phase 31 | Pending |
| CATS-02 | Phase 31 | Pending |
| CATS-03 | Phase 31 | Pending |
| EDIT-01 | Phase 31 | Pending |
| EDIT-02 | Phase 31 | Pending |
| EDIT-03 | Phase 31 | Pending |
| STOCK-01 | Phase 31 | Pending |
| STOCK-02 | Phase 32 | Pending |
| HIST-01 | Phase 32 | Pending |
| HIST-02 | Phase 32 | Pending |
| HIST-03 | Phase 32 | Pending |
| VIEW-01 | Phase 31 | Pending |

**Coverage:**
- v1.9 requirements: 12 total
- Mapped to phases: 12/12 ✓
- Unmapped: 0

**Mapping rationale:**
- **Phase 30** (Backend API Foundation): Enabler phase — builds API endpoints and migration that Phase 31-32 depend on. No v1 requirements mapped directly.
- **Phase 31** (Settings, Editing & Stock-In): Frontend surfaces for category management, material editing, stock-in recording, and overview display. 8 requirements become observable here.
- **Phase 32** (Enriched History): Visual history enhancements — color-coding, user attribution, clickable links. 4 requirements become observable here. STOCK-02 maps here because stock-in entries are only visible in the enriched history feed.
- **Phase 33** (Type Safety & Coverage): Quality gate — ts-rs type generation and test coverage. No v1 requirements mapped directly (validates Phase 30-32).

---
*Requirements defined: 2026-05-01*
*Last updated: 2026-05-01 — traceability updated with phase mappings*