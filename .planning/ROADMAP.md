# Roadmap: Schreinerei SaaS

## Milestones

- ✅ **v1.0 MVP** - Phases 1-5 (shipped 2026-04-29)
- ✅ **v1.1 Organization-Based Tenancy** - Phase 6 (shipped 2026-04-29)
- ✅ **v1.2 Frontend Polish** - Phase 7 (shipped 2026-04-29)
- ✅ **v1.3 Bug Fixes** - Phases 8-10 (shipped 2026-04-29)
- ✅ **v1.4 Core Feature Fixes** - Phase 11 (shipped 2026-04-30)
- ✅ **v1.5 Testing & Quality Foundation** - Phases 12-17 (shipped 2026-04-30)
- ✅ **v1.6 User Experience & Missing Functionality** - Phases 18-21 (shipped 2026-04-30)
- 🚧 **v1.7 Active Project Context** - Phases 22-24 (in progress)

## Phases

### ✅ Completed Milestones

<details>
<summary>v1.0-v1.6 (Phases 1-21) — SHIPPED</summary>

**v1.0 MVP (Phases 1-5):** Complete SaaS application with multi-tenant auth, inventory, sites, fleet, and PWA.

**v1.1 Organization-Based Tenancy (Phase 6):** Keycloak Organizations for native multi-tenant isolation.

**v1.2 Frontend Polish (Phase 7):** Connected UI dialogs to backend APIs.

**v1.3 Bug Fixes (Phases 8-10):** Authentication fixes, UI bugs, and API integration.

**v1.4 Core Feature Fixes (Phase 11):** FK constraint resolution, user ID mapping.

**v1.5 Testing & Quality Foundation (Phases 12-17):** Domain tests, ts-rs, E2E assertions, QA playbook.

**v1.6 User Experience & Missing Functionality (Phases 18-21):** Delete/edit operations, reservation workflow, E2E coverage.

</details>

### 🚧 v1.7 Active Project Context (In Progress)

**Milestone Goal:** Enable automatic assignment of materials/tools to the currently active construction site.

- [ ] **Phase 22: Backend Foundation & User Preferences** - Store and validate user's active Baustelle preference
- [ ] **Phase 23: Frontend UI & Auto-Assignment** - Persistent indicator, toggle, and form pre-fill
- [ ] **Phase 24: Opt-Out Dialog & E2E Tests** - Confirmation dialog with auto-confirm and test coverage

## Phase Details

### Phase 22: Backend Foundation & User Preferences
**Goal**: Backend stores and validates user's active Baustelle preference
**Depends on**: Phase 21 (v1.6 complete)
**Requirements**: PREF-01, PREF-02, PREF-03, DEDU-01, DEDU-02, DEDU-03
**Success Criteria** (what must be TRUE):
  1. User can set their active Baustelle preference via API
  2. System validates Baustelle exists and is accessible (tenant-scoped)
  3. System clears preference if Baustelle becomes invalid (archived/deleted)
  4. Material deductions can be linked to a Baustelle (FK column added)
  5. Deduction details include Baustelle name when linked
**Plans**: 4 plans

Plans:
- [ ] 22-01-PLAN.md — UserPreferences Repository & Service with validation
- [ ] 22-02-PLAN.md — Add site_id to WithdrawMaterial command
- [x] 22-03-PLAN.md — API endpoints for preferences (GET/PATCH)
- [ ] 22-04-PLAN.md — Deduction history with Baustelle name

### Phase 23: Frontend UI & Auto-Assignment
**Goal**: Users can see and change their active Baustelle, with auto-assignment to forms
**Depends on**: Phase 22
**Requirements**: ACTV-01, ACTV-02, ACTV-03, ACTV-04, ACTV-05, ACTV-06, ACTV-07, AUTO-01, AUTO-02, AUTO-03, AUTO-04
**Success Criteria** (what must be TRUE):
  1. User sees persistent indicator showing active Baustelle name and color
  2. User can toggle active Baustelle from overview page or dashboard
  3. Only one Baustelle can be active per user at a time
  4. Active state persists across page navigation and browser refresh
  5. Material, reservation, and time entry forms pre-fill active Baustelle (changeable)
**Plans**: TBD
**UI hint**: yes

### Phase 24: Opt-Out Dialog & E2E Tests
**Goal**: Users can confirm or change auto-assignments with unobtrusive dialog, verified by tests
**Depends on**: Phase 23
**Requirements**: DLOG-01, DLOG-02, DLOG-03, DLOG-04, DLOG-05, TEST-16, TEST-17, TEST-18
**Success Criteria** (what must be TRUE):
  1. Confirmation dialog shows on auto-assignment (non-blocking)
  2. Dialog auto-confirms after 5 seconds if user takes no action
  3. User can change project or dismiss to leave unassigned from dialog
  4. E2E tests verify active Baustelle workflow end-to-end
**Plans**: TBD
**UI hint**: yes

## Progress

**Execution Order:** Phases execute in numeric order: 22 → 23 → 24

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 22. Backend Foundation | v1.7 | 1/4 | In progress | 22-03 |
| 23. Frontend UI & Auto-Assignment | v1.7 | 0/TBD | Not started | - |
| 24. Opt-Out Dialog & E2E Tests | v1.7 | 0/TBD | Not started | - |

---

*Roadmap updated: 2026-04-30 — Plan 22-03 completed*
