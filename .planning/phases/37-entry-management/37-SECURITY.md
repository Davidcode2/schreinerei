---
phase: 33
slug: entry-management
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-01
---

# Phase 37 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| browser → activity delete API | Untrusted user input chooses site/activity IDs and attempts destructive actions | site ID, activity ID, authenticated user context |
| activity service → PostgreSQL | Trusted backend must enforce tenant, ownership, and cleanup invariants before issuing deletes | tenant-scoped activity rows, attachment rows |
| React activity feed → delete mutation | Untrusted clicks may target stale or unauthorized entries | backend-derived `can_delete`, site ID, activity ID |
| delete mutation → backend route | UI submits destructive requests but cannot enforce authorization | DELETE request path params |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-37-01 | S/E | `delete_activity` route | mitigate | Resolve requester to tenant-local user and compare in service (`src/modules/sites/application/site_service.rs:127-138`, `525-542`) | closed |
| T-37-02 | T | activity + attachment delete SQL | mitigate | Scope activity lookup/delete and attachment delete by `tenant_id`, `site_id`, `activity_id` / attachment id (`src/modules/sites/infrastructure/site_repository.rs:585-619`, `622-668`; `src/modules/sites/application/site_service.rs:531-553`) | closed |
| T-37-03 | I | activity read DTO | mitigate | API exposes boolean `can_delete` only in DTO and generated TS bindings (`src/modules/sites/api/routes.rs:576-603`, `frontend/src/types/generated.ts:5`) | closed |
| T-37-04 | R | status-change history | mitigate | Delete authorization only allows `note`/`photo`, so `status_change` is rejected (`src/modules/sites/application/site_service.rs:122-125`, `537-542`; `src/modules/sites/domain/activity.rs:9-22`) | closed |
| T-37-05 | D | attachment cleanup path | accept | Accepted MVP risk documented in Accepted Risks Log; no extra throttling beyond authenticated access | closed |
| T-37-06 | T/E | delete button visibility | mitigate | Feed renders delete affordance solely from backend `activity.can_delete` (`frontend/src/pages/sites/ActivityFeed.tsx:261-271`, `frontend/src/types/sites.ts:107-118`) | closed |
| T-37-07 | R | destructive confirmation UX | mitigate | Delete action passes through confirmation dialog before mutation (`frontend/src/pages/sites/ActivityFeed.tsx:332-348`, `421-430`; `frontend/src/components/shared/DeleteConfirmDialog.tsx:28-45`) | closed |
| T-37-08 | I | media feed card wiring | accept | Accepted low-risk disclosure posture documented in Accepted Risks Log | closed |
| T-37-09 | D | repeated mutation clicks | mitigate | Confirm/cancel actions disable while pending and dialog reset is blocked during pending state (`frontend/src/components/shared/DeleteConfirmDialog.tsx:37-44`, `frontend/src/pages/sites/ActivityFeed.tsx:423-430`) | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-37-01 | T-37-05 | Activity cleanup only deletes single activity-scoped rows and Phase 37 keeps existing auth boundaries; additional throttling deferred at MVP scale. | security audit workflow | 2026-05-01 |
| AR-37-02 | T-37-08 | Delete affordance adds no new sensitive fields beyond already-rendered entry/media metadata in the feed. | security audit workflow | 2026-05-01 |

---

## Unregistered Flags

None. No `## Threat Flags` section was present in Phase 37 summary artifacts.

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-01 | 9 | 9 | 0 | OpenAI gpt-5.4 security auditor |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-01
