---
phase: 19-delete-operations
plan: 03
subsystem: frontend
tags: [ui, delete, confirmation, alert-dialog, hooks]
requires: [backend-delete-routes]
provides: [delete-confirm-dialog, delete-buttons, delete-hooks]
affects: [inventory-ui, sites-ui, fleet-ui]
tech-stack:
  added: [shadcn-alert-dialog, delete-confirm-dialog]
  patterns: [confirmation-dialog, mutation-hooks, toast-feedback]
key-files:
  created:
    - frontend/src/components/shared/DeleteConfirmDialog.tsx
    - frontend/src/components/ui/alert-dialog.tsx
  modified:
    - frontend/src/components/inventory/MaterialCard.tsx
    - frontend/src/components/sites/SiteCard.tsx
    - frontend/src/components/fleet/ResourceCard.tsx
    - frontend/src/lib/api/hooks/useInventory.ts
    - frontend/src/lib/api/hooks/useSites.ts
    - frontend/src/lib/api/hooks/useFleet.ts
    - frontend/src/lib/api/client.ts
decisions:
  - Use AlertDialog component for delete confirmation
  - German text for confirmation dialog
  - Trash icon button in card headers
  - Toast notifications for success/failure
metrics:
  duration: ~15 minutes
  completed: 2026-04-30
---

# Phase 19 Plan 03: Frontend AlertDialog + Delete Buttons Summary

## One-liner
Added delete buttons with AlertDialog confirmation to inventory, sites, and fleet pages with error handling for conflict responses.

## What Changed

### Frontend Changes

1. **AlertDialog Component**
   - Installed shadcn/ui alert-dialog component
   - Created DeleteConfirmDialog wrapper with German text

2. **Delete Mutation Hooks**
   - Added useDeleteMaterial to useInventory.ts
   - Added useDeleteSite to useSites.ts
   - Added useDeleteVehicle, useDeleteTool to useFleet.ts
   - All hooks invalidate relevant queries on success

3. **Card Components Updated**
   - MaterialCard: Added delete button with confirmation
   - SiteCard: Added delete button with confirmation
   - ResourceCard: Added delete button with confirmation

4. **API Client**
   - Updated error handling to extract 'error' key from response
   - Supports 409 Conflict error messages

## Files Changed

| File | Changes |
|------|---------|
| frontend/src/components/ui/alert-dialog.tsx | New shadcn/ui component |
| frontend/src/components/shared/DeleteConfirmDialog.tsx | New confirmation dialog |
| frontend/src/components/inventory/MaterialCard.tsx | Added delete button and dialog |
| frontend/src/components/sites/SiteCard.tsx | Added delete button and dialog |
| frontend/src/components/fleet/ResourceCard.tsx | Added delete button and dialog |
| frontend/src/lib/api/hooks/useInventory.ts | Added useDeleteMaterial |
| frontend/src/lib/api/hooks/useSites.ts | Added useDeleteSite |
| frontend/src/lib/api/hooks/useFleet.ts | Added useDeleteVehicle, useDeleteTool |
| frontend/src/lib/api/client.ts | Updated error handling |

## Delete Confirmation Flow

1. User clicks trash icon on card
2. AlertDialog opens with German confirmation text
3. User clicks "Löschen" (Delete) button
4. Mutation executes DELETE request
5. On success: Toast shows "gelöscht", query cache invalidated
6. On error: Toast shows error message (e.g., "Cannot delete: 2 active reservation(s) exist")

## Requirements Covered

- **DEL-01**: User can delete a site with confirmation dialog ✓
- **DEL-02**: User can delete a material with confirmation dialog ✓
- **DEL-03**: User can delete a vehicle with confirmation dialog ✓
- **DEL-04**: User can delete a tool with confirmation dialog ✓
- **DEL-05**: User sees dependency conflict message when delete fails ✓

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check: PASSED

- AlertDialog component exists
- DeleteConfirmDialog exists with German text
- All delete mutation hooks exist
- Delete buttons present on all cards
- API client handles 409 errors
