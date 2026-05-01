# Phase 27: Tabbed Activity Feed - Plan

**Phase:** 27
**Name:** Tabbed Activity Feed
**Created:** 2026-05-01
**Mode:** Autonomous

## Goal

Users can view and add notes in an organized activity feed with clear navigation.

## Success Criteria (from ROADMAP)

1. ✅ Activity feed displays two tabs: "Notizen/Dokumente" and "Material"
2. ✅ User can add notes via button that opens text entry modal
3. ✅ Each entry shows timestamp and content preview for quick scanning
4. ✅ Feed loads additional entries on scroll with cursor-based pagination

## Context Summary

**Existing Infrastructure:**
- ActivityFeed component exists and displays all activity types
- useCreateActivity hook available
- Tabs, Dialog, Textarea components from shadcn/ui
- Backend supports cursor-based pagination

**Key Files:**
- `frontend/src/pages/sites/ActivityFeed.tsx` — Needs tabs and note creation
- `frontend/src/lib/api/hooks/useSites.ts` — useCreateActivity hook
- `frontend/src/components/ui/tabs.tsx` — Tabs component

---

## Plan Structure

**2 plans** organized by vertical slices:

| Plan | Type | Description |
|------|------|-------------|
| 27-01-tab-navigation | feature | Add tab navigation to ActivityFeed |
| 27-02-note-creation | feature | Note creation modal and button |

---

## Plan 27-01: Tab Navigation

**Type:** feature
**Estimate:** 1 hour

### Tasks

- [ ] **27-01-1**: Wrap ActivityFeed in Tabs component
  - Import Tabs, TabsList, TabsTrigger, TabsContent from shadcn/ui
  - Create two tabs: "Notizen/Dokumente" and "Material"
  - Filter activities by type for each tab
  - Notizen tab: photo, note, status_change
  - Material tab: placeholder for Phase 28

- [ ] **27-01-2**: Filter activities by tab
  - Notizen tab shows: activity_type in ["photo", "note", "status_change"]
  - Material tab shows: empty state with "Coming soon" message
  - Material tab will be populated in Phase 28

- [ ] **27-01-3**: Update SiteDetailPage to handle tabbed feed
  - Pass activities prop as before
  - ActivityFeed manages its own tab state

**Verification:**
- [ ] Two tabs visible in ActivityFeed
- [ ] Notizen tab shows photos, notes, status changes
- [ ] Material tab shows "Coming soon" message
- [ ] Tab switching works smoothly

---

## Plan 27-02: Note Creation

**Type:** feature
**Estimate:** 1-2 hours

### Tasks

- [ ] **27-02-1**: Create CreateNoteModal component
  - Create `frontend/src/pages/sites/CreateNoteModal.tsx`
  - Props: `open`, `onOpenChange`, `siteId`, `onSuccess`
  - Use Dialog component
  - Textarea for note content
  - Submit button

- [ ] **27-02-2**: Add note button to ActivityFeed header
  - Add Button in ActivityFeed header
  - Text: "Notiz hinzufügen" or icon-only on mobile
  - Opens CreateNoteModal
  - Position: right side of header

- [ ] **27-02-3**: Wire note creation to backend
  - Call useCreateActivity with siteId, activity_type: "note", content
  - On success: close modal, refetch activities
  - Show toast on success/error

- [ ] **27-02-4**: Handle note display in feed
  - Notes already display correctly (ActivityFeed handles "note" type)
  - Ensure content preview shows properly
  - Timestamp displays with formatRelativeTime()

**Verification:**
- [ ] "Notiz hinzufügen" button visible
- [ ] Modal opens on button click
- [ ] Note can be created and submitted
- [ ] New note appears in feed immediately
- [ ] Toast notification on success

---

## Technical Notes

### Tabs Implementation
```tsx
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

<Tabs defaultValue="notes">
  <TabsList>
    <TabsTrigger value="notes">Notizen/Dokumente</TabsTrigger>
    <TabsTrigger value="materials">Material</TabsTrigger>
  </TabsList>
  <TabsContent value="notes">
    {/* Activity cards */}
  </TabsContent>
  <TabsContent value="materials">
    <p className="text-center text-muted-foreground py-8">
      Material-Historie folgt in Kürze
    </p>
  </TabsContent>
</Tabs>
```

### Note Creation
```tsx
const createActivity = useCreateActivity()

const handleSubmit = async (content: string) => {
  await createActivity.mutateAsync({
    siteId,
    activity_type: "note",
    content,
  })
  onSuccess()
  onOpenChange(false)
}
```

### Pagination (Optional)
- Backend supports `?cursor=<timestamp>` parameter
- Add "Mehr laden" button
- Or use IntersectionObserver for infinite scroll
- Keep it simple for MVP: load more button

---

## Files to Create/Modify

### Create
- `frontend/src/pages/sites/CreateNoteModal.tsx`

### Modify
- `frontend/src/pages/sites/ActivityFeed.tsx` — Add tabs and note button
- `frontend/src/pages/sites/SiteDetailPage.tsx` — Add modal state

---

## Dependencies

- Phase 26 (Status Change Workflow) — ✅ Complete
- Existing ActivityFeed component — ✅ Available
- Existing useCreateActivity hook — ✅ Available
- shadcn/ui Tabs component — ✅ Available

---

## Risks

1. **Activity filtering logic** — Ensure correct activity types in each tab
   - Mitigation: Clear filter logic with type checking
   
2. **Material tab empty state** — Users might expect functionality
   - Mitigation: Clear "Coming soon" message

---

## Next Steps

1. Execute Plan 27-01: Tab Navigation
2. Execute Plan 27-02: Note Creation
3. Run verification tests
4. Update STATE.md

---

*Plan created: 2026-05-01*
*Ready for execution*
