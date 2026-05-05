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

## Progress

**Execution Order:**
Phases execute in numeric order: 30 → 31 → 32 → 33 → 34 → 35 → 36 → 37 → 38 → 39 → 40 → 41 → 42 → 43

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

## Next Planning Inputs

The next milestone is not defined yet. Use these planning inputs before starting the next GSD cycle:

- `.planning/FEATURES.md` for the current-vs-desired feature comparison
- `.planning/REQUIREMENTS.md` section `Product Backlog from 2026-05 Note` for requirement slicing

Strong next-milestone candidates:

1. Project feed completion and dashboard behavior
2. Expiry-aware inventory and replenishment alerts
3. Asset maintenance intervals and reminders
4. Project-linked analytics, budget, and invoice foundation

---

*Roadmap last updated: 2026-05-04*
*Next: `/gsd-new-milestone`*
