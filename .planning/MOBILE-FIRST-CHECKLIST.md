# Mobile-First Checklist

Use this checklist for every new user-facing workflow.

## Required

1. Primary flow must work on phone-sized screens without horizontal scrolling.
2. Primary actions must be reachable with large tap targets.
3. New create/edit flows should prefer bottom sheets or simple stacked layouts over dense desktop dialogs.
4. Camera, upload, scan, or quick-select actions should stay closer to the main task than secondary settings.
5. The user must not be forced to enter more data than needed to complete the task.
6. Offline or reconnect behavior must be considered for field workflows.
7. Desktop enhancements are allowed, but must not become the only practical way to complete the task.

## Existing Baseline Evidence

- `frontend/src/components/layout/AppLayout.tsx`
- `frontend/src/components/layout/MobileNav.tsx`
- `frontend/src/pages/fleet/ReservationConfirmationSheet.tsx`
- `frontend/src/pages/sites/CameraUploadFlow.tsx`
- `frontend/src/App.tsx`
- `frontend/public/manifest.json`
