# Project Research Summary

**Project:** Schreinerei SaaS — Fleet Calendar on Fleet Page v1.11
**Domain:** Carpentry SaaS — fleet reservation UX
**Researched:** 2026-05-01
**Confidence:** HIGH

## Executive Summary

This milestone is a UX-focused extension of an existing fleet calendar and reservation system, not a new capability area. The codebase already has the core building blocks: a separate `CalendarView`, a `ReservationDialog`, fleet calendar/reservation hooks, and backend APIs for calendar data, availability checks, and reservation creation. The right move is to embed the current calendar experience into `FleetPage`, replace first-tap modal opening with a two-tap date-range selection flow, and keep time selection optional in a follow-up confirmation step.

No new dependency is required. The existing backend contracts are already sufficient for the main milestone goal because reservations are still created with `start_time` and `end_time`; the change is mostly about how the frontend collects those values. The biggest risks are incorrect range logic, mobile confirmation UI covering the selection, and duplicated calendar implementations drifting apart.

## Key Findings

### Stack additions

No new stack additions are required.

Reuse:
- `useCalendar()` and reservation mutations from `frontend/src/lib/api/hooks/useFleet.ts`
- existing `ReservationDialog` form logic where possible
- deterministic color hashing pattern from `frontend/src/lib/active-site/siteColor.ts`

### Feature table stakes

- Calendar visible directly on the fleet page
- Two-tap date-range selection on the same resource row
- Same-day two-tap support for one-day bookings
- Confirmation shown only after the second selection
- Cancel clears the temporary selection
- Existing reservations remain visible during selection

### Watch Out For

- Do not let a range span two different resources
- Always sort the selected dates before submit
- Keep the confirmation UI low on the screen so the grid stays visible
- Use a distinct pending-selection style so it is not confused with real reservations
- Derive colors from stable resource ids, not row order

## Implications for Requirements and Roadmap

The natural milestone split is frontend-heavy:

1. Fleet page embedding and shared calendar extraction
2. Two-tap selection, confirmation, and optional time entry
3. Reservation rendering polish, colors, route cleanup, and regression coverage

Backend changes should stay minimal unless implementation reveals a missing DTO for the confirmation flow.

## Sources

- `frontend/src/pages/fleet/FleetPage.tsx`
- `frontend/src/pages/fleet/CalendarView.tsx`
- `frontend/src/pages/fleet/ReservationDialog.tsx`
- `frontend/src/lib/api/hooks/useFleet.ts`
- `frontend/src/lib/active-site/siteColor.ts`
- `src/modules/fleet/api/routes.rs`

---
*Research completed: 2026-05-01*
*Ready for requirements: yes*
