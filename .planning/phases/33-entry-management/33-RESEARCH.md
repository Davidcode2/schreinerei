# Phase 33: Entry Management — Research

**Phase:** 33 — Entry Management  
**Date:** 2026-05-01  
**Discovery Level:** 0 (existing stack/patterns, no new dependencies)

## Phase Boundary

Add creator-only deletion for Baustelle activity entries so users can:

- see a delete affordance only on entries they are allowed to remove
- confirm the destructive action before deletion happens
- remove the entry from the feed together with its stored attachments

This phase builds on the existing activity feed, authenticated attachment pipeline, and prior delete-confirmation patterns. It should not change upload behavior, viewer routing, or tenant isolation rules.

## Standard Stack

- **Frontend:** Vite, React 18, TypeScript, React Query, React Router, Tailwind, shadcn/ui
- **Backend:** Rust, Axum 0.8, SQLx 0.8, PostgreSQL
- **Auth:** Keycloak JWT → tenant-scoped request context
- **Testing:** Vitest frontend tests, Rust tests in touched modules
- **Established delete UX pattern:** destructive action + AlertDialog confirmation + toast feedback + query invalidation

## Current Implementation Reality

### Frontend constraints

| File | Current behavior | Phase 33 impact |
|------|------------------|-----------------|
| `frontend/src/pages/sites/ActivityFeed.tsx` | Renders activity cards and viewer links, but no per-entry actions | Must render creator-only delete affordance and confirmation flow |
| `frontend/src/lib/api/hooks/useSites.ts` | Exposes activity list/create/upload hooks, but no activity delete mutation | Needs a delete mutation that invalidates the site activity query |
| `frontend/src/types/sites.ts` | `Activity` lacks any server-derived permission field | Needs a permission flag so UI does not guess ownership client-side |
| `frontend/src/components/shared/DeleteConfirmDialog.tsx` | Existing reusable delete confirmation wrapper | Can be reused for activity deletion UX |

### Backend constraints

| File | Current behavior | Phase 33 impact |
|------|------------------|-----------------|
| `src/modules/sites/api/routes.rs` | Exposes GET/POST for site activities, no DELETE route | Needs DELETE route and `ActivityResponse` permission flag |
| `src/modules/sites/application/site_service.rs` | Creates/lists activities and resolves tenant-local users | Must enforce creator-only delete server-side and annotate read responses safely |
| `src/modules/sites/infrastructure/site_repository.rs` | Lists activities and linked attachments, but has no activity delete/read-by-id methods | Needs tenant/site-scoped fetch and delete helpers |
| `migrations/005_sites_schema.sql` + attachment migrations | `site_activities` hard-delete naturally; linked `site_activity_attachments.activity_id` rows cascade on delete | Must also explicitly clean up photo-upload attachments that are referenced only through `photo_url` |

## Gaps Against Phase Requirements

1. **ENTRY-01:** No server or UI support exists for deleting activities.
2. **ENTRY-02:** No confirmation dialog exists in the activity feed.
3. **ENTRY-03:** Linked document attachments would cascade on activity delete, but photo-upload attachments stored before activity linking can remain orphaned unless explicitly removed.

## Critical Constraint: Local User IDs vs Keycloak Subject IDs

The frontend auth user ID comes from the Keycloak JWT subject (`payload.sub`), but `site_activities.user_id` stores the tenant-local `users.id` UUID.

Implication:

- **Do not** decide creator ownership in the frontend by comparing `activity.user_id` to the authenticated frontend user.
- **Do** expose a server-derived boolean such as `can_delete` on the activity read DTO.

This matches the Phase 23 identity-mapping pattern: never use raw JWT subject values as tenant-local foreign-key identities.

## Established Patterns To Reuse

1. **Ownership enforcement in service layer**  
   Phase 20 time-entry delete checks owner/admin in `SiteService` after resolving the tenant-local user ID.

2. **Delete confirmation UX**  
   Phase 19 introduced AlertDialog-based destructive confirmation with toast feedback.

3. **Tenant-scoped repository access**  
   All site repository methods filter by `tenant_id`; activity delete must preserve that rule for both activity rows and attachment rows.

4. **Authenticated media pipeline**  
   Phase 29/31/32 require attachment cleanup to operate on attachment IDs/rows, never on public blob URLs.

## Recommended Architecture

### 1. Expose delete permission from the server

Extend the activity response contract with a server-derived `can_delete` boolean.

Why:

- avoids incorrect frontend ownership checks against Keycloak subject IDs
- keeps authorization logic on the trusted side of the boundary
- makes creator-only rendering deterministic for React components and tests

### 2. Add a site-scoped DELETE route

Recommended route shape:

- `DELETE /api/v1/sites/{site_id}/activities/{activity_id}`

Why:

- matches existing site-scoped activity routing
- lets the backend validate tenant + site + activity ownership together
- keeps React Query invalidation simple (`["activities", siteId]`)

### 3. Use hard delete for user-created note/photo activities

`site_activities` has no soft-delete column and behaves like time entries: user-generated operational records, not long-lived master data.

Recommendation:

- hard-delete user-created `note` and `photo` activities
- do **not** allow delete for `status_change` entries; they are system-generated workflow history

### 4. Clean up both linked and legacy photo attachments

There are two attachment shapes to account for:

- **Document attachments:** linked by `site_activity_attachments.activity_id` and already cascade on activity delete
- **Photo activities:** often store only `photo_url` pointing at `/api/v1/attachments/{attachment_id}` while the attachment row remains unlinked

Deletion logic must therefore:

- delete the activity row
- rely on cascade for linked document attachments
- parse and delete the tenant-scoped attachment row referenced by protected `photo_url` when present

### 5. Keep the UI logic permission-driven, not type-guessing

The feed should render the delete affordance only when `can_delete` is true. Status-change entries should remain non-deletable because they are system audit/history items.

## Likely File Impact

### Backend

- `src/modules/sites/api/routes.rs` — add DELETE route, extend `ActivityResponse`
- `src/modules/sites/application/site_service.rs` — activity delete orchestration, permission annotation
- `src/modules/sites/infrastructure/site_repository.rs` — activity lookup/delete helpers and photo attachment cleanup
- `frontend/src/types/generated.ts` — regenerated DTOs

### Frontend

- `frontend/src/types/sites.ts` — add `can_delete` to local activity type
- `frontend/src/lib/api/hooks/useSites.ts` — add `useDeleteActivity`
- `frontend/src/lib/api/hooks/useSites.test.tsx` — verify delete hook contract/invalidation
- `frontend/src/pages/sites/ActivityFeed.tsx` — creator-only delete button + confirmation wiring
- `frontend/src/pages/sites/ActivityFeed.test.tsx` — cover visibility, confirmation, mutation, and non-deletable states

## Common Pitfalls

1. **Do not compare frontend auth IDs to `activity.user_id`** — they are different identity domains.
2. **Do not trust the client to hide unauthorized deletes** — enforce creator-only deletion in the backend.
3. **Do not forget photo-upload attachment cleanup** — `photo_url` attachments may not be linked by `activity_id`.
4. **Do not make `status_change` entries deletable** — preserve system workflow history.
5. **Do not regress tenant scoping** — activity lookup, attachment cleanup, and deletion must all stay tenant-bound.

## Architectural Responsibility Map

| Layer | Expected change | Risk |
|-------|------------------|------|
| Presentation | Render creator-only delete affordance and confirmation flow | Medium |
| Presentation | Use server-derived `can_delete` flag and invalidate activity queries after deletion | Medium |
| Application | Resolve tenant-local ownership and orchestrate delete cleanup | High |
| Infrastructure | Delete activity rows and photo-url attachment rows with tenant/site scoping | High |
| Domain | Minimal or none; only add helper logic if needed for protected attachment ID parsing | Low |

## Planning Guidance

Plan this as a backend foundation plan followed by a frontend integration plan. The key dependency is the server-derived permission contract plus complete attachment cleanup semantics. Once the API exposes `can_delete` and a site-scoped delete route, the frontend work becomes a straightforward delete-button + confirmation integration.
