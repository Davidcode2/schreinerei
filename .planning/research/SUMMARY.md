# Project Research Summary

**Project:** Schreinerei — Construction Management SaaS (v1.8)
**Domain:** Activity Feed, File Attachments, Site Status Workflow
**Researched:** 2026-05-01
**Confidence:** HIGH

## Executive Summary

Schreinerei is a multi-tenant offline-first PWA for construction site management. Version 1.8 adds an enhanced activity feed with file attachments, a status workflow UI, and material extraction history. The product already has strong foundations: a working activity feed component, site status state machine, and offline sync infrastructure with IndexedDB.

The recommended approach builds incrementally on existing patterns: tabbed activity feed UI first (low complexity, leverages existing code), then status change modal with audit trail, then material history query optimization, and finally file storage infrastructure with S3-compatible MinIO. Each phase delivers user value independently while avoiding the critical pitfalls identified: N+1 queries in the activity feed, race conditions in status transitions, and security vulnerabilities in file uploads.

Key risks center on file upload security (path traversal, malicious payloads) and offline sync conflicts. The research strongly recommends starting with simple conflict detection (version mismatch = reject with error) rather than complex CRDTs, and implementing the FileStorage trait as a port/adapter pattern to allow easy migration from local filesystem (dev) to S3/MinIO (production).

## Key Findings

### Recommended Stack

The stack extends the existing Axum + React architecture with minimal new dependencies. File uploads use Axum's built-in multipart support and react-dropzone on the frontend. Image processing is handled by the pure-Rust `image` crate—no native dependencies, works in all environments. Storage uses MinIO (S3-compatible) deployed in the existing Kubernetes cluster, avoiding external dependencies.

**Core technologies:**
- **axum::extract::Multipart** — File upload handling — Built into Axum 0.8 already in use, streaming support, zero new dependencies
- **image 0.25** — Image thumbnails and validation — Pure Rust, no native deps, handles resize/thumbnail ops
- **aws-sdk-s3 1.x** — Object storage client — Works with MinIO in Kubernetes, official AWS SDK, native async
- **react-dropzone 14.x** — Frontend file uploads — Industry standard, works with React 18/19, integrates with existing shadcn/ui
- **xstate 5.x** — Frontend state machine — For upload workflow states (idle → uploading → success/error), prevents invalid UI states

### Expected Features

The research identified clear table stakes for construction management apps. Most are already implemented; the gaps are file attachments and status change UI. The unified activity feed (notes, photos, documents, materials in one place) is a key differentiator—most competitors fragment this across multiple tabs.

**Must have (table stakes):**
- Activity Feed — Already built, needs tabbed UI and file attachments
- Status Transitions — State machine exists in backend, needs frontend modal
- Material Tracking — Already has site_id link in StockEntry, needs display in feed
- Photo Upload — Field exists, needs upload UI and storage backend
- Offline Support — PWA exists, files need offline queue strategy

**Should have (competitive):**
- Unified Activity Feed — One place for notes, documents, photos, materials. Simplicity differentiator.
- Material Context in Feed — "5x Schrauben (Befestigungsmaterial) → Baustelle Müller" shows category + site.
- Offline File Capture — Critical for field workers, most competitors are online-only.
- Status Change Audit Trail — Auto-generated activity entries for transparency.

**Defer (v2+):**
- Document attachments beyond photos — Start with images only; documents add preview complexity
- Real-time WebSocket sync — Polling sufficient for MVP team sizes
- Rich text editor for notes — Plain text is faster, simpler, offline-friendly

### Architecture Approach

The architecture extends the existing modular monolith with a FileStorage port/adapter pattern. This allows starting with local filesystem for development and switching to S3/MinIO for production without code changes. The Activity domain is enhanced with an `attachments` field (JSONB array) and new `Attachment` struct. Status changes already emit `SiteStatusChanged` events—implementation just needs to create activity records on each transition.

**Major components:**
1. **FileStorage trait** (Port) — `store()`, `retrieve()`, `delete()`, `get_url()` methods. Implemented by `LocalStorage` (dev) and `S3Storage` (prod).
2. **Enhanced Activity domain** — Adds `attachments: Vec<Attachment>` and `ActivityType::Document` variant. Attachments stored as JSONB metadata.
3. **Status change flow** — `SiteService::change_status()` validates transition, updates site, creates activity, emits event. Uses optimistic locking (version column).
4. **Material history query** — UNION of `site_activities` and `stock_entries` with JOIN to materials for category context.

### Critical Pitfalls

The research identified six critical pitfalls that must be addressed during implementation:

1. **File Upload Security** — Never use user-provided filenames (UUID only), validate magic bytes not extensions, strip EXIF metadata, store outside webroot. Path traversal and polyglot files are real attack vectors.

2. **Activity Feed N+1 Queries** — Eagerly JOIN users, materials, categories in single query. Don't lazy-load related entities. Cursor-based pagination avoids OFFSET performance cliff.

3. **Status Transition Race Condition** — Add `version` column to sites table. Update with `WHERE id = ? AND version = ?`. If 0 rows affected, concurrent modification occurred—reject or retry.

4. **Offline Sync Conflicts** — Add version/timestamp to all entities. Before applying queued action, check if server version > client version. For MVP: reject conflicting updates with clear error, let user re-enter.

5. **Multi-Tenant File Leakage** — Authorize every file access: `file.tenant_id == current_user.tenant_id`. UUIDs are not secret. Use tenant-scoped storage paths and presigned URLs with tenant claim.

6. **Material History Link Breaks** — Soft delete sites (never hard delete). Denormalize `site_name` at extraction time for snapshot. FK constraint with RESTRICT prevents orphaned references.

## Implications for Roadmap

Based on the research, suggested phase structure for v1.8:

### Phase 1: Status Change Workflow
**Rationale:** Lowest complexity with high user visibility. State machine already exists in backend—just needs UI integration. Builds confidence before tackling file uploads.
**Delivers:** Status change modal with validation, audit trail (auto-created activity entries), restricted transitions
**Addresses:** Status Transitions, Status Change Audit Trail features
**Avoids:** Status race condition pitfall via optimistic locking

### Phase 2: Tabbed Activity Feed
**Rationale:** Organizes existing content (notes, photos) without new infrastructure. Low risk, immediate UX improvement.
**Delivers:** "Notizen/Dokumente" tab with existing ActivityFeed, "Material" tab placeholder
**Uses:** Existing ActivityFeed component, existing ActivityType enum
**Implements:** Frontend tab navigation component

### Phase 3: Material History Tab
**Rationale:** Leverages existing `site_id` link in StockEntry. Just needs query optimization and display. No new infrastructure.
**Delivers:** Materials tab showing stock entries linked to site, with category context and user attribution
**Uses:** Existing StockEntryWithSite, JOIN query for category
**Avoids:** N+1 query pitfall via eager loading
**Addresses:** Material Context in Feed differentiator

### Phase 4: File Storage Infrastructure
**Rationale:** Foundation for photo uploads. Must be done before file upload UI. Port/adapter pattern allows dev/prod flexibility.
**Delivers:** FileStorage trait, LocalStorage adapter (dev), S3Storage adapter (prod), migration for attachments metadata
**Uses:** aws-sdk-s3, MinIO in Kubernetes
**Implements:** FileStorage port/adapter architecture
**Avoids:** File upload security pitfall via UUID naming, magic byte validation

### Phase 5: Photo Upload & Attachments
**Rationale:** Depends on file storage infrastructure. Critical for field workers. Offline queue extends existing sync infrastructure.
**Delivers:** Photo capture UI, upload API, activity attachments field, offline upload queue
**Uses:** react-dropzone, image crate, existing IndexedDB sync queue
**Implements:** Enhanced Activity domain with attachments
**Avoids:** Multi-tenant file leakage via tenant authorization, Offline sync conflicts via version checking

### Phase Ordering Rationale

- **Status workflow first:** Minimal dependencies, builds on existing state machine, delivers immediate value
- **Feed tabs second:** No backend changes, pure frontend reorganization, prepares for material history
- **Material history third:** Leverages existing data, needs query work but no new infrastructure
- **File storage fourth:** Foundation for uploads, can run in parallel with phases 1-3
- **Photo uploads last:** Depends on all previous phases, most complex feature

The research strongly recommends **not** implementing real-time WebSockets for v1.8—polling is sufficient for expected team sizes. Similarly, document uploads should be deferred; photos cover 90% of field use cases with 10% of the complexity.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 5 (Photo Uploads):** Complex offline-first file handling. IndexedDB blob storage, sync retry logic, conflict detection. Consider spike during planning.
- **Phase 4 (File Storage):** MinIO deployment configuration for existing Kubernetes cluster. May need infrastructure research.

Phases with standard patterns (skip research-phase):
- **Phase 1 (Status Workflow):** Well-documented state machine patterns, existing backend code
- **Phase 2 (Tabbed Feed):** Standard React tab navigation, existing component
- **Phase 3 (Material History):** Standard SQL JOINs, existing domain model

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All technologies verified against official docs (Context7, docs.rs, npm). Axum multipart, image crate, aws-sdk-s3 all production-ready. |
| Features | HIGH | Direct codebase analysis of existing implementation. Gap analysis based on actual code, not speculation. |
| Architecture | HIGH | Existing modular monolith pattern well-understood. Port/adapter for FileStorage is standard DDD pattern. |
| Pitfalls | HIGH | OWASP File Upload Cheat Sheet, Dexie consistency docs, SQLx patterns all verified. Existing codebase patterns confirmed. |

**Overall confidence:** HIGH

The research is grounded in direct codebase analysis (existing Activity domain, Site status machine, StockEntry model) and verified against authoritative sources (OWASP, official library docs, Context7). No speculative recommendations—all patterns have been implemented successfully in production systems.

### Gaps to Address

- **MinIO deployment:** The existing Kubernetes cluster needs MinIO configuration. May require DevOps research during Phase 4 planning. Alternative: LocalStorage adapter allows development to proceed without MinIO.
- **Presigned URL strategy:** For production, presigned URLs are more scalable than proxying through backend. Needs decision: 5-minute expiry? Include tenant claim in signature? Defer to Phase 5 planning.
- **Image thumbnail generation:** The `image` crate can generate thumbnails, but storage strategy (separate file vs. embedded metadata) needs decision during Phase 5. Recommendation: store thumbnails as separate files with `_thumb` suffix.

## Sources

### Primary (HIGH confidence)
- **Context7 /image-rs/image** — Image resize, thumbnail, format support, EXIF handling
- **Context7 /awslabs/aws-sdk-rust** — S3 upload, ByteStream API, async patterns
- **Context7 /react-dropzone/react-dropzone** — File upload hooks, validation, React 18 integration
- **docs.rs/axum** — Multipart extraction, WebSocket support, streaming
- **OWASP File Upload Cheat Sheet** — Security validation patterns, path traversal prevention
- **Existing codebase** — `src/modules/sites/domain/`, `frontend/src/lib/offline/`, `src/modules/inventory/`

### Secondary (MEDIUM confidence)
- **Buildertrend Features** — Construction PM feature patterns, competitive analysis
- **Dexie.js Consistency Docs** — IndexedDB sync patterns, conflict detection
- **min.io/docs** — MinIO Kubernetes deployment, S3 API compatibility

### Tertiary (LOW confidence)
- **Procore Library** — Construction industry context, marketing content (not technical docs)

---
*Research completed: 2026-05-01*
*Ready for roadmap: yes*
