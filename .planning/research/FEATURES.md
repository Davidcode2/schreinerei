# Feature Landscape

**Domain:** Carpentry SaaS — Fleet reservation UX
**Researched:** 2026-05-01

## Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Calendar visible on fleet page | Reservation visibility should not require a second page | Low | Existing `CalendarView` can be embedded or extracted into a shared component |
| Date-range selection by two taps | Users expect to choose a range directly in the calendar | Med | Replace current first-tap dialog open behavior |
| Same-day booking support | One-day reservations are common | Low | Second tap may be on the same day |
| Deferred confirmation after range selection | Prevents accidental modal interruptions | Med | Selection state lives in calendar until second tap |
| Cancel clears pending selection | Users need a safe escape hatch | Low | Modal cancel should undo the temporary range |
| Existing bookings remain visible while selecting | Avoids blind booking attempts | Low | Calendar already renders reservations by day |

## Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Bottom-sheet confirmation modal | Keeps the selected range visible while confirming | Med | Better mobile UX than the current centered dialog |
| Optional time entry | Date-only booking stays fast, time detail stays available when needed | Med | Existing API already accepts precise times |
| Resource-specific colors in calendar | Makes scanning reservations across vehicles and tools much faster | Med | Can reuse deterministic hash-color pattern on the client |
| Embedded page replaces separate calendar entry point | Simplifies the fleet workflow and reduces navigation overhead | Low | Remove or demote the calendar icon path |

## Anti-Features

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Modal on first tap | Breaks range selection and repeats the current UX problem | Wait until second selection before opening confirmation |
| Mandatory time input for every booking | Slows down the common case | Make times opt-in behind a checkbox |
| Separate mobile and desktop flows | Increases complexity for little value | Use one selection model with responsive layout tweaks |
| Backend-managed color palette | Not needed to deliver the UX improvement | Compute stable colors in frontend from resource id |
| Preserving `/fleet/calendar` as the main booking path | Conflicts with the milestone goal of direct visibility | Move the main experience to `/fleet` |

## Feature Dependencies

```text
Embedded calendar on /fleet
  -> requires reuse/extraction of current CalendarView grid

Two-tap range selection
  -> requires local selection state in calendar cells
  -> requires range sorting before submit

Bottom-sheet confirmation modal
  -> depends on completed range selection
  -> can hand off final values into existing ReservationDialog or reservation create mutation

Optional times
  -> depends on confirmation modal state
  -> uses existing start_time/end_time reservation payload

Resource colors
  -> depends on deterministic resource-to-color mapping in frontend
```

## MVP Recommendation

Prioritize:
1. Embed the calendar into the fleet page and remove the extra navigation step
2. Replace first-tap modal creation with two-tap range selection
3. Add confirmation/cancel flow after range completion
4. Add optional times and stable resource colors once the selection flow is correct

Defer:
- month-view redesign
- drag-to-select interactions
- per-resource legend/filtering beyond the current fleet tabs

## Sources

- `frontend/src/pages/fleet/FleetPage.tsx`
- `frontend/src/pages/fleet/CalendarView.tsx`
- `frontend/src/pages/fleet/ReservationDialog.tsx`
