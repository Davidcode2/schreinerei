# 23-03 Summary

- Added active-site prefill and override support for material withdrawal:
  - `frontend/src/pages/inventory/WithdrawDialog.tsx`
  - `frontend/src/pages/inventory/InventoryDetailPage.tsx`
  - `frontend/src/lib/api/hooks/useInventory.ts`
- Added active-site prefill and override support for reservations:
  - `frontend/src/pages/fleet/ReservationDialog.tsx`
- Added conditional active-site prefill and override for time entry (`work_type === site`):
  - `frontend/src/pages/sites/TimeEntryDialog.tsx`
- Updated local request typings to allow explicit `null` site assignment when users clear selection:
  - `frontend/src/types/inventory.ts`
  - `frontend/src/types/fleet.ts`
  - `frontend/src/types/sites.ts`

Verification:
- Included in `npm run build` check in `frontend/`.
- Build is currently blocked by unrelated pre-existing TypeScript/test setup issues.
