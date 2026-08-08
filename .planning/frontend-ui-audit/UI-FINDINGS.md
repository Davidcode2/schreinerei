# Frontend UI Findings

Audit date: 2026-08-07. Environment: Chromium at 1440x1000 and 390x844, realistic isolated database, authenticated admin.

## P1

### UI-01: Reservation cancellation renders as an unlabeled red button

The reservation status transition includes `cancelled`, but `transitionLabels.cancelled` is an empty string. This creates a destructive button with no visible name or accessible text in the priority edit dialog.

- Evidence: [desktop](screenshots/21-dialog-edit-tool-reservation-priority.png), [mobile](screenshots/34-dialog-edit-tool-reservation-mobile-priority.png)
- Source: `frontend/src/components/fleet/StatusTransitionButtons.tsx:13-35,63-83`
- Fix: set `cancelled: "Stornieren"`; add an icon only as secondary reinforcement; assert accessible names for every valid transition.

### UI-02: Mobile reservation footer obscures content and fragments destructive actions

At 390x844 the sticky footer covers the lower form/action area. `Löschen` wraps below `Abbrechen`/`Speichern`, while status actions are partially hidden above the footer. The user cannot understand the complete action hierarchy without awkward internal scrolling.

- Evidence: [mobile priority dialog](screenshots/34-dialog-edit-tool-reservation-mobile-priority.png)
- Source: `frontend/src/pages/fleet/ReservationDialog.tsx:227,240,304-316,433-470`
- Fix: use one mobile footer grid with primary/secondary actions in one row and a full-width destructive action separated above it; add safe bottom padding to the scroll body; keep status transitions inside the scroll body with no overlap.

### UI-03: Reservation and appointment deletion lack confirmation

`ReservationDialog.handleDelete` immediately performs the mutation. Appointment edit does the same. Both are easy to trigger from dense dialogs and cannot be undone.

- Source: `frontend/src/pages/fleet/ReservationDialog.tsx:200-212,435-445`
- Source: `frontend/src/pages/sites/SitePlanningCalendar.tsx:303,640-651`
- Fix: use the existing `DeleteConfirmDialog`/`AlertDialog` pattern; name the affected reservation/appointment and keep focus on cancel by default.

## P2

### UI-04: Mobile seven-day resource calendar is too compressed to read or target

The mobile grid forces seven days beside an 88px resource column. Resource names, reservation owner, project and status all truncate; targets become narrow and difficult to distinguish.

- Evidence: [tools mobile](screenshots/33-tools-mobile.png)
- Source: `frontend/src/pages/fleet/CalendarView.tsx:52-53,214,296-405`
- Fix: use a horizontally scrollable desktop-width grid on mobile, or a mobile 3-day/day view. Preserve at least 120px per day and a sticky resource column.

### UI-05: Dashboard stat title collides with its icon on mobile

`Materialwarnungen` fills the card width and runs underneath the right-aligned icon in the fixed two-column mobile stats grid.

- Evidence: [dashboard mobile](screenshots/31-dashboard-mobile.png)
- Source: `frontend/src/pages/DashboardPage.tsx:77-99`
- Fix: reserve icon width in `StatsCard`, allow title wrapping with right padding, or switch the four cards to one column below approximately 420px.

### UI-06: Initial background sync produces persistent success noise

Every login/new browser context shows `Synchronisierung gestartet...` followed by `Daten synchronisiert`, even with zero queued changes. The success toast obscures page controls and every early screenshot, especially on mobile.

- Evidence: most screenshots, especially [dashboard mobile](screenshots/31-dashboard-mobile.png)
- Source: `frontend/src/lib/offline/sync.ts:20-40,83-113`
- Fix: make initial cache refresh silent; toast only manual sync, recovered connectivity, queued writes, or errors. Use one stable toast ID if progress feedback is retained.

### UI-07: Invoice sender address loses line separation

Settings stores a multiline sender address, but the invoice dialog renders it in a single-line input. `Hauptstrasse 18\n73453 Abtsgmuend` appears visually concatenated as `Hauptstrasse 1873453 Abtsgmuend`.

- Evidence: [invoice dialog](screenshots/16-dialog-create-invoice.png), compare [settings](screenshots/25-settings.png)
- Source: `frontend/src/pages/sites/CreateInvoiceDialog.tsx:90,179-184`
- Fix: use a textarea like Billing Settings, preserve newlines in the value, and retain them in the invoice snapshot/PDF.

### UI-08: Reservation datetime inputs are cramped in the desktop dialog

The 448px dialog uses two native datetime-local inputs side by side. Chromium's localized AM/PM value is clipped, making the selected time ambiguous.

- Evidence: [priority dialog](screenshots/21-dialog-edit-tool-reservation-priority.png)
- Source: `frontend/src/pages/fleet/ReservationDialog.tsx:227,352-370`
- Fix: use a single column until a wider breakpoint or increase the dialog to `sm:max-w-lg`; verify German and long AM/PM browser renderings.

### UI-09: Project planning sheet wastes desktop width

The bottom sheet spans the full 1440px viewport, producing very long form controls and weak grouping while the save actions start below the initial viewport.

- Evidence: [project planning sheet](screenshots/11-sheet-project-planning.png)
- Source: `frontend/src/pages/sites/ProjectPlanningSheet.tsx:175-324`
- Fix: constrain the form content to roughly 900-1050px centered inside the sheet and keep a visible sticky action footer.

### UI-10: German copy contains ASCII transliterations and one spelling inconsistency

Visible strings include `Reservierung bestaetigen`, `fuer`, `pruefen`, `ueberschreiben`, `standardmaessig`, and `Bestatigen` while the rest of the product uses German diacritics.

- Evidence: [calendar confirmation](screenshots/22-sheet-calendar-reservation-confirmation.png), [invoice dialog](screenshots/16-dialog-create-invoice.png)
- Source: `frontend/src/pages/fleet/ReservationConfirmationSheet.tsx:168-171,254`
- Source: `frontend/src/pages/sites/CreateInvoiceDialog.tsx:163,227`
- Source: `frontend/src/pages/inventory/InventoryDetailPage.tsx:394`
- Fix: replace user-visible transliterations with `bestätigen`, `für`, `prüfen`, `überschreiben`, `standardmäßig`; add a lightweight copy grep/lint check.

## P3

### UI-11: Calendar reservation cards do not expose an explicit accessible name

Occupied calendar entries use `role="button"` on a `div` and derive their name from truncated descendants. There is no clear name containing resource, date and action.

- Source: `frontend/src/pages/fleet/CalendarView.tsx:349-400`
- Fix: use a real `button` and `aria-label="Reservierung von <user> für <resource> am <date> bearbeiten"`.

### UI-12: Resource cards use standalone icon-only trash controls in dense layouts

Trash icons sit directly beside QR badges with little separation, increasing accidental deletion risk. Confirmation helps, but the target meaning is not visually explicit.

- Evidence: [tools desktop](screenshots/20-tools-calendar-and-list.png), [tools mobile](screenshots/33-tools-mobile.png)
- Fix: add tooltips and resource-specific accessible labels, increase separation, and consider moving destructive actions into an overflow menu.

### UI-13: QR vehicle/tool result routes to the fleet overview, not the resource

The result CTA says `Details anzeigen`, but both vehicles and tools navigate to `/fleet` rather than `/fleet/:id` or `/tools/:id`.

- Source: `frontend/src/components/qr/QrResultDialog.tsx:47-53,96-119`
- Fix: route by type and ID: vehicle -> `/fleet/${id}`, tool -> `/tools/${id}`.

### UI-14: Manual QR entry cannot open after camera failure

The error panel offers `Code manuell eingeben`, but the manual form is gated by `showManualEntry && !error`; clicking without clearing `error` leaves the user in the same error state.

- Source: `frontend/src/components/qr/QrScanner.tsx:13,39,81-105`
- Fix: clear the error when choosing manual entry or remove `!error` from the manual form condition.

### UI-15: Existing dashboard E2E tests assert obsolete copy

Two tests expect `Aktive Baustellen` and `Niedrige Bestände`; the current UI intentionally uses `Aktive Projekte` and `Materialwarnungen`. This creates false failures and hides real regressions.

- Source: `frontend/tests/dashboard.spec.ts:16-31`
- Fix: update assertions to current headings and prefer role-based regions/test IDs over broad text-content checks.

## Positive Observations

- Desktop page hierarchy, typography and status color system are consistent.
- Expiry and low-stock states are clearly differentiated and actionable.
- Most long dialogs correctly use a fixed header/footer with an internal scroll body.
- Seeded vehicle display colors carry consistently from calendar rows to cards.
- Project appointments distinguish customer, deployment and milestone states well.
- The mobile navigation sheet is clear and preserves active-project context.
