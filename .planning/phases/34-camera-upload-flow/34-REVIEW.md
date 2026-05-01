---
phase: 34-camera-upload-flow
reviewed: 2026-05-01T12:00:00Z
depth: quick
files_reviewed: 3
files_reviewed_list:
  - frontend/src/pages/sites/CameraUploadFlow.tsx
  - frontend/src/pages/sites/CameraUploadFlow.test.tsx
  - frontend/src/pages/sites/SiteDetailPage.tsx
findings:
  critical: 0
  warning: 5
  info: 2
  total: 7
status: issues_found
---

# Phase 34: Code Review Report

**Reviewed:** 2026-05-01T12:00:00Z
**Depth:** quick (pattern-matching + per-file analysis)
**Files Reviewed:** 3
**Status:** issues_found

## Summary

Review of the Camera Upload Flow implementation — a new `CameraUploadFlow` component that replaces the camera button's previous behavior (opening CreateNoteModal) with a native file picker + dialog flow. The implementation is functional and follows existing codebase patterns well. No critical security vulnerabilities or auth bypass issues found. However, there are memory leak concerns with object URLs, a partial-failure consistency gap, a misleading function name, and missing client-side file validation. One test does not verify what its description claims.

## Warnings

### WR-01: Object URL memory leak on component unmount

**File:** `frontend/src/pages/sites/CameraUploadFlow.tsx:56-57, 106-116`
**Issue:** `URL.createObjectURL` creates a blob URL that holds a reference to the file data in memory. The only cleanup is `resetForm()`, which calls `URL.revokeObjectURL`. However, if the component unmounts unexpectedly (e.g., user navigates away, parent re-renders with `open=false` without calling `handleOpenChange`), the `useEffect` cleanup for the `open` state transition resets `pickerTriggered` but does **not** revoke the object URL. There's no `useEffect` return cleanup function to handle this.
**Fix:** Add a cleanup effect that revokes the object URL on unmount:

```tsx
useEffect(() => {
  return () => {
    // Revoke object URL on unmount to prevent memory leak
    if (previewUrl) {
      URL.revokeObjectURL(previewUrl)
    }
  }
}, [previewUrl])
```

### WR-02: Object URL leak on file re-selection

**File:** `frontend/src/pages/sites/CameraUploadFlow.tsx:55-57`
**Issue:** When a user selects a file, `URL.createObjectURL(file)` creates a new blob URL. If the user dismisses the dialog, picks a new file (re-triggering the input), the previous `previewUrl` is overwritten in state without being revoked first. This leaks the previous blob URL.
**Fix:** Revoke the old URL before setting a new one in `handleFileChange`:

```tsx
const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
  const file = event.target.files?.[0]
  if (!file) {
    onOpenChange(false)
    return
  }

  // Revoke previous preview URL if it exists
  if (previewUrl) {
    URL.revokeObjectURL(previewUrl)
  }

  setSelectedFile(file)
  const url = URL.createObjectURL(file)
  setPreviewUrl(url)
}
```

### WR-03: No rollback on partial upload failure

**File:** `frontend/src/pages/sites/CameraUploadFlow.tsx:85-101`
**Issue:** The `handleSubmit` function performs two sequential async operations: first `uploadPhoto.mutateAsync()` to upload the file, then `createActivity.mutateAsync()` to create the activity record. If the photo upload succeeds but activity creation fails, the uploaded photo is orphaned on the server — stored but never associated with an activity. The error toast is shown, but the uploaded file remains as waste.
**Fix:** This may require a backend change (e.g., a combined endpoint), which is out of scope for this review. At minimum, document this as a known gap. A practical mitigation is to inform the user more specifically: "Foto hochgeladen, aber Notiz konnte nicht gespeichert werden" when the second step fails. Consider also catching the activity creation error separately and retrying or offering the user a retry option without re-uploading the photo.

### WR-04: Misleading function name — `openPhotoModal` opens CameraUploadFlow

**File:** `frontend/src/pages/sites/SiteDetailPage.tsx:63-65`
**Issue:** The function `openPhotoModal` now opens `CameraUploadFlow` (a native camera picker), not a modal. The name is stale and actively misleading — a future developer might think this opens CreateNoteModal with a photo type, when it actually opens an entirely different component.
**Fix:** Rename to `openCameraFlow` or simply inline it:

```tsx
// Option A: Rename
const openCameraFlow = () => {
  setShowCameraFlow(true)
}

// Option B: Inline (preferred — trivial wrapper)
onClick={() => setShowCameraFlow(true)}
```

### WR-05: No runtime file type validation

**File:** `frontend/src/pages/sites/CameraUploadFlow.tsx:48-58`
**Issue:** The `accept="image/*"` attribute on the file input is a browser hint only — it does not prevent a user from selecting non-image files (e.g., via drag-and-drop on some browsers, or by changing the file picker filter). There is no runtime check of `file.type` before uploading. While the backend should validate this, defense-in-depth recommends a client-side check as well.
**Fix:** Add a file type check in `handleFileChange`:

```tsx
const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
  const file = event.target.files?.[0]
  if (!file) {
    onOpenChange(false)
    return
  }

  if (!file.type.startsWith("image/")) {
    toast.error("Bitte wählen Sie eine Bilddatei aus")
    onOpenChange(false)
    return
  }

  // ... rest of handler
}
```

## Info

### IN-01: Test for "submit without file" doesn't verify what its description says

**File:** `frontend/src/pages/sites/CameraUploadFlow.test.tsx:180-195`
**Issue:** Test 7 is titled "shows validation error if submitting without a file selected" but it only verifies that the submit button (`Hochladen`) is not in the document when no file is selected. It does not actually trigger a submit action or verify that `toast.error("Bitte wählen Sie ein Foto aus")` is shown. The plan's test 7 description says "Shows validation error if submitting photo without a file selected" — this suggests a toast error should be verified. Since the dialog is conditional on `selectedFile !== null`, the button simply isn't rendered, making the toast path unreachable in the UI flow. The component's `handleSubmit` early-return toast is defensive code for a state that shouldn't be reachable via normal UI interaction (since the submit button is disabled when `!selectedFile`).
**Fix:** Either update the test description to accurately reflect what's being tested (e.g., "upload button is not shown when no file is selected"), or restructure the test to verify the toast error by triggering `handleSubmit` directly (e.g., via a component ref or by programmatically removing the `disabled` attribute).

### IN-02: setTimeout for file input click is timing-dependent

**File:** `frontend/src/pages/sites/CameraUploadFlow.tsx:40`
**Issue:** The `setTimeout(() => fileInputRef.current?.click(), 50)` approach to triggering the file picker relies on a 50ms delay. This is a common React pattern for ensuring the DOM element is ready, but in slow environments (e.g., on low-end mobile devices), 50ms may not be sufficient. The null-conditional `?.` prevents a crash but could silently fail to open the picker.
**Fix:** Consider using `requestAnimationFrame` for more reliable timing, or triggering the click based on a ref callback:

```tsx
useEffect(() => {
  if (open && !pickerTriggered) {
    requestAnimationFrame(() => {
      fileInputRef.current?.click()
    })
    setPickerTriggered(true)
  }
  if (!open) {
    setPickerTriggered(false)
  }
}, [open, pickerTriggered])
```

---

_Reviewed: 2026-05-01T12:00:00Z_
_Reviewer: OpenCode (gsd-code-reviewer)_
_Depth: quick_