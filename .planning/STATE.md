---
gsd_state_version: 1.0
milestone: v1.14
milestone_name: Project Costing, Planning & Billing Basis
status: ready
stopped_at: Milestone v1.14 complete and next seed ready for activation
last_updated: "2026-05-08T15:30:00Z"
last_activity: 2026-05-08 -- Completed Phase 51
progress:
  total_phases: 5
  completed_phases: 5
  total_plans: 5
  completed_plans: 5
  percent: 100
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-05)

**Core value:** Mitarbeiter finden alles schnell, Chefs haben den Uberblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.
**Current focus:** Milestone v1.14 is complete. The next step is choosing the next captured milestone seed to activate.

## Current Position

Phase: —
Plan: —
Status: v1.14 complete, ready for next milestone activation
Last activity: 2026-05-08 -- Completed Phase 51

## Performance Metrics

**Velocity:**

- Total plans completed across merged milestones: 30
- Milestones merged into this branch: 5

**By Phase:**

| Phase | Plans | Status |
|-------|-------|--------|
| 30. Backend API Foundation | 2/2 | Complete |
| 31. Settings, Editing & Stock-In | 4/4 | Complete |
| 32. Enriched History | 2/2 | Complete |
| 33. Type Safety & Coverage | 5/5 | Complete |
| 34. Camera Upload Flow | 1/1 | Complete |
| 35. Document Upload Rework | 3/3 | Complete |
| 36. Media Viewer | 3/3 | Complete |
| 37. Entry Management | 2/2 | Complete |
| 38. Fleet Page Calendar Integration | 1/1 | Complete |
| 39. Range Selection & Confirmation Flow | 2/2 | Complete |
| 40. Calendar Visibility, Colors & Cleanup | 2/2 | Complete |
| 41. Projects Boundary Alias | 1/1 | Complete |
| 42. Request Context Extractor | 1/1 | Complete |
| 43. Mobile-First Guardrails | 1/1 | Complete |
| 44. Project Model Foundation | 3/3 | Complete |
| Phase 45. Unified Project Timeline | 2/2 | Complete |
| Phase 46. Project-Linked Execution Capture | 1/1 | Complete |
| Phase 47. Project Dashboard Visibility | 1/1 | Complete |
| Phase 48. Project Costing Aggregates | 1/1 | Complete |
| Phase 49. Project Budget & Billing Metadata | 1/1 | Complete |
| Phase 50. Invoice-Ready Project Summary | 1/1 | Complete |
| Phase 51. Historical Project Reporting | 1/1 | Complete |
| Phase 52. Project Planning View | 1/1 | Complete |

## Accumulated Context

### Decisions

- v1.9 inventory settings live at `/settings/inventory` instead of a generic settings fallback.
- v1.9 stock-in is a separate command from stock adjustment.
- v1.9 enriched inventory history stays stock-movement-focused; metadata-only edit audit entries remain deferred.
- v1.10 keeps camera upload separate from document composition.
- v1.10 preserves upload-first, activity-second orchestration for attachments.
- v1.10 exposes only backend-derived `can_delete` for activity deletion.
- v1.10 uses route-backed media viewer overlays for shareable deep links.
- v1.11 embeds the calendar on `/fleet` and removes the standalone calendar entry path.
- v1.11 reservation creation uses a two-tap date-range flow with a bottom confirmation sheet.
- v1.11 resource colors are deterministic frontend-derived accents, not backend-managed state.
- v1.12 introduces `projects` as a safe architectural alias over the current `sites` module.
- v1.12 makes `TenantContext` an extractor so handlers stop rebuilding request context manually.
- v1.12 codifies mobile-first delivery in `.planning/MOBILE-FIRST-CHECKLIST.md`.
- [Phase 45]: Use one shared composer component for both note and camera entrypoints. — Mixed-media validation, preview cleanup, and upload ordering now live in one place.
- [Phase 45]: Keep unified submissions on the existing attachment-backed note activity path. — This reuses the protected attachment model and avoids new photo-only timeline records.
- [Phase 45]: Promote the timeline above secondary management cards on the detail page. — Workers now see the canonical project memory before planning and administrative panels.
- [Phase 45]: Show both exact and relative timestamps on each timeline card. — The existing created_at field remains the single source of truth while adding more precise context.
- [Phase 48]: Reuse existing project-linked time and stock booking rows as the only cost-basis source of truth. — No accounting shadow tables should be introduced before billing semantics exist.
- [Phase 48]: Aggregate labor and material separately, then compose the result. — This avoids SQL join multiplication between time rows and stock rows.
- [Phase 49]: Keep budget and billing metadata on the existing project aggregate. — Persist lightweight fields on `sites` and keep actuals on the summary endpoint.
- [Phase 49]: Do not fabricate monetary actuals without price or rate inputs. — Show budget beside operational actuals until invoice-ready export work adds richer billing semantics.
- [Phase 50]: Keep invoice-ready output as a dedicated read-only export contract. — Reuse project metadata and actuals without collapsing them into the existing site or summary DTOs.
- [Phase 50]: Export structured JSON first. — No PDF generation, pricing engine, or tax logic should be introduced in this phase.
- [Phase 51]: Keep historical reporting on the projects surface. — Use read-only filtered report rows instead of broadening the dashboard into a manager KPI suite.
- [Phase 52]: Reuse the fleet calendar with a `site_id` filter. — Project planning stays on the project detail page rather than becoming a second planner.

### Pending Todos

- Choose and activate the next milestone seed.
- Use `.planning/FEATURES.md` as the comparison baseline for shipped vs desired product scope.
- Use `.planning/REQUIREMENTS.md` `Product Backlog from 2026-05 Note` section to slice the next milestone.

### Blockers/Concerns

- Metadata-only inventory audit model is still unresolved and intentionally deferred.
- Real PostgreSQL integration coverage remains intentionally deferred to a future milestone.
- Fleet calendar local-day derivation may still need timezone-safe cleanup later.
- The newly captured product note is much broader than a single milestone; future work must stay sliced by bounded context and delivery risk.

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| History model | `location_changed` / `min_quantity_changed` audit entries need a separate model | Deferred | v1.9 |
| Offline | Photo queue replay runtime coverage | Backlog | v1.8 |
| Testing | Integration tests with real PostgreSQL | v2.0 | v1.5 |
| Fleet UX | Timezone-sensitive local day-key handling | Monitor | v1.11 |
| Fleet UX | Resource colors are deterministic but not uniqueness-proven | Monitor | v1.11 |

## Session Continuity

Last session: 2026-05-08T15:30:00Z
Stopped at: Milestone v1.14 complete and next seed ready for activation
Resume file: `.planning/phases/51-historical-project-reporting/51-PLAN.md`
