# Phase 40 Research — Calendar Visibility, Colors & Cleanup

**Date:** 2026-05-01  
**Phase:** 36 — Calendar Visibility, Colors & Cleanup

## Question

What do we need to know to plan Phase 40 well so the embedded fleet calendar stays clear while selecting, uses deterministic resource colors, and fully replaces the old standalone entry point?

## Existing Implementation Snapshot

### Current embedded calendar state after Phase 39

- `frontend/src/pages/fleet/CalendarView.tsx`
  - already powers both `/fleet` embedding and the standalone `/fleet/calendar` route.
  - keeps reservation chips visible in occupied cells and overlays pending/completed selection styling only on empty cells.
  - colors reservation chips by reservation status (`confirmed`, `in_use`, `pending`) instead of resource identity.
  - still derives local day keys with `toISOString().split("T")[0]`.

### Current fleet entry points

- `frontend/src/pages/fleet/FleetPage.tsx`
  - embeds `CalendarView` at the top of `/fleet`.
  - still shows the header button linking to `/fleet/calendar`.
- `frontend/src/App.tsx`
  - still registers `/fleet/calendar` as a standalone route rendering `CalendarView`.
- `frontend/src/pages/fleet/index.ts`
  - still re-exports `CalendarView` for the standalone route import path.

### Existing deterministic color pattern available now

- `frontend/src/lib/active-site/siteColor.ts`
  - already uses a small palette plus a hash of stable identity to derive deterministic frontend-only colors.
  - proves the project already accepts client-side hash-based colors without backend storage.

## Constraints From Roadmap / Current State

- Phase 39 already delivered two-tap selection and the bottom confirmation sheet; Phase 40 must preserve that flow, not replace it.
- Project state explicitly says vehicle and machine colors should stay stable per resource without backend color storage.
- The embedded `/fleet` calendar is now the canonical booking experience, so removing the old entry point must not create a second implementation path.
- Requirements do not ask for new backend endpoints or schema changes.

## Recommended Architecture

### 1. Derive resource colors from `resource_type + resource_id`

Recommended helper: `frontend/src/pages/fleet/resourceCalendarColor.ts`

Why:
- `FCONF-06` is a deterministic input/output rule and benefits from a tiny pure helper plus unit tests.
- Hashing `resource_type:resource_id` avoids row-order drift when the API order changes or a week loads different resources.
- Keeping the helper frontend-only matches both project decisions and requirements (`Backend-managed resource colors` is out of scope in `REQUIREMENTS.md`).

### 2. Apply resource color as a row/chip accent without hiding reservation content

Recommended UI treatment:
- keep reservation text readable by retaining status-aware contrast classes,
- add the stable resource color to row labels / occupied-cell accents / chip borders or markers,
- keep pending/completed selection styling additive instead of replacing existing reservation content.

Why:
- `FCONF-05` is about visibility during selection, not merely data presence.
- A resource identity accent can coexist with reservation-status meaning if it is used as a border, dot, or tinted wrapper rather than replacing all chip colors blindly.

### 3. Remove the standalone calendar as a primary entry path

Recommended scope:
- remove the `/fleet/calendar` header button from `FleetPage.tsx`,
- remove the `/fleet/calendar` route from `App.tsx`,
- remove the barrel export if it becomes unused.

Why:
- `FCAL-03` says the user should not need the separate entry point for the main booking experience.
- Phase 38 kept the standalone route only as a transition path; Phase 40 is the cleanup phase that should end that transition.

### 4. Extend regression coverage around the embedded calendar contract

Recommended tests:
- unit tests for deterministic resource color mapping,
- `CalendarView.test.tsx` coverage proving reserved chips remain visible after starting a new selection and that stable resource-color markers are derived from identity,
- `FleetPage.test.tsx` coverage proving the embedded calendar remains above the lists while the old calendar CTA is gone and the list-based reservation dialog path still works.

## Risks / Pitfalls To Plan Against

1. **Row-order color drift**
   - deriving colors from array index would fail when resource order changes across reloads or weeks.
2. **Selection overlays hiding bookings**
   - a refactor that turns occupied cells into selection-only surfaces would violate `FCONF-05`.
3. **Partial cleanup**
   - removing only the button or only the route would leave stale standalone entry behavior behind.
4. **Calendar/list-path regression**
   - the embedded calendar should become primary without breaking the existing list-based `ReservationDialog` buttons.

## Recommended Verification Strategy

- Unit-test the resource color helper with repeated calls, different resource identities, and cross-type collisions.
- Extend `CalendarView.test.tsx` to verify an existing reservation label remains visible after the first tap on another empty day and that resource-color output is stable for the same row.
- Extend `FleetPage.test.tsx` to verify the old calendar-entry button is absent while list-based reservation still opens `ReservationDialog`.
- Use a repo search in verification to prove `/fleet/calendar` is no longer registered in the frontend route tree or fleet-page CTA.

## Outcome For Planning

This phase can be planned as two focused frontend execution plans that run in parallel:

1. **Calendar visibility + deterministic color contract** — helper, UI wiring, and calendar regression tests.
2. **Entry-point cleanup** — remove the standalone route/CTA and lock the embedded path in tests.
