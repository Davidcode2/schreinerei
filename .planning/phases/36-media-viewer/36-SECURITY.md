---
phase: 32
slug: media-viewer
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-01
---

# Phase 36 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| API response → SPA viewer | Server-provided activity metadata becomes viewer/sidebar content | creator name, note text, attachment metadata |
| DB join → API DTO | Tenant-scoped user data is merged into activity responses | user display name/email fallback |
| Router URL → viewer selection | Route params select a media target inside the authenticated site detail page | activity ID, attachment ID, slug |
| Protected blob fetch → browser download/preview | Authenticated media responses are converted to object URLs | protected attachment bytes |
| Clipboard write → share flow | Generated viewer links leave app context via clipboard | internal app route |
| Feed metadata → route generation | Existing activity metadata is transformed into deep links | site/activity/attachment IDs, filename slug |
| Keyboard/pointer interaction → router navigation | User interaction opens fullscreen viewer routes | client navigation event |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-36-01 | I | `list_activities` query | mitigate | Tenant-scoped `LEFT JOIN users ... AND users.tenant_id = site_activities.tenant_id`; `COALESCE(..., site_activities.user_id::text)` fallback in same row. Evidence: `src/modules/sites/infrastructure/site_repository.rs:572-588`, `tests/site_activity_list_activities_test.rs:205-246`. | closed |
| T-36-02 | T | `ActivityResponse.creator_name` | mitigate | `ActivityResponse` is server-serialized only and populated from domain activity fields sourced by repository SQL, not request input. Evidence: `src/modules/sites/api/routes.rs:574-603`, `src/modules/sites/infrastructure/site_repository.rs:572-588`, `src/modules/sites/infrastructure/site_repository.rs:1011-1027`. | closed |
| T-36-03 | D | activity list route | accept | Accepted: additional metadata remains a bounded string on existing `LIMIT $3` activity list query. Logged in Accepted Risks Log (`AR-36-01`). | closed |
| T-36-04 | S | route param selection | mitigate | Viewer target resolves only from already-fetched activities and unmatched params return `null`; open viewer with null target renders inline error panel instead of guessing. Evidence: `frontend/src/pages/sites/SiteDetailPage.tsx:55-67`, `frontend/src/pages/sites/SiteDetailPage.tsx:274-279`, `frontend/src/pages/sites/mediaViewerRoute.ts:42-80`, `frontend/src/pages/sites/MediaViewer.tsx:117-127`. | closed |
| T-36-05 | I | share/download actions | mitigate | Share action copies canonical app route built via `buildMediaViewerPath`; download and preview both use authenticated `apiClient.getBlob`. Evidence: `frontend/src/pages/sites/mediaViewerRoute.ts:23-31`, `frontend/src/pages/sites/SiteDetailPage.tsx:64-67`, `frontend/src/pages/sites/MediaViewer.tsx:61-68`, `frontend/src/pages/sites/MediaViewer.tsx:92-115`. | closed |
| T-36-06 | D | fullscreen viewer | accept | Accepted: preview/download is user-initiated per selected target; no prefetch loop added. Logged in Accepted Risks Log (`AR-36-02`). | closed |
| T-36-07 | T | feed route generation | mitigate | Feed links are built from activity/attachment IDs plus slugified filename through shared helper. Evidence: `frontend/src/pages/sites/ActivityFeed.tsx:173-205`, `frontend/src/pages/sites/ActivityFeed.tsx:209-225`, `frontend/src/pages/sites/mediaViewerRoute.ts:13-31`. | closed |
| T-36-08 | R | clickable fallback tiles | accept | Accepted: fallback tile opens same authenticated viewer target and performs no mutation. Logged in Accepted Risks Log (`AR-36-03`). | closed |
| T-36-09 | E | status-change cards | mitigate | Status-change entries are explicitly excluded from `hasDocumentAttachments`, preventing `ViewerTileLink` rendering for non-media records; regression test asserts no viewer link exists. Evidence: `frontend/src/pages/sites/ActivityFeed.tsx:243-246`, `frontend/src/pages/sites/ActivityFeed.test.tsx:244-270`. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-36-01 | T-36-03 | `creator_name` adds one bounded string field to an existing list query already limited by `limit <= 100`; residual DoS risk is low and accepted in the threat model. | Phase 36 threat model | 2026-05-01 |
| AR-36-02 | T-36-06 | Viewer blob fetches are initiated only when a user opens/downloads one selected item; no background fetch loop or unbounded preview job exists. | Phase 36 threat model | 2026-05-01 |
| AR-36-03 | T-36-08 | Fallback tile navigation reaches the same authenticated viewer surface and does not enable privileged action or mutation. | Phase 36 threat model | 2026-05-01 |

---

## Unregistered Flags

No `## Threat Flags` sections were present in Phase 36 summary artifacts.

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-01 | 9 | 8 | 1 | OpenAI gpt-5.4 security auditor |
| 2026-05-01 | 9 | 9 | 0 | OpenAI gpt-5.4 security auditor |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-01
