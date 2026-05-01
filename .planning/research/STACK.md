# Technology Stack

**Project:** Schreinerei SaaS — Fleet Calendar on Fleet Page v1.11
**Researched:** 2026-05-01

## Recommended Stack

### Core Framework (UNCHANGED)

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| React | 18 | Frontend UI | Existing fleet pages and dialogs already use it |
| Vite | 6 | Frontend build | Existing app build pipeline |
| TypeScript | 5.x | Frontend typing | Existing fleet DTOs and generated types |
| Tailwind CSS | 4 | Styling | Existing calendar and fleet page styling |
| shadcn/ui | current | Dialog, button, card primitives | Existing reservation dialog already uses them |
| TanStack Query | current | Calendar and reservation data fetching | Existing `useCalendar`, `useCreateReservation`, `useAvailability` hooks |
| React Router | v6 | Fleet route composition | Existing `/fleet` and `/fleet/calendar` routes |
| Rust + Axum + SQLx | existing | Reservation and calendar APIs | Existing backend already serves fleet calendar and reservations |
| ts-rs | existing | Rust to TypeScript DTO sync | Existing calendar/availability/generated types pattern |

### New Capability Additions

No new package is required for this milestone.

Use existing building blocks:
- Reuse `useCalendar()` for embedded fleet-page calendar data
- Reuse `ReservationDialog` create flow after date-range selection is complete
- Reuse hash-based color assignment pattern from `frontend/src/lib/active-site/siteColor.ts`
- Reuse existing reservation create API with explicit `start_time` and `end_time`

### Likely Type Changes

| Area | Change | Why |
|------|--------|-----|
| Frontend local state | Add selected start/end day and `withTimes` flag | Supports two-tap selection before modal open |
| Calendar DTOs | Optional extension only if backend color metadata is added | Current API already returns enough data for reservations; colors can stay client-side |
| Generated types | No required backend change for the core UX flow | Existing reservation payload already supports time ranges |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Calendar placement | Embed calendar into `/fleet` | Keep separate `/fleet/calendar` as primary | Adds an extra navigation step to a feature users need often |
| Range selection | Two taps then confirm | Open modal on first tap | Current flow interrupts selection and feels clumsy on mobile |
| Time entry | Optional checkbox in confirmation step | Always require datetime inputs | Adds friction for date-only bookings |
| Resource colors | Deterministic client-side colors by resource id | New backend color field | Extra backend work is unnecessary for this milestone |
| Confirmation UI | Bottom-sheet style modal | Center modal over calendar | Center modal hides the selected dates on smaller screens |

## Installation

No new dependencies.

If backend DTOs change during implementation:

```bash
cargo test --features ts-rs/export
```

## Sources

- `frontend/src/pages/fleet/FleetPage.tsx`
- `frontend/src/pages/fleet/CalendarView.tsx`
- `frontend/src/pages/fleet/ReservationDialog.tsx`
- `frontend/src/lib/api/hooks/useFleet.ts`
- `frontend/src/lib/active-site/siteColor.ts`
- `src/modules/fleet/api/routes.rs`
