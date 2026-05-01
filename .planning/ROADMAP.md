# Roadmap: Schreinerei

## Milestones

- ✅ **v1.0 MVP** — Phases 1-5 (shipped 2026-04-29) — [Archive](milestones/v1-ROADMAP.md)
- ✅ **v1.1 Organization Tenancy** — Phase 6 (shipped 2026-04-29) — [Archive](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Frontend Polish** — Phase 7 (shipped 2026-04-29) — [Archive](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Bug Fixes** — Phases 8-10 (shipped 2026-04-29) — [Archive](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Core Feature Fixes** — Phase 11 (shipped 2026-04-30) — [Archive](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Testing & Quality** — Phases 12-17 (shipped 2026-04-30) — [Archive](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 UX & Missing Features** — Phases 18-21 (shipped 2026-04-30) — [Archive](milestones/v1.6-ROADMAP.md)
- ✅ **v1.7 Active Project Context** — Phases 22-25 (shipped 2026-04-30) — [Archive](milestones/v1.7-ROADMAP.md)
- ✅ **v1.8 Activity Feed & Site Status** — Phases 26-29 (shipped 2026-05-01) — [Archive](milestones/v1.8-ROADMAP.md)
- ⏸ **v1.9 Inventory Features** — Phase 30 completed, Phases 31-33 deferred
- 🚧 **v1.11 Fleet Calendar on Fleet Page** — Phases 34-36 (in progress)

## Phases

<details>
<summary>✅ v1.0-v1.8 (Phases 1-29) — Shipped</summary>

- [x] Phase 1: Auth & IAM Foundation
- [x] Phase 2: Inventory Management
- [x] Phase 3: Construction Sites
- [x] Phase 4: Fleet & Tools
- [x] Phase 5: PWA & Offline
- [x] Phase 6: Organization Tenancy
- [x] Phase 7: Frontend Polish
- [x] Phase 8-10: Bug Fixes
- [x] Phase 11: Core Feature Fixes
- [x] Phase 12-17: Testing & Quality
- [x] Phase 18-21: UX & Missing Features
- [x] Phase 22-25: Active Project Context
- [x] Phase 26-29: Activity Feed & Site Status

</details>

### ⏸ v1.9 Inventory Features (Deferred)

**Milestone Goal:** Make inventory fully manageable — edit anything, track everything, see the full history.

- [x] **Phase 30: Backend API Foundation** — Domain commands, migration, and API endpoints for categories, material editing, stock-in, and enriched history (completed 2026-05-01)
- [ ] **Phase 31: Settings, Editing & Stock-In** — Deferred
- [ ] **Phase 32: Enriched History** — Deferred
- [ ] **Phase 33: Type Safety & Coverage** — Deferred

### 🚧 v1.11 Fleet Calendar on Fleet Page (In Progress)

**Milestone Goal:** Make fleet reservations faster and clearer by embedding the calendar on the fleet page and replacing first-tap modal booking with an explicit date-range selection flow.

- [ ] **Phase 34: Fleet Page Calendar Integration** — Embed the existing calendar experience into the fleet page and preserve the current fleet content below it
- [ ] **Phase 35: Range Selection & Confirmation Flow** — Replace first-tap booking with two-tap range selection, confirmation, cancel, and optional times
- [ ] **Phase 36: Calendar Visibility, Colors & Cleanup** — Preserve booking visibility, add stable resource colors, remove the old primary calendar entry point, and cover regressions

## Phase Details

### Phase 34: Fleet Page Calendar Integration
**Goal:** Users can see and use the fleet calendar directly from the fleet page without losing the existing fleet content below it
**Depends on:** Nothing (first phase of v1.11)
**Requirements**: FCAL-01, FCAL-02
**Success Criteria** (what must be TRUE):
  1. Fleet page renders the reservation calendar as the first major section below the page header
  2. Existing fleet tabs and lists remain usable below the embedded calendar
  3. Embedded calendar reuses the current fleet calendar data source instead of introducing a duplicate booking implementation
**Plans**: TBD
**UI hint**: yes

### Phase 35: Range Selection & Confirmation Flow
**Goal:** Users can create reservations by selecting a date range first, then confirming it in a bottom-positioned modal
**Depends on:** Phase 34
**Requirements**: FSEL-01, FSEL-02, FSEL-03, FSEL-04, FCONF-01, FCONF-02, FCONF-03, FCONF-04
**Success Criteria** (what must be TRUE):
  1. First tap on a day starts a pending selection for that resource instead of opening the reservation dialog immediately
  2. Second tap on the same resource completes a reservation range, including same-day bookings
  3. The app always sorts the selected dates so the earlier day becomes the start of the reservation
  4. A bottom-positioned confirmation modal appears after the second tap and shows the selected range
  5. User can cancel to clear the selection or optionally enable time entry before confirming
**Plans**: TBD
**UI hint**: yes

### Phase 36: Calendar Visibility, Colors & Cleanup
**Goal:** The embedded calendar clearly shows current reservations, uses stable resource colors, and replaces the old primary calendar entry path with regression coverage
**Depends on:** Phase 35
**Requirements**: FCAL-03, FCONF-05, FCONF-06
**Success Criteria** (what must be TRUE):
  1. Existing reserved date ranges remain clearly visible while a user is making a new selection
  2. Vehicles and tools render with stable unique colors derived from their identity rather than row position
  3. Users no longer rely on the separate fleet calendar entry point to access the main booking experience
  4. Automated coverage protects the embedded calendar flow and range-selection regressions
**Plans**: TBD
**UI hint**: yes

## Progress

**Execution Order:**
Phases execute in numeric order: 34 → 35 → 36

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 34. Fleet Page Calendar Integration | v1.11 | 0/? | Not started | - |
| 35. Range Selection & Confirmation Flow | v1.11 | 0/? | Not started | - |
| 36. Calendar Visibility, Colors & Cleanup | v1.11 | 0/? | Not started | - |

---

*Roadmap last updated: 2026-05-01*
*Next: `/gsd-plan-phase 34`*
