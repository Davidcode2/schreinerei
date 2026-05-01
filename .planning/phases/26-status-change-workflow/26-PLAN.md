# Phase 26: Status Change Workflow - Plan

**Phase:** 26
**Name:** Status Change Workflow
**Created:** 2026-05-01
**Mode:** Autonomous

## Goal

Users can change site status through a controlled workflow with validation and audit trail.

## Success Criteria (from ROADMAP)

1. ✅ User can open status change modal by tapping status chip on site detail page
2. ✅ User can select valid status transitions (geplant → aktiv → abgeschlossen) with backend validation
3. ✅ "Aktiv" button renamed to "Auswählen" avoiding confusion with status name
4. ✅ Each status change creates activity entry visible in feed for audit trail
5. ✅ Concurrent status changes are rejected with clear error message (optimistic locking)

## Context Summary

**Existing Infrastructure:**
- Backend status state machine already complete (Site.can_transition_to())
- SiteStatusChanged event already defined
- ActivityType::StatusChange already exists
- StatusBadge, ActivityFeed, Dialog components available
- useUpdateSite hook available

**Key Files:**
- `frontend/src/pages/sites/SiteDetailPage.tsx` — StatusBadge needs onClick
- `frontend/src/components/sites/SiteCard.tsx` — Button text needs rename
- `frontend/src/pages/sites/ActivityFeed.tsx` — Needs status_change case
- `src/modules/sites/domain/site.rs` — Status transition logic
- `src/modules/sites/domain/activity.rs` — Activity domain

---

## Plan Structure

**3 plans** organized by vertical slices:

| Plan | Type | Description |
|------|------|-------------|
| 26-01-status-change-modal | feature | Status change modal with StatusBadge integration |
| 26-02-activity-feed-support | feature | ActivityFeed support for status changes |
| 26-03-button-rename | fix | Rename "Aktiv" to "Auswählen" button |

---

## Plan 26-01: Status Change Modal

**Type:** feature
**Estimate:** 2-3 hours

### Tasks

- [ ] **26-01-1**: Create StatusChangeModal component
  - Create `frontend/src/pages/sites/StatusChangeModal.tsx`
  - Props: `open`, `onOpenChange`, `siteId`, `currentStatus`, `onSuccess`
  - Use shadcn/ui Dialog component
  - Show current status as highlighted chip
  - Show only valid transition buttons based on status
  - Title: "Baustellen-Status ändern"

- [ ] **26-01-2**: Implement status transition buttons
  - Planned status: Show "Aktivieren" button ( Planned → Active )
  - Active status: Show "Abschließen" button ( Active → Completed )
  - Completed status: Show "Archivieren" button ( Completed → Archived )
  - Use ArrowRight icon to indicate direction
  - Button triggers useUpdateSite mutation

- [ ] **26-01-3**: Wire StatusBadge to open modal
  - File: `frontend/src/pages/sites/SiteDetailPage.tsx`
  - Line 79: Add onClick handler to StatusBadge
  - Add state for modal open/close
  - Add cursor-pointer and hover effects
  - Pass current site status to modal

- [ ] **26-01-4**: Handle mutation success
  - Call useUpdateSite with `{ id, status: newStatus }`
  - On success: close modal, refetch site data
  - On error: handle concurrent change error (see task 26-01-5)

- [ ] **26-01-5**: Implement error handling UX
  - Catch 400 error from backend (invalid transition)
  - Show toast: "Status wurde bereits geändert. Aktualisieren..."
  - Auto-refresh site data after 1 second
  - Keep modal open with updated status

**Verification:**
- [ ] Modal opens when tapping StatusBadge
- [ ] Only valid transitions shown as buttons
- [ ] Status change creates activity (check ActivityFeed)
- [ ] Concurrent change shows appropriate error
- [ ] Modal closes after successful change

---

## Plan 26-02: Activity Feed Support

**Type:** feature
**Estimate:** 1 hour

### Tasks

- [ ] **26-02-1**: Add status_change case to ActivityFeed
  - File: `frontend/src/pages/sites/ActivityFeed.tsx`
  - Add case for `activity.activity_type === "status_change"`
  - Display: "{old_status} → {new_status}" text
  - Use ArrowRight icon in status-colored circle

- [ ] **26-02-2**: Implement status change activity display
  - Map status values to German: planned → Geplant, active → Aktiv, etc.
  - Show transition: "Geplant → Aktiv"
  - Add user attribution: "Geändert von {user_name}"
  - Style with appropriate icon and colors

- [ ] **26-02-3**: Handle status change in activity data
  - Backend already creates ActivityType::StatusChange
  - Frontend needs to parse activity metadata
  - Extract old_status and new_status from activity content

**Verification:**
- [ ] Status changes appear in ActivityFeed
- [ ] Display shows old → new status
- [ ] User attribution visible
- [ ] Arrow icon used for status changes

---

## Plan 26-03: Button Rename

**Type:** fix
**Estimate:** 15 minutes

### Tasks

- [ ] **26-03-1**: Rename "Aktiv" button to "Auswählen"
  - File: `frontend/src/components/sites/SiteCard.tsx`
  - Line 72: Change from `{isActive ? "Aktiv" : "Aktiv setzen"}` to `{isActive ? "Auswählen" : "Auswählen"}`
  - This is the active Baustelle selection button

- [ ] **26-03-2**: Consider updating badge text
  - Line 118: Badge shows "Aktiv" when site is selected
  - Consider changing to "Ausgewählt" for consistency
  - Or keep as "Aktiv" if clear from context

**Verification:**
- [ ] Button shows "Auswählen" text
- [ ] No confusion between status and selection
- [ ] Existing tests still pass

---

## Technical Notes

### Status Transition Logic (Backend)
```rust
// Already implemented in src/modules/sites/domain/site.rs
pub fn can_transition_to(&self, new_status: SiteStatus) -> bool {
    match (&self.status, &new_status) {
        (SiteStatus::Planned, SiteStatus::Active) => true,
        (SiteStatus::Active, SiteStatus::Completed) => true,
        (SiteStatus::Completed, SiteStatus::Archived) => true,
        _ if self.status == new_status => true,
        _ => false,
    }
}
```

### Activity Creation (Backend)
- Backend emits `SiteStatusChanged` event
- Event contains: `site_id`, `old_status`, `new_status`, `changed_by`
- Activity is created automatically
- Frontend just displays it

### Status Badge Click Handler
```tsx
// In SiteDetailPage.tsx
const [statusModalOpen, setStatusModalOpen] = useState(false)

<StatusBadge 
  status={site.status} 
  className="cursor-pointer hover:underline"
  onClick={() => setStatusModalOpen(true)}
/>
```

### Modal Button Logic
```tsx
// In StatusChangeModal.tsx
const getTransitionButton = (currentStatus: SiteStatus) => {
  switch (currentStatus) {
    case 'planned':
      return <Button onClick={() => handleChange('active')}>
        <ArrowRight className="h-4 w-4 mr-2" />
        Aktivieren
      </Button>
    case 'active':
      return <Button onClick={() => handleChange('completed')}>
        <ArrowRight className="h-4 w-4 mr-2" />
        Abschließen
      </Button>
    // ... etc
  }
}
```

---

## Files to Create/Modify

### Create
- `frontend/src/pages/sites/StatusChangeModal.tsx`

### Modify
- `frontend/src/pages/sites/SiteDetailPage.tsx` — Add modal state and handler
- `frontend/src/pages/sites/ActivityFeed.tsx` — Add status_change case
- `frontend/src/components/sites/SiteCard.tsx` — Rename button

---

## Dependencies

- Phase 25 (v1.7 Active Project Context) — ✅ Complete
- Existing StatusBadge component — ✅ Available
- Existing Dialog component — ✅ Available
- Existing useUpdateSite hook — ✅ Available

---

## Risks

1. **Optimistic locking edge cases** — Multiple users changing status simultaneously
   - Mitigation: Clear error message with auto-refresh
   
2. **Activity metadata parsing** — Backend might not send old_status/new_status
   - Mitigation: Check API response, adjust if needed

---

## Next Steps

1. Execute Plan 26-01: Status Change Modal
2. Execute Plan 26-02: Activity Feed Support
3. Execute Plan 26-03: Button Rename
4. Run verification tests
5. Update STATE.md

---

*Plan created: 2026-05-01*
*Ready for execution: `/gsd-execute-phase 26`*
