# Roadmap: Schreinerei SaaS

## Milestones

- ✅ **v1.0 MVP** - Phases 1-5 (shipped 2026-04-29)
- ✅ **v1.1 Organization-Based Tenancy** - Phase 6 (shipped 2026-04-29)
- ✅ **v1.2 Frontend Polish** - Phase 7 (shipped 2026-04-29)
- ✅ **v1.3 Bug Fixes** - Phases 8-10 (shipped 2026-04-29)
- ✅ **v1.4 Core Feature Fixes** - Phase 11 (shipped 2026-04-30)
- ✅ **v1.5 Testing & Quality Foundation** - Phases 12-17 (shipped 2026-04-30)
- ✅ **v1.6 User Experience & Missing Functionality** - Phases 18-21 (shipped 2026-04-30)
- 🚧 **v1.7 Gap Closure** - Phases 24-25 (planned 2026-04-30)

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

- [x] **Phase 22: Backend Foundation & User Preferences** - Store and validate user's active Baustelle preference ✅
- [x] **Phase 23: Frontend UI & Auto-Assignment** - Persistent indicator, toggle, and form pre-fill ✅

### 🚧 v1.7 Gap Closure (Planned)

**Milestone Goal:** Close audit gaps from v1.7 milestone verification and restore full end-to-end coverage.

- [x] **Phase 24: Phase Verification & Requirement Revalidation** - Add missing verification artifacts for Phases 22/23 and revalidate requirement evidence ✅
- [ ] **Phase 25: Deduction History Site Name End-to-End Wiring** - Wire frontend consumer/render path for history `site_name` and close DEDU-03

## Phase Details

### ✅ Phase 22: Backend Foundation & User Preferences
**Goal**: Backend stores and validates user's active Baustelle preference
**Depends on**: Phase 21 (v1.6 complete)
**Requirements**: PREF-01, PREF-02, PREF-03, DEDU-01, DEDU-02, DEDU-03
**Completed**: 2026-04-30

**Success Criteria Met:**
1. ✅ User can set their active Baustelle preference via API
2. ✅ System validates Baustelle exists and is accessible (tenant-scoped)
3. ✅ System clears preference if Baustelle becomes invalid (archived/deleted)
4. ✅ Material deductions can be linked to a Baustelle (FK column added)
5. ✅ Deduction details include Baustelle name when linked

**Plans Completed:**
- [x] 22-01 — UserPreferences Repository & Service with validation
- [x] 22-02 — Add site_id to WithdrawMaterial command
- [x] 22-03 — API endpoints for preferences (GET/PATCH)
- [x] 22-04 — Deduction history with Baustelle name

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
**Plans**: 4 plans
Plans:
- [x] 23-01-PLAN.md — Preferences hooks, deterministic color utility, and persistent active-site indicator in app layout
- [x] 23-02-PLAN.md — Active-site toggle controls on sites overview and dashboard cards
- [x] 23-03-PLAN.md — Auto-prefill active site in withdrawal/reservation/time-entry forms with user override
- [x] 23-04-PLAN.md — Fix preferences FK-safe local-user mapping for active-site toggle (UAT gap closure)
**UI hint**: yes

## Progress

**Execution Order:** Phases execute in numeric order: 22 → 23

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 22. Backend Foundation | v1.7 | 4/4 | ✅ Complete | 2026-04-30 |
| 23. Frontend UI & Auto-Assignment | v1.7 | 4/4 | ✅ Complete | 2026-04-30 |
| 24. Phase Verification & Requirement Revalidation | v1.7 | 2/2 | ✅ Complete | 2026-04-30 |
| 25. Deduction History Site Name End-to-End Wiring | v1.7 | 0/0 | 📝 Planned | — |

---

### Phase 24: Phase Verification & Requirement Revalidation
**Goal**: Produce missing verification artifacts for Phases 22 and 23 and revalidate requirement evidence coverage.
**Depends on**: Phase 23
**Requirements**: PREF-01, PREF-02, PREF-03, DEDU-01, DEDU-02, ACTV-01, ACTV-02, ACTV-03, ACTV-04, ACTV-05, ACTV-06, ACTV-07, AUTO-01, AUTO-02, AUTO-03, AUTO-04
**Plans**: 2 plans
Plans:
- [x] 24-01-PLAN.md — Create Phase 22 verification artifact and backend requirement revalidation rows (PREF/DEDU)
- [x] 24-02-PLAN.md — Create Phase 23 verification artifact and complete consolidated ACTV/AUTO revalidation verdict
**Gap Closure**: Closes milestone audit verification gaps for requirements and missing phase verification evidence.

### Phase 25: Deduction History Site Name End-to-End Wiring
**Goal**: Wire frontend consumption/rendering of deduction history `site_name` so backend producer is consumed end-to-end.
**Depends on**: Phase 24
**Requirements**: DEDU-03
**Gap Closure**: Closes integration + flow gap where history endpoint data is not consumed by frontend.

---

*Roadmap updated: 2026-04-30 — Added gap-closure phases 24-25 after v1.7 milestone audit (v1.7-MILESTONE-AUDIT.md)*
