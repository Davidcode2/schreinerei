# Phase 27: Tabbed Activity Feed - Context

**Gathered:** 2026-05-01
**Status:** Ready for planning
**Source:** ROADMAP.md + codebase analysis

<domain>
## Phase Boundary

Users can view and add notes in an organized activity feed with clear navigation. The activity feed is reorganized with tabs for different content types.

**What's in scope:**
- Add tab navigation to ActivityFeed (Notizen/Dokumente | Material)
- Add note creation button and modal
- Each entry shows timestamp and content preview
- Cursor-based pagination for feed entries

**What's NOT in scope:**
- Photo upload (Phase 29)
- Material history backend (Phase 28)
- Status changes (Phase 26 - already complete)

**Dependencies:**
- Phase 26 complete ✅
- ActivityFeed component exists ✅
- Activity type support exists ✅

</domain>

<decisions>
## Implementation Decisions

### Tab Navigation
- Use shadcn/ui Tabs component
- Two tabs: "Notizen/Dokumente" and "Material"
- Default tab: "Notizen/Dokumente"
- Tab state managed locally in component

### Note Creation Flow
- Add "Notiz hinzufügen" button in ActivityFeed header
- Opens modal with textarea
- Submit creates ActivityType::Note
- Content field stores note text

### Feed Display
- Show timestamp (relative time)
- Show content preview (line-clamp-2)
- Photo notes show preview image
- Status changes show arrow icon (already implemented)

### Pagination
- Use cursor-based pagination
- Load more button or infinite scroll
- Backend already supports cursor parameter

### the agent's Discretion
- Exact tab styling and layout
- Button placement in header
- Modal design and size
- Pagination UX (button vs infinite scroll)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Frontend Files
- `frontend/src/pages/sites/ActivityFeed.tsx` — Current ActivityFeed component
- `frontend/src/pages/sites/SiteDetailPage.tsx` — Uses ActivityFeed
- `frontend/src/lib/api/hooks/useSites.ts` — useCreateActivity hook (lines 194-209)
- `frontend/src/components/ui/tabs.tsx` — shadcn/ui Tabs component

### Backend Files
- `src/modules/sites/domain/activity.rs` — Activity domain with ActivityType::Note
- `src/modules/sites/api/routes.rs` — Activity creation endpoint

### Type Definitions
- `frontend/src/types/sites.ts` — Activity type, CreateActivityRequest

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **Tabs component** — shadcn/ui Tabs available
- **ActivityFeed component** — Already displays activities, needs tabs
- **useCreateActivity hook** — Already exists for creating notes
- **Dialog component** — For note creation modal
- **Textarea component** — shadcn/ui Textarea available

### Established Patterns
- **Activity display** — Already implemented for photo/note/status_change
- **Modal pattern** — See StatusChangeModal, TimeEntryDialog
- **Tab navigation** — Standard shadcn/ui Tabs pattern

### Integration Points
- **ActivityFeed.tsx** — Wrap content in Tabs component
- **SiteDetailPage.tsx** — Pass activities to tabbed feed
- **useCreateActivity** — Call with activity_type: "note" and content

</code_context>

<specifics>
## Specific Requirements

### FEED-01: Activity feed has two tabs
- Add Tabs component to ActivityFeed
- Tab 1: "Notizen/Dokumente" — shows photo, note, status_change activities
- Tab 2: "Material" — placeholder for Phase 28
- Material tab shows "Coming soon" message

### FEED-02: Add note button opens modal
- Add Button in ActivityFeed header
- Opens CreateNoteModal
- Modal has Textarea for note content
- Submit creates Activity with type: "note"

### FEED-03: Each entry shows timestamp and preview
- Already implemented: formatRelativeTime() function
- Already implemented: line-clamp-2 for content
- Ensure all activity types show properly

### FEED-04: Cursor-based pagination
- Backend already supports ?cursor parameter
- Add "Mehr laden" button at bottom
- Or implement infinite scroll with IntersectionObserver
- Load next page and append to list

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 27-tabbed-activity-feed*
*Context gathered: 2026-05-01*
