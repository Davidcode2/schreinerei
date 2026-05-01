# Requirements: Schreinerei SaaS

**Defined:** 2026-05-01
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## v1.11 Requirements

### Fleet Page Layout

- [ ] **FCAL-01**: User can see the fleet reservation calendar directly at the top of the fleet page
- [ ] **FCAL-02**: User can continue using the existing fleet page content below the embedded calendar
- [ ] **FCAL-03**: User no longer needs the separate fleet calendar entry point to reach the main booking experience

### Date Range Selection

- [ ] **FSEL-01**: User can tap one day in a resource row to start a pending reservation selection
- [ ] **FSEL-02**: User can tap a second day in the same resource row to complete a reservation date range
- [ ] **FSEL-03**: User can tap the same day twice to create a one-day reservation
- [ ] **FSEL-04**: The app sorts the two selected days so the final reservation range always has the earlier date first

### Confirmation and Calendar Feedback

- [ ] **FCONF-01**: After the second date is selected, user sees a bottom-positioned confirmation modal showing the selected date range
- [ ] **FCONF-02**: User can cancel the confirmation modal to clear the pending selection
- [ ] **FCONF-03**: User can confirm the selected range to continue reservation creation
- [ ] **FCONF-04**: User can optionally enable time entry and provide start and end times for the booking
- [ ] **FCONF-05**: User can still see existing reserved date ranges in the calendar while making a selection
- [ ] **FCONF-06**: Each vehicle or machine is shown with a stable unique color in the calendar

## Future Requirements

Deferred to future release.

### Fleet Calendar Enhancements

- **FCAL-04**: User can switch between week and month calendar views
- **FSEL-05**: User can drag across days to select a range instead of tapping twice
- **FCONF-07**: User can filter the calendar to a subset of vehicles or tools while keeping the same booking flow

### Deferred Previous Milestone Work

- **INV-31**: User can edit inventory categories via a settings page
- **INV-32**: User can edit inventory item location and minimum quantity
- **INV-33**: User can record stock-in and see enriched inventory history

## Out of Scope

| Feature | Reason |
|---------|--------|
| Drag-to-select calendar interaction | Two-tap range selection is sufficient for this milestone |
| Month view redesign | Milestone is focused on embedding and fixing the existing week-based booking UX |
| Backend-managed resource colors | Stable client-side colors are enough for this milestone |
| Separate mobile and desktop reservation flows | One responsive flow is simpler and less error-prone |
| New reservation backend model | Existing reservation APIs already support the needed date/time data |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| FCAL-01 | TBD | Pending |
| FCAL-02 | TBD | Pending |
| FCAL-03 | TBD | Pending |
| FSEL-01 | TBD | Pending |
| FSEL-02 | TBD | Pending |
| FSEL-03 | TBD | Pending |
| FSEL-04 | TBD | Pending |
| FCONF-01 | TBD | Pending |
| FCONF-02 | TBD | Pending |
| FCONF-03 | TBD | Pending |
| FCONF-04 | TBD | Pending |
| FCONF-05 | TBD | Pending |
| FCONF-06 | TBD | Pending |

**Coverage:**
- v1.11 requirements: 13 total
- Mapped to phases: 0
- Unmapped: 13 ⚠️

---
*Requirements defined: 2026-05-01*
*Last updated: 2026-05-01 after milestone v1.11 definition*
