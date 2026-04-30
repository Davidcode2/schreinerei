# Requirements: Schreinerei SaaS v1.6

**Defined:** 2026-04-30
**Milestone:** v1.6 User Experience & Missing Functionality
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.

## v1.6 Requirements

Requirements for completing missing CRUD functionality and improving UX.

### Bug Fixes

- [x] **FIX-01**: User cannot submit time entry with hours <= 0 (validation blocks invalid input) ✓
- [x] **FIX-02**: User sees inline validation error messages below form fields when input is invalid ✓

### Delete Operations

- [x] **DEL-01**: User can delete a site with confirmation dialog (soft delete) ✓
- [x] **DEL-02**: User can delete a material with confirmation dialog (soft delete) ✓
- [x] **DEL-03**: User can delete a vehicle with confirmation dialog (soft delete) ✓
- [x] **DEL-04**: User can delete a tool with confirmation dialog (soft delete) ✓
- [x] **DEL-05**: User sees dependency conflict message when delete is blocked by FK constraints ✓

### Edit Operations

- [x] **EDIT-01**: User can edit an existing time entry (hours, work type, notes) ✓
- [x] **EDIT-02**: User can delete their own time entries ✓
- [x] **EDIT-03**: User can edit an existing reservation (dates, resource, notes) ✓

### Reservation Workflow

- [x] **RESV-01**: User can transition reservation status (confirm, start, complete, cancel) via UI buttons ✓
- [x] **RESV-02**: User can create a reservation by clicking empty time slots in calendar view ✓
- [x] **RESV-03**: User sees which existing reservation conflicts when availability warning appears ✓

### UX Improvements

- [x] **UX-01**: User sees low stock warning badge when material quantity falls below minimum ✓ (already implemented in MaterialCard.tsx)
- [x] **UX-02**: User can initiate QR scan by clicking QR code button on inventory page ✓

### E2E Test Coverage

- [ ] **TEST-12**: E2E test for delete operations on all entity types
- [ ] **TEST-13**: E2E test for edit operations on time entries and reservations
- [ ] **TEST-14**: E2E test for reservation status transitions
- [ ] **TEST-15**: E2E test for calendar click-to-create reservation

## v1.7+ Requirements

Deferred to future milestone.

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
- **EXT-01**: Organization identity provider support
- **EXT-02**: Multi-organization user support

### Restore Functionality

- **REST-01**: Admin can restore soft-deleted items
- **REST-02**: Soft-deleted items excluded from lists but retained in database

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Hard delete | Soft delete required for offline sync compatibility |
| Restore UI | Admin-only feature, defer to v1.7+ |
| Optimistic locking | Status transition race conditions deferred to v1.7+ |
| Offline conflict resolution | Known tech debt, requires deeper design |
| Native mobile app | PWA-first strategy, later consideration |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| FIX-01 | Phase 18 | Complete ✓ |
| FIX-02 | Phase 18 | Complete ✓ |
| DEL-01 | Phase 19 | Complete ✓ |
| DEL-02 | Phase 19 | Complete ✓ |
| DEL-03 | Phase 19 | Complete ✓ |
| DEL-04 | Phase 19 | Complete ✓ |
| DEL-05 | Phase 19 | Complete ✓ |
| EDIT-01 | Phase 20 | Complete ✓ |
| EDIT-02 | Phase 20 | Complete ✓ |
| EDIT-03 | Phase 20 | Complete ✓ |
| RESV-01 | Phase 20 | Complete ✓ |
| RESV-02 | Phase 20 | Complete ✓ |
| RESV-03 | Phase 20 | Complete ✓ |
| UX-01 | Phase 18 | Complete (pre-existing) |
| UX-02 | Phase 18 | Complete ✓ |
| TEST-12 | Phase 21 | Pending |
| TEST-13 | Phase 21 | Pending |
| TEST-14 | Phase 21 | Pending |
| TEST-15 | Phase 21 | Pending |

**Coverage:**
- v1.6 requirements: 19 total
- Mapped to phases: 19
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-30*
*Last updated: 2026-04-30 for v1.6 milestone*
