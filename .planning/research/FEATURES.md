# Feature Research

**Domain:** CRUD Application UX Patterns (Delete, Edit, Status Workflows, Alerts)
**Researched:** 2026-04-30
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Delete confirmation dialog | Prevents accidental data loss. Standard in all CRUD apps since the 1980s. | LOW | AlertDialog component exists in shadcn/ui. Pattern: destructive variant, no default focus. |
| Edit existing records | Users make mistakes. Must be correctable. Basic CRUD operation. | LOW | Requires: populate form with existing data, submit to PATCH endpoint. |
| Inline validation feedback | Users need to know what's wrong BEFORE submitting. Generic toast errors after submit = poor UX. | MEDIUM | React state for field errors, display below inputs. TimeEntryDialog currently missing this. |
| Hours > 0 validation | Zero/negative hours make no sense. Backend already rejects, frontend should prevent. | LOW | Add min={0.5} and disable submit when hours <= 0. TimeEntryDialog bug (BUG-TIME-001). |
| Status transition buttons | If status exists, users expect to change it. Reservations have Pending→Confirmed→InUse→Completed. | MEDIUM | Requires UI for allowed transitions only. Backend enforces via state machine. |
| Delete button visibility | If delete endpoint exists, button should be visible. Fleet has endpoints, UI missing. | LOW | Add to dropdown menu or list row actions. Use Trash icon. |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Low stock alerts (proactive) | Users know when to reorder without checking inventory manually. Reduces stockouts. | MEDIUM | Backend has `is_low_stock` computed field and `/low-stock` endpoint. UI needs badge/warning. |
| Calendar click-to-create | Faster reservation creation. Direct interaction vs navigating to separate form. | MEDIUM | CalendarView exists. Add onClick for empty slots, pre-fill resource + time. |
| Overlap conflict details | Users see WHO has the resource booked, not just "not available." Better planning. | MEDIUM | Availability endpoint returns conflicts? If not, need to add to response. |
| Undo for destructive actions | "Undo delete" for 30 seconds reduces anxiety. Not common in CRUD apps. | HIGH | Requires soft-delete + restore. Defer to v2. |
| Bulk operations | Delete multiple items at once. Efficiency for power users. | HIGH | Not needed for pilot. |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| "Are you sure?" for EVERY delete | Prevents accidental deletion | Overuse leads to automation blindness. Users click yes without reading. NN/G: "if you cry wolf too many times, people will stop paying attention" | Use confirmation only for destructive/irreversible actions. For recoverable items, just delete with toast "Deleted. Undo?" |
| Delete confirmation typing "DELETE" | Extra safety for critical data | Overkill for non-critical items. Adds friction. | Reserve for truly dangerous operations (e.g., delete entire organization). Materials/sites not that critical. |
| Real-time validation on every keystroke | Immediate feedback | Can be annoying for fields like hours where user is still typing. Validate on blur or after short debounce. | Validate on blur + on submit. |
| Undo for ALL operations | Users feel safe | Massive complexity. Every operation needs reverse operation. | Implement for delete only (soft delete). Other operations can be re-edited. |
| Confirm dialog default to "Yes" | Faster workflow | Defeats purpose of confirmation. User can tab+enter without reading. | No default. Force conscious choice. |

## Feature Dependencies

```
[Delete Button UI]
    └──requires──> [DELETE API endpoint exists]
                       └──requires──> [Soft delete OR cascade handling]

[Status Transition UI]
    └──requires──> [PATCH endpoint for status field]
    └──requires──> [State machine validation in backend]
    └──requires──> [UI knows allowed transitions per current status]

[Edit Dialog]
    └──requires──> [PATCH endpoint exists]
    └──requires──> [GET by ID to populate form]
    └──requires──> [Form validation same as create]

[Inline Validation]
    └──requires──> [Validation rules defined]
    └──requires──> [Error state per field]
    └──enhances──> [Submit button disabled state]

[Low Stock Alerts UI]
    └──requires──> [Backend is_low_stock field] ✓ EXISTS
    └──requires──> [Low stock API endpoint] ✓ EXISTS at /api/v1/inventory/low-stock

[Calendar Click-to-Create]
    └──requires──> [ReservationDialog component] ✓ EXISTS
    └──requires──> [onClick handler for empty slots]
    └──requires──> [Pre-fill resourceId, startTime from click position]
```

### Dependency Notes

- **Delete Button requires DELETE endpoint:** Sites and Materials lack DELETE endpoints. Must add to backend first.
- **Time Entry Edit requires PATCH endpoint:** Time entries have no update/delete endpoints. Backend work needed.
- **Status Transition UI requires state machine:** ReservationStatus already has `can_transition_to()` method. SiteStatus needs similar.
- **Inline Validation enhances Submit:** Add field-level errors + disable submit when invalid. Prevents API errors.

## Current Backend API Status

| Resource | GET | POST | PATCH | DELETE | Notes |
|----------|-----|------|-------|--------|-------|
| Sites | ✓ | ✓ | ✓ | ✗ | Missing DELETE endpoint |
| Materials | ✓ | ✓ | ✗ | ✗ | Missing PATCH and DELETE |
| Vehicles | ✓ | ✓ | ✓ | ✓ | All CRUD ready |
| Tools | ✓ | ✓ | ✓ | ✓ | All CRUD ready |
| Reservations | ✓ | ✓ | ✓ | ✓ | DELETE = cancel (status change) |
| Time Entries | ✓ | ✓ | ✗ | ✗ | Missing PATCH and DELETE |

## Expected Behavior by Feature

### 1. Delete Confirmation Dialog

**Pattern:** AlertDialog (shadcn/ui) with destructive styling.

```
┌─────────────────────────────────────┐
│  Material löschen                   │
│                                     │
│  Möchten Sie "Schrauben M6x20"      │
│  wirklich löschen?                  │
│                                     │
│  Diese Aktion kann nicht rückgängig │
│  gemacht werden.                    │
│                                     │
│  [Abbrechen]  [Löschen]             │
│               (destructive)         │
└─────────────────────────────────────┘
```

**Implementation Notes:**
- Use AlertDialog component from shadcn/ui
- Button variants: outline (cancel), destructive (delete)
- No default focus (user must consciously choose)
- Specific item name in message (per NN/G guidelines)
- Toast on success: "Material gelöscht"

**Complexity:** LOW — AlertDialog exists, pattern straightforward.

### 2. Edit Dialog (Existing Records)

**Pattern:** Reuse create dialog with pre-populated data.

```
┌─────────────────────────────────────┐
│  Material bearbeiten                │
│                                     │
│  Name: [Schrauben M6x20    ]       │
│  Menge: [150              ]        │
│  Min. Bestand: [50       ]         │
│                                     │
│  [Abbrechen]  [Speichern]          │
└─────────────────────────────────────┘
```

**Implementation Notes:**
- Dialog should accept `mode: 'create' | 'edit'` and `initialData?: Material`
- On edit: populate form with existing values
- Submit to PATCH endpoint with partial update
- Toast on success: "Material aktualisiert"

**Complexity:** LOW — Reuse existing dialog, add mode prop.

### 3. Status Transition Workflow (Reservations)

**State Machine:**
```
                    ┌──────────────┐
                    │   Pending    │
                    └──────┬───────┘
                           │
              ┌────────────┼────────────┐
              ▼            │            ▼
        ┌──────────┐       │      ┌───────────┐
        │Cancelled │       │      │ Confirmed │
        └──────────┘       │      └─────┬─────┘
                           │            │
                           │      ┌─────┼─────┐
                           │      ▼           ▼
                           │  ┌────────┐  ┌────────┐
                           │  │ InUse  │  │Cancelled│
                           │  └────┬───┘  └────────┘
                           │       │
                           │       ▼
                           │  ┌───────────┐
                           └─►│ Completed │
                              └───────────┘
```

**UI Pattern:** Status badge + dropdown menu for allowed transitions.

```
┌─────────────────────────────────────────────────┐
│  Reservierung #1234                             │
│  Status: [Bestätigt ▼]                          │
│          ├── Starten (→ InUse)                  │
│          └── Stornieren (→ Cancelled)           │
└─────────────────────────────────────────────────┘
```

**Implementation Notes:**
- Badge color per status (pending=yellow, confirmed=blue, in_use=green, completed=gray, cancelled=red)
- Dropdown shows only valid transitions (use `can_transition_to()` logic)
- Call PATCH `/api/v1/fleet/reservations/{id}` with `{status: "in_use"}`

**Complexity:** MEDIUM — State machine logic, multiple UI states.

### 4. Low Stock Alerts

**Pattern:** Badge + dedicated section.

```
┌─────────────────────────────────────┐
│  Inventar                    [⚠️ 3] │
├─────────────────────────────────────┤
│  Schrauben M6x20                    │
│  Bestand: 15 (Min: 50) ⚠️          │
│                                     │
│  Dübel 8mm                          │
│  Bestand: 0 (Min: 100) 🔴          │
└─────────────────────────────────────┘
```

**Implementation Notes:**
- Badge in navigation showing count of low-stock items
- Yellow badge for low stock, red for zero stock
- Dedicated "Nachbestellen" section showing all low-stock
- Backend already has `/api/v1/inventory/low-stock` endpoint

**Complexity:** MEDIUM — Needs new UI component, notification logic.

### 5. Inline Validation Feedback

**Pattern:** Error message below field, red border, disabled submit.

```
┌─────────────────────────────────────┐
│  Zeit buchen                        │
│                                     │
│  Stunden: [0        ]               │
│           ▲ Stunden muss > 0 sein   │
│                                     │
│  [Abbrechen]  [Speichern]           │
│               (disabled)            │
└─────────────────────────────────────┘
```

**Implementation Notes:**
- Track `errors` state object: `{ hours: "Stunden muss > 0 sein" }`
- Validate on blur + on change (after first blur)
- Disable submit when any error exists
- Red border on input with error
- Use `<p className="text-sm text-destructive">` for error text

**Complexity:** MEDIUM — Need error state management, validation logic.

## MVP Definition for v1.6

### Launch With (This Milestone)

- [x] Hours > 0 validation (TimeEntryDialog) — Fix BUG-TIME-001
- [x] Delete confirmation dialogs — Sites, Materials, Vehicles, Tools
- [x] Edit capability — Time entries, Reservations (backend endpoints needed for time)
- [x] Status transition UI — Reservations (Confirmed→InUse→Completed, Cancel)
- [x] Low stock badge/alert — Show in inventory list

### Implementation Order

1. **Backend:** Add missing DELETE/PATCH endpoints (Sites, Materials, Time Entries)
2. **Frontend:** Delete confirmation dialogs (reuse AlertDialog)
3. **Frontend:** Edit dialogs (extend existing dialogs with mode prop)
4. **Frontend:** Inline validation (TimeEntryDialog first)
5. **Frontend:** Status transition dropdown (Reservations)
6. **Frontend:** Low stock badge (InventoryListPage)
7. **E2E:** Tests for all new operations

### Add After Validation (v1.7)

- [ ] Calendar click-to-create — Enhances reservation UX
- [ ] Overlap conflict details — Requires backend change
- [ ] Undo for delete — Requires soft-delete implementation

### Future Consideration (v2+)

- [ ] Bulk operations — Not needed for pilot
- [ ] Audit log for deletions — Compliance feature
- [ ] Archive vs delete — Soft-delete with recovery

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Delete confirmation dialog | HIGH | LOW | P1 |
| Hours validation fix | HIGH | LOW | P1 |
| Edit time entries | HIGH | MEDIUM | P1 |
| Edit reservations | HIGH | LOW | P1 |
| Status transition UI | MEDIUM | MEDIUM | P2 |
| Low stock alerts | MEDIUM | MEDIUM | P2 |
| Inline validation feedback | MEDIUM | MEDIUM | P2 |
| Calendar click-to-create | LOW | MEDIUM | P3 |
| Overlap conflict details | LOW | MEDIUM | P3 |
| Undo delete | LOW | HIGH | P3 |

**Priority key:**
- P1: Must have for this milestone (blocking issues)
- P2: Should have, add in this milestone
- P3: Nice to have, future consideration

## Sources

- **NN/G Confirmation Dialog Guidelines:** https://www.nngroup.com/articles/confirmation-dialog/
  - "Use specific information that allows users to understand the effects of their action"
  - "Do not use confirmation dialogs for routine actions"
  - "Avoid giving confirmation dialogs a default Yes answer"
  - "Instead of Yes/No, provide response options that summarize what will happen"
- **Existing Codebase Analysis:**
  - shadcn/ui Dialog and AlertDialog components
  - ReservationDialog, TimeEntryDialog patterns
  - Backend API routes status (routes.rs files)
  - ReservationStatus state machine (common/types.rs)
- **Issue Backlog:** .planning/ISSUE-BACKLOG.md (24 documented issues)

---
*Feature research for: Schreinerei v1.6 UX & Missing Functionality*
*Researched: 2026-04-30*
