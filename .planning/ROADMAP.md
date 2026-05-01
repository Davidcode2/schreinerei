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
- 🚧 **v1.9 Inventory Features** — Phases 30-33 (in progress)

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

### 🚧 v1.9 Inventory Features (In Progress)

**Milestone Goal:** Make inventory fully manageable — edit anything, track everything, see the full history.

- [x] **Phase 30: Backend API Foundation** — Domain commands, migration, and API endpoints for categories, material editing, stock-in, and enriched history (completed 2026-05-01)
- [x] **Phase 31: Settings, Editing & Stock-In** — Category settings page, material edit dialog, stock-in dialog, and category on overview (completed 2026-05-01)
- [x] **Phase 32: Enriched History** — Color-coded history feed with user attribution and clickable Baustelle links (completed 2026-05-01)
- [x] **Phase 33: Type Safety & Coverage** — ts-rs type generation, validation tests, and E2E tests (completed 2026-05-01)

## Phase Details

### Phase 30: Backend API Foundation
**Goal:** All API endpoints exist for frontend to consume — categories, material edits, stock-in, and enriched history data
**Depends on:** Nothing (first phase of v1.9)
**Requirements**: (enabler phase — no v1 requirements mapped directly; all requirements in Phase 31-32 depend on this phase)
**Success Criteria** (what must be TRUE):
  1. Category update endpoint handles PATCH requests with name validation and returns updated category
  2. Category delete endpoint returns Conflict error when any material rows exist for the category so inventory history is preserved
  3. Material PATCH endpoint accepts location and min_quantity partial updates (Option<T> fields)
  4. Stock-in endpoint records positive quantity changes as MaterialAdded entries with notes
  5. Inventory history endpoint returns entry_type, user_name, and category_name for each entry
**Plans**:
- [x] 30-01-PLAN.md — Domain commands, migration, and repository methods
- [x] 30-02-PLAN.md — API endpoints and TypeScript type generation

### Phase 31: Settings, Editing & Stock-In
**Goal:** Users can manage categories, edit material properties, record stock-in, and see categories on the overview
**Depends on**: Phase 30
**Requirements**: CATS-01, CATS-02, CATS-03, EDIT-01, EDIT-02, EDIT-03, STOCK-01, VIEW-01
**Success Criteria** (what must be TRUE):
  1. User can navigate to inventory settings page via gear icon and edit or delete categories (with FK constraint error messaging)
  2. User can edit a material's location and minimum quantity through an edit dialog
  3. User can set available quantity to an arbitrary number (stock correction)
  4. User can record stock-in with amount and notes via a dedicated dialog
  5. Category name is displayed on each material entry in the inventory overview
**Plans**: 4 plans
- [x] 31-01-PLAN.md — Shared inventory mutation hooks, settings route, and inventory gear entrypoint
- [x] 31-02-PLAN.md — Category management UI and overview category labels
- [x] 31-03-PLAN.md — Material edit dialog, direct stock correction, and stock-in dialog
- [x] 31-04-PLAN.md — Gap closure for blocked category delete copy and regression coverage
**UI hint**: yes

### Phase 32: Enriched History
**Goal:** Inventory history is visually differentiated with color-coded types, user attribution, and navigable links
**Depends on**: Phase 31
**Requirements**: STOCK-02, HIST-01, HIST-02, HIST-03
**Success Criteria** (what must be TRUE):
  1. History events display color-coded badges by stock entry type (for example stock-in, withdrawal, adjustment)
  2. Each history entry shows the user who performed the action (e.g., "von Max Mustermann")
  3. Baustelle names in withdrawal entries are clickable links that navigate to the site detail page
  4. Stock-in records appear in the history feed with MaterialAdded type and visual distinction
**Plans**: 2 plans
 - [x] 32-01-PLAN.md — Add the dedicated enriched-history query hook and regression coverage
 - [x] 32-02-PLAN.md — Render enriched history badges, attribution, and Baustelle links on the detail page
**UI hint**: yes
**Deferred note**: `location_changed` and `min_quantity_changed` history entries are intentionally deferred until a separate audit model exists; do not force metadata edits into `stock_entries`.

### Phase 33: Type Safety & Coverage
**Goal:** Generated types are consistent between Rust and TypeScript, and all new flows have automated test coverage
**Depends on**: Phase 32
**Requirements**: (quality gate — no v1 requirements mapped directly; validates Phase 30-32 deliverables)
**Success Criteria** (what must be TRUE):
  1. All new backend DTOs have ts-rs exports matching committed frontend types with zero drift
  2. Backend validation tests cover category CRUD operations and FK constraint enforcement
  3. E2E tests verify settings page, stock-in dialog, and material edit flows end-to-end
  4. E2E tests verify history feed renders enriched entries with correct colors, attribution, and links
**Plans**: 3 plans
- [x] 33-01-PLAN.md — Align inventory frontend DTO usage with ts-rs generated bindings and verify drift stays at zero
- [x] 33-02-PLAN.md — Add backend validation coverage for inventory DTO semantics and category delete conflict guards
- [x] 33-03-PLAN.md — Add API-verified Playwright coverage for settings, editing, stock-in, and enriched history

## Progress

**Execution Order:**
Phases execute in numeric order: 30 → 31 → 32 → 33

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 30. Backend API Foundation | v1.9 | 2/2 | Complete    | 2026-05-01 |
| 31. Settings, Editing & Stock-In | v1.9 | 4/4 | Complete    | 2026-05-01 |
| 32. Enriched History | v1.9 | 2/2 | Complete    | 2026-05-01 |
| 33. Type Safety & Coverage | v1.9 | 3/3 | Complete   | 2026-05-01 |

---

*Roadmap last updated: 2026-05-01*
*Next: `/gsd-progress`*
