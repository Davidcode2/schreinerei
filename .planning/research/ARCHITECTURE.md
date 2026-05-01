# Architecture Patterns: Fleet Calendar v1.11

**Domain:** Carpentry SaaS — fleet module UX extension
**Researched:** 2026-05-01
**Overall confidence:** HIGH

## Recommended Architecture

This milestone should extend the existing fleet frontend and reuse the existing fleet backend APIs wherever possible.

```text
frontend/src/pages/fleet/
  FleetPage.tsx            # extend: embed calendar at top of page, remove separate primary entry
  CalendarView.tsx         # refactor: extract reusable calendar grid / selection behavior
  ReservationDialog.tsx    # reuse or adapt for post-selection confirmation and optional times

frontend/src/lib/api/hooks/
  useFleet.ts              # reuse existing useCalendar/useCreateReservation/useAvailability hooks

frontend/src/lib/
  active-site/siteColor.ts # pattern to mirror for resource color hashing

src/modules/fleet/
  api/routes.rs            # unchanged unless DTOs need extension
  application/fleet_service.rs
  infrastructure/fleet_repository.rs
```

## Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|----------------|-------------------|
| `FleetPage` | Hosts page header, tabs, embedded calendar, and existing list content | calendar component, vehicles/tools/reservations lists |
| calendar grid component | Week navigation, cell rendering, reservation visibility, selection state | `useCalendar`, confirmation UI |
| confirmation modal / sheet | Shows selected range, optional times, confirm/cancel actions | reservation mutation or `ReservationDialog` |
| `ReservationDialog` | Existing reservation form and validation | `useAvailability`, `useCreateReservation`, `useUpdateReservation` |
| `useCalendar` | Fetches weekly occupancy data | `/api/v1/fleet/calendar` |

## Data Flow

**Embedded fleet calendar:**

```text
FleetPage
  -> renders calendar section at top
  -> calendar requests useCalendar(start_date, end_date)
  -> grid renders reservations and availability context per resource/day
```

**Two-tap selection flow:**

```text
first tap
  -> store pending start day + resource
second tap on same resource
  -> sort selected dates into start/end
  -> open confirmation modal from bottom
cancel
  -> clear pending selection
confirm
  -> submit reservation with date-only or date+time values
```

**Reservation creation:**

```text
confirmation modal
  -> optional checkbox enables explicit time inputs
  -> build final start_time/end_time
  -> create reservation via existing mutation
  -> invalidate calendar and reservations queries
```

## Patterns to Follow

### Pattern 1: Reuse Existing Fleet Query Hooks

Keep the calendar data source on `useCalendar()` and reservation creation on `useCreateReservation()` so the current invalidation behavior remains intact.

### Pattern 2: Extract, Don't Duplicate, Calendar Layout

The existing `/fleet/calendar` page already contains the week header, day columns, and resource row rendering. Refactor this into a reusable component or move the logic into the fleet page instead of keeping two drifting implementations.

### Pattern 3: Keep Selection State Client-Side Until Confirm

The backend should not know about an in-progress selection. Only confirmed ranges should become reservations.

### Pattern 4: Keep Times Optional in the UI Layer

The backend already accepts `start_time` and `end_time`. The new checkbox is a UI concern, not a new domain concept.

### Pattern 5: Stable Colors from Resource Identity

Mirror the `siteColor.ts` hashing approach so each vehicle or tool keeps the same visual identity without storing colors in the database.

## Integration Notes

- Current `CalendarView` only opens the dialog when a day cell is empty. The new UX should preserve conflict visibility while allowing range selection first.
- Current `ReservationDialog` is centered and datetime-first. The milestone likely needs a lighter confirmation step before the full form logic, or a bottom-sheet adaptation of the same behavior.
- Existing calendar response returns `resource_type`, `resource_id`, `resource_name`, and per-day reservations. That is enough for embedded display and stable color derivation.
- If `/fleet/calendar` remains temporarily for compatibility, it should use the same extracted calendar component as `/fleet`.

## Sources

- `frontend/src/pages/fleet/FleetPage.tsx`
- `frontend/src/pages/fleet/CalendarView.tsx`
- `frontend/src/pages/fleet/ReservationDialog.tsx`
- `frontend/src/lib/api/hooks/useFleet.ts`
- `frontend/src/lib/active-site/siteColor.ts`
- `src/modules/fleet/api/routes.rs`
