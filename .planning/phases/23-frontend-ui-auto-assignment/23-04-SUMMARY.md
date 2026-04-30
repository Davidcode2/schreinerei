---
phase: 23-frontend-ui-auto-assignment
plan: 04
subsystem: api
tags: [rust, sqlx, iam, preferences, tenancy]
requires:
  - phase: 23-02
    provides: frontend toggle interactions that depend on stable preferences API
provides:
  - FK-safe tenant-local user mapping for preferences reads and writes
  - regression integration coverage for preferences/user ID mapping
affects: [phase-23, active-site-toggle]
tech-stack:
  added: []
  patterns: [resolve local user from auth subject before FK writes]
key-files:
  created: [tests/preferences_user_mapping_test.rs]
  modified: [src/modules/iam/api/routes.rs, src/modules/iam/application/user_service.rs]
key-decisions:
  - "Preferences endpoints now resolve tenant-local users.id via UserService helper before calling UserPreferencesService."
  - "Regression coverage validates FK-safe mapping and tenant isolation for shared Keycloak subjects."
patterns-established:
  - "Identity mapping pattern: never use raw JWT subject as local FK target."
requirements-completed: [ACTV-02, ACTV-03, ACTV-04]
duration: 24min
completed: 2026-04-30
---

# Phase 23 Plan 04: Preferences FK-safe User Mapping Summary

**Preferences GET/PATCH now consistently resolve tenant-local `users.id` before writing `user_preferences`, removing the active-site toggle 500 FK path.**

## Performance

- **Duration:** 24 min
- **Started:** 2026-04-30T00:00:00Z
- **Completed:** 2026-04-30T00:24:00Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments
- Added a dedicated `get_or_create_user_id_from_auth` helper and routed preferences handlers through it.
- Added integration regression tests that assert preferences rows reference tenant-local `users.id` and remain FK-safe.
- Re-verified tenant isolation behavior to ensure no cross-tenant identity leakage.

## Task Commits

1. **Task 1: Resolve tenant-local user before preferences operations** - `50c3efe` (fix)
2. **Task 2: Add regression integration test for FK-safe preferences mapping** - `ed96f63` (test)
3. **Task 3: Verify end-to-end preferences contract and close risk checks** - `4e54d94` (docs)

## Files Created/Modified
- `src/modules/iam/application/user_service.rs` - Added helper returning tenant-local `UserId` from auth identity.
- `src/modules/iam/api/routes.rs` - Preferences handlers now call services with resolved local user ID.
- `tests/preferences_user_mapping_test.rs` - Added 3 SQLx integration tests for mapping, clear flow, and tenant isolation.

## Decisions Made
- Resolve identity once at route boundary and pass only local `UserId` into preferences service calls.
- Keep tenant context unchanged from auth claims and apply it uniformly on user resolution + preference writes.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- `gsd-sdk` CLI was unavailable in PATH in this workspace, so workflow state artifacts were updated directly in `.planning/*` files.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 23 backend blocker for active-site toggle is closed with regression evidence.
- No follow-up phase is required for v1.7; this closes the active-site toggle milestone scope.

## Self-Check: PASSED

- Summary file exists: `.planning/phases/23-frontend-ui-auto-assignment/23-04-SUMMARY.md`
- Commit `50c3efe` found in git history
- Commit `ed96f63` found in git history
- Commit `4e54d94` found in git history
