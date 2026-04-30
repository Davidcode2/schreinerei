# Roadmap: Schreinerei SaaS

## Milestones

- ✅ **v1.0 MVP** - Phases 1-5 (shipped 2026-04-29)
- ✅ **v1.1 Organization-Based Tenancy** - Phase 6 (shipped 2026-04-29)
- ✅ **v1.2 Frontend Polish** - Phase 7 (shipped 2026-04-29)
- ✅ **v1.3 Bug Fixes** - Phases 8-10 (shipped 2026-04-29)
- ✅ **v1.4 Core Feature Fixes** - Phases 11-13 (shipped 2026-04-30)
- ✅ **v1.5 Testing & Quality Foundation** - Phases 14-17 (shipped 2026-04-30)
- 🚧 **v1.6 User Experience & Missing Functionality** - Phases 18-21 (in progress)

## Phases

<details>
<summary>✅ v1.0 MVP (Phases 1-5) - SHIPPED 2026-04-29</summary>

### Phase 1: Setup & Auth Foundation
**Goal**: Project scaffolding and multi-tenant authentication
**Plans**: 3 plans

Plans:
- [x] 01-01: Project structure and dependencies
- [x] 01-02: Keycloak JWT validation
- [x] 01-03: Tenant context middleware

### Phase 2: Inventory Module
**Goal**: Material management with QR codes and stock tracking
**Plans**: 3 plans

Plans:
- [x] 02-01: Inventory domain and repository
- [x] 02-02: Inventory API routes
- [x] 02-03: QR code generation

### Phase 3: Sites Module
**Goal**: Construction site management with time tracking
**Plans**: 2 plans

Plans:
- [x] 03-01: Sites domain and API
- [x] 03-02: Time tracking functionality

### Phase 4: Fleet Module
**Goal**: Vehicle and tool reservations with calendar
**Plans**: 2 plans

Plans:
- [x] 04-01: Fleet domain and API
- [x] 04-02: Reservation system

### Phase 5: PWA Frontend
**Goal**: Mobile-first progressive web app with offline support
**Plans**: 2 plans

Plans:
- [x] 05-01: React PWA setup with routing
- [x] 05-02: Offline sync with IndexedDB

</details>

<details>
<summary>✅ v1.1 Organization-Based Tenancy (Phase 6) - SHIPPED 2026-04-29</summary>

### Phase 6: Keycloak Organizations
**Goal**: Migrate to Keycloak Organizations for native multi-tenant isolation
**Plans**: 3 plans

Plans:
- [x] 06-01: Database migration for organization ID
- [x] 06-02: Backend JWT organization claim
- [x] 06-03: Frontend organization scope request

</details>

<details>
<summary>✅ v1.2 Frontend Polish (Phase 7) - SHIPPED 2026-04-29</summary>

### Phase 7: Dialog Implementation
**Goal**: All create dialogs functional and connected to APIs
**Plans**: 3 plans

Plans:
- [x] 07-01: Material and site dialogs
- [x] 07-02: Vehicle and tool dialogs
- [x] 07-03: User invitation dialog

</details>

<details>
<summary>✅ v1.3 Bug Fixes (Phases 8-10) - SHIPPED 2026-04-29</summary>

### Phase 8: Auth Fixes
**Goal**: Token exchange and authentication flow stable
**Plans**: 3 plans

Plans:
- [x] 08-01: Token exchange retry logic
- [x] 08-02: Session persistence
- [x] 08-03: E2E auth tests

### Phase 9: UI Fixes
**Goal**: Fleet dropdown and user management working
**Plans**: 3 plans

Plans:
- [x] 09-01: Fleet "Neu" dropdown menu
- [x] 09-02: User management with real data
- [x] 09-03: Sync status toasts

### Phase 10: E2E Test Suite
**Goal**: 18 E2E tests for regression prevention
**Plans**: 3 plans

Plans:
- [x] 10-01: Inventory E2E tests
- [x] 10-02: Sites and time tracking E2E tests
- [x] 10-03: Fleet and reservation E2E tests

</details>

<details>
<summary>✅ v1.4 Core Feature Fixes (Phases 11-13) - SHIPPED 2026-04-30</summary>

### Phase 11: FK Constraint Resolution
**Goal**: Resolve Keycloak user ID to local user ID for all FK references
**Plans**: 3 plans

Plans:
- [x] 11-01: User resolution pattern
- [x] 11-02: Auto-provisioning users
- [x] 11-03: WorkType enum alignment

### Phase 12: Calendar & Reservation Fixes
**Goal**: Calendar accepts multiple date formats, nullable user names
**Plans**: 2 plans

Plans:
- [x] 12-01: Date format flexibility
- [x] 12-02: Nullable user name handling

### Phase 13: Integration Verification
**Goal**: All core features working end-to-end
**Plans**: 1 plan

Plans:
- [x] 13-01: Full integration test

</details>

<details>
<summary>✅ v1.5 Testing & Quality Foundation (Phases 14-17) - SHIPPED 2026-04-30</summary>

### Phase 14: Backend Domain Tests
**Goal**: 116 backend tests for domain logic validation
**Plans**: 2 plans

Plans:
- [x] 14-01: Domain entity tests
- [x] 14-02: State machine tests

### Phase 15: ts-rs Type Generation
**Goal**: Auto-generate TypeScript types from Rust DTOs
**Plans**: 2 plans

Plans:
- [x] 15-01: Add ts-rs to all DTOs
- [x] 15-02: Frontend type imports

### Phase 16: Frontend Testing
**Goal**: Vitest + MSW + Testing Library infrastructure
**Plans**: 2 plans

Plans:
- [x] 16-01: MSW handlers setup
- [x] 16-02: Component tests

### Phase 17: E2E Data Assertions
**Goal**: E2E tests verify data persistence through API calls
**Plans**: 2 plans

Plans:
- [x] 17-01: API-based assertions
- [x] 17-02: QA Playbook documentation

</details>

### 🚧 v1.6 User Experience & Missing Functionality (In Progress)

**Milestone Goal:** Make the app user-friendly and complete missing core functionalities.

#### ✅ Phase 18: Bug Fixes & UX Improvements (Complete)
**Goal**: Fix validation bugs and wire existing UX features
**Depends on**: Phase 17
**Requirements**: FIX-01, FIX-02, UX-02
**Success Criteria** (what must be TRUE):
  1. User cannot submit time entry with zero or negative hours
  2. User sees inline error messages below form fields when input is invalid
  3. User can initiate QR scan by clicking QR code button on inventory page
**Plans**: 2 plans
**UI hint**: yes

> **Note:** UX-01 (low stock badge) is already implemented in MaterialCard.tsx - shows AlertTriangle icon and StatusBadge when `is_low_stock` is true.

Plans:
- [x] 18-01: TimeEntryDialog validation improvements (FIX-01, FIX-02)
- [x] 18-02: QR button wiring (UX-02)

#### ✅ Phase 19: Delete Operations (Complete)
**Goal**: Users can safely delete entities with confirmation and clear error feedback
**Depends on**: Phase 18
**Requirements**: DEL-01, DEL-02, DEL-03, DEL-04, DEL-05
**Success Criteria** (what must be TRUE):
  1. User can delete a site with confirmation dialog (soft delete)
  2. User can delete a material with confirmation dialog (soft delete)
  3. User can delete a vehicle with confirmation dialog (soft delete)
  4. User can delete a tool with confirmation dialog (soft delete)
  5. User sees dependency conflict message when delete is blocked by FK constraints
**Plans**: 3 plans
**UI hint**: yes

Plans:
- [x] 19-01: Soft delete migration + DELETE routes for materials and sites (DEL-01, DEL-02, DEL-05)
- [x] 19-02: Fleet dependency checks for vehicles and tools (DEL-03, DEL-04, DEL-05)
- [x] 19-03: Frontend AlertDialog + delete buttons with error handling (DEL-01, DEL-02, DEL-03, DEL-04, DEL-05)

#### Phase 20: Edit & Reservation Workflow
**Goal**: Users can edit records and manage reservations through complete workflow
**Depends on**: Phase 19
**Requirements**: EDIT-01, EDIT-02, EDIT-03, RESV-01, RESV-02, RESV-03
**Success Criteria** (what must be TRUE):
  1. User can edit an existing time entry (hours, work type, notes)
  2. User can delete their own time entries
  3. User can edit an existing reservation (dates, resource, notes)
  4. User can transition reservation status via UI buttons (confirm, start, complete, cancel)
  5. User can create a reservation by clicking empty time slots in calendar view
  6. User sees which existing reservation conflicts when availability warning appears
**Plans**: TBD
**UI hint**: yes

Plans:
- [ ] 20-01: Backend update routes
- [ ] 20-02: Frontend edit UI for time entries
- [ ] 20-03: Frontend edit UI for reservations
- [ ] 20-04: Status transition buttons
- [ ] 20-05: Calendar click-to-create

#### Phase 21: E2E Test Coverage
**Goal**: All new functionality verified through E2E tests
**Depends on**: Phase 20
**Requirements**: TEST-12, TEST-13, TEST-14, TEST-15
**Success Criteria** (what must be TRUE):
  1. E2E tests verify delete operations on all entity types
  2. E2E tests verify edit operations on time entries and reservations
  3. E2E tests verify reservation status transitions
  4. E2E tests verify calendar click-to-create reservation
**Plans**: TBD

Plans:
- [ ] 21-01: Delete operations E2E tests
- [ ] 21-02: Edit operations E2E tests
- [ ] 21-03: Status transitions E2E tests
- [ ] 21-04: Calendar click-to-create E2E tests

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → ... → 17 → 18 → 19 → 20 → 21

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Setup & Auth | v1.0 | 3/3 | Complete | 2026-04-29 |
| 2. Inventory | v1.0 | 3/3 | Complete | 2026-04-29 |
| 3. Sites | v1.0 | 2/2 | Complete | 2026-04-29 |
| 4. Fleet | v1.0 | 2/2 | Complete | 2026-04-29 |
| 5. PWA Frontend | v1.0 | 2/2 | Complete | 2026-04-29 |
| 6. Keycloak Orgs | v1.1 | 3/3 | Complete | 2026-04-29 |
| 7. Dialogs | v1.2 | 3/3 | Complete | 2026-04-29 |
| 8. Auth Fixes | v1.3 | 3/3 | Complete | 2026-04-29 |
| 9. UI Fixes | v1.3 | 3/3 | Complete | 2026-04-29 |
| 10. E2E Suite | v1.3 | 3/3 | Complete | 2026-04-29 |
| 11. FK Resolution | v1.4 | 3/3 | Complete | 2026-04-30 |
| 12. Calendar Fixes | v1.4 | 2/2 | Complete | 2026-04-30 |
| 13. Integration | v1.4 | 1/1 | Complete | 2026-04-30 |
| 14. Backend Tests | v1.5 | 2/2 | Complete | 2026-04-30 |
| 15. ts-rs Types | v1.5 | 2/2 | Complete | 2026-04-30 |
| 16. Frontend Tests | v1.5 | 2/2 | Complete | 2026-04-30 |
| 17. E2E Data | v1.5 | 2/2 | Complete | 2026-04-30 |
| 18. Bug Fixes & UX | v1.6 | 2/2 | Complete | 2026-04-30 |
| 19. Delete Ops | v1.6 | 3/3 | Complete | 2026-04-30 |
| 20. Edit & Resv | v1.6 | 0/5 | Not started | - |
| 21. E2E Tests | v1.6 | 0/4 | Not started | - |

---
*Last updated: 2026-04-30*
