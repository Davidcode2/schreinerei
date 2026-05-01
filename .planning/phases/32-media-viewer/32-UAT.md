---
status: complete
phase: 32-media-viewer
source:
  - 32-media-viewer-01-SUMMARY.md
  - 32-media-viewer-02-SUMMARY.md
  - 32-media-viewer-03-SUMMARY.md
started: 2026-05-01T18:39:05Z
updated: 2026-05-01T18:51:08Z
---

## Current Test

[testing complete]

## Tests

### 1. Open Media Viewer from Activity Feed
expected: On a site detail page, clicking an image or PDF media tile in the activity feed opens the fullscreen media viewer instead of leaving you on a dead link or static tile. The URL changes to `/sites/{siteId}/media/{activityId}/{attachmentId}/{slug?}` and the site detail page remains the underlying route owner.
result: pass

### 2. Close Viewer and Browser Navigation
expected: With the viewer open, closing it with the close button, Escape, or browser back returns you to the canonical `/sites/{siteId}` detail route instead of trapping you in the overlay or navigating somewhere unexpected.
result: pass

### 3. Viewer Metadata and Fallback Copy
expected: The viewer shows a metadata sidebar with the media title, a human-readable creator name, an absolute timestamp, and a note section. If the selected media has no note, the sidebar shows `Keine Notiz vorhanden.` instead of raw IDs or blank broken space.
result: pass

### 4. Copy Deep Link
expected: Clicking `Link kopieren` copies the current viewer URL and shows feedback (`Link kopiert` on success, or `Link konnte nicht kopiert werden` if clipboard access fails).
result: pass

### 5. Download Protected Media
expected: Clicking `Herunterladen` downloads the currently viewed protected file with its original filename, without exposing a public attachment URL.
result: pass

### 6. Open PDF or Legacy Photo Entry
expected: A PDF tile or legacy photo entry from the activity feed opens in the same viewer flow. PDF entries show an embedded preview when possible, and if preview rendering fails the viewer stays open with an error/fallback panel plus a download action.
result: pass

## Summary

total: 6
passed: 6
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
