---
gsd_state_version: 1.0
milestone: v1.7
milestone_name: milestone
status: verifying
last_updated: "2026-05-01T10:09:52.351Z"
progress:
  total_phases: 4
  completed_phases: 2
  total_plans: 9
  completed_plans: 7
  percent: 78
---

# State: Schreinerei v1.8 Activity Feed & Site Status

**Updated:** 2026-05-01

## Project Reference

**Core Value:** Mitarbeiter finden alles schnell, Chefs haben den Überblick. Weniger Suchzeit, weniger Fehler.

**Current Milestone:** v1.8 Activity Feed & Site Status

**Milestone Goal:** Bring Baustellen to life with status tracking, activity feeds, and linked material history.

## Current Position

Phase: 29 (photo-upload-attachments) — EXECUTING
Plan: 4 of 4
**Phase:** 28 — Material History Tab
**Status:** Phase complete — ready for verification
**Next Action:** `/gsd-verify-phase 28`

```
Progress: [████████░░] 78%
```

## Active Context

**Working On:**

- Phase 28: Material History Tab linked to Baustellen
- Building on Phase 27 tabbed activity feed

**Current Focus:**
Phase 29 — photo-upload-attachments

- Show category and extracting user context
- Keep backend query eager-loaded (no N+1)

**Recent Completion:**

- ✅ Phase 26: Status Change Workflow complete (2026-05-01)
- ✅ Phase 27: Tabbed Activity Feed complete (2026-05-01)
- ✅ Phase 28: Material History Tab complete (2026-05-01)

## Performance Metrics

| Metric | Value |
|--------|-------|
| Phases Completed | 3/4 |
| Requirements Done | 14/21 |
| Tests | 116 backend + 28 frontend + 22 E2E |
| Code Lines | ~12,290 Rust + ~9,100 TypeScript |
| Velocity | v1.7: 17 reqs, v1.8: 5/21 in progress |
| Phase 29 P01 | 2 min | 2 tasks | 4 files |
| Phase 29 P02 | 27 min | 3 tasks | 6 files |
| Phase 29 P03 | 6 min | 3 tasks | 7 files |
| Phase 29 P04 | 20 min | 2 tasks | 8 files |

## Accumulated Context

### Decisions

- Phase 26: Status change modal with valid transition buttons only
- Phase 26: ActivityFeed displays status changes with arrow icon
- Phase 26: Error handling uses toast notifications with auto-refresh
- Phase 26: "Aktiv" → "Auswählen" to avoid confusion
- [Phase 29]: Attachment API uses opaque attachment UUID routes — Avoid exposing internal storage keys in public paths
- [Phase 29]: Attachment lookup returns NotFound for tenant mismatch — Prevents cross-tenant existence disclosure
- [Phase 29]: Create photo activity during upload and then update photo_url to attachment-backed URL for immediate feed visibility. — Ensures uploaded photos appear in the activity feed immediately with stable, tenant-safe URLs.
- [Phase 29]: Generate original/thumbnail storage keys server-side using UUIDs and MIME-derived extensions; never trust client filenames. — Mitigates path/key tampering risks and satisfies filename confidentiality requirements.
- [Phase 29]: Frontend photo flow uploads file first and reuses backend-provided photo_url for activity creation. — This keeps URL construction canonical on the backend and prevents frontend string assembly drift.
- [Phase 29]: ApiClient detects FormData and skips JSON serialization/headers for multipart uploads. — Multipart uploads fail when payloads are JSON-stringified or forced to application/json; FormData must pass through unchanged.
- [Phase ?]: Queue photo uploads as data URLs — Ensures queued binary payload survives reload before replay
- [Phase ?]: Validate queued photo payload before replay — Drops malformed payloads and prevents invalid upload attempts

### Todos

None yet.

### Blockers

None.

### Learnings

- Backend status state machine was already complete
- ActivityType::StatusChange already existed
- useUpdateSite hook handles optimistic updates
- Toast notifications work well for error handling

## Session Continuity

**Last Session:** 2026-05-01T10:09:13.321Z
**Handoff:** Phase 28 implemented, ready for verification

**Key Files:**

- `.planning/ROADMAP.md` — Phase structure (Phase 26 complete)
- `.planning/REQUIREMENTS.md` — 21 v1.8 requirements (5 complete)
- `.planning/phases/26-status-change-workflow/` — Phase 26 artifacts

**Next Steps:**

1. `/gsd-verify-phase 28` — Verify HIST-01..HIST-05 coverage
2. `/gsd-plan-phase 29` — Plan photo upload and attachment flow
3. `/gsd-execute-phase 29` — Implement Phase 29 plans

---

*State updated: 2026-05-01 after Phase 26 completion*
