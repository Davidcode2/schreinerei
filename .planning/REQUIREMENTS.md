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
| PREF-01 | 22 | Pending |
| PREF-02 | 22 | Pending |
| PREF-03 | 22 | Pending |
| DEDU-01 | 22 | Pending |
| DEDU-02 | 22 | Pending |
| DEDU-03 | 22 | Pending |
| ACTV-01 | 23 | Complete |
| ACTV-02 | 23 | Complete |
| ACTV-03 | 23 | Complete |
| ACTV-04 | 23 | Complete |
| ACTV-05 | 23 | Complete |
| ACTV-06 | 23 | Complete |
| ACTV-07 | 23 | Complete |
| AUTO-01 | 23 | Complete |
| AUTO-02 | 23 | Complete |
| AUTO-03 | 23 | Complete |
| AUTO-04 | 23 | Complete |

**Coverage:**
- v1.7 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-30*
*Last updated: 2026-04-30 after removing obsolete Phase 24 scope*
