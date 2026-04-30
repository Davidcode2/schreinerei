# Requirements: Schreinerei SaaS

**Defined:** 2026-04-30
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick.

## v1.5 Requirements

Testing & Quality Foundation — comprehensive testing strategy and issue documentation.

### Backend Unit Tests

- [x] **TEST-01**: Domain layer unit tests exist for all modules (iam, inventory, sites, fleet)
- [x] **TEST-02**: Domain tests are inline in domain files with `#[cfg(test)]`
- [x] **TEST-03**: Domain tests cover business rules and validation logic

### Frontend Testing

- [ ] **TEST-04**: Vitest configured with React Testing Library and MSW
- [ ] **TEST-05**: Test utilities and fixtures created (render with providers, mock data)
- [ ] **TEST-06**: Key components tested (forms, dialogs, lists)
- [ ] **TEST-07**: ts-rs derive macros added to all backend DTOs
- [ ] **TEST-08**: TypeScript types auto-generated from Rust structs
- [ ] **TEST-09**: CI fails if generated types differ from committed

### E2E Data Assertions

- [x] **TEST-10**: E2E tests verify data persisted via API calls
- [x] **TEST-11**: Data assertions pattern documented for future E2E tests

### Agent QA Playbook

- [ ] **QA-01**: Playwright learnings documented (UI navigation patterns)
- [ ] **QA-02**: QA Playbook created (run tests, logs, validate features)
- [ ] **QA-03**: Parameter mismatch prevention strategy documented
- [ ] **QA-04**: ts-rs usage documented in AGENTS.md

### Feature Audit

- [ ] **AUDIT-01**: Baustellen feature audit complete (bugs, issues, missing functionality)
- [ ] **AUDIT-02**: Inventory feature audit complete
- [ ] **AUDIT-03**: Time Booking feature audit complete
- [ ] **AUDIT-04**: Vehicles/Machines feature audit complete
- [ ] **AUDIT-05**: Reservations feature audit complete
- [ ] **AUDIT-06**: Comprehensive issue backlog documented

## v1.6+ Requirements (Future)

### Integration Tests

- **INT-01**: Integration tests with real PostgreSQL for inventory module
- **INT-02**: Integration tests for sites module
- **INT-03**: Integration tests for fleet module
- **INT-04**: Multi-tenant isolation tests for all modules

### Application Layer Tests

- **APP-01**: Repository traits extracted for mocking
- **APP-02**: Application layer tests with mockall

### Extended E2E

- **E2E-01**: Offline scenario tests with Playwright
- **E2E-02**: Performance budget tests

### Self-Service Registration

- **SS-01**: Public website with organization registration
- **SS-02**: Self-service organization creation flow
- **SS-03**: Organization admin dashboard
- **SS-04**: Member invitation via email

## Out of Scope

| Feature | Reason |
|---------|--------|
| Application layer tests | Requires repository trait extraction - defer to v1.6 |
| Integration tests with real DB | Requires testcontainers setup - defer to v1.6 |
| Offline E2E scenarios | Complex setup - defer to v1.6 |
| Visual regression tests | Not priority for this milestone |
| Performance budget tests | Premature optimization |
| Fixing issues found in audit | This milestone records issues only |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| TEST-01 | Phase 12 | ✓ Complete |
| TEST-02 | Phase 12 | ✓ Complete |
| TEST-03 | Phase 12 | ✓ Complete |
| QA-01 | Phase 13 | Pending |
| QA-02 | Phase 13 | Pending |
| QA-03 | Phase 13 | Pending |
| QA-04 | Phase 13 | Pending |
| TEST-04 | Phase 14 | Pending |
| TEST-05 | Phase 14 | Pending |
| TEST-06 | Phase 14 | Pending |
| TEST-07 | Phase 15 | Pending |
| TEST-08 | Phase 15 | Pending |
| TEST-09 | Phase 15 | Pending |
| TEST-10 | Phase 16 | ✓ Complete |
| TEST-11 | Phase 16 | ✓ Complete |
| AUDIT-01 | Phase 17 | Pending |
| AUDIT-02 | Phase 17 | Pending |
| AUDIT-03 | Phase 17 | Pending |
| AUDIT-04 | Phase 17 | Pending |
| AUDIT-05 | Phase 17 | Pending |
| AUDIT-06 | Phase 17 | Pending |

**Coverage:**
- v1.5 requirements: 21 total
- Mapped to phases: 21
- Unmapped: 0
- Complete: 15/21

---
*Requirements defined: 2026-04-30*
*Last updated: 2026-04-30 after phase reordering*
*Phase 12 completed: 2026-04-30*
