# Project Research Summary

**Project:** Schreinerei SaaS - v1.6 User Experience & Missing Functionality
**Domain:** Construction management SaaS (CRUD operations, offline-first PWA)
**Researched:** 2026-04-30
**Confidence:** HIGH

## Executive Summary

This milestone focuses on completing missing CRUD functionality (delete, edit, status transitions) in an existing construction management SaaS. The application uses a well-structured Hexagonal Architecture with DDD bounded contexts, and the existing infrastructure supports all planned features with minimal additions.

The recommended approach is backend-first implementation: add missing DELETE/PATCH routes with soft-delete semantics, then build frontend UI components that follow established patterns (AlertDialog for confirmations, mode-prop dialogs for edit vs create). Key risks include foreign key constraint handling during deletes (users need clear error messages about why deletes fail), and status transition race conditions (needs database-level conditional updates).

Critical finding: Backend is missing DELETE/PATCH routes for sites, materials, and time entries. Fleet module has all routes but frontend doesn't use them. Low stock alerts exist in backend but have no UI visibility.

## Key Findings

### Recommended Stack

The milestone requires **minimal new dependencies**. The existing React + Rust + PostgreSQL stack handles all planned features. Only one frontend component addition is needed.

**Core technologies:**
- **AlertDialog (shadcn/ui)** — Delete confirmations — Standard shadcn pattern, install via `npx shadcn@latest add alert-dialog`
- **React Query mutations** — Delete/update operations — Already in use, add optimistic updates for better UX
- **Sonner toasts** — User feedback — Already integrated, extend to new operations
- **Playwright E2E** — Testing — Already established patterns in `tests/helpers/`

**NOT recommended:**
- react-hook-form — Existing controlled input pattern is sufficient for simple dialogs
- zod — Backend validation handles errors, add inline state locally instead
- Calendar library — Custom calendar works, just needs onClick handlers

### Expected Features

**Must have (table stakes):**
- Delete confirmation dialogs — Prevents accidental data loss, users expect this in all CRUD apps
- Edit existing records — Users make mistakes, must be correctable
- Inline validation feedback — Users need to know what's wrong BEFORE submitting
- Hours > 0 validation — Fix BUG-TIME-001, zero/negative hours make no sense
- Status transition buttons — If status exists, users expect to change it

**Should have (competitive):**
- Low stock alerts (proactive) — Users know when to reorder without manual checks
- Calendar click-to-create — Faster reservation creation via direct interaction

**Defer (v1.7+):**
- Undo for destructive actions — Requires soft-delete + restore, complexity deferred
- Calendar overlap conflict details — Requires backend response changes
- Bulk operations — Not needed for pilot

### Architecture Approach

The application uses Hexagonal Architecture with three layers per module: API (routes.rs), Application (service.rs), Domain (entities, state machines), and Infrastructure (repository.rs). Each bounded context (iam, inventory, sites, fleet) is independent, enabling parallel development.

**Major components:**
1. **Backend API routes** — Add DELETE/PATCH for sites, materials, time entries following fleet module patterns
2. **Frontend mutation hooks** — React Query hooks with optimistic updates for delete/update operations
3. **Status state machines** — Existing `can_transition_to()` methods in domain, needs UI to trigger transitions
4. **Soft delete layer** — Add `deleted_at` column to all deletable entities for audit trail

### Critical Pitfalls

1. **Delete without soft delete breaks audit trails** — Add `deleted_at` column before exposing delete API, never use hard delete in production

2. **Foreign key constraints block deletes without clear UX** — Check dependencies before delete, return meaningful errors like "Cannot delete: 12 time entries exist. Archive instead?"

3. **Status transition race conditions** — Use database-level conditional updates (`WHERE status = 'confirmed'`) or version column to prevent concurrent modification

4. **Delete UI without confirmation leads to accidental data loss** — AlertDialog required for all delete operations, no default focus, specific item name in message

5. **Backend routes exist but frontend doesn't use them** — Fleet has DELETE routes for vehicles/tools but no UI buttons. Track API-UI gaps in issue tracker.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Bug Fixes (Low Risk)
**Rationale:** No backend changes, immediate UX improvement, builds confidence
**Delivers:** Hours validation fix, QR button wiring
**Addresses:** BUG-TIME-001, BUG-INV-002
**Avoids:** Pitfall 8 (validation only after submit)

### Phase 2: Backend Delete Routes (Medium Risk)
**Rationale:** Backend-first enables parallel frontend work, establishes patterns for remaining phases
**Delivers:** DELETE routes for sites, materials, time entries with soft delete semantics
**Uses:** Existing fleet module patterns, SQLx queries
**Implements:** Soft delete with `deleted_at` column, dependency checks before delete
**Avoids:** Pitfall 1 (no audit trail), Pitfall 2 (unclear FK errors)

### Phase 3: Backend Update Routes (Medium Risk)
**Rationale:** Enables edit functionality, status transitions already validated by domain layer
**Delivers:** PATCH routes for materials, time entries with partial update support
**Uses:** Existing update patterns from sites module
**Implements:** Conditional status transitions with version checking
**Avoids:** Pitfall 4 (status race conditions)

### Phase 4: Frontend Delete UI (Low Risk)
**Rationale:** Backend complete, reusable AlertDialog component first, then specific implementations
**Delivers:** Delete buttons with confirmation dialogs for all entity types
**Uses:** shadcn/ui AlertDialog, React Query mutations with optimistic updates
**Avoids:** Pitfall 5 (accidental data loss), Pitfall 6 (unused backend routes)

### Phase 5: Frontend Edit UI (Medium Risk)
**Rationale:** Refactor existing dialogs for edit mode, wire update mutations
**Delivers:** Edit capability for time entries, reservations, materials
**Uses:** Mode-prop pattern for create/edit dialogs
**Avoids:** Pitfall 7 (dialog without mode distinction)

### Phase 6: Status Transitions UI (Medium Risk)
**Rationale:** Backend validates transitions, UI just needs to show valid options
**Delivers:** Status transition buttons for reservations (Pending→Confirmed→InUse→Completed)
**Uses:** Existing `can_transition_to()` state machine logic
**Avoids:** Pitfall 12 (missing status UI), Pitfall 4 (race conditions via conditional updates)

### Phase 7: Low Stock & Calendar Enhancements (Low Risk)
**Rationale:** Backend and hooks exist, just needs UI components
**Delivers:** Low stock badges/alerts, calendar click-to-create
**Uses:** Existing `/api/v1/inventory/low-stock` endpoint, ReservationDialog
**Avoids:** Pitfall 9 (unused low stock feature), Pitfall 10 (calendar without context)

### Phase 8: E2E Tests (Medium Risk)
**Rationale:** Tests after implementation ensures coverage, catches integration issues
**Delivers:** E2E tests for delete, edit, status transitions, calendar interactions
**Uses:** Playwright, existing test helpers
**Avoids:** All pitfalls verified through UI flow testing

### Phase Ordering Rationale

- **Backend before frontend:** Deleting entities requires backend routes with proper validation first
- **Bug fixes first:** Immediate value, no dependencies, builds momentum
- **Delete before edit:** Simpler operation, establishes confirmation patterns
- **Status transitions late:** Requires both update routes and UI patterns to be solid
- **Tests last:** Full coverage requires all features implemented

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 3 (Update routes):** Time entry update semantics — can users edit any field or only hours?
- **Phase 6 (Status UI):** Permission model for status transitions — who can confirm/cancel?

Phases with standard patterns (skip research-phase):
- **Phase 1 (Bug fixes):** Simple validation wiring, existing patterns
- **Phase 4 (Delete UI):** AlertDialog pattern well-documented in shadcn/ui
- **Phase 8 (E2E tests):** Playwright patterns already established in codebase

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Existing infrastructure fully supports features, minimal additions needed |
| Features | HIGH | Clear table stakes from CRUD patterns, specific bugs documented in issue backlog |
| Architecture | HIGH | Well-structured codebase with established patterns to follow |
| Pitfalls | HIGH | Based on codebase analysis + established CRUD/CRUD UX best practices |

**Overall confidence:** HIGH

### Gaps to Address

**Permission model:** Research doesn't clarify who can delete/edit what. During planning:
- Define ownership rules (users can only edit own time entries? admins can edit all?)
- Implement role checks in service layer

**Offline sync strategy:** Known tech debt (no conflict resolution). Defer to v1.7+ as noted in PROJECT.md:
- Queue soft-delete operations for offline sync
- Implement tombstone pattern for deleted records

**QR button destination:** BUG-INV-002 notes QR button has no onClick. During implementation:
- Decide: opens scanner dialog or shows material QR code?
- Check if QR scanning feature is fully designed

## Sources

### Primary (HIGH confidence)
- **Codebase analysis** — Backend routes, domain logic, frontend dialogs, existing patterns
- **shadcn/ui documentation** — AlertDialog component patterns
- **NN/G Confirmation Dialog Guidelines** — UX best practices for destructive action confirmations

### Secondary (MEDIUM confidence)
- **ISSUE-BACKLOG.md** — Documented 24 gaps and bugs across modules
- **PROJECT.md** — Known tech debt, offline-first architecture constraints

### Tertiary (LOW confidence)
- **State machine testing patterns** — Assumed based on domain layer structure, verify during implementation

---
*Research completed: 2026-04-30*
*Ready for roadmap: yes*
