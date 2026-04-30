# Phase 23 Research — Frontend UI & Auto-Assignment

**Date:** 2026-04-30  
**Scope:** Phase 23 (ACTV-01..07, AUTO-01..04)

## Existing Backend Contracts (from Phase 22)

- `GET /api/v1/preferences` returns `PreferencesResponse { active_site_id: string | null }`
- `PATCH /api/v1/preferences` accepts `UpdatePreferencesRequest { active_site_id: string | null }`
- `WithdrawRequest` already supports `site_id: string | null` in generated TS types
- `CreateReservationRequest` already supports `site_id: string | null`
- `CreateTimeEntryRequest` already supports `site_id: string | null`

## Existing Frontend Patterns

1. **Data fetching + mutations** use TanStack Query hooks in `frontend/src/lib/api/hooks/*`.
2. **Cross-page persistence** already exists via Zustand `persist` middleware (`authStore`).
3. **Forms are local-state driven**:
   - Material withdrawal: `InventoryDetailPage` + `WithdrawDialog`
   - Reservation: `ReservationDialog`
   - Time entry: `TimeEntryDialog`
4. **Persistent shell rendering** is in `AppLayout` (`DesktopSidebar`, `MobileNav`) — best location for always-visible active-site indicator.

## Recommended Frontend Architecture for Phase 23

### 1) Active Site State Layer

Create a dedicated active-site feature layer in frontend with:
- React Query hooks for preferences endpoint (`usePreferences`, `useUpdatePreferences`)
- Utility for deterministic site color hashing (`siteId/name -> Tailwind color token/class`)
- Optional lightweight client store or derived query selectors for current active site display

### 2) Persistent Indicator

Render an active-site badge/indicator in shared layout components (`DesktopSidebar` and `MobileNav`) so it remains visible across navigation.

### 3) Toggle Entry Points

Add set/clear active-site actions at:
- Sites overview (`SitesListPage` / `components/sites/SiteCard.tsx`)
- Dashboard active site cards (`components/dashboard/SiteCard.tsx`)

Both entry points should call the same mutation path to enforce single source of truth.

### 4) Auto-Assignment Prefill

Pre-fill and keep editable in forms:
- Withdraw dialog -> pass selected/active site to `WithdrawRequest.site_id`
- Reservation dialog -> initialize `siteId` from active preference
- Time entry dialog (work_type=`site`) -> default `site_id` from active preference, allow removal/change

## Risks and Pitfalls

1. **Stale UI after preference update**
   - Mitigation: invalidate preference query + dependent queries that render active indicator.
2. **Site deleted/archived while active**
   - Backend already auto-clears; frontend must treat `active_site_id: null` as authoritative after refetch.
3. **Color inconsistency across components**
   - Mitigation: single color utility used by all badges/indicators/cards.
4. **Form regressions from optional `site_id` propagation**
   - Mitigation: add focused Vitest coverage for each dialog’s defaulting behavior.

## Verification Strategy

- Unit/UI tests in Vitest for:
  - active-site indicator rendering states
  - toggle mutation calls
  - form prefill + user override behavior
- Build/lint check:
  - `npm run test:run`
  - `npm run build`

## Architectural Responsibility Map

- **UI Layer:** indicator components, badges, toggle buttons, form controls
- **Application/Data Layer:** preferences hooks, mutation invalidation, active-site mapping logic
- **Domain Rules (frontend projection):** one active site shown at a time, hash-based color assignment, prefill only defaults (user may override)
