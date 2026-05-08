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
- ✅ **v1.9 Inventory Features** — Phases 30-33 (shipped 2026-05-01) — [Archive](milestones/v1.9-ROADMAP.md)
- ✅ **v1.10 Baustelle Activity Stream Features** — Phases 34-37 (shipped 2026-05-01) — [Archive](milestones/v1.10-ROADMAP.md)
- ✅ **v1.11 Fleet Calendar on Fleet Page** — Phases 38-40 (shipped 2026-05-01) — [Archive](milestones/v1.11-ROADMAP.md)
- ✅ **v1.12 Architecture Guardrails** — Phases 41-43 (shipped 2026-05-04) — [Archive](milestones/v1.12-ROADMAP.md)
- ✅ **v1.13 Project Workflow Foundation** — Phases 44-47 (shipped 2026-05-07)
- **v1.14 Project Costing, Planning & Billing Basis** — Phases 48-52 (planned)

### v1.12 Mobile Modal Improvements (In Progress)

**Milestone Goal:** Improve mobile UX by making tall modals usable on small screens through step-based navigation.

- [ ] **Phase 41: Mobile Modal Improvements** — Split tall modals into steps with swipe navigation, larger close button, and dot indicators

Plans:
- [x] 41-01-PLAN.md — Infrastructure: close button fix, StepIndicator, StepContainer, useSwipeGesture
- [x] 41-02-PLAN.md — Apply step-based layout to AddMaterialDialog

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

### ✅ v1.9 Inventory Features (Shipped)

**Milestone Goal:** Make inventory fully manageable — edit anything, track everything, see the full history.

- [x] **Phase 30: Backend API Foundation** — Domain commands, migration, and API endpoints for categories, material editing, stock-in, and enriched history
- [x] **Phase 31: Settings, Editing & Stock-In** — Category settings page, material edit dialog, stock-in dialog, and overview category labels
- [x] **Phase 32: Enriched History** — Color-coded history feed with user attribution and clickable Baustelle links
- [x] **Phase 33: Type Safety & Coverage** — ts-rs alignment, backend validation tests, and browser coverage

### ✅ v1.10 Baustelle Activity Stream Features (Shipped)

**Milestone Goal:** Extend the Baustelle activity stream with separate upload flows, attachment-backed entries, fullscreen media viewing, and creator-only deletion.

- [x] **Phase 34: Camera Upload Flow** — Dedicated camera-first upload flow on the site activity stream
- [x] **Phase 35: Document Upload Rework** — Multi-attachment document composer with mixed note/image/PDF entries
- [x] **Phase 36: Media Viewer** — Route-backed fullscreen viewer with share, download, and metadata
- [x] **Phase 37: Entry Management** — Creator-only deletion with confirmation and attachment cleanup

### ✅ v1.11 Fleet Calendar on Fleet Page (Shipped)

**Milestone Goal:** Make fleet reservations faster and clearer by moving booking directly onto `/fleet`.

- [x] **Phase 38: Fleet Page Calendar Integration** — Embed the calendar directly on the fleet page
- [x] **Phase 39: Range Selection & Confirmation Flow** — Two-tap date-range selection with bottom-sheet confirmation
- [x] **Phase 40: Calendar Visibility, Colors & Cleanup** — Keep reservations visible during selection, add stable resource colors, and remove the standalone calendar entry path

### ✅ v1.12 Architecture Guardrails (Shipped)

**Milestone Goal:** Harden the architecture around project boundaries, request-scoped tenant context, and mobile-first delivery guardrails.

- [x] **Phase 41: Projects Boundary Alias** — Expose a `projects` boundary over the current `sites` module without breaking runtime behavior
- [x] **Phase 42: Request Context Extractor** — Make `TenantContext` extractable and remove manual auth-to-context rebuilding from API routes
- [x] **Phase 43: Mobile-First Guardrails** — Codify the existing mobile-first baseline as an explicit checklist for future work

### ✅ v1.13 Project Workflow Foundation (Shipped)

**Milestone Goal:** Turn the current Baustelle surface into a clearer project workflow so workers capture context once, book productive work/material against the right project with less friction, and managers see all relevant projects without hidden defaults.

- [x] **Phase 44: Project Model Foundation** — Broaden the current site model into a project execution surface that supports both external Baustellen and internal workshop projects
- [x] **Phase 45: Unified Project Timeline** — Make the project timeline the canonical context channel and unify note/photo/document entry creation (completed 2026-05-07)
- [x] **Phase 46: Project-Linked Execution Capture** — Require and default project linkage for real material and productive time capture where that reduces manual input
- [x] **Phase 47: Project Dashboard Visibility** — Show relevant projects regardless of status and make filtering explicit in the dashboard experience

## Progress

**Execution Order:**
Phases execute in numeric order: 30 → 31 → 32 → 33 → 34 → 35 → 36 → 37 → 38 → 39 → 40 → 41 → 42 → 43 → 44 → 45 → 46 → 47 → 48 → 49 → 50 → 51 → 52

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 30. Backend API Foundation | v1.9 | 2/2 | Complete | 2026-05-01 |
| 31. Settings, Editing & Stock-In | v1.9 | 4/4 | Complete | 2026-05-01 |
| 32. Enriched History | v1.9 | 2/2 | Complete | 2026-05-01 |
| 33. Type Safety & Coverage | v1.9 | 5/5 | Complete | 2026-05-01 |
| 34. Camera Upload Flow | v1.10 | 1/1 | Complete | 2026-05-01 |
| 35. Document Upload Rework | v1.10 | 3/3 | Complete | 2026-05-01 |
| 36. Media Viewer | v1.10 | 3/3 | Complete | 2026-05-01 |
| 37. Entry Management | v1.10 | 2/2 | Complete | 2026-05-01 |
| 38. Fleet Page Calendar Integration | v1.11 | 1/1 | Complete | 2026-05-01 |
| 39. Range Selection & Confirmation Flow | v1.11 | 2/2 | Complete | 2026-05-01 |
| 40. Calendar Visibility, Colors & Cleanup | v1.11 | 2/2 | Complete | 2026-05-01 |
| 41. Projects Boundary Alias | v1.12 | 1/1 | Complete | 2026-05-04 |
| 42. Request Context Extractor | v1.12 | 1/1 | Complete | 2026-05-04 |
| 43. Mobile-First Guardrails | v1.12 | 1/1 | Complete | 2026-05-04 |
| 44. Project Model Foundation | v1.13 | 3/3 | Complete | 2026-05-05 |
| 45. Unified Project Timeline | v1.13 | 2/2 | Complete   | 2026-05-07 |
| 46. Project-Linked Execution Capture | v1.13 | 1/1 | Complete | 2026-05-07 |
| 47. Project Dashboard Visibility | v1.13 | 1/1 | Complete | 2026-05-07 |
| 48. Project Costing Aggregates | v1.14 | 1/1 | In Progress | — |
| 49. Project Budget & Billing Metadata | v1.14 | 0/1 | Planned | — |
| 50. Invoice-Ready Project Summary | v1.14 | 0/1 | Planned | — |
| 51. Historical Project Reporting | v1.14 | 0/1 | Planned | — |
| 52. Project Planning View | v1.14 | 0/1 | Planned | — |

## Next Planning Inputs

The current milestone is defined and ready for phase planning. Use these planning inputs before starting execution:

- `.planning/FEATURES.md` for the current-vs-desired feature comparison
- `.planning/REQUIREMENTS.md` section `Product Backlog from 2026-05 Note` for requirement slicing

Current milestone phase summary:

| # | Phase | Goal | Requirements | Success Criteria |
|---|-------|------|--------------|------------------|
| 44 | Project Model Foundation | Broaden `sites` into a practical project execution model | PROJ-10, PROJ-11 | 4 |
| 45 | Unified Project Timeline | Make one project timeline the canonical context channel | PROJ-18, PROJ-19, PROJ-20 | 4 |
| 46 | Project-Linked Execution Capture | Tie real material/time capture to projects with low-friction defaults | PROJ-13, PROJ-14 | 4 |
| 47 | Project Dashboard Visibility | Remove hidden status filtering from manager-facing project overview | PROJ-12 | 3 |

### Phase Details

### Phase 44: Project Model Foundation
Goal: Broaden the current site model into a project execution surface that supports external Baustellen and internal workshop work without creating a second competing entity.
Requirements: `PROJ-10`, `PROJ-11`
Success criteria:
1. User-facing project terminology is available without breaking current runtime behavior.
2. Project records can represent external and internal work contexts.
3. Project planning fields remain editable from the main project surface.
4. Existing data can continue to flow through the same aggregate without parallel entity creation.

**Plans:** 3 plans

Plans:
- [x] 44-01-PLAN.md — Extend the existing `sites` aggregate and API contracts with a project-type-capable backend model.
- [x] 44-02-PLAN.md — Turn the existing `/sites` create/detail flow into the main project planning and assignment surface.
- [x] 44-03-PLAN.md — Propagate project-aware terminology and selectors into active-context, time-booking, fleet, and dashboard surfaces.

### Phase 45: Unified Project Timeline
Goal: Make the project timeline the canonical context channel and unify note/photo/document creation into one flow.
Requirements: `PROJ-18`, `PROJ-19`, `PROJ-20`
Success criteria:
1. Users create one timeline entry that can include note text and attachments from a unified composer.
2. Project detail clearly presents the timeline as the main execution context surface.
3. Timeline entries render timestamps and attachment previews consistently.
4. Existing camera/document capabilities remain usable while converging on the unified flow.

### Phase 46: Project-Linked Execution Capture
Goal: Reduce office follow-up by ensuring real productive material/time capture is attributable to a project with minimal extra input.
Requirements: `PROJ-13`, `PROJ-14`
Success criteria:
1. Real material consumption is linked to a project unless it is disposal or correction.
2. Productive time booking can be linked to external or internal projects.
3. Active project context pre-fills the most common capture flows.
4. Users are not forced through extra manual steps when the active project is already known.

### Phase 47: Project Dashboard Visibility
Goal: Let managers see relevant projects regardless of status and apply filters explicitly instead of relying on hidden defaults.
Requirements: `PROJ-12`
Success criteria:
1. Dashboard includes relevant planned, active, and completed projects by default.
2. Status filtering is explicit and user-controlled.
3. The new behavior does not regress the current dashboard’s speed or readability.

### ✅ v1.14 Project Costing, Planning & Billing Basis (Planned)

**Milestone Goal:** Turn disciplined project-linked time and material capture into reusable project aggregates, billing metadata, invoice-ready summaries, project planning views, and historical reporting.

- [ ] **Phase 48: Project Costing Aggregates** — Add canonical per-project labor and material aggregates on project detail
- [ ] **Phase 49: Project Budget & Billing Metadata** — Add budget and billing metadata on the project and show budget-vs-actual
- [ ] **Phase 50: Invoice-Ready Project Summary** — Add structured invoice-ready summaries sourced from project aggregates and billing metadata
- [ ] **Phase 51: Historical Project Reporting** — Add read-only reporting filters for managers over historical projects
- [ ] **Phase 52: Project Planning View** — Show workers, reservations, and project timing together from a project-centric planning surface

---

*Roadmap last updated: 2026-05-08*
*Next: execute Phase 48 project costing aggregates*
