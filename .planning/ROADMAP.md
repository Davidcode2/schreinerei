# Roadmap: v1.5 Testing & Quality Foundation

**Milestone:** v1.5
**Created:** 2026-04-30
**Phases:** 6 (Phase 12-17)
**Requirements:** 21
**Granularity:** Coarse

## Goal

Establish comprehensive testing strategy, document all issues in existing features, and create an agent QA playbook for future development efficiency.

## Phases

- [ ] **Phase 12: Backend Domain Tests** — Unit tests for pure business logic in all modules
- [ ] **Phase 13: Agent QA Playbook** — Document validation procedures for future efficiency
- [ ] **Phase 14: Frontend Test Infrastructure** — Vitest setup and key component tests
- [ ] **Phase 15: ts-rs Type Generation** — Auto-generate TypeScript types from Rust DTOs
- [ ] **Phase 16: E2E Data Assertions** — Verify data persistence in E2E tests
- [ ] **Phase 17: Feature Audit** — Audit all features and document issues

## Phase Details

### Phase 12: Backend Domain Tests
**Goal:** Domain layer has comprehensive unit tests for all modules with zero dependencies
**Depends on:** Nothing (first phase)
**Requirements:** TEST-01, TEST-02, TEST-03
**Success Criteria** (what must be TRUE):
  1. All domain modules (iam, inventory, sites, fleet) have unit tests
  2. Tests are inline in domain files with `#[cfg(test)]` attribute
  3. Business rules and validation logic are covered (status transitions, quantity checks)
  4. All tests pass in under 1 second (pure unit tests, no DB)
**Plans:** TBD

### Phase 13: Agent QA Playbook
**Goal:** Future agent sessions can efficiently validate features using documented procedures
**Depends on:** Nothing (can start immediately)
**Requirements:** QA-01, QA-02, QA-03, QA-04
**Success Criteria** (what must be TRUE):
  1. Playwright learnings documented (UI navigation patterns, common selectors)
  2. QA Playbook created in `.planning/QA-PLAYBOOK.md` (run tests, check logs, validate features)
  3. Parameter mismatch prevention strategy documented (ts-rs + type guards)
  4. ts-rs usage documented in AGENTS.md for future DTO additions
**Plans:** TBD

### Phase 14: Frontend Test Infrastructure
**Goal:** Frontend has test infrastructure and key component tests for regression prevention
**Depends on:** Phase 13 (uses QA learnings)
**Requirements:** TEST-04, TEST-05, TEST-06
**Success Criteria** (what must be TRUE):
  1. Vitest configured with React Testing Library, MSW, and jsdom
  2. Test utilities exist (render with providers, mock data factories, QueryClient setup)
  3. Key components tested (material form, site dialog, vehicle/tool dialogs)
  4. Tests run in CI without flaky failures
**Plans:** TBD
**UI hint:** yes

### Phase 15: ts-rs Type Generation
**Goal:** TypeScript types are auto-generated from Rust DTOs, preventing frontend-backend type drift
**Depends on:** Phase 12 (needs stable DTOs)
**Requirements:** TEST-07, TEST-08, TEST-09
**Success Criteria** (what must be TRUE):
  1. ts-rs derive macros added to all backend DTOs (request/response structs)
  2. TypeScript types generated to `frontend/src/types/generated.ts`
  3. CI pipeline fails if generated types differ from committed version
  4. Frontend imports types from generated file instead of manual definitions
**Plans:** TBD

### Phase 16: E2E Data Assertions
**Goal:** E2E tests verify data persistence through API calls, not just UI presence
**Depends on:** Phase 12, Phase 14
**Requirements:** TEST-10, TEST-11
**Success Criteria** (what must be TRUE):
  1. Existing E2E tests verify data persisted via API calls after UI actions
  2. Data assertion pattern documented in `tests/e2e/README.md` for future tests
**Plans:** TBD

### Phase 17: Feature Audit
**Goal:** All existing features audited with bugs, issues, and missing functionality documented
**Depends on:** Phase 13 (uses QA Playbook for systematic audit)
**Requirements:** AUDIT-01, AUDIT-02, AUDIT-03, AUDIT-04, AUDIT-05, AUDIT-06
**Success Criteria** (what must be TRUE):
  1. Baustellen feature audit complete with documented issues
  2. Inventory feature audit complete with documented issues
  3. Time Booking feature audit complete with documented issues
  4. Vehicles/Machines feature audit complete with documented issues
  5. Reservations feature audit complete with documented issues
  6. Comprehensive issue backlog in `.planning/ISSUE-BACKLOG.md`
**Plans:** TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 12. Backend Domain Tests | 0/1 | Not started | - |
| 13. Agent QA Playbook | 0/1 | Not started | - |
| 14. Frontend Test Infrastructure | 0/1 | Not started | - |
| 15. ts-rs Type Generation | 0/1 | Not started | - |
| 16. E2E Data Assertions | 0/1 | Not started | - |
| 17. Feature Audit | 0/1 | Not started | - |

## Coverage

| Category | Requirements | Phases |
|----------|-------------|--------|
| Backend Unit Tests | TEST-01, TEST-02, TEST-03 | Phase 12 |
| Agent QA Playbook | QA-01, QA-02, QA-03, QA-04 | Phase 13 |
| Frontend Testing | TEST-04, TEST-05, TEST-06 | Phase 14 |
| ts-rs Type Generation | TEST-07, TEST-08, TEST-09 | Phase 15 |
| E2E Data Assertions | TEST-10, TEST-11 | Phase 16 |
| Feature Audit | AUDIT-01 to AUDIT-06 | Phase 17 |

**Total:** 21/21 requirements mapped ✓

## Dependencies

```
Phase 12 ─┬─► Phase 15
          │
          └─► Phase 14 ──► Phase 16

Phase 13 ─┬─► Phase 14
          │
          └─► Phase 17
```

- Phase 12, 13 can run in parallel (independent)
- Phase 14 depends on Phase 13 (uses QA learnings)
- Phase 15 depends on Phase 12 (needs stable DTOs)
- Phase 16 depends on Phase 12, 14 (needs test patterns established)
- Phase 17 depends on Phase 13 (uses QA Playbook)

---

*Roadmap created: 2026-04-30*
*Reordered: 2026-04-30 — QA Playbook moved to Phase 13 for earlier frontend testing benefit*
