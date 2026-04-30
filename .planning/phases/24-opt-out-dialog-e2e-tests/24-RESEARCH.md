# Phase 24 Research: Opt-Out Dialog & E2E Tests

## Scope

Phase 24 goal: deliver non-blocking auto-assignment confirmation UX with 5-second auto-confirm and end-to-end coverage.

Requirements in scope: `DLOG-01..DLOG-05`, `TEST-16..TEST-18`.

## Existing Code Patterns (verified)

1. **Dialog primitives already exist**
   - `frontend/src/components/ui/dialog.tsx`
   - `frontend/src/components/ui/alert-dialog.tsx`
   - Existing modal usage in:
     - `frontend/src/pages/inventory/WithdrawDialog.tsx`
     - `frontend/src/pages/fleet/ReservationDialog.tsx`
     - `frontend/src/pages/sites/TimeEntryDialog.tsx`

2. **Form submission hooks are mutation-based**
   - Inventory mutation: `useWithdrawMaterial()` in `frontend/src/lib/api/hooks/useInventory.ts`
   - Fleet mutation: `useCreateReservation()` in `frontend/src/lib/api/hooks/useFleet.ts`
   - Time mutation: `useCreateTimeEntry()` in `frontend/src/lib/api/hooks/useSites.ts`

3. **E2E stack is Playwright**
   - Config: `frontend/playwright.config.ts`
   - Existing API helpers: `frontend/tests/helpers/api.ts`
   - Existing domain specs: `frontend/tests/inventory.spec.ts`, `frontend/tests/fleet.spec.ts`, `frontend/tests/sites.spec.ts`

4. **Preferences backend endpoint already exists (from Phase 22)**
   - `GET /api/v1/preferences`
   - `PATCH /api/v1/preferences`
   - DTO in generated types: `PreferencesResponse`, `UpdatePreferencesRequest` (`frontend/src/types/generated.ts`)

## Implementation Constraints

- Dialog must be **non-blocking**: user can continue working and submit without explicit dialog action (`DLOG-05`).
- Default behavior after 5 seconds must be equivalent to explicit "confirm" (`DLOG-02`).
- User must be able to switch assignment in dialog (`DLOG-03`) or dismiss to send unassigned (`DLOG-04`).
- Keep tenant/user scoping through existing authenticated APIs (no client-side trust shortcuts).

## Suggested Plan Split

1. Shared auto-assignment dialog contract + state/timer orchestration component
2. Integrate dialog into three create flows (withdrawal, reservation, time entry)
3. E2E coverage for full active-site workflow and dialog behavior

## Risks & Mitigations

- **Race between auto-confirm timer and manual submit**
  - Mitigation: single source-of-truth state machine (`pending` -> `confirmed` / `unassigned` / `changed`) with idempotent finalize handler.
- **Flaky E2E timing around 5-second auto-confirm**
  - Mitigation: assertions based on observable request payload (`site_id`) and resilient waits (`toPass`, polling).
- **Cross-form divergence**
  - Mitigation: shared hook/component used by all 3 forms, avoid copy-paste logic.
