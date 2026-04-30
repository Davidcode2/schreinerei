# Domain Pitfalls: Adding Delete/Edit/Status Transitions to Existing CRUD App

**Project:** Schreinerei SaaS v1.6
**Domain:** Construction management SaaS with offline-first PWA
**Researched:** 2026-04-30
**Confidence:** HIGH (based on codebase analysis + established patterns)

---

## Critical Pitfalls

### Pitfall 1: Delete Without Soft Delete Breaks Audit Trails

**What goes wrong:**
Hard delete permanently removes records. Users accidentally delete sites, materials, or reservations, losing historical data (time entries, stock withdrawal history, reservation patterns). Cannot recover from mistakes.

**Why it happens:**
Developers start with simple `DELETE FROM` thinking "users won't make mistakes." Later, business requirements for audit trails emerge but data is already gone.

**How to avoid:**
- Add `deleted_at` timestamp column to all deletable entities
- Modify queries to exclude `WHERE deleted_at IS NULL`
- Keep FK constraints intact (soft-deleted records still satisfy FKs)
- Implement `restore` endpoint for accidental deletions

**Warning signs:**
- Backend has `DELETE` routes (fleet has them) but no `deleted_at` column
- Users ask "can I undo this?" before deleting
- Time entries reference deleted sites → orphaned records

**Phase to address:** Backend delete implementation phase (before UI)

---

### Pitfall 2: Foreign Key Constraints Block Deletes Without Clear UX

**What goes wrong:**
User tries to delete a site that has time entries, or a vehicle with active reservations. Backend returns FK violation error, frontend shows generic "Delete failed" toast. User doesn't understand WHY delete failed.

**Why it happens:**
Database FK constraints exist but aren't checked before delete attempt. Frontend doesn't know what "dependent entities" exist.

**How to avoid:**
1. **Before delete, check dependencies:**
   ```rust
   // In service layer, before delete
   let has_time_entries = repo.has_time_entries(site_id).await?;
   if has_time_entries {
       return Err(AppError::Conflict("Site has time entries. Archive instead?"));
   }
   ```

2. **Return dependency info to frontend:**
   ```typescript
   // DELETE response
   { 
     success: false, 
     error: "CONFLICT",
     dependencies: { time_entries: 12, activities: 3 }
   }
   ```

3. **UI shows clear message:** "Cannot delete: 12 time entries exist. Archive site instead?"

**Warning signs:**
- Delete button exists but sometimes fails silently
- Users report "delete doesn't work"
- Database logs show FK constraint violations

**Phase to address:** Backend delete implementation (dependency checks)

---

### Pitfall 3: Offline Delete Sync Conflicts

**What goes wrong:**
User A deletes a vehicle while offline. User B creates a reservation for that vehicle. When A syncs, the vehicle is deleted but B's reservation now references a non-existent vehicle. OR: Both users edit the same reservation offline → last-write-wins, losing changes.

**Why it happens:**
Offline-first apps queue operations in IndexedDB. No conflict resolution strategy exists. Sync applies operations in arrival order, not causal order.

**Current state (from PROJECT.md):**
> "No conflict resolution for offline edits" — Known tech debt

**How to avoid:**
1. **Soft delete prevents orphan FKs:** Deleted records still exist, satisfy FK constraints
2. **Version vectors or timestamps:** Add `version` column, reject updates with stale version
   ```rust
   UPDATE reservations SET ... WHERE id = $1 AND version = $2
   // If 0 rows affected → conflict occurred
   ```
3. **Conflict detection in sync:**
   ```typescript
   // In sync worker
   if (serverVersion > localVersion) {
     // Prompt user or auto-merge
   }
   ```
4. **Tombstone for deletes:** Sync `deleted_at` timestamp, not actual delete

**Warning signs:**
- Offline operations occasionally fail on sync
- Users report "my changes disappeared"
- Data inconsistency between devices

**Phase to address:** Offline sync refactor (defer to v1.7+ as per tech debt)

---

### Pitfall 4: Status Transition Race Conditions

**What goes wrong:**
Two users view same reservation (status: Confirmed). User A clicks "Start Use", User B clicks "Cancel". Both requests hit backend. Depending on order, status ends up in wrong state or one user sees error.

**Why it happens:**
No optimistic locking. Backend checks `can_transition_to()` but between check and update, another request changes status.

**Current code (reservation.rs:37-55):**
```rust
pub fn can_transition_to(&self, new_status: ReservationStatus) -> bool {
    // This checks CURRENT state, not concurrent state
}
```

**How to avoid:**
1. **Database-level check in UPDATE:**
   ```sql
   UPDATE reservations 
   SET status = 'in_use', version = version + 1
   WHERE id = $1 AND status = 'confirmed'  -- Conditional update
   -- If 0 rows affected, status changed by another transaction
   ```

2. **Or use version column:**
   ```sql
   UPDATE reservations 
   SET status = $1, version = version + 1
   WHERE id = $2 AND version = $3
   -- If 0 rows, another client modified it
   ```

3. **Frontend refreshes on conflict:**
   ```typescript
   try {
     await updateReservation(...)
   } catch (ConflictError) {
     toast.error("Reservation was modified. Refreshing...")
     refetch()
   }
   ```

**Warning signs:**
- Intermittent "invalid status transition" errors
- Status shows one thing in list, another in detail view
- Users report "I clicked but nothing happened"

**Phase to address:** Backend status transition implementation

---

### Pitfall 5: Delete UI Without Confirmation Leads to Accidental Data Loss

**What goes wrong:**
User accidentally clicks delete button (fat finger, double-click). Data immediately gone. No undo. User frustration, support tickets.

**Why it happens:**
Delete button implemented as simple `onClick={handleDelete}` without confirmation dialog. "Delete quickly" sounds efficient until accidents happen.

**How to avoid:**
1. **Confirmation dialog for destructive actions:**
   ```tsx
   <AlertDialog>
     <AlertDialogTrigger asChild>
       <Button variant="destructive">Löschen</Button>
     </AlertDialogTrigger>
     <AlertDialogContent>
       <AlertDialogHeader>
         <AlertDialogTitle>Site wirklich löschen?</AlertDialogTitle>
         <AlertDialogDescription>
           Diese Aktion kann nicht rückgängig gemacht werden.
         </AlertDialogDescription>
       </AlertDialogHeader>
       <AlertDialogFooter>
         <AlertDialogCancel>Abbrechen</AlertDialogCancel>
         <AlertDialogAction onClick={handleDelete}>Löschen</AlertDialogAction>
       </AlertDialogFooter>
     </AlertDialogContent>
   </AlertDialog>
   ```

2. **Type-to-confirm for critical deletions:**
   ```
   Type the site name "Müller Baustelle" to confirm deletion:
   [_________________________]
   [Delete] [Cancel]
   ```

**Warning signs:**
- Delete button is same size/prominence as edit button
- No AlertDialog component imported in list pages
- Users ask "is there an undo?"

**Phase to address:** Frontend delete UI implementation

---

## Moderate Pitfalls

### Pitfall 6: Backend Delete Routes Exist But Frontend Doesn't Use Them

**What goes wrong:**
Fleet API has `DELETE /api/v1/fleet/vehicles/{id}` and `DELETE /api/v1/fleet/tools/{id}` routes. Frontend has no delete buttons for vehicles/tools. Code is written but unreachable.

**Why it happens:**
Backend-first development. API implemented but UI never connected. Tests pass for API but E2E tests don't cover UI flow.

**How to avoid:**
- Track API-UI gaps in issue tracker (already done in ISSUE-BACKLOG.md: MISSING-FLEET-001)
- E2E tests should verify user can complete operation through UI, not just API
- Code review checklist: "Does every backend route have a UI entry point?"

**Warning signs:**
- API routes without corresponding frontend hooks
- E2E tests use API helpers to create data, not UI dialogs
- Feature marked "done" but users can't find it in app

**Phase to address:** Frontend delete UI (connect existing routes)

---

### Pitfall 7: Edit Dialog Reuses Create Dialog Without Mode Distinction

**What goes wrong:**
Edit dialog is Create dialog with pre-filled values. Dialog title still says "Create". Submit button says "Save" but calls create API, not update API. User creates duplicate instead of editing.

**Why it happens:**
Duplicating dialog component seems faster than adding mode prop. "I'll add that later" → never added.

**Current pattern (TimeEntryDialog.tsx, ReservationDialog.tsx):**
Both dialogs only support create. No `entry` or `reservation` prop for edit mode.

**How to avoid:**
1. **Pass entity to edit:**
   ```tsx
   interface TimeEntryDialogProps {
     entry?: TimeEntry  // If provided, edit mode
     onCreate: (data: CreateTimeEntry) => void
     onUpdate: (id: string, data: UpdateTimeEntry) => void
   }
   ```

2. **Conditional title and submit:**
   ```tsx
   <DialogTitle>
     {entry ? "Zeiteintrag bearbeiten" : "Zeit buchen"}
   </DialogTitle>
   <Button onClick={entry ? handleUpdate : handleCreate}>
     {entry ? "Aktualisieren" : "Speichern"}
   </Button>
   ```

3. **Initialize form with existing values:**
   ```tsx
   const [hours, setHours] = useState(entry?.hours ?? 1)
   ```

**Warning signs:**
- Dialog title doesn't change when editing
- Clicking "save" creates duplicate entries
- Edit operation requires delete + recreate

**Phase to address:** Frontend edit dialogs

---

### Pitfall 8: Input Validation Feedback Only After Submit

**What goes wrong:**
User fills form, clicks submit, backend returns validation error, toast shows generic "Operation failed". User doesn't know which field is wrong. Resubmits multiple times, same error.

**Current state (from BUG-TIME-002):**
> "The TimeEntryDialog shows no inline validation messages. Users only see a generic toast error after submission fails."

**Why it happens:**
Frontend relies entirely on backend validation. "Backend is source of truth" → but UX suffers.

**How to avoid:**
1. **Client-side validation mirrors backend:**
   ```tsx
   const [errors, setErrors] = useState<Record<string, string>>({})
   
   const validate = (): boolean => {
     const newErrors: Record<string, string> = {}
     if (hours <= 0) newErrors.hours = "Stunden müssen größer als 0 sein"
     if (!workDate) newErrors.workDate = "Datum ist erforderlich"
     setErrors(newErrors)
     return Object.keys(newErrors).length === 0
   }
   ```

2. **Inline error display:**
   ```tsx
   <div className="space-y-2">
     <Label>Stunden</Label>
     <Input ... />
     {errors.hours && (
       <p className="text-sm text-destructive">{errors.hours}</p>
     )}
   </div>
   ```

3. **Disable submit until valid:**
   ```tsx
   <Button disabled={!isValid || mutation.isPending}>
   ```

**Warning signs:**
- Users submit 3+ times before getting it right
- Support tickets asking "what format does X field accept?"
- Backend logs show high validation error rate

**Phase to address:** Frontend validation (BUG-TIME-001, BUG-TIME-002)

---

### Pitfall 9: Low Stock Alerts Not Shown in UI

**What goes wrong:**
Backend has `min_quantity`, `is_low_stock()`, and `/api/v1/inventory/low-stock` endpoint. Frontend doesn't show any visual indicator. Materials run out without warning.

**Current state:**
- Material domain has `is_low_stock()` method
- API has `list_low_stock` route
- Frontend has no visual indicator

**Why it happens:**
Backend implemented feature, frontend backlog didn't prioritize. "Works in API" ≠ "Users can see it".

**How to avoid:**
1. **Visual indicator in list:**
   ```tsx
   {material.is_low_stock && (
     <Badge variant="destructive" className="gap-1">
       <AlertTriangle className="h-3 w-3" />
       Niedrig
     </Badge>
   )}
   ```

2. **Dedicated low-stock view:**
   ```tsx
   // In navigation
   <NavLink to="/inventory/low-stock">
     Inventory {lowStockCount > 0 && `(${lowStockCount})`}
   </NavLink>
   ```

3. **Toast on stock withdrawal that triggers low:**
   ```tsx
   if (result.material.is_low_stock) {
     toast.warning(`${material.name} ist niedrig (${material.quantity} verbleibend)`)
   }
   ```

**Warning signs:**
- Backend has feature, frontend doesn't use it
- Users manually track reorder points in spreadsheets
- Stockouts surprise users

**Phase to address:** Low stock UI implementation

---

### Pitfall 10: Calendar Click-to-Create Doesn't Pre-fill Time/Resource

**What goes wrong:**
Calendar shows empty slot. User clicks. ReservationDialog opens but `resourceId` and `startTime` are empty. User has to re-select what they just clicked.

**Current state (from BUG-RES-002):**
> "Calendar view shows reservations but clicking on empty time slots does nothing."

**Why it happens:**
Calendar component renders slots but has no `onClick` handler. Or handler opens dialog without passing context.

**How to avoid:**
1. **Pass click context to dialog:**
   ```tsx
   const handleSlotClick = (resourceId: string, startTime: Date) => {
     setReservationDialog({
       open: true,
       resourceId,
       startTime: startTime.toISOString(),
       endTime: new Date(startTime.getTime() + 2 * 60 * 60 * 1000).toISOString()
     })
   }
   ```

2. **Dialog accepts initial values:**
   ```tsx
   <ReservationDialog
     open={dialog.open}
     resourceId={dialog.resourceId}
     startTime={dialog.startTime}
     endTime={dialog.endTime}
   />
   ```

**Warning signs:**
- Users navigate away from calendar to create reservations
- Calendar is read-only, not interactive
- "Nice view but useless" feedback

**Phase to address:** Calendar click-to-create

---

## Minor Pitfalls

### Pitfall 11: QR Button Exists But Does Nothing

**What goes wrong:**
QR button in InventoryListPage renders but has no onClick. User clicks, nothing happens. Feature appears broken.

**Current state (from BUG-INV-002):**
> "QR code button exists but has no onClick handler - it's a static button."

**How to avoid:**
- Wire button to scanner dialog or QR detail view
- If feature not ready, hide button or show "coming soon" tooltip
- E2E test should verify button is functional

**Phase to address:** QR button wiring

---

### Pitfall 12: Status Transition UI Missing Despite Backend Support

**What goes wrong:**
Reservation has status (Pending → Confirmed → InUse → Completed/Cancelled). Backend validates transitions. Frontend has no buttons to trigger transitions. Reservations stuck in initial state.

**Current state (from MISSING-RES-002):**
> "No UI buttons to confirm, start, complete, or cancel reservations."

**How to avoid:**
1. **Action buttons based on current status:**
   ```tsx
   {reservation.status === 'pending' && (
     <>
       <Button onClick={() => updateStatus('confirmed')}>Bestätigen</Button>
       <Button variant="destructive" onClick={() => updateStatus('cancelled')}>Stornieren</Button>
     </>
   )}
   {reservation.status === 'confirmed' && (
     <Button onClick={() => updateStatus('in_use')}>Starten</Button>
   )}
   ```

2. **Status badge shows current state:**
   ```tsx
   <Badge variant={statusVariant[res.status]}>
     {statusLabels[res.status]}
   </Badge>
   ```

**Phase to address:** Reservation status UI

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|----------------|------------|
| Backend delete routes | Soft delete not implemented, hard delete loses audit trail | Add `deleted_at` column before exposing delete API |
| Frontend delete UI | Accidental deletions without confirmation | AlertDialog required for all delete operations |
| Time entry edit | Dialog reused without mode distinction | Pass existing entry to dialog, conditional title/submit |
| Reservation status UI | Race conditions on status update | Database-level status check in UPDATE WHERE clause |
| Low stock alerts | Backend endpoint unused | Poll `/low-stock` or subscribe to events |
| Calendar click-to-create | Dialog doesn't receive click context | Pass resourceId and time from slot click |
| E2E tests | Tests use API helpers, don't verify UI flow | Test full user journey through UI |
| Offline sync (v1.7+) | Delete conflicts with offline edits | Tombstone sync, version vectors |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Backend delete → Frontend UI | API exists, UI doesn't connect | Track in issue tracker, E2E test UI flow |
| ts-rs types | Generated types drift from API responses | Run `cargo test --features ts-rs/export` after DTO changes |
| IndexedDB sync | Delete operations not queued for offline | Queue soft-delete operation, sync `deleted_at` |
| Multi-tenant delete | TenantId not validated on delete | Always include `WHERE tenant_id = $1` in delete queries |
| Status transitions | Frontend allows invalid transitions | Backend rejects but frontend doesn't disable invalid buttons |

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hard delete | Simpler queries | No audit trail, no recovery | Never in production app |
| Delete without confirmation | Faster to implement | Accidental data loss, support burden | Never for destructive actions |
| Edit via delete+create | Reuses create dialog | Orphaned references, lost metadata | Never |
| Client-side only validation | Faster feedback | Backend validation bypassed | Never - always validate both sides |
| Skip E2E for "simple" CRUD | Faster tests | Regression bugs in "simple" features | Never - CRUD is exactly what needs testing |

---

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Delete without tenant check | Cross-tenant data deletion | `WHERE tenant_id = $1` in every delete query |
| Edit without ownership check | User modifies another user's data | `WHERE user_id = $1` or role-based check |
| Status transition without authorization | User confirms own reservation | Check permissions: `can_confirm_reservation(user, reservation)` |
| No rate limiting on delete | API abuse, mass deletion | Rate limit at infrastructure level (noted as tech debt) |

---

## "Looks Done But Isn't" Checklist

Before marking delete/edit/status features complete:

- [ ] **Delete button:** AlertDialog confirms before deletion
- [ ] **Delete backend:** Returns meaningful error if FK constraints block delete
- [ ] **Delete sync:** Soft delete works offline (tombstone sync)
- [ ] **Edit dialog:** Title changes to "Edit" when editing existing entity
- [ ] **Edit submit:** Calls PATCH/PUT, not POST
- [ ] **Validation:** Inline errors shown before submit, not just toast after
- [ ] **Status buttons:** Only valid transitions shown as buttons
- [ ] **Status race:** Concurrent updates handled (version check or conditional update)
- [ ] **E2E tests:** Test delete, edit, status through UI, not just API helpers
- [ ] **Multi-tenant:** Delete/update queries include tenant_id filter

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Hard delete used, data lost | HIGH | Restore from backup, no granular recovery possible |
| Soft delete implemented after data exists | MEDIUM | Add `deleted_at NULL`, backfill NULL for existing records |
| Edit creates duplicates | LOW | Teach users to use correct button, delete duplicates |
| Status race condition | LOW | Refresh UI, retry operation with current status |
| Offline sync conflict | MEDIUM | Manual merge or "server wins" resolution |

---

## Sources

- Codebase analysis: Backend API routes, domain logic, frontend dialogs
- ISSUE-BACKLOG.md: Documented gaps and bugs
- PROJECT.md: Known tech debt (no conflict resolution for offline edits)
- Established patterns: State machine tests in domain layer, ts-rs type generation
- shadcn/ui patterns: AlertDialog for confirmations

---

*Pitfalls research for: Schreinerei SaaS v1.6 - User Experience & Missing Functionality*
*Researched: 2026-04-30*
*Confidence: HIGH (based on codebase inspection + established domain patterns)*
