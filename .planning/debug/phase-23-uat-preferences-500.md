---
status: investigating
trigger: "UAT gap: PATCH/GET /api/v1/preferences returns 500 with foreign key violation on user_preferences_user_id_fkey during active site select/clear (Phase 23 Test 2)"
created: 2026-04-30T20:00:00Z
updated: 2026-04-30T20:12:00Z
---

## Current Focus
<!-- OVERWRITE on each update - reflects NOW -->

hypothesis: /api/v1/preferences uses JWT subject UUID directly as users.id, but users.id is local DB ID; missing user sync causes FK violation on user_preferences.user_id
test: trace preferences route call path and confirm no get_or_create_from_auth/user lookup before user_preferences upsert
expecting: preferences writes bind ctx.user_id (from JWT sub) into user_preferences.user_id referencing users(id), causing FK failure when no matching users.id row exists
next_action: finalize root cause report with file-level evidence

## Symptoms
<!-- Written during gathering, then IMMUTABLE -->

expected: selecting/clearing active site works without error
actual: PATCH/GET /api/v1/preferences returns 500 Database error
errors: insert or update on table "user_preferences" violates foreign key constraint "user_preferences_user_id_fkey"
reproduction: Test 2 in phase 23 UAT
started: observed during Phase 23 UAT

## Eliminated
<!-- APPEND only - prevents re-investigating -->

## Evidence
<!-- APPEND only - facts discovered -->

- timestamp: 2026-04-30T20:05:00Z
  checked: .planning/phases/23-frontend-ui-auto-assignment/23-UAT.md
  found: Test 2 fails with 500 Database error and explicit FK violation on user_preferences_user_id_fkey during active site toggle.
  implication: Failure occurs during persistence of user preferences, not frontend toggle logic.

- timestamp: 2026-04-30T20:07:00Z
  checked: src/auth/middleware.rs and src/modules/iam/application/user_service.rs
  found: auth middleware sets AuthenticatedUser.user_id from JWT claims.sub UUID, and TenantContext copies this directly; get_or_create_from_auth maps keycloak ID via users.keycloak_user_id but preferences endpoints do not call it.
  implication: Preferences path uses Keycloak subject UUID directly without resolving/creating corresponding local users.id.

- timestamp: 2026-04-30T20:09:00Z
  checked: src/modules/iam/api/routes.rs and src/modules/iam/infrastructure/user_preferences_repository.rs
  found: GET/PATCH /api/v1/preferences call service with ctx.user_id, and repository upserts user_preferences with user_id = ctx.user_id.
  implication: Any missing users.id for that UUID will fail at insert/update.

- timestamp: 2026-04-30T20:10:00Z
  checked: migrations/011_user_preferences.sql
  found: user_preferences.user_id has FK REFERENCES users(id).
  implication: user_preferences writes require local users table row by id; keycloak UUID alone is insufficient unless synchronized to users.id.

## Resolution
<!-- OVERWRITE as understanding evolves -->

root_cause: 
fix: 
verification: 
files_changed: []
