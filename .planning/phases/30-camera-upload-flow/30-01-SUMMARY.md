---
phase: 30-camera-upload-flow
plan: 01
status: complete
started: "2026-05-01"
completed: "2026-05-01"
requirements:
  - CAM-01
  - CAM-02
  - CAM-03
---

# Plan 30-01: Camera Upload Flow

## Objective

Create a dedicated CameraUploadFlow component that lets users capture or select photos directly from the activity stream with an optional text note, completely separate from the existing CreateNoteModal.

## What Was Built

### CameraUploadFlow Component (`CameraUploadFlow.tsx`)

- New React component providing a direct camera/gallery picker flow
- Opens native camera picker via hidden `<input type="file" accept="image/*" capture="environment">` when `open` prop becomes true
- After file selection, shows a Dialog with:
  - Image preview using `URL.createObjectURL`
  - Optional note textarea labeled "Notiz hinzufügen (optional)"
  - "Abbrechen" (Cancel) and "Hochladen" (Upload) buttons
- Online flow: `useUploadSitePhoto()` → get `photo_url` → `useCreateActivity()` with `activity_type: "photo"` and optional `content`
- Offline flow: `queuePhotoUploadAction()` with file and optional content
- Success toast: "Foto hinzugefügt" (online) or "Foto offline gespeichert" (offline)
- Error toast: "Foto konnte nicht hochgeladen werden"
- Cancelling/closing revokes object URL and clears form state

### SiteDetailPage Wiring (`SiteDetailPage.tsx`)

- Camera button now opens `CameraUploadFlow` (native picker) instead of `CreateNoteModal` with `initialActivityType="photo"`
- FileText button still opens `CreateNoteModal` for notes
- Removed `noteModalActivityType` state (no longer needed — CreateNoteModal is note-only now)
- Removed `handleNoteModalOpenChange` helper (simplified)
- CreateNoteModal no longer receives `initialActivityType` prop

### Test Coverage (`CameraUploadFlow.test.tsx`)

8 tests covering:
1. Renders camera file input with `capture="environment"` and `accept="image/*"`
2. Shows image preview after file selection
3. Shows optional note textarea after file selection
4. Calls upload then createActivity when submitting online
5. Calls `queuePhotoUploadAction` when submitting offline
6. Clears form and closes on successful submit
7. No submit button visible when no file selected (validation)
8. Note field is optional — submit works with photo only

## Files Modified

- `frontend/src/pages/sites/CameraUploadFlow.tsx` — CREATED
- `frontend/src/pages/sites/CameraUploadFlow.test.tsx` — CREATED
- `frontend/src/pages/sites/SiteDetailPage.tsx` — MODIFIED (wired CameraUploadFlow, removed photo modal indirection)

## Key Decisions

1. **Separate component, not modal modification** — CameraUploadFlow is a standalone component, keeping CreateNoteModal untouched for Phase 31 rework
2. **No backend changes needed** — existing `CreateActivity` and `UploadSitePhoto` endpoints fully support the camera flow
3. **Native picker via file input** — `<input capture="environment">` triggers native camera/gallery on mobile, simple file picker on desktop
4. **Offline support preserved** — `queuePhotoUploadAction()` already supports `content` field

## Self-Check: PASSED

- All 8 tests pass
- TypeScript compilation succeeds (`npx tsc --noEmit`)
- Camera button opens native picker (not CreateNoteModal)
- FileText button still opens CreateNoteModal for notes
- CreateNoteModal.tsx was NOT modified