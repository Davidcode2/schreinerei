# Domain Pitfalls

**Domain:** Carpentry SaaS — fleet reservation UX
**Researched:** 2026-05-01

## Critical Pitfalls

### Pitfall 1: Range Selection Crosses Resources by Accident

**What goes wrong:** User taps one vehicle, then taps a different resource row, and the app incorrectly creates a date range across two resources.

**Prevention:** Selection state must include both `resource_id` and `resource_type`. Second tap should only complete a range for the same resource. Tapping a different resource should either restart selection on that resource or require an explicit reset.

### Pitfall 2: Date Sorting Is Not Applied

**What goes wrong:** User taps the later date first and the earlier date second, but the app submits the range in reverse order.

**Prevention:** Always sort the two selected dates before building `start_time` and `end_time`. Treat this as a dedicated helper with unit-testable logic.

### Pitfall 3: Confirmation Modal Hides the Chosen Dates on Mobile

**What goes wrong:** A centered dialog covers the week grid, so users cannot verify what they selected.

**Prevention:** Use a bottom-sheet style confirmation surface or a mobile-first dialog placement that leaves the calendar visible behind it.

## Moderate Pitfalls

### Pitfall 4: Cancel Leaves a Stale Highlight Behind

**What goes wrong:** User cancels the modal, but the temporary selected range remains highlighted, making the calendar look booked or active when it is not.

**Prevention:** Cancel must clear both the anchor day and any derived range state.

### Pitfall 5: Existing Reservations Become Hard to Distinguish from Pending Selection

**What goes wrong:** The temporary selection uses the same visual treatment as confirmed reservations.

**Prevention:** Use distinct styles for pending selection vs. existing bookings. Resource color can remain the same family, but pending state needs a separate outline, opacity, or pattern.

### Pitfall 6: Time Checkbox Adds Validation Friction to Date-Only Bookings

**What goes wrong:** Time inputs are rendered immediately or validated even when the user wants only a date range.

**Prevention:** Hide or disable time inputs unless the checkbox is active, and default date-only bookings to sensible start/end values.

### Pitfall 7: Fleet Page Becomes Too Tall or Crowded

**What goes wrong:** Embedding the calendar above the existing tabs pushes important content too far down or creates poor scrolling behavior.

**Prevention:** Make the calendar compact by default, preserve the existing tab content below it, and verify mobile vertical spacing.

### Pitfall 8: Duplicate Calendar Implementations Drift

**What goes wrong:** `/fleet` and `/fleet/calendar` each render slightly different booking logic.

**Prevention:** Extract shared calendar logic into one reusable component or migrate both routes to the same implementation until the old route is removed.

## Minor Pitfalls

### Pitfall 9: Resource Colors Change Between Renders

**What goes wrong:** Colors are derived from array position instead of resource identity, so they shift when list ordering changes.

**Prevention:** Hash `resource_id` or another stable identifier, not the row index.

### Pitfall 10: Timezone Boundaries Break Same-Day Bookings

**What goes wrong:** Converting local day picks to ISO timestamps can move the booking into the previous or next day.

**Prevention:** Keep local date selection separate from final datetime serialization. Only convert to ISO when the final start/end values are known.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Calendar embedding | Fleet page duplication or layout regression | Extract shared grid logic and verify mobile scroll behavior |
| Range selection | Reverse-order taps create invalid ranges | Sort dates before submit |
| Confirmation step | Modal obscures calendar | Use bottom-sheet presentation |
| Optional times | Date-only flow blocked by time validation | Gate time inputs behind checkbox |
| Resource coloring | Colors drift per render | Hash stable resource ids |

## Sources

- `frontend/src/pages/fleet/CalendarView.tsx`
- `frontend/src/pages/fleet/FleetPage.tsx`
- `frontend/src/pages/fleet/ReservationDialog.tsx`
- `frontend/src/lib/active-site/siteColor.ts`
