# Phase 26: Status Change Workflow - Verification

**Phase:** 26
**Verified:** 2026-05-01
**Status:** passed

## Success Criteria Verification

### ✓ STAT-01: User can change site status via modal
**Evidence:**
- Created `StatusChangeModal.tsx` with valid transition buttons
- Modal shows appropriate actions: "Aktivieren", "Abschließen", "Archivieren"
- Backend validates transitions via `can_transition_to()`

### ✓ STAT-02: Chip tap opens status change modal
**Evidence:**
- Modified `SiteDetailPage.tsx` line 79-82
- Added onClick handler to StatusBadge wrapper
- Added cursor-pointer and hover effects
- StatusChangeModal receives current site status

### ✓ STAT-03: "Aktiv" button renamed to "Auswählen"
**Evidence:**
- Modified `SiteCard.tsx` line 72
- Changed button text from "Aktiv" / "Aktiv setzen" to "Auswählen"
- Changed badge text from "Aktiv" to "Ausgewählt" (line 118)
- No confusion between active Baustelle selection and site status

### ✓ STAT-04: Status change creates activity entry
**Evidence:**
- Backend creates SiteStatusChanged event automatically
- ActivityType::StatusChange exists in backend
- ActivityFeed displays status changes with "Geplant → Aktiv" format
- Green ArrowRight icon used for status changes

### ✓ STAT-05: Concurrent status changes handled
**Evidence:**
- StatusChangeModal catches errors from useUpdateSite
- Shows toast: "Status wurde bereits geändert. Aktualisieren..."
- Auto-refreshes site data after 1 second
- Backend rejects invalid transitions with 400 error

## Files Created

- `frontend/src/pages/sites/StatusChangeModal.tsx` — Status change modal component

## Files Modified

- `frontend/src/pages/sites/SiteDetailPage.tsx` — Added modal state and onClick handler
- `frontend/src/pages/sites/ActivityFeed.tsx` — Added status_change case
- `frontend/src/components/sites/SiteCard.tsx` — Renamed button text

## Manual Verification Steps

1. Navigate to site detail page
2. Click on StatusBadge — modal should open
3. Verify only valid transition buttons appear
4. Click transition button — status should change
5. Check ActivityFeed — status change should appear with arrow icon
6. Verify "Auswählen" button appears in sites list (not "Aktiv")

## Notes

- Backend validation already complete (no backend changes needed)
- Status change events are created automatically by backend
- ActivityFeed parses JSON content to extract old/new status
- Error handling uses toast notifications for better UX

---

*Verification completed: 2026-05-01*
*All success criteria passed*
