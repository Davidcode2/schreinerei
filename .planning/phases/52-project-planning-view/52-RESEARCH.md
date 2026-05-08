# Phase 52: Project Planning View - Research

**Researched:** 2026-05-08 [VERIFIED: system date]
**Domain:** Project-centric planning on the existing `sites` + fleet reservation model [VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/fleet/api/routes.rs]
**Confidence:** HIGH for current reuse locations; MEDIUM for the exact final API/UI shape because the project-scoped fleet filter does not exist yet [VERIFIED: codebase inspection][ASSUMED]

## User Constraints

- Reuse the existing assignment backend, the current project detail surface, and the existing fleet calendar/reservation model instead of inventing a second planner. [VERIFIED: user request]
- Return a concise research report covering reuse locations, backend/API shape, minimal UI, verification, and scope boundaries. [VERIFIED: user request]
- Do research only; do not write implementation code. [VERIFIED: user request]

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROJ-15 | Managers can assign workers and vehicles to projects and see the planning context in calendar form. [VERIFIED: .planning/REQUIREMENTS.md] | This research maps the existing worker-assignment, project-detail, and fleet-calendar seams to a single project-centric view without adding a second planner. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/sites/ProjectAssignmentsSection.tsx][VERIFIED: frontend/src/pages/fleet/CalendarView.tsx][ASSUMED] |
</phase_requirements>

## Summary

The repo already has almost every primitive Phase 52 needs: project timing fields live on `sites`, worker assignment CRUD already hangs off `/api/v1/sites/:id/assignments`, and fleet reservations already link to `site_id` and render in the embedded weekly calendar on `/fleet`. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/006_fleet_schema.sql][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/fleet/CalendarView.tsx]

The missing piece is not a new planner domain; it is a **project-scoped read path over the existing fleet planner**. Today the fleet calendar can filter by `resource_type`, but not by `site_id`, and calendar reservation chips expose `site_name` but not `site_id`, which makes exact project filtering on the project page unreliable if names collide. [VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs][VERIFIED: frontend/src/types/fleet.ts]

**Primary recommendation:** keep `/sites/:id` as the planning entry point, keep worker assignments on the existing site API, and extend the existing fleet calendar/read contracts with optional `site_id` filtering plus `site_id` on reservation summaries so `SiteDetailPage` can embed a project-filtered reservation calendar beside timing and assignments. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/fleet/api/routes.rs][ASSUMED]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Persist project timing fields and worker assignments | Database / Storage [VERIFIED: migrations/005_sites_schema.sql] | API / Backend [VERIFIED: src/modules/sites/infrastructure/site_repository.rs] | `sites` and `site_assignments` are already the durable planning sources of truth. [VERIFIED: migrations/005_sites_schema.sql] |
| Persist asset bookings linked to a project | Database / Storage [VERIFIED: migrations/006_fleet_schema.sql] | API / Backend [VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs] | `reservations.site_id` already ties bookings to projects. [VERIFIED: migrations/006_fleet_schema.sql][VERIFIED: src/modules/fleet/domain/reservation.rs] |
| Filter and shape project-specific planning data | API / Backend [VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: src/modules/sites/api/routes.rs] | Database / Storage [VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs] | Tenant-safe project filtering belongs in backend queries, not string matching in the client. [VERIFIED: AGENTS.md][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs][ASSUMED] |
| Render project-centric planning view | Browser / Client [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx] | API / Backend [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useFleet.ts] | The existing project detail page already composes multiple project queries and is the smallest surface to extend. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx] |

## 1. Current code/schema locations to reuse

1. **Project timing and metadata** — `migrations/005_sites_schema.sql` defines `start_date`, `end_date`, and `estimated_days` on `sites`, and `src/modules/sites/domain/site.rs` exposes them on the `Site` aggregate. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: src/modules/sites/domain/site.rs]
2. **Worker assignment backend** — `site_assignments` is defined in `migrations/005_sites_schema.sql`; CRUD lives in `src/modules/sites/infrastructure/site_repository.rs`, `src/modules/sites/application/site_service.rs`, and `src/modules/sites/api/routes.rs`. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/api/routes.rs]
3. **Current project detail surface** — `frontend/src/pages/sites/SiteDetailPage.tsx` already shows project timing, assignment count, the “Planen” action, and the existing `ProjectAssignmentsSection`. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/sites/ProjectAssignmentsSection.tsx]
4. **Project planning editor** — `frontend/src/pages/sites/ProjectPlanningSheet.tsx` already edits project type, start/end, and estimated days from the main project surface. [VERIFIED: frontend/src/pages/sites/ProjectPlanningSheet.tsx]
5. **Reservation data model** — `migrations/006_fleet_schema.sql` defines `reservations.site_id`; `src/modules/fleet/domain/reservation.rs` models `site_id: Option<SiteId>`. [VERIFIED: migrations/006_fleet_schema.sql][VERIFIED: src/modules/fleet/domain/reservation.rs]
6. **Fleet read APIs** — `src/modules/fleet/api/routes.rs` already exposes `GET /api/v1/fleet/reservations` and `GET /api/v1/fleet/calendar`; `src/modules/fleet/infrastructure/fleet_repository.rs` already joins reservations to `sites` for `site_name`. [VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs]
7. **Canonical booking UI** — `frontend/src/pages/fleet/CalendarView.tsx`, `ReservationConfirmationSheet.tsx`, and `ReservationDialog.tsx` already implement the weekly booking flow the project view should reuse. [VERIFIED: frontend/src/pages/fleet/CalendarView.tsx][VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx]
8. **Current query seams** — `frontend/src/lib/api/hooks/useSites.ts` and `frontend/src/lib/api/hooks/useFleet.ts` already split project detail, assignments, reservations, and calendar data into reusable hooks. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useFleet.ts]

## 2. Recommended backend/API shape

- **Do not add a new planning table or a second planner endpoint.** Reuse `sites`, `site_assignments`, and `reservations` as the only planning sources of truth. [VERIFIED: user request][VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/006_fleet_schema.sql][ASSUMED]
- **Keep project timing on `GET /api/v1/sites/:id` and worker assignments on `GET /api/v1/sites/:id/assignments`.** Those contracts already exist and match the user request to reuse the current project detail surface and assignment backend. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: frontend/src/lib/api/hooks/useSites.ts]
- **Extend `GET /api/v1/fleet/calendar` with an optional `site_id` query parameter.** This is the cleanest way to reuse the existing planner while letting the project page ask for “show me bookings relevant to this project.” [VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: frontend/src/lib/api/hooks/useFleet.ts][ASSUMED]
- **Extend `GET /api/v1/fleet/reservations` with an optional `site_id` query parameter** for list/detail side panels and future parity with the calendar filter. [VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs][ASSUMED]
- **Add `site_id` to `ReservationSummaryResponse`.** The calendar summary currently includes `site_name` but not `site_id`, which makes exact client-side project matching ambiguous. [VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: frontend/src/types/fleet.ts][ASSUMED]
- **Keep the reservation write contract unchanged** (`POST /api/v1/fleet/reservations` with `site_id`). The project view should pass the current project ID into the existing booking flow rather than inventing a separate project-booking mutation. [VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx][ASSUMED]

### Recommended contract sketch

| Endpoint | Change | Why |
|----------|--------|-----|
| `GET /api/v1/sites/:id` | no change [VERIFIED: src/modules/sites/api/routes.rs] | Already returns timing fields used by project planning. [VERIFIED: src/modules/sites/api/routes.rs] |
| `GET /api/v1/sites/:id/assignments` | no change [VERIFIED: src/modules/sites/api/routes.rs] | Already powers worker assignment UI. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts] |
| `GET /api/v1/fleet/calendar?start_date&end_date&resource_type?&site_id?` | add optional `site_id` filter [ASSUMED] | Reuses the existing calendar surface for project-scoped planning. [VERIFIED: frontend/src/pages/fleet/CalendarView.tsx][ASSUMED] |
| `GET /api/v1/fleet/reservations?resource_type?&resource_id?&user_id?&site_id?` | add optional `site_id` filter [ASSUMED] | Supports project-scoped reservation lists and edit affordances without a new aggregate DTO. [VERIFIED: src/modules/fleet/api/routes.rs][ASSUMED] |

## 3. Minimal UI surface

1. **Keep the planning view on `SiteDetailPage`** instead of adding a new route. The page already has the planning button, project timing, and assignment section. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx]
2. **Replace the current assignments-only “Projektplanung” card with one stacked planning card** that shows: timing summary (start/end/estimated days), existing `ProjectAssignmentsSection`, and an embedded project-filtered fleet calendar. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/sites/ProjectAssignmentsSection.tsx][VERIFIED: frontend/src/pages/fleet/CalendarView.tsx][ASSUMED]
3. **Reuse `CalendarView` rather than building a second calendar component.** The project surface should pass a project/site filter into the existing weekly calendar and reuse the same reservation detail and confirmation flows. [VERIFIED: frontend/src/pages/fleet/CalendarView.tsx][VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx][ASSUMED]
4. **Prefill the current project in reservation creation/edit from the project view** so the project-centric surface behaves like a focused entry point into the same fleet planner. The current fleet flows already accept and submit `site_id`. [VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx][VERIFIED: src/modules/fleet/api/routes.rs][ASSUMED]
5. **Do not add drag-and-drop, gantt, or multi-week staffing UI in this slice.** The smallest useful UI is one project card that joins timing, people, and linked bookings. [VERIFIED: user request][ASSUMED]

## 4. Verification plan

1. **Backend filter coverage** — verify tenant-scoped `site_id` filtering for both fleet calendar and reservation list endpoints so one tenant/project cannot see another tenant’s bookings. [VERIFIED: AGENTS.md][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs][ASSUMED]
2. **DTO parity coverage** — verify reservation summaries expose `site_id` consistently wherever the project page needs exact matching. [VERIFIED: src/modules/fleet/api/routes.rs][ASSUMED]
3. **Project-detail rendering test** — extend `SiteDetailPage.test.tsx` to verify one project surface shows timing, worker assignments, and only reservations linked to the current project. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.test.tsx][ASSUMED]
4. **Calendar regression test** — keep `CalendarView.test.tsx` green for selection behavior, visible existing reservations, and reservation details while adding project-filter behavior. [VERIFIED: frontend/src/pages/fleet/CalendarView.test.tsx]
5. **Reservation defaulting test** — verify opening booking from the project view preselects the current project and still uses the existing create/update reservation endpoints. [VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx][ASSUMED]

**Suggested commands if implemented later:** `cargo test`, `cargo fmt --check`, `SQLX_OFFLINE=true cargo clippy --tests -- -D warnings`, `cargo export-types`, `npm --prefix frontend run test:run -- SiteDetailPage CalendarView`, and `npm --prefix frontend run build`. [VERIFIED: AGENTS.md][VERIFIED: Cargo.toml][VERIFIED: frontend/package.json][ASSUMED]

## 5. Out-of-scope boundaries

- No second planner route, planner table, or planner-only bounded context. [VERIFIED: user request][ASSUMED]
- No replacement of `/fleet` as the canonical booking UX; the project view should be a focused reuse of that same flow. [VERIFIED: .planning/requirement-reviews/assets/AST-15.md][ASSUMED]
- No staff-capacity, holiday, or working-time-model planning from `STAFF-10` to `STAFF-12`. [VERIFIED: .planning/REQUIREMENTS.md]
- No new reservation domain like `project_id` or structured `purpose` in this slice; the current reservation model is still `site_id`-based and the request explicitly asked to reuse it. [VERIFIED: src/modules/fleet/domain/reservation.rs][VERIFIED: user request][ASSUMED]
- No drag-and-drop scheduling, gantt view, or automatic worker-to-vehicle matching. [VERIFIED: user request][ASSUMED]

## Standard Stack

| Library / System | Version | Purpose | Why standard here |
|------------------|---------|---------|-------------------|
| `axum` | `0.8` [VERIFIED: Cargo.toml] | Extend existing REST query shapes. [VERIFIED: Cargo.toml][VERIFIED: src/modules/fleet/api/routes.rs] | Current backend routing stack already owns both project and fleet APIs. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/fleet/api/routes.rs] |
| `sqlx` | `0.8` [VERIFIED: Cargo.toml] | Add optional reservation/calendar filters in repository SQL. [VERIFIED: Cargo.toml][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs] | Current persistence layer already expresses all relevant planning queries. [VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs] |
| React + TanStack Query | `react 19.2.5`, `@tanstack/react-query 5.100.6` [VERIFIED: frontend/package.json] | Compose project detail, assignments, and calendar hooks on one page. [VERIFIED: frontend/package.json][VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useFleet.ts] | Existing UI already uses this stack on both surfaces to be joined. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/fleet/CalendarView.tsx] |
| `ts-rs` | `12` [VERIFIED: Cargo.toml] | Keep any DTO additions synchronized into the frontend. [VERIFIED: Cargo.toml][VERIFIED: src/modules/fleet/api/routes.rs] | The repo already uses generated contracts for these APIs. [VERIFIED: AGENTS.md][VERIFIED: src/modules/fleet/api/routes.rs] |

## Architecture Patterns

- **Pattern: compose, don’t duplicate.** Join existing site detail data (`useSite`), worker assignments (`useSiteAssignments`), and fleet calendar data (`useCalendar`) on one page instead of creating a parallel planning domain. [VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useFleet.ts][ASSUMED]
- **Pattern: tenant-safe filtering in backend queries.** Apply project filters at the fleet repository/service layer, not by matching `site_name` strings in the browser. [VERIFIED: AGENTS.md][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs][ASSUMED]
- **Pattern: canonical booking flow reuse.** Keep `CalendarView` + `ReservationConfirmationSheet` + `ReservationDialog` as the only booking UX and let the project page call into them. [VERIFIED: frontend/src/pages/fleet/CalendarView.tsx][VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx]

## Don't Hand-Roll

| Problem | Don't build | Use instead | Why |
|---------|-------------|-------------|-----|
| Project planner backend | New planning aggregate/table [ASSUMED] | Existing `sites`, `site_assignments`, and `reservations` tables [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/006_fleet_schema.sql] | The current schema already stores the three data axes this feature needs. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/006_fleet_schema.sql] |
| Project booking UI | New calendar component [ASSUMED] | Existing `CalendarView` and reservation sheets/dialogs [VERIFIED: frontend/src/pages/fleet/CalendarView.tsx][VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx] | Reuse preserves booking behavior and avoids planner drift. [VERIFIED: .planning/requirement-reviews/assets/AST-15.md][ASSUMED] |
| Client-side project matching | Filter by `site_name` string [ASSUMED] | Exact `site_id` filter/field in fleet APIs [ASSUMED] | Project names are not a safe relational key. [VERIFIED: src/modules/fleet/api/routes.rs][ASSUMED] |

## Common Pitfalls

### Pitfall 1: Filtering reservations by `site_name`

The current calendar summaries only expose `site_name`, not `site_id`. Client-side string matching would be fragile and can collide across similarly named projects. [VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: frontend/src/types/fleet.ts]

### Pitfall 2: Building a separate project planner

Adding a second planner screen or backend aggregate would violate the explicit reuse constraint and risks booking-flow drift from `/fleet`. [VERIFIED: user request][VERIFIED: .planning/requirement-reviews/assets/AST-15.md][ASSUMED]

### Pitfall 3: Making project context mandatory too early in fleet UX

The current fleet booking flow intentionally stays light-touch and project-optional. Project view work should prefill project context, not add more required steps to the canonical fleet flow. [VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: .planning/requirement-reviews/assets/AST-15.md][ASSUMED]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The cleanest Phase 52 implementation is to extend fleet read APIs with optional `site_id` filters instead of adding a project-planning endpoint. | Recommended backend/API shape | If stakeholders want a one-call aggregate contract, an extra site-planning endpoint may still be justified. |
| A2 | `SiteDetailPage` should remain the project planning entry point rather than adding a dedicated route. | Minimal UI surface | If product wants a full-screen manager workspace, the UI recommendation is too small. |
| A3 | The project view should reuse the existing booking flow by preselecting the current project rather than redefining reservation create/edit UX. | Minimal UI surface | If booking from a project needs materially different inputs, additional UI work is required. |

## Open Questions

1. **Should the embedded project calendar show all resources with project-linked bookings highlighted, or only rows that already have a booking for this project?** [VERIFIED: user request][ASSUMED]
2. **Is booking creation from the project page required in the first slice, or is read-only visibility plus an “im Fuhrpark öffnen” action acceptable?** [VERIFIED: .planning/requirement-reviews/projects/PROJ-15.md][ASSUMED]

## Environment Availability

Skipped: this phase is an in-repo schema/API/UI extension on the existing Rust + React stack and does not introduce a new external dependency. [VERIFIED: Cargo.toml][VERIFIED: frontend/package.json][VERIFIED: user request]

## Security Domain

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | yes [VERIFIED: AGENTS.md] | Reuse existing Keycloak-authenticated site and fleet routes. [VERIFIED: AGENTS.md][VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/fleet/api/routes.rs] |
| V4 Access Control | yes [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/fleet/application/fleet_service.rs] | Preserve tenant scoping and current admin/owner checks on assignment and reservation operations. [VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/fleet/application/fleet_service.rs] |
| V5 Input Validation | yes [VERIFIED: src/modules/fleet/api/routes.rs] | Validate `site_id` query params and keep project filtering in backend query parsing. [VERIFIED: src/modules/fleet/api/routes.rs][ASSUMED] |
| V6 Cryptography | no new crypto [VERIFIED: scope] | Reuse existing auth stack only. [VERIFIED: AGENTS.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/REQUIREMENTS.md` and `.planning/requirement-reviews/projects/PROJ-15.md` - feature intent and explicit reuse goal. [VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: .planning/requirement-reviews/projects/PROJ-15.md]
- `.planning/requirement-reviews/assets/AST-15.md` - current fleet flow is the canonical booking UX to extend, not replace. [VERIFIED: .planning/requirement-reviews/assets/AST-15.md]
- `migrations/005_sites_schema.sql` and `migrations/006_fleet_schema.sql` - current planning schema on `sites`, `site_assignments`, and `reservations`. [VERIFIED: migrations/005_sites_schema.sql][VERIFIED: migrations/006_fleet_schema.sql]
- `src/modules/sites/*` and `src/modules/fleet/*` files cited above - current backend seams to reuse. [VERIFIED: src/modules/sites/api/routes.rs][VERIFIED: src/modules/sites/application/site_service.rs][VERIFIED: src/modules/sites/infrastructure/site_repository.rs][VERIFIED: src/modules/fleet/api/routes.rs][VERIFIED: src/modules/fleet/application/fleet_service.rs][VERIFIED: src/modules/fleet/infrastructure/fleet_repository.rs]
- `frontend/src/pages/sites/*`, `frontend/src/pages/fleet/*`, and hook files cited above - current UI composition seams and test coverage. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][VERIFIED: frontend/src/pages/sites/ProjectAssignmentsSection.tsx][VERIFIED: frontend/src/pages/sites/ProjectPlanningSheet.tsx][VERIFIED: frontend/src/pages/fleet/CalendarView.tsx][VERIFIED: frontend/src/pages/fleet/ReservationConfirmationSheet.tsx][VERIFIED: frontend/src/pages/fleet/ReservationDialog.tsx][VERIFIED: frontend/src/lib/api/hooks/useSites.ts][VERIFIED: frontend/src/lib/api/hooks/useFleet.ts]

## Metadata

**Confidence breakdown:**
- Reuse locations: HIGH - the current schema, APIs, hooks, and pages already expose the relevant primitives. [VERIFIED: cited files above]
- Backend/API recommendation: MEDIUM - the `site_id` filter/read-shape extension is a design recommendation inferred from current gaps, not an implemented contract. [VERIFIED: src/modules/fleet/api/routes.rs][ASSUMED]
- Minimal UI recommendation: MEDIUM - the project page reuse path is strongly supported by the codebase, but the exact embedded-calendar presentation is still a product choice. [VERIFIED: frontend/src/pages/sites/SiteDetailPage.tsx][ASSUMED]

**Research date:** 2026-05-08 [VERIFIED: system date]
**Valid until:** 2026-06-07 unless Phase 52 implementation or fleet query contracts change first. [VERIFIED: .planning/ROADMAP.md][ASSUMED]
