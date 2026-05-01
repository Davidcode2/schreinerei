# Phase 39 Research — Range Selection & Confirmation Flow

**Date:** 2026-05-01  
**Phase:** 35 — Range Selection & Confirmation Flow

## Question

What do we need to know to plan the two-tap date-range selection flow on the embedded fleet calendar well?

## Existing Implementation Snapshot

### Current fleet calendar behavior

- `frontend/src/pages/fleet/CalendarView.tsx`
  - owns the week navigation state and renders the 7-day resource grid.
  - treats an empty day cell as directly bookable.
  - on first click, immediately creates `selectedSlot` with default `08:00` → `17:00` times and opens `ReservationDialog`.
  - already keeps reservation rows visible while browsing the week grid.

### Existing reservation creation behavior

- `frontend/src/pages/fleet/ReservationDialog.tsx`
  - already handles reservation creation, edit mode, availability checks, optional site assignment, and notes.
  - requires concrete `startTime` / `endTime` values from the caller.
  - uses `useCreateReservation`, `useAvailability`, `usePreferences`, and `useSites`.

### Existing frontend primitives available now

- `frontend/src/components/ui/sheet.tsx`
  - already supports `side="bottom"` and gives us the correct bottom-sheet animation / placement for `FCONF-01`.
- `frontend/src/lib/api/hooks/useFleet.ts`
  - already exposes `useCreateReservation`, `useCalendar`, and `useAvailability`.
- `frontend/src/test/mocks/handlers.ts`
  - already supports POST `/api/v1/fleet/reservations`, so targeted frontend tests can cover confirm submission without backend work.

## Constraints From Phase 38 / Project State

- Phase 38 explicitly kept `/fleet` and `/fleet/calendar` on the same `CalendarView` implementation.
- Project state says selection must stay tied to a single resource row.
- Requirements only change the embedded calendar interaction for this phase; removing the standalone entry point and stable colors belong to Phase 40.
- No new backend model is needed; existing reservation APIs already accept start/end datetimes.

## Recommended Architecture

### 1. Extract range-selection logic into a small pure helper/module

Reasoning:
- `FSEL-02`, `FSEL-03`, and `FSEL-04` are deterministic input/output behaviors.
- A pure helper or reducer keeps the selection rules testable without rendering the whole calendar.
- It gives the executor a stable contract for: first tap starts selection, second tap on same resource completes selection, same-day double tap is valid, and dates are sorted.

### 2. Keep `ReservationDialog` for existing list-based reservation entry points

Reasoning:
- `FleetPage.tsx` still opens `ReservationDialog` from vehicle/tool lists.
- Replacing the dialog everywhere would widen scope and risk regressions outside the calendar flow.
- Phase 39 only needs a new confirmation experience after calendar range completion, so a dedicated confirmation sheet is a narrower change.

### 3. Add a dedicated bottom confirmation sheet for calendar-created reservations

Recommended component: `ReservationConfirmationSheet.tsx`

Why this is the best fit:
- Matches `FCONF-01` directly using the existing sheet primitive.
- Lets the calendar flow show the chosen date range immediately while preserving the grid behind it.
- Can support a simple checkbox-controlled time section for `FCONF-04` without disturbing edit-mode reservation UX.

### 4. Keep reservation visibility in the grid while selection is pending

Reasoning:
- Current `CalendarView` already renders reservation chips from `entry.reservations`.
- Phase 39 must add pending selection feedback without replacing or hiding those chips.
- Phase 40 owns color improvements; this phase should keep the current reservation rendering intact and layer selection state on top.

## Planning Implications

### Files likely involved

- `frontend/src/pages/fleet/CalendarView.tsx`
- `frontend/src/pages/fleet/ReservationConfirmationSheet.tsx` (new)
- `frontend/src/pages/fleet/calendarRangeSelection.ts` or similar helper (new)
- `frontend/src/pages/fleet/CalendarView.test.tsx` (new)
- `frontend/src/pages/fleet/calendarRangeSelection.test.ts` (new)

### Risks / pitfalls to explicitly plan against

1. **Cross-resource leakage**
   - A second tap on a different resource must not accidentally produce a mixed-resource range.
2. **Date sorting drift**
   - Reverse-order taps must normalize before display and before submission.
3. **Calendar/list flow regression**
   - Resource-card reservation buttons in `FleetPage.tsx` should continue using the existing dialog path.
4. **Optional times UX**
   - Time entry must be opt-in, not required by default.
5. **Range confirmation without clearing state**
   - Cancel must fully clear pending selection and close the sheet.

## Recommended Verification Strategy

- Unit-test the range-selection helper for same-day, reverse-order, and cross-resource cases.
- Add focused `CalendarView` tests that verify:
  - first tap creates pending selection but does not submit,
  - second tap opens bottom confirmation sheet,
  - cancel clears selection,
  - confirm posts a reservation,
  - enabling time entry exposes editable datetime inputs.

## Outcome For Planning

This phase can be planned as two sequential frontend plans:

1. **TDD plan** for the reusable selection contract.
2. **Execution plan** for wiring calendar interaction + bottom confirmation sheet + frontend regression tests.
