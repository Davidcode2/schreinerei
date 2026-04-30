# Requirements: Schreinerei SaaS v1.7

**Defined:** 2026-04-30
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## v1.7 Requirements

Active Project Context — auto-assign materials, reservations, and time entries to the currently active Baustelle.

### Backend — User Preferences

- [ ] **PREF-01**: User can store their active Baustelle preference server-side
- [ ] **PREF-02**: System validates active Baustelle exists and is not archived/deleted
- [ ] **PREF-03**: System clears preference automatically if Baustelle becomes invalid

### Backend — Material Deductions

- [ ] **DEDU-01**: Material deductions can be linked to a Baustelle (FK column)
- [ ] **DEDU-02**: WithdrawMaterial command accepts optional site_id parameter
- [ ] **DEDU-03**: Deduction details include Baustelle name when linked

### Frontend — Active Site UI

- [ ] **ACTV-01**: User sees persistent indicator showing active Baustelle name and color
- [ ] **ACTV-02**: User can toggle active Baustelle from overview page
- [ ] **ACTV-03**: User can toggle active Baustelle from dashboard view
- [ ] **ACTV-04**: Only one Baustelle can be active per user at a time
- [ ] **ACTV-05**: Baustellen have auto-assigned colors (hash-based, no manual selection)
- [ ] **ACTV-06**: Active state persists across page navigation and browser refresh
- [ ] **ACTV-07**: Frontend state syncs with server preference on load and after changes

### Auto-Assignment

- [ ] **AUTO-01**: Material withdrawal form pre-fills active Baustelle
- [ ] **AUTO-02**: Tool/vehicle reservation form pre-fills active Baustelle
- [ ] **AUTO-03**: Time entry form pre-fills active Baustelle when work_type is 'site'
- [ ] **AUTO-04**: User can change or remove assignment before submission

### Opt-Out Dialog

- [ ] **DLOG-01**: Confirmation dialog shows on auto-assignment (unobtrusive, non-blocking)
- [ ] **DLOG-02**: Dialog auto-confirms after 5 seconds if user takes no action
- [ ] **DLOG-03**: User can change project from the dialog
- [ ] **DLOG-04**: User can dismiss to leave unassigned
- [ ] **DLOG-05**: Dialog does not block other app actions

### E2E Tests

- [ ] **TEST-16**: E2E test for setting active Baustelle
- [ ] **TEST-17**: E2E test for auto-assignment on material withdrawal
- [ ] **TEST-18**: E2E test for opt-out dialog interaction

## v2.0 Requirements

Deferred to future release.

### Baustelle Lifecycle

- **LIFE-01**: Restrict active context to 'active' status Baustellen only
- **LIFE-02**: Define status transition workflow (currently Planned → Active → Completed → Archived exists but no UI to change status)

### Offline Enhancement

- **OFFL-01**: Active preference stored in IndexedDB for offline access
- **OFFL-02**: Preference syncs when connectivity restored
- **OFFL-03**: System handles stale active project during offline operations

### Dashboard Integration

- **DASH-01**: Dashboard filters by active Baustelle
- **DASH-02**: Dashboard shows active Baustelle statistics

## Out of Scope

| Feature | Reason |
|---------|--------|
| Manual color selection | Hash-based auto-assignment is simpler, no user decisions |
| Global active project | Per-user context is correct; teams work different sites |
| Active project for non-site work | work_type 'workshop', 'travel', 'other' should not auto-assign |
| Status-based active restriction | Deferred — Baustelle status lifecycle needs more thought (no UI to change status currently) |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| PREF-01 | — | Pending |
| PREF-02 | — | Pending |
| PREF-03 | — | Pending |
| DEDU-01 | — | Pending |
| DEDU-02 | — | Pending |
| DEDU-03 | — | Pending |
| ACTV-01 | — | Pending |
| ACTV-02 | — | Pending |
| ACTV-03 | — | Pending |
| ACTV-04 | — | Pending |
| ACTV-05 | — | Pending |
| ACTV-06 | — | Pending |
| ACTV-07 | — | Pending |
| AUTO-01 | — | Pending |
| AUTO-02 | — | Pending |
| AUTO-03 | — | Pending |
| AUTO-04 | — | Pending |
| DLOG-01 | — | Pending |
| DLOG-02 | — | Pending |
| DLOG-03 | — | Pending |
| DLOG-04 | — | Pending |
| DLOG-05 | — | Pending |
| TEST-16 | — | Pending |
| TEST-17 | — | Pending |
| TEST-18 | — | Pending |

**Coverage:**
- v1.7 requirements: 25 total
- Mapped to phases: 0
- Unmapped: 25 ⚠️

---
*Requirements defined: 2026-04-30*
*Last updated: 2026-04-30 after v1.7 requirements defined*
