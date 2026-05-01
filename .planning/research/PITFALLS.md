# Pitfalls Research: Activity Feed, File Uploads & Status Workflows

**Domain:** Schreinerei SaaS — Adding activity feed, site status workflow, notes/documents, and material history to existing multi-tenant offline-first PWA
**Researched:** 2026-05-01
**Confidence:** HIGH

## Critical Pitfalls

### Pitfall 1: File Upload Path Traversal & Execution

**What goes wrong:**
User uploads a file with a crafted filename like `../../../etc/passwd` or `..%2F..%2Fconfig.rs`. The file gets stored outside the intended directory, or worse, a `.php`/`.rs` file gets executed when accessed. For images, malicious payloads hidden in EXIF data or polyglot files (files valid as both image and executable) can execute code.

**Why it happens:**
Developers trust user-provided filenames. File extension validation is bypassed with double extensions (`.jpg.php`), null bytes (`.php%00.jpg`), or case variations (`.pHp`). MIME type from the `Content-Type` header is spoofable and unreliable.

**How to avoid:**
1. **Never use user-provided filenames** — Generate UUID-based filenames server-side
2. **Store files outside webroot** or use a separate storage service (S3, MinIO)
3. **Validate file signatures (magic bytes)**, not extensions — First 4-16 bytes identify true file type
4. **Use an allowlist of MIME types** that business requires (images only: `image/jpeg`, `image/png`, `image/webp`)
5. **Strip metadata from images** using image rewriting (destroys steganographic payloads)
6. **Set filesystem permissions to read-only** for upload directories (no execute)

**Warning signs:**
- Files stored with original names in `public/uploads/`
- Code checks only file extension: `if filename.endsWith('.jpg')`
- Files served directly from static file routes without sanitization
- No file size limits (ZIP bombs, billion laughs attack)

**Phase to address:**
Phase implementing file uploads (Activity Feed: Notes/Documents)

---

### Pitfall 2: Activity Feed N+1 Query Explosion

**What goes wrong:**
Activity feed loads 50 activities, then makes separate queries for each user's name, each site's name, each material's category. With 50 activities, this becomes 150+ database queries. Page load time increases from 50ms to 3+ seconds.

**Why it happens:**
ORM-style lazy loading or separate fetch calls for related entities. The query looks innocent: "get activities for site X" but accessing `activity.user.name` triggers a separate query per row.

**How to avoid:**
1. **Use SQL JOINs eagerly** — Single query with `LEFT JOIN users`, `LEFT JOIN materials`, etc.
2. **Denormalize frequently accessed fields** — Store `user_name` directly on activity row (updated via domain event when user changes name)
3. **Use cursor-based pagination** — `WHERE created_at < ? ORDER BY created_at DESC LIMIT 20` avoids OFFSET performance cliff
4. **Count total server-side** — Don't fetch all rows to count them

**Warning signs:**
- Query count increases linearly with page size
- Database query logs show hundreds of small queries for one page load
- Slow page loads when activity count grows past 100

**Phase to address:**
Phase implementing activity feed API

---

### Pitfall 3: Status Transition Race Condition

**What goes wrong:**
Two users simultaneously change site status. User A transitions `Planned → Active` at 10:00:00.000. User B transitions `Planned → Completed` at 10:00:00.050. Without locking, both succeed, leaving site in `Completed` state while skipping the required `Active` phase. Business rules (e.g., "must be active before completing") are violated.

**Why it happens:**
Optimistic concurrency not implemented. The check `site.can_transition_to(new_status)` and the update happen in separate transactions or without version checking. Time-of-check to time-of-use (TOCTOU) vulnerability.

**How to avoid:**
1. **Use optimistic locking** — Add `version` column to sites table. Update with `WHERE id = ? AND version = ?`. If 0 rows affected, another transaction modified it — retry or reject.
2. **Database-level constraint** — Store status transitions in a separate table with unique constraint on `(site_id, from_status, to_status)` for audit trail
3. **Application-level lock** — `SELECT ... FOR UPDATE` before status change (pessimistic locking)
4. **Event-sourced status** — Status is derived from transition events, not a mutable column

**Implementation for this project:**
```rust
// In site_repository.rs
pub async fn update_status(&self, site: &Site) -> Result<bool, Error> {
    let result = sqlx::query!(
        "UPDATE sites SET status = $1, updated_at = $2, version = version + 1 
         WHERE id = $3 AND tenant_id = $4 AND version = $5",
        site.status as _,
        site.updated_at,
        site.id,
        site.tenant_id,
        site.version,
    )
    .execute(&self.pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
```

**Warning signs:**
- Status changes without version checking
- `UPDATE sites SET status = ? WHERE id = ?` without version in WHERE clause
- Tests pass sequentially but fail under concurrent load
- "Impossible" states appear in database (Completed site with no Active period)

**Phase to address:**
Phase implementing site status workflow

---

### Pitfall 4: Offline Sync Conflict Data Loss

**What goes wrong:**
User A goes offline, creates note "Foundation poured" on Site X. User B (online) creates note "Foundation cancelled" on same site. User A reconnects. Current implementation processes queue in order, potentially overwriting B's note with A's stale data, or silently dropping A's note due to foreign key changes.

**Why it happens:**
No conflict detection or resolution strategy. The queue processes actions without checking if the underlying data has changed. No timestamps or version vectors to detect conflicts. Last-write-wins, but "last" is defined by sync order, not actual time.

**How to avoid:**
1. **Add `updated_at` and `version` to all entities** — Reject updates where client's version < server's version
2. **Conflict detection** — Before applying queued action, fetch current server state. If changed since action was created, prompt user.
3. **Idempotent operations** — Use client-generated IDs (UUID) so retries don't create duplicates
4. **Operational transforms for text** — Notes with conflicting edits can be merged using OT or CRDTs
5. **For this project** — Start simple: reject conflicting updates with clear error message, let user re-enter data

**Warning signs:**
- No version/timestamp comparison before applying queued actions
- Server errors during sync silently drop actions after retries
- "My changes disappeared" user reports after being offline
- Duplicated entities when sync retries after network timeout

**Phase to address:**
Phase implementing offline queue for activities (extend existing queue with conflict detection)

---

### Pitfall 5: Multi-Tenant Data Leakage via File URLs

**What goes wrong:**
File URLs are predictable: `/api/v1/files/{uuid}`. User from Tenant A guesses UUID of file belonging to Tenant B and accesses it. Or file metadata (EXIF GPS, document author) leaks tenant information.

**Why it happens:**
UUIDs are not secret. File access checks only authentication, not tenant authorization. Files are stored with tenant-unaware naming. Presigned URLs don't include tenant context.

**How to avoid:**
1. **Authorize every file access** — Check `file.tenant_id == current_user.tenant_id` before serving
2. **Use tenant-scoped paths** — `/api/v1/files/{tenant_id}/{file_id}` (but still verify auth)
3. **Presigned URLs with tenant claim** — Include tenant in signed URL payload, verify on access
4. **Strip metadata** — Remove EXIF, document properties before storage
5. **Separate storage buckets per tenant** — S3 bucket policies enforce isolation

**Warning signs:**
- File access endpoint checks only `user.is_authenticated`, not tenant
- Files stored as `/uploads/{uuid}.jpg` without tenant prefix
- Presigned URLs without tenant verification

**Phase to address:**
Phase implementing file storage (Phase 2: Activity Feed)

---

### Pitfall 6: Material History Link Breaks When Site Deleted

**What goes wrong:**
Material deductions reference `site_id`. When a site is deleted (soft delete or hard delete), the material history query returns orphaned rows. UI shows "Unknown Site" or crashes. Referential integrity violated.

**Why it happens:**
Foreign key constraint missing or `ON DELETE SET NULL` used. Soft delete makes site "invisible" but history still references it. No consideration for "what happens to linked data" during site lifecycle.

**How to avoid:**
1. **Use soft delete for sites** — Never hard delete, mark `deleted_at` timestamp
2. **Add FK constraint with RESTRICT** — Prevent site deletion while material deductions reference it
3. **Denormalize site name** — Store `site_name` on deduction row at extraction time (snapshot, not reference)
4. **Tombstone pattern** — When site deleted, update all references to point to a "Deleted Site" tombstone record
5. **For this project** — Already using soft delete pattern; add `site_name_snapshot` to material_deductions

**Warning signs:**
- `site_id` column has no FK constraint
- FK constraint uses `ON DELETE SET NULL` or `ON DELETE CASCADE`
- Site deletion succeeds while material history references it
- UI shows blank site name for old deductions

**Phase to address:**
Phase implementing material extraction history

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Store files with original filename | User recognizes their file | Path traversal, XSS via SVG/HTML uploads | **Never** |
| Skip version checking on status update | Simpler code, faster initial dev | Race conditions, inconsistent state | **Never** for status workflow |
| Last-write-wins for offline sync | No conflict UI needed | Silent data loss, user frustration | MVP only with warning; fix before pilot |
| Load all activities, filter in frontend | Skip backend pagination work | OOM crash at 10k+ activities | Prototype only |
| Store activity user_id without name | Simpler schema | N+1 queries, slow feed | Only if JOINs used for display |
| No file size limit | No UX for "file too large" | Storage exhaustion, ZIP bombs | **Never** |

## Integration Gotchas

Common mistakes when connecting to external services or existing modules.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Keycloak user lookup | Query Keycloak on every activity | Cache user name locally; update via event |
| File storage (S3/MinIO) | Use same bucket for all tenants | Tenant-scoped paths + bucket policies OR separate buckets |
| IndexedDB sync | Full table overwrite on every sync | Delta sync using `updated_at > last_sync_time` |
| Activity feed pagination | OFFSET-based pagination | Cursor-based: `WHERE created_at < cursor` |
| Status transitions | Allow any admin to change any status | Check business rules: only assigned users can activate |
| Material deduction link | FK to sites without soft delete handling | Denormalize site_name at extraction time |

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Activity feed without index on `site_id, created_at` | Slow feed load as activities grow | Add composite index from day 1 | 1,000+ activities per site |
| Fetching all materials for history | Page takes 5+ seconds | Server-side pagination, lazy load | 100+ materials linked to site |
| Storing images in database | Backup grows 10x, slow queries | Store files in object storage, DB has URL | 1,000+ images |
| No connection pooling | "too many connections" errors | Configure SQLx pool with limits | 10+ concurrent users |
| Sync polling every 5 seconds | Server CPU spike, battery drain | WebSocket for real-time, or 30+ second interval | 50+ users online |

## Security Mistakes

Domain-specific security issues beyond general web security.

| Mistake | Risk | Prevention |
|---------|------|------------|
| Predictable file URLs | Cross-tenant file access | UUID + tenant authorization check |
| No MIME validation | Malicious file upload (polyglot, EXIF payload) | Validate magic bytes, strip metadata |
| SVG file upload | XSS via embedded `<script>` tags | Convert SVGs to PNG, or block SVG entirely |
| Presigned URL without expiry | File accessible forever even after delete | 5-minute expiry max |
| Status change without audit trail | No accountability, can't debug issues | Store every transition with user, timestamp |
| Activity creation without site membership check | User spams another tenant's site | Verify `user` is assigned to `site` |

## UX Pitfalls

Common user experience mistakes in this domain.

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Status change without confirmation modal | Accidental "Complete" on active site | Show consequences: "This will archive all materials. Continue?" |
| Activity feed without "Load More" | User scrolls forever, page sluggish | Virtualized list + "Load older" button at top |
| Offline indicator only in settings | User doesn't know data isn't syncing | Banner: "Offline — 3 changes pending" |
| File upload with no progress | User thinks upload failed, uploads again | Progress bar + success toast |
| Material history without site context | User sees deduction without knowing which site | Always show linked site name (clickable) |
| "Aktiv" button confusing with status "aktiv" | User thinks they're setting status | Rename button to "Auswählen" (Select) |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Activity feed:** Often missing tenant isolation check — verify activities filtered by `site.tenant_id`
- [ ] **File upload:** Often missing cleanup for orphaned files (upload succeeds, entity creation fails) — verify transactional cleanup
- [ ] **Status workflow:** Often missing audit trail — verify every transition logged with user, timestamp, reason
- [ ] **Material history:** Often missing deleted site handling — verify UI shows "Deleted Site" not crash
- [ ] **Offline sync:** Often missing conflict handling — verify queue rejects actions where server version > client version
- [ ] **Pagination:** Often missing total count for UI — verify response includes `total_pages` or `has_more`
- [ ] **File deletion:** Often missing actual file deletion (only DB record deleted) — verify storage cleanup

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Cross-tenant file access | HIGH | 1. Audit all file access logs 2. Rotate all file URLs 3. Review tenant isolation |
| Race condition corrupted state | MEDIUM | 1. Query for impossible states 2. Manual correction with audit log entry 3. Add optimistic locking |
| Orphaned activities after site delete | MEDIUM | 1. Restore site from backup 2. Or link to "Deleted Site" tombstone |
| File storage exhaustion | HIGH | 1. Emergency storage expansion 2. Add file size limits 3. Clean up orphaned files |
| Sync conflict data loss | MEDIUM | 1. Check activity history for clues 2. Manual re-entry 3. Add conflict detection |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| File upload security | Phase: Activity Feed (file uploads) | Security review with file upload checklist |
| Activity feed N+1 | Phase: Activity Feed API | Query count assertion in tests |
| Status race condition | Phase: Site Status Workflow | Concurrent transition test |
| Offline sync conflict | Phase: Offline Queue Extension | Conflict detection test with version mismatch |
| Multi-tenant file leakage | Phase: Activity Feed (file storage) | Tenant isolation test with cross-tenant file access attempt |
| Material history link break | Phase: Material History | Test with soft-deleted site |

## Sources

- **OWASP File Upload Cheat Sheet** — https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html (HIGH confidence)
- **Dexie.js Consistency Documentation** — https://dexie.org/docs/cloud/consistency (HIGH confidence)
- **XState Guards and Concurrency** — https://context7.com/statelyai/xstate/llms.txt (HIGH confidence)
- **SQLx Pagination Patterns** — https://github.com/alexandrughinea/sqlx-paginated (HIGH confidence)
- **Existing codebase patterns** — `src/modules/sites/domain/site.rs`, `frontend/src/lib/offline/` (HIGH confidence)
- **Project tech debt** — PROJECT.md: "No conflict resolution for offline edits"

---
*Pitfalls research for: Schreinerei SaaS v1.8 Activity Feed & Site Status*
*Researched: 2026-05-01*
