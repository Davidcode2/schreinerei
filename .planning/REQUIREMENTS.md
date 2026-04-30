# Requirements: Schreinerei SaaS v1.7

**Defined:** 2026-04-30
**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler, keine vergessenen Bestellungen.

## v1.7 Requirements

Active Project Context — auto-assign materials, reservations, and time entries to the currently active Baustelle.

### Backend — User Preferences

- [x] **PREF-01**: User can store their active Baustelle preference server-side
- [x] **PREF-02**: System validates active Baustelle exists and is not archived/deleted
- [x] **PREF-03**: System clears preference automatically if Baustelle becomes invalid

### Backend — Material Deductions

- [x] **DEDU-01**: Material deductions can be linked to a Baustelle (FK column)
- [x] **DEDU-02**: WithdrawMaterial command accepts optional site_id parameter
- [ ] **DEDU-03**: Deduction details include Baustelle name when linked

### Frontend — Active Site UI

- [x] **ACTV-01**: User sees persistent indicator showing active Baustelle name and color
- [x] **ACTV-02**: User can toggle active Baustelle from overview page
- [x] **ACTV-03**: User can toggle active Baustelle from dashboard view
- [x] **ACTV-04**: Only one Baustelle can be active per user at a time
- [x] **ACTV-05**: Baustellen have auto-assigned colors (hash-based, no manual selection)
- [x] **ACTV-06**: Active state persists across page navigation and browser refresh
- [x] **ACTV-07**: Frontend state syncs with server preference on load and after changes

### Auto-Assignment

- [x] **AUTO-01**: Material withdrawal form pre-fills active Baustelle
- [x] **AUTO-02**: Tool/vehicle reservation form pre-fills active Baustelle
- [x] **AUTO-03**: Time entry form pre-fills active Baustelle when work_type is 'site'
- [x] **AUTO-04**: User can change or remove assignment before submission

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
| PREF-01 | 24 | Complete |
| PREF-02 | 24 | Complete |
| PREF-03 | 24 | Complete |
| DEDU-01 | 24 | Complete |
| DEDU-02 | 24 | Complete |
| DEDU-03 | 25 | Pending |
| ACTV-01 | 24 | Complete |
| ACTV-02 | 24 | Complete |
| ACTV-03 | 24 | Complete |
| ACTV-04 | 24 | Complete |
| ACTV-05 | 24 | Complete |
| ACTV-06 | 24 | Complete |
| ACTV-07 | 24 | Complete |
| AUTO-01 | 24 | Complete |
| AUTO-02 | 24 | Complete |
| AUTO-03 | 24 | Complete |
| AUTO-04 | 24 | Complete |

**Coverage:**
- v1.7 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-30*
*Last updated: 2026-04-30 after milestone audit gap reassignment to phases 24-25*
