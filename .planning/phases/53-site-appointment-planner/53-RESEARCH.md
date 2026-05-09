# Phase 53: Site Appointment Planner - Scheduler Research

**Researched:** 2026-05-09
**Decision:** Prefer the custom hour-grid planner over adopting a third-party scheduler.
**Status:** Recommended and already aligned with the current codebase direction.

## Summary

The site planner use case needs a React-friendly weekly hour grid with strong mobile behavior, lightweight appointment editing, and easy visual integration into the existing Tailwind and Radix-based UI. The current repo already has the backend appointment model, generated DTOs, hooks, and a custom `SitePlanningCalendar` implementation, while `frontend/package.json` does not include a scheduler dependency. That makes a custom planner the lowest-friction path.

`react-big-calendar` and TOAST UI Calendar are both viable FOSS starting points, but each would still require non-trivial adaptation to match the app's mobile-first design system and appointment semantics. FullCalendar is not a clean FOSS fit for the richer resource-scheduler direction because the most relevant scheduler capabilities are Premium.

## Codebase Fit

- Frontend stack: React 19, Vite, Tailwind v4, Radix UI, TanStack Query.
- Existing planner domain: `migrations/023_site_appointments.sql` plus `src/modules/sites/api/routes.rs` and `src/modules/sites/application/site_service.rs`.
- Existing frontend contracts: `frontend/src/types/generated.ts`, `frontend/src/types/sites.ts`, and `frontend/src/lib/api/hooks/useSites.ts`.
- Existing custom planner UI: `frontend/src/pages/sites/SitePlanningCalendar.tsx`.
- Existing detail-page integration: `frontend/src/pages/sites/SiteDetailPage.tsx`.

## Comparison

| Option | License / FOSS fit | Maintenance | Customization burden | Mobile fit | Design-system compatibility | Verdict |
|--------|---------------------|-------------|----------------------|------------|-----------------------------|---------|
| `react-big-calendar` | MIT; clean FOSS fit | Mature and active enough for a baseline planner | Medium to high; good day/week primitives, but styling and interaction tuning would still be ours | Acceptable, but desktop-first defaults need work | Moderate; works in React, but visual integration relies on overriding library markup and CSS | Good fallback, not the best final fit |
| TOAST UI Calendar | MIT; clean FOSS fit | Established, but React usage is more wrapper-driven than native-feeling | High; richer built-ins, but theming and behavior shaping are heavier | Better built-in scheduler feel, but still needs mobile simplification | Weaker; less natural fit with Tailwind/Radix composition and local component patterns | Strong prototype option, higher integration cost |
| FullCalendar | Core React integration is MIT, but relevant scheduler/resource features are Premium | Well maintained | Medium for simple calendar use, but blocked for the real scheduler path | Fine technically | Reasonable technically | Reject for this use case because the resource scheduler path is not clean FOSS |
| Custom planner | Fully ours; no license risk | Internal maintenance only | Front-loaded once, then exact-fit changes stay cheap | Best fit because layout and gestures can be purpose-built | Best fit; uses existing primitives, styles, and dialog patterns directly | Recommended |

## Library Notes

### `react-big-calendar`

- Best FOSS off-the-shelf match if we wanted to avoid writing a calendar shell.
- Strongest fit for basic week/day hour-grid views.
- Main downside is not capability but adaptation cost: custom card rendering, mobile density, and Tailwind-first styling still need substantial glue.

### TOAST UI Calendar

- Strong built-in scheduling feature set with drag and resize support.
- Better if the product goal were a generalized calendar workspace out of the box.
- Less attractive here because the app does not need a generic enterprise calendar surface; it needs a narrow planner embedded into project detail.

### FullCalendar

- Fine for standard calendar views.
- Not a clean choice if the product may later want richer scheduler/resource behavior without accepting a commercial dependency.

## Recommendation

Choose the custom planner path.

Why:

1. The codebase already moved there: dedicated `site_appointments` storage, site-scoped appointment APIs, generated types, and a custom `SitePlanningCalendar` component already exist.
2. The product need is narrow and domain-specific: customer appointments plus worker-related time blocks on a site detail page, not a generic org-wide calendar suite.
3. Mobile fit and design-system alignment matter more than raw calendar feature breadth.
4. The custom planner avoids later migration pain if the team outgrows a library's styling model or licensing boundary.

## Delivery Guidance

- Keep the planner embedded on the site detail surface.
- Continue using the dedicated `site_appointments` model instead of reusing fleet reservations for project planning.
- Treat `react-big-calendar` as the fallback if the custom shell becomes too expensive to maintain.
- Do not adopt FullCalendar for this planner unless the team explicitly accepts Premium scheduler licensing.

## Decision Record

This research supersedes the earlier Phase 52 assumption that the project planner should reuse the fleet calendar with a `site_id` filter. The better fit for the dedicated site appointment planner is a custom hour-grid surface backed by `site_appointments`.
