# Requirements: Schreinerei SaaS

**Defined:** 2026-04-29
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.

## v1.2 Requirements (Frontend Polish)

### Inventory Module

- [ ] **INVT-08**: User can add new material via dialog form
- [ ] **INVT-09**: User can scan QR code to navigate to material detail

### Sites Module

- [ ] **SITE-09**: User can create new site via dialog form

### Fleet Module

- [ ] **FLEET-08**: User can add new vehicle via dialog form
- [ ] **FLEET-09**: User can add new tool via dialog form

### User Management

- [ ] **USER-01**: Admin can invite user via email dialog
- [ ] **USER-02**: Settings page displays real users from API

### Error Handling

- [ ] **ERR-01**: QR scanner shows graceful error when camera denied
- [ ] **ERR-02**: QR scanner provides retry option after error

## v1.3 Requirements (Future)

### Self-Service

- **SS-01**: Public website with organization registration
- **SS-02**: Self-service organization creation flow
- **SS-03**: Organization admin dashboard
- **SS-04**: Member invitation via email

### Extended Features

- **EXT-01**: Organization identity provider support
- **EXT-02**: Multi-organization user support

## Out of Scope

| Feature | Reason |
|---------|--------|
| Multi-org user support | Single org per user for v1.x |
| Public website/landing page | Deferred to v1.3 |
| Self-service registration | Deferred to v1.3 |
| Organization identity providers | Not needed for pilot |
| Stripe integration | Deferred to v1.3+ |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| INVT-08 | Phase 7 | Pending |
| INVT-09 | Phase 7 | Pending |
| SITE-09 | Phase 7 | Pending |
| FLEET-08 | Phase 7 | Pending |
| FLEET-09 | Phase 7 | Pending |
| USER-01 | Phase 7 | Pending |
| USER-02 | Phase 7 | Pending |
| ERR-01 | Phase 7 | Pending |
| ERR-02 | Phase 7 | Pending |

**Coverage:**
- v1.2 requirements: 9 total
- Mapped to phases: 9
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-29*
*Last updated: 2026-04-29 after v1.2 milestone start*
