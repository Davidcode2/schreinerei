# Feature Research: Active Project Context

**Domain:** Active Baustelle (Construction Site) Context
**Researched:** 2026-04-30
**Confidence:** MEDIUM (based on codebase analysis + UX patterns from similar apps)

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Persistent status indicator | Users need to know their current context at all times. Similar to Slack workspace indicator, GitHub repo switcher. | LOW | Badge/chip in header or bottom nav. Shows active Baustelle name + color. |
| Easy context switch | Users work on multiple projects. Must be able to change active Baustelle quickly. | LOW | Toggle on overview page + dashboard. Dropdown in header as alternative. |
| Auto-assignment visibility | When actions are auto-assigned, users must SEE this happening. Hidden automation = confusion. | MEDIUM | Show "Wird gebucht auf: [Baustelle Name]" in dialogs. Pre-fill field, allow change. |
| Single active project per user | One user = one active Baustelle at a time. Industry standard for field workers. | LOW | Store in user preferences or session. User-scoped, not global. |
| Opt-out capability | Sometimes user needs to book to different project. Must be able to override. | MEDIUM | Dialog shows pre-filled Baustelle with option to change or clear. 5-second auto-confirm with change/dismiss. |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Auto-assigned colors per Baustelle | Visual distinction at a glance. Reduces errors. "I'm on the blue project." | LOW | Hash-based color assignment from Baustelle name or ID. Store in Baustelle entity. |
| Context-aware dashboard | When active Baustelle is set, dashboard shows project-specific data (hours, materials, reservations). | MEDIUM | Filter existing dashboard queries by active site_id. |
| Smart context suggestions | Based on GPS location or calendar, suggest which Baustelle to activate. | HIGH | Requires GPS integration or calendar parsing. Defer to v2. |
| Context history | "Recently active Baustellen" for quick switching. | LOW | Store last 5 active Baustellen per user. Show in dropdown. |
| Cross-device sync | Active Baustelle syncs across mobile/tablet/desktop. | LOW | Store in backend user preferences, not localStorage. |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Multiple active Baustellen | "I work on multiple projects at once" | Cognitive load. Which Baustelle gets the material? Confusing UX. | Single active Baustelle. If truly working on multiple, switch context per action. |
| Auto-switch based on GPS | "Phone knows which site I'm at" | GPS drift, indoor locations, multiple sites nearby. False positives frustrate users. | Manual switch with location hint ("Near Baustelle X, switch?"). |
| Forced context (no opt-out) | "Ensures data quality" | User frustration when needing to book to different project. Workarounds emerge. | Pre-fill with easy override. 5-second confirmation dialog allows change. |
| Global active Baustelle | "Everyone on same project" | Different users work on different sites. One size doesn't fit all. | Per-user active Baustelle. Team leads can see all active contexts if needed. |
| Permanent assignment | "Always book to same Baustelle" | Forgets to change when switching projects. Data pollution. | Active Baustelle persists until changed, but shows indicator. Nudge to confirm on Mondays. |

## Feature Dependencies

```
[Active Baustelle Status Indicator]
    └──requires──> [Backend: Store active_site_id per user]
    └──requires──> [Frontend: User preferences context/hook]

[Auto-Assignment to Material Withdrawals]
    └──requires──> [Active Baustelle Status Indicator]
    └──requires──> [Backend: Add site_id to WithdrawRequest]
    └──requires──> [Database: Add site_id FK to material_deductions table]
    └──enhances──> [WithdrawDialog: Pre-fill site_id, show "booking to X"]

[Auto-Assignment to Reservations]
    └──requires──> [Active Baustelle Status Indicator]
    └──requires──> [ReservationDialog: Pre-fill site_id from context] ✓ site_id field exists
    └──enhances──> [Reservation visibility: Show which Baustelle resources are booked for]

[Auto-Assignment to Time Entries]
    └──requires──> [Active Baustelle Status Indicator]
    └──requires──> [TimeEntryDialog: Pre-fill site_id from context] ✓ site_id field exists
    └──conflicts──> [work_type: workshop/travel/other] → Don't auto-assign for non-site work types

[Color Assignment per Baustelle]
    └──requires──> [Backend: Add color field to Site entity]
    └──requires──> [Frontend: Display color in status indicator]
    └──enhances──> [Visual distinction in lists and calendar]

[Opt-out Dialog]
    └──requires──> [Active Baustelle Status Indicator]
    └──requires──> [Dialog: 5-second auto-confirm with change/dismiss options]
```

### Dependency Notes

- **Material withdrawals need backend change:** Current `WithdrawRequest` only has `notes`, no `site_id`. Need to add FK to `material_deductions` table.
- **Reservations and Time Entries already have site_id:** Frontend just needs to pre-fill from active context.
- **work_type affects auto-assignment:** Only auto-assign when `work_type === "site"`. For workshop/travel/other, leave site_id empty.
- **Color assignment:** Can be computed client-side from Baustelle ID (hash-based) OR stored in database. Recommendation: compute client-side for simplicity, no schema change.

## Expected Behavior by Feature

### 1. Active Baustelle Status Indicator

**Pattern:** Persistent chip/badge in header or bottom navigation.

```
┌─────────────────────────────────────────────────┐
│  Schreinerei App                    [🏠 Müller] │  ← Header with active Baustelle
│                                     (blue chip)  │
├─────────────────────────────────────────────────┤
│  Dashboard                                       │
│  ...                                             │
└─────────────────────────────────────────────────┘
```

**Alternative (Mobile Bottom Nav):**
```
┌─────────────────────────────────────────────────┐
│  Content...                                      │
│                                                  │
├─────────────────────────────────────────────────┤
│  [Home] [Inventar] [Baustellen] [Fuhrpark]       │
│                    ▲                             │
│                    └── Active: Müller (blue)     │
└─────────────────────────────────────────────────┘
```

**Implementation Notes:**
- Store `activeSiteId` in React context + backend user preferences
- Fetch on app load, persist on change
- Color = hash of site ID (consistent across sessions)
- Clicking opens quick-switch dropdown

**Complexity:** LOW — Standard UI pattern, context management.

### 2. Context Switch Toggle

**Pattern:** Two locations for switching:

1. **Baustellen Overview Page:** Each Baustelle row has "Als aktiv setzen" button
2. **Dashboard:** Quick-switch dropdown or button

```
┌─────────────────────────────────────────────────┐
│  Baustellen                          [Müller ▼] │  ← Current active shown
├─────────────────────────────────────────────────┤
│  Renovierung Müller                              │
│  Kunde: Familie Müller                           │
│  Status: Aktiv                                   │
│  [Als aktiv setzen] ← Only shown if not active   │
│                                                  │
│  Neubau Schmidt                                  │
│  Kunde: Herr Schmidt                             │
│  Status: Geplant                                 │
│  [Als aktiv setzen]                              │
└─────────────────────────────────────────────────┘
```

**Implementation Notes:**
- Button only shown for Baustellen with status `active` (not `planned`, `completed`, `archived`)
- On click: Update active context, show toast "Aktive Baustelle: Müller"
- Refresh any context-dependent views

**Complexity:** LOW — Simple state update + UI feedback.

### 3. Auto-Assignment with Opt-out Dialog

**Pattern:** Pre-fill + 5-second confirmation with change/dismiss.

**Scenario: Material Withdrawal**
```
┌─────────────────────────────────────────────────┐
│  Material entnehmen                              │
│  Entnahme von Schrauben M6x20                    │
│                                                  │
│  Menge: [5]                                      │
│                                                  │
│  ┌─────────────────────────────────────────────┐ │
│  │ 📍 Wird gebucht auf: Müller (aktiv)        │ │
│  │    [Ändern] [Ohne Baustelle]               │ │
│  └─────────────────────────────────────────────┘ │
│                                                  │
│  Notiz: (optional)                               │
│  [________________________________]              │
│                                                  │
│  Wird automatisch bestätigt in 5...              │
│                                                  │
│  [Abbrechen]  [5 Stück entnehmen]                │
└─────────────────────────────────────────────────┘
```

**Behavior:**
1. Dialog opens with active Baustelle pre-selected
2. Shows "Wird gebucht auf: [Name]" message
3. User can:
   - Click "Ändern" to select different Baustelle
   - Click "Ohne Baustelle" to clear assignment
   - Wait 5 seconds for auto-confirm
   - Click "Entnehmen" to confirm immediately
4. On auto-confirm or manual confirm: Submit with site_id

**Implementation Notes:**
- Timer resets if user interacts (changes Baustelle, types in notes)
- "Ändern" opens Baustelle selector dropdown
- Store site_id in withdrawal record

**Complexity:** MEDIUM — Timer logic, multiple interaction paths.

### 4. Work Type Logic for Time Entries

**Pattern:** Auto-assign only for `work_type: "site"`.

```
┌─────────────────────────────────────────────────┐
│  Zeit buchen                                     │
│                                                  │
│  Art der Arbeit:                                 │
│  [Baustelle] [Werkstatt] [Fahrt] [Sonstiges]    │
│     ▲                                            │
│     └── Selected → Auto-assign to active Baustelle
│                                                  │
│  If "Baustelle" selected:                        │
│  ┌─────────────────────────────────────────────┐ │
│  │ 📍 Baustelle: Müller (aktiv)               │ │
│  │    [Ändern]                                │ │
│  └─────────────────────────────────────────────┘ │
│                                                  │
│  If other work type selected:                    │
│  (No Baustelle field shown)                      │
│                                                  │
│  Stunden: [4]                                    │
│  Datum: [2024-04-30]                             │
│                                                  │
│  [Abbrechen]  [Buchen]                           │
└─────────────────────────────────────────────────┘
```

**Implementation Notes:**
- Only show Baustelle selector when `work_type === "site"`
- Pre-fill with active Baustelle
- If user switches work_type to non-site, clear site_id

**Complexity:** LOW — Conditional UI, existing site_id field.

### 5. Color Assignment per Baustelle

**Pattern:** Hash-based color from Baustelle ID.

```typescript
// Generate consistent color from ID
function getBaustelleColor(id: string): string {
  const colors = [
    '#3B82F6', // blue
    '#10B981', // green
    '#F59E0B', // amber
    '#EF4444', // red
    '#8B5CF6', // purple
    '#EC4899', // pink
    '#06B6D4', // cyan
    '#84CC16', // lime
  ]
  
  // Simple hash
  let hash = 0
  for (let i = 0; i < id.length; i++) {
    hash = id.charCodeAt(i) + ((hash << 5) - hash)
  }
  return colors[Math.abs(hash) % colors.length]
}
```

**Implementation Notes:**
- Client-side computation, no database change
- Use in status indicator chip, calendar events, list badges
- Ensure contrast with white text (all colors above are dark enough)

**Complexity:** LOW — Pure function, no state.

## Backend Changes Required

| Table | Change | Migration |
|-------|--------|-----------|
| `material_deductions` | Add `site_id UUID NULL` with FK to `sites` | Yes |
| `users` (or new table) | Add `active_site_id UUID NULL` with FK to `sites` | Yes |

**Alternative for user active site:**
- Store in `user_preferences` table (JSONB field)
- Or new `active_contexts` table for flexibility

**API Changes:**
- `GET /api/v1/users/me/preferences` — Returns `{ active_site_id: string | null }`
- `PATCH /api/v1/users/me/preferences` — Update active site
- `WithdrawRequest` — Add optional `site_id` field
- Material deduction response — Include `site_id` and `site_name`

## MVP Definition for v1.7

### Launch With (This Milestone)

- [x] Backend: Add `active_site_id` to user preferences
- [x] Backend: Add `site_id` to material deductions
- [x] Frontend: Active Baustelle status indicator (header chip)
- [x] Frontend: Toggle to set active Baustelle (overview + dashboard)
- [x] Frontend: Auto-assign to reservations (pre-fill site_id)
- [x] Frontend: Auto-assign to time entries (when work_type = site)
- [x] Frontend: Auto-assign to material withdrawals (new site_id field)
- [x] Frontend: Opt-out dialog with 5-second auto-confirm
- [x] Frontend: Color assignment per Baustelle (hash-based)

### Implementation Order

1. **Backend:** Add migrations for `active_site_id` and `material_deductions.site_id`
2. **Backend:** Add user preferences endpoints (GET/PATCH)
3. **Backend:** Update `WithdrawRequest` DTO and handler
4. **Frontend:** Create `ActiveContextProvider` with React context
5. **Frontend:** Add status indicator chip to header
6. **Frontend:** Add "Als aktiv setzen" button to Baustellen list
7. **Frontend:** Update WithdrawDialog with site_id pre-fill + opt-out
8. **Frontend:** Update ReservationDialog with site_id pre-fill
9. **Frontend:** Update TimeEntryDialog with conditional site_id pre-fill
10. **Frontend:** Add color utility function
11. **E2E:** Test active context flow end-to-end

### Add After Validation (v1.8)

- [ ] Context-aware dashboard (filter by active Baustelle)
- [ ] Context history (recently active Baustellen)
- [ ] Cross-device sync (backend preferences)

### Future Consideration (v2+)

- [ ] Smart context suggestions (GPS/calendar)
- [ ] Team context visibility (see who's on which Baustelle)
- [ ] Context-based notifications (alerts for active Baustelle)

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Active Baustelle indicator | HIGH | LOW | P1 |
| Context switch toggle | HIGH | LOW | P1 |
| Auto-assign to reservations | HIGH | LOW | P1 |
| Auto-assign to time entries | HIGH | LOW | P1 |
| Auto-assign to withdrawals | HIGH | MEDIUM | P1 |
| Opt-out dialog | HIGH | MEDIUM | P1 |
| Color per Baustelle | MEDIUM | LOW | P2 |
| Context-aware dashboard | MEDIUM | MEDIUM | P3 |
| Context history | LOW | LOW | P3 |
| Smart suggestions | LOW | HIGH | P3 |

**Priority key:**
- P1: Must have for this milestone (core functionality)
- P2: Should have, add when possible
- P3: Nice to have, future consideration

## Integration with Existing Code

### Files to Modify

| File | Change |
|------|--------|
| `frontend/src/lib/ActiveContext.tsx` | New file: Context provider + hook |
| `frontend/src/components/layout/Header.tsx` | Add status indicator chip |
| `frontend/src/pages/sites/SitesListPage.tsx` | Add "Als aktiv setzen" button |
| `frontend/src/pages/inventory/WithdrawDialog.tsx` | Add site_id field + opt-out UI |
| `frontend/src/pages/fleet/ReservationDialog.tsx` | Pre-fill site_id from context |
| `frontend/src/pages/sites/TimeEntryDialog.tsx` | Conditional site_id pre-fill |
| `src/modules/inventory/domain/entities.rs` | Add `site_id` to MaterialDeduction |
| `src/modules/iam/domain/user_preferences.rs` | New file: Active context preferences |
| `src/modules/inventory/application/handlers.rs` | Update withdraw handler |

### New Files to Create

| File | Purpose |
|------|---------|
| `frontend/src/lib/ActiveContext.tsx` | React context for active Baustelle |
| `frontend/src/hooks/useActiveBaustelle.ts` | Hook to access/update active context |
| `frontend/src/utils/baustelleColor.ts` | Color generation from ID |
| `src/db/migrations/XXX_add_site_to_deductions.sql` | Migration for material_deductions.site_id |
| `src/db/migrations/XXX_add_active_site_to_users.sql` | Migration for user preferences |

## Edge Cases and Error Handling

| Scenario | Expected Behavior |
|----------|-------------------|
| Active Baustelle is archived | Clear active context, show notification "Baustelle wurde archiviert" |
| User has no active Baustelle | Show "Keine aktive Baustelle" in indicator, no auto-assignment |
| Baustelle deleted while active | Same as archived — clear context, notify |
| Multiple devices, different active Baustelle | Last write wins (backend preferences sync) |
| User changes active Baustelle mid-dialog | Dialog keeps original Baustelle (snapshot on open) |
| Offline mode | Use cached active Baustelle, sync on reconnect |

## Sources

- **Codebase Analysis:**
  - `frontend/src/pages/inventory/WithdrawDialog.tsx` — Current withdrawal flow
  - `frontend/src/pages/fleet/ReservationDialog.tsx` — Reservation with site_id field
  - `frontend/src/pages/sites/TimeEntryDialog.tsx` — Time entry with site_id field
  - `frontend/src/types/inventory.ts` — WithdrawRequest DTO (no site_id)
  - `frontend/src/types/fleet.ts` — Reservation has site_id field
  - `frontend/src/types/sites.ts` — TimeEntry has site_id field

- **UX Patterns:**
  - Slack workspace indicator (persistent, clickable, color-coded)
  - GitHub repository context (visible in header, quick switch)
  - Jira project selector (dropdown with search)
  - Notion workspace switcher (persistent, colorful)

- **PROJECT.md Requirements:**
  - Status indicator showing which Baustelle is active
  - Auto-assigned colors per Baustelle
  - Toggle to set active Baustelle (overview + dashboard)
  - Single active Baustelle per user (user-scoped)
  - Auto-assignment: material deductions and tool reservations
  - Opt-out dialog: 5-second confirmation with change/dismiss

---
*Feature research for: Schreinerei v1.7 Active Project Context*
*Researched: 2026-04-30*
