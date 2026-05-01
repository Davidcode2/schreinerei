# Architecture Research

**Domain:** Activity Feed, File Attachments, Status Workflow Integration
**Researched:** 2026-05-01
**Confidence:** HIGH (based on existing codebase analysis)

## Existing Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                        API Layer (Axum Routes)                      │
│  /api/v1/sites/{id}/activities, /api/v1/sites/{id}/status          │
├─────────────────────────────────────────────────────────────────────┤
│                     Application Layer (Services)                     │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐     │
│  │   SiteService   │  │ InventoryService│  │   FleetService  │     │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘     │
│           │                    │                    │               │
├───────────┴────────────────────┴────────────────────┴───────────────┤
│                       Domain Layer (Pure)                            │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐   │
│  │  Site   │  │Activity │  │Material │  │Vehicle  │  │Reservation│  │
│  │(status) │  │ (note)  │  │ (stock) │  │  (res)  │  │ (status) │   │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────┘   │
│                                                                      │
│  Domain Events: SiteStatusChanged, StockWithdrawn, etc.            │
├──────────────────────────────────────────────────────────────────────┤
│                    Infrastructure Layer (Adapters)                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐               │
│  │SiteRepository│  │MaterialRepo  │  │ FileStorage  │               │
│  │  (PostgreSQL)│  │ (PostgreSQL) │  │  (NEW)       │               │
│  └──────────────┘  └──────────────┘  └──────────────┘               │
└──────────────────────────────────────────────────────────────────────┘
```

## Integration Points for v1.8

### 1. Activity Feed Enhancement

**Current State:**
- `site_activities` table exists with: `activity_type`, `content`, `photo_url`
- `ActivityType` enum: `Photo`, `Note`, `StatusChange`
- `photo_url` is just a string (no actual upload)

**Required Changes:**

| Component | Change | New/Modified |
|-----------|--------|--------------|
| `Activity` domain | Add `attachments` field for multiple documents | Modified |
| `ActivityType` enum | Add `Document` type | Modified |
| `site_activities` table | Add JSONB `metadata` column for attachments | Modified |
| File storage adapter | NEW | New |
| API routes | Add multipart upload endpoint | New |

### 2. Status Workflow State Machine

**Current State:**
- `SiteStatus` enum: `Planned`, `Active`, `Completed`, `Archived`
- `can_transition_to()` method validates transitions
- No audit trail of status changes

**Required Changes:**

| Component | Change | New/Modified |
|-----------|--------|--------------|
| `SiteStatus` | Already exists with valid transitions | None |
| `SiteService` | Add `change_status()` with event emission | Modified |
| `SiteStatusChanged` event | Already exists in `events.rs` | None |
| Status change activity | Auto-create on status change | Modified |

### 3. Material Extraction History

**Current State:**
- `stock_entries` table has `material_id`, `quantity_change`, `notes`
- No link to `site_id`

**Required Changes:**

| Component | Change | New/Modified |
|-----------|--------|--------------|
| `stock_entries` table | Add `site_id` column (nullable) | Modified |
| `StockWithdrawn` event | Already has material info | None |
| Activity feed query | Join `stock_entries` for materials tab | New query |

## New Components

### File Storage Adapter (Port + Adapters)

**Port (Trait) in `common/storage.rs`:**

```rust
#[async_trait]
pub trait FileStorage: Send + Sync {
    /// Store a file and return its key/identifier
    async fn store(
        &self,
        tenant_id: TenantId,
        folder: &str,
        filename: &str,
        content: Vec<u8>,
        content_type: &str,
    ) -> Result<String, AppError>;

    /// Retrieve a file by key
    async fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, AppError>;

    /// Delete a file by key
    async fn delete(&self, key: &str) -> Result<(), AppError>;

    /// Get a signed/ public URL for serving
    fn get_url(&self, key: &str) -> String;
}
```

**Adapters (choose one for v1):**

| Adapter | When to Use | Notes |
|---------|-------------|-------|
| `LocalStorage` | Development, single-server deploy | Files in `/var/lib/schreinerei/uploads` |
| `S3Storage` | Production, scalable | AWS S3, MinIO, or any S3-compatible |
| `GcsStorage` | GCP deployment | Google Cloud Storage |

**Recommendation for v1.8:** `LocalStorage` for simplicity, with path pattern:
```
/var/lib/schreinerei/uploads/{tenant_id}/{folder}/{uuid}.{ext}
```

### Enhanced Activity Domain

**Updated `Activity` struct:**

```rust
pub struct Activity {
    pub id: ActivityId,
    pub tenant_id: TenantId,
    pub site_id: SiteId,
    pub user_id: UserId,
    pub activity_type: ActivityType,
    pub content: Option<String>,
    pub attachments: Vec<Attachment>,  // NEW: multiple files
    pub created_at: DateTime<Utc>,
}

pub struct Attachment {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub storage_key: String,
    pub thumbnail_key: Option<String>,  // For images
}

pub enum ActivityType {
    Photo,
    Note,
    Document,      // NEW
    StatusChange,  // System-generated
    MaterialUsed,  // NEW: from stock_entries
}
```

### Activity Feed Query Enhancement

**Two-tab structure requires union query:**

```sql
-- Notes/Documents tab
SELECT 
    'activity' as source,
    sa.id,
    sa.site_id,
    sa.user_id,
    sa.activity_type,
    sa.content,
    sa.created_at,
    NULL as material_name,
    NULL as quantity
FROM site_activities sa
WHERE sa.site_id = $1 AND sa.tenant_id = $2
ORDER BY sa.created_at DESC

UNION ALL

-- Materials tab (from stock_entries)
SELECT
    'material' as source,
    se.id,
    se.site_id,
    se.user_id,
    'material_used' as activity_type,
    se.notes as content,
    se.created_at,
    m.name as material_name,
    se.quantity_change as quantity
FROM stock_entries se
JOIN materials m ON se.material_id = m.id
WHERE se.site_id = $1 AND se.tenant_id = $2 AND se.quantity_change < 0
ORDER BY created_at DESC
```

## Data Flow

### Upload Flow (New)

```
Frontend (FormData)
    ↓
POST /api/v1/sites/{id}/activities (multipart)
    ↓
Axum Multipart Extractor
    ↓
SiteService::create_activity_with_attachments()
    ↓
├─ FileStorage::store() → storage_key
├─ Create Activity with attachment metadata
├─ SiteRepository::create_activity()
└─ EventBus::publish(ActivityAdded)
    ↓
ActivityResponse (with attachment URLs)
```

### Status Change Flow (Enhanced)

```
Frontend (Status Modal)
    ↓
PATCH /api/v1/sites/{id}/status
    ↓
SiteService::change_status()
    ↓
├─ Site.can_transition_to() validation
├─ SiteRepository::update_site()
├─ Create Activity (ActivityType::StatusChange)
├─ SiteRepository::create_activity()
└─ EventBus::publish(SiteStatusChanged)
    ↓
SiteResponse (updated)
```

### Material History Flow (Enhanced)

```
Frontend (Materials Tab)
    ↓
GET /api/v1/sites/{id}/materials
    ↓
SiteService::get_material_history()
    ↓
SiteRepository::list_material_entries()  -- NEW method
    ↓
JOIN stock_entries + materials
    ↓
MaterialHistoryResponse[]
```

## Project Structure Extensions

```
src/
├── common/
│   ├── storage.rs           # NEW: FileStorage trait
│   └── storage/
│       ├── mod.rs           # NEW
│       ├── local.rs         # NEW: Local filesystem adapter
│       └── s3.rs            # FUTURE: S3 adapter
│
├── modules/
│   └── sites/
│       ├── domain/
│       │   ├── activity.rs  # MODIFIED: Add attachments
│       │   └── site.rs      # UNCHANGED: Already has status
│       ├── application/
│       │   └── site_service.rs  # MODIFIED: Add methods
│       ├── infrastructure/
│       │   └── site_repository.rs  # MODIFIED: Add queries
│       └── api/
│           └── routes.rs    # MODIFIED: Add multipart routes
│
└── migrations/
    └── 012_activity_attachments.sql  # NEW: Add metadata column
```

## Architectural Patterns

### Pattern 1: Port/Adapter for File Storage

**What:** Define a trait (Port) in domain/application layer, implement in infrastructure.

**When to use:** External dependencies that may change (storage provider).

**Trade-offs:**
- Pros: Easy to swap providers, testable with mock adapters
- Cons: Additional abstraction layer

**Example:**

```rust
// In application layer
pub struct SiteService<S: FileStorage> {
    site_repo: SiteRepository,
    storage: S,
}

impl<S: FileStorage> SiteService<S> {
    pub async fn create_activity_with_attachments(
        &self,
        create: CreateActivity,
        files: Vec<UploadFile>,
        ctx: &TenantContext,
    ) -> Result<Activity, AppError> {
        let mut attachments = Vec::new();
        for file in files {
            let key = self.storage.store(
                ctx.tenant_id,
                "activities",
                &file.filename,
                file.content,
                &file.content_type,
            ).await?;
            attachments.push(Attachment {
                id: Uuid::new_v4(),
                filename: file.filename,
                content_type: file.content_type,
                size_bytes: file.size,
                storage_key: key,
                thumbnail_key: None,
            });
        }
        // Create activity with attachments...
    }
}
```

### Pattern 2: Domain Events for Cross-Module Communication

**What:** Status changes emit events that can be consumed by other modules.

**When to use:** When action triggers side effects (activity creation, notifications).

**Trade-offs:**
- Pros: Decoupled, auditable, replay-able
- Cons: Eventually consistent, more complex

**Example (already exists):**

```rust
// SiteService::change_status
pub async fn change_status(
    &self,
    site_id: SiteId,
    new_status: SiteStatus,
    ctx: &TenantContext,
) -> Result<Site, AppError> {
    let site = self.site_repo.find_site_by_id(ctx.tenant_id, site_id).await?
        .ok_or(AppError::NotFound("Site not found".to_string()))?;

    if !site.can_transition_to(new_status) {
        return Err(AppError::Validation(
            format!("Invalid status transition from {} to {}", site.status, new_status)
        ));
    }

    let old_status = site.status;
    let updated = self.site_repo.update_site(ctx.tenant_id, site_id, &UpdateSite {
        status: Some(new_status),
        ..Default::default()
    }).await?;

    // Emit event
    let event = SiteStatusChangedPayload {
        site_id,
        old_status: old_status.to_string(),
        new_status: new_status.to_string(),
        changed_by: ctx.user_id,
    }.into_event(ctx.tenant_id);

    self.site_repo.publish_event(&event).await?;

    // Create activity record
    self.site_repo.create_activity(ctx.tenant_id, ctx.user_id, &CreateActivity {
        site_id,
        activity_type: ActivityType::StatusChange,
        content: Some(format!("{} → {}", old_status, new_status)),
        photo_url: None,
    }).await?;

    Ok(updated)
}
```

### Pattern 3: Union Queries for Activity Feed

**What:** Combine activities from multiple sources (notes, materials, status) in one feed.

**When to use:** When displaying a timeline of heterogeneous events.

**Trade-offs:**
- Pros: Single query, consistent ordering
- Cons: More complex SQL, needs discriminator column

**Example:**

```rust
// SiteRepository
pub async fn list_activity_feed(
    &self,
    tenant_id: TenantId,
    site_id: SiteId,
    tab: ActivityTab,
    limit: i32,
) -> Result<Vec<ActivityFeedItem>, AppError> {
    match tab {
        ActivityTab::NotesDocuments => {
            // Query site_activities only
        }
        ActivityTab::Materials => {
            // Query stock_entries with JOIN
        }
    }
}
```

## Anti-Patterns to Avoid

### Anti-Pattern 1: Storing Files in Database

**What people do:** Store file blobs in PostgreSQL BYTEA columns.

**Why it's wrong:** Bloats database, slow backups, can't serve directly, no CDN.

**Do this instead:** Store files on filesystem or object storage, store only keys/URLs in DB.

### Anti-Pattern 2: Blocking File Uploads

**What people do:** Await full upload before continuing request.

**Why it's wrong:** Large files block the async runtime, timeouts.

**Do this instead:** Stream uploads with `axum::extract::Multipart`, process chunks.

### Anti-Pattern 3: Status Changes Without Audit Trail

**What people do:** Just update status column, no history.

**Why it's wrong:** No way to see who changed what when.

**Do this instead:** Create activity records for all status changes (already done).

## Database Changes

### Migration 012: Activity Attachments

```sql
-- Add metadata column for attachments
ALTER TABLE site_activities 
ADD COLUMN metadata JSONB DEFAULT '{}';

-- Add site_id to stock_entries for material tracking
ALTER TABLE stock_entries
ADD COLUMN site_id UUID REFERENCES sites(id) ON DELETE SET NULL;

CREATE INDEX idx_stock_entries_site ON stock_entries(site_id);

-- Update existing activities with empty metadata
UPDATE site_activities SET metadata = '{}' WHERE metadata IS NULL;
```

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| 0-100 users | Local file storage, single server |
| 100-1000 users | S3-compatible storage, CDN for static files |
| 1000+ users | Consider separating file upload service |

### Scaling Priorities

1. **First bottleneck:** Disk I/O for file uploads → Move to S3
2. **Second bottleneck:** Activity feed queries → Add materialized view or cache

## Build Order

Based on dependencies, recommended implementation order:

1. **File Storage Infrastructure** (no dependencies)
   - `common/storage.rs` trait
   - `common/storage/local.rs` adapter
   - Configuration for upload path

2. **Database Migration**
   - `012_activity_attachments.sql`
   - Run migration, verify

3. **Enhanced Activity Domain**
   - Update `Activity` struct
   - Add `Attachment` struct
   - Update `ActivityType` enum

4. **Repository Updates**
   - `SiteRepository::create_activity_with_attachments()`
   - `SiteRepository::list_material_history()` (for materials tab)

5. **Service Layer**
   - `SiteService::change_status()` with activity creation
   - `SiteService::upload_attachment()` method

6. **API Routes**
   - `POST /api/v1/sites/{id}/activities` with multipart
   - `GET /api/v1/sites/{id}/materials` for history

7. **Frontend Integration**
   - File upload component
   - Status change modal
   - Materials tab in activity feed

## Integration Checklist

- [ ] FileStorage trait in `common/storage.rs`
- [ ] LocalStorage adapter implementation
- [ ] Migration for `metadata` and `site_id` columns
- [ ] `Attachment` domain type
- [ ] `ActivityType::Document` variant
- [ ] `SiteRepository::list_material_history()` method
- [ ] `SiteService::change_status()` with activity creation
- [ ] Multipart upload route
- [ ] Material history route
- [ ] Frontend file upload component
- [ ] Status change modal
- [ ] Materials tab in ActivityFeed

## Sources

- Existing codebase analysis (HIGH confidence)
- Axum multipart documentation: https://docs.rs/axum/latest/axum/extract/struct.Multipart.html
- PostgreSQL JSONB patterns: https://www.postgresql.org/docs/current/datatype-json.html
- Tower middleware for file size limits: https://docs.rs/tower-http/latest/tower_http/limit/index.html

---
*Architecture research for: Schreinerei v1.8 Activity Feed & Site Status*
*Researched: 2026-05-01*
