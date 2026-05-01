# Phase 26: Status Change Workflow - Context

**Gathered:** 2026-05-01
**Status:** Ready for planning
**Source:** ROADMAP.md + codebase analysis + autonomous discuss

<domain>
## Phase Boundary

Users can change site status through a controlled workflow with validation and audit trail. The status change modal integrates with the existing SiteDetailPage and creates activity feed entries for audit visibility.

**What's in scope:**
- Status change modal UI triggered from StatusBadge on SiteDetailPage
- Backend validation of status transitions (already exists)
- Activity feed entries for status changes (StatusChange type already exists)
- Rename "Aktiv" button to "Auswählen" in SiteCard component
- Optimistic locking error handling with user-friendly UX
- Frontend-backend integration

**What's NOT in scope:**
- Changing the status state machine (already implemented)
- Creating new status types
- Modifying backend status transition logic
- Photo/note activities (separate Phase 29)

**Status Flow:**
Planned → Active → Completed → Archived

**Existing Infrastructure:**
- ✅ Backend: SiteStatus enum, can_transition_to() validation, SiteStatusChanged event
- ✅ Backend: ActivityType::StatusChange enum variant
- ✅ Frontend: StatusBadge component, ActivityFeed component, Dialog components
- ✅ Frontend: useUpdateSite and useActivities hooks

</domain>

<decisions>
## Implementation Decisions

### Modal Interaction Design
- Show only valid transition buttons — Planned shows "Aktivieren", Active shows "Abschließen", Completed shows "Archivieren"
- Single-step modal with clear action button ("Status ändern") and immediate execution
- Show current status as highlighted chip, target status as action button with arrow icon
- Modal title: "Baustellen-Status ändern"

**Rationale:** Users see only valid actions (no confusion), single-step is faster, arrow icon indicates directionality.

### Status Chip Trigger
- Tap on StatusBadge opens modal (add cursor-pointer and hover effect)
- Visual affordance: underline or subtle glow on hover
- Mobile behavior: same as desktop — tap anywhere on badge

**Rationale:** StatusBadge is natural interaction point, minimal UI change needed, consistent with mobile patterns.

### Activity Feed Display
- Activity text format: "{old_status} → {new_status}" (e.g., "Geplant → Aktiv")
- Icon style: Arrow icon (ArrowRight) in status-colored circle
- Show user attribution: "Geändert von {user_name}"
- Insert at top of activity feed (most recent first)

**Rationale:** Clear visual transition, arrow icon is universally understood, user attribution provides audit trail visibility.

### Error Handling UX
- Error message: Toast notification "Status wurde bereits geändert. Aktualisieren..." with auto-refresh
- Recovery action: Auto-refresh site data after 1 second, keep modal open with updated status
- No optimistic update — wait for server confirmation before updating UI

**Rationale:** Non-blocking toast better for mobile, auto-refresh shows current state, server-authoritative prevents confusion.

### "Aktiv" Button Rename (STAT-03)
- Change button text in SiteCard.tsx line 72 from "Aktiv" / "Aktiv setzen" to "Auswählen" / "Auswählen"
- This is the active Baustelle selection button, NOT the status
- Also update line 118 badge text if needed

**Rationale:** Avoid confusion between "active Baustelle" (user's current work context) and "Aktiv" status (site lifecycle state).

### the agent's Discretion
- Exact styling of status change modal (spacing, colors, typography)
- Toast notification duration and exact wording
- Error message tone and formatting
- Animation/transition effects for modal open/close

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Backend Files (Status Logic)
- `src/modules/sites/domain/site.rs` — Site aggregate with can_transition_to() method (lines 24-37)
- `src/modules/sites/domain/events.rs` — SiteStatusChangedPayload event (lines 27-46)
- `src/modules/sites/domain/activity.rs` — ActivityType::StatusChange enum (lines 1-80)
- `src/modules/sites/infrastructure/site_repository.rs` — Status transition validation (lines 146-150)
- `src/modules/sites/api/routes.rs` — Site update endpoint
- `src/common/types.rs` — SiteStatus enum definition (lines 309-342)

### Frontend Files (UI Components)
- `frontend/src/pages/sites/SiteDetailPage.tsx` — Site detail page with StatusBadge (line 79)
- `frontend/src/pages/sites/SitesListPage.tsx` — Sites list with active Baustelle toggle
- `frontend/src/components/sites/SiteCard.tsx` — SiteCard with "Aktiv" button to rename (line 72)
- `frontend/src/pages/sites/ActivityFeed.tsx` — Activity feed component (needs StatusChange support)
- `frontend/src/components/ui/dialog.tsx` — shadcn/ui Dialog component
- `frontend/src/lib/api/hooks/useSites.ts` — useUpdateSite hook (lines 54-66)

### Type Definitions
- `frontend/src/types/sites.ts` — SiteStatus type (line 7), Activity type

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **StatusBadge component** — Already displays status with color coding, needs onClick handler
- **Dialog component** — shadcn/ui Dialog available for modal
- **ActivityFeed component** — Exists but only handles photo/note types, needs StatusChange support
- **useUpdateSite hook** — Already exists with optimistic update handling
- **useActivities hook** — Already fetches activities for a site
- **Arrow icons** — ArrowRight from lucide-react available

### Established Patterns
- **Dialog usage** — See AddSiteDialog.tsx, TimeEntryDialog.tsx for modal patterns
- **Toast notifications** — Using sonner library, see SitesListPage.tsx lines 44, 46
- **Status transitions** — Backend already validates with can_transition_to()
- **Activity creation** — Backend creates activities automatically, frontend just displays

### Integration Points
- **SiteDetailPage.tsx line 79** — StatusBadge needs onClick to open modal
- **SiteCard.tsx line 72** — Button text needs change from "Aktiv" to "Auswählen"
- **ActivityFeed.tsx** — Needs case for activity_type === "status_change"
- **useUpdateSite** — Call with { id, status: newStatus } to trigger status change

</code_context>

<specifics>
## Specific Requirements

### STAT-01: User can change site status via modal
- Create StatusChangeModal component
- Wire StatusBadge onClick to open modal
- Pass current site status to modal
- Display valid transition options based on can_transition_to()

### STAT-02: Chip tap opens status change modal
- Add onClick handler to StatusBadge in SiteDetailPage
- Add cursor-pointer class
- Add hover effect (underline or background change)
- Open modal with current site context

### STAT-03: "Aktiv" button renamed to "Auswählen"
- File: `frontend/src/components/sites/SiteCard.tsx`
- Line 72: Change from `{isActive ? "Aktiv" : "Aktiv setzen"}` to `{isActive ? "Auswählen" : "Auswählen"}`
- Line 118: Badge shows "Aktiv" — consider changing to "Ausgewählt" for clarity

### STAT-04: Status change creates activity entry
- Backend already creates SiteStatusChanged event
- Backend already has ActivityType::StatusChange
- Frontend needs to handle status_change in ActivityFeed
- Display: "{old_status} → {new_status}" with arrow icon

### STAT-05: Concurrent status changes handled
- Backend returns 400 with error message (already implemented)
- Frontend shows toast: "Status wurde bereits geändert. Aktualisieren..."
- Auto-refresh site data after 1 second
- Keep modal open with updated status

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 26-status-change-workflow*
*Context gathered: 2026-05-01 via autonomous discuss*
