# Stack Research

**Domain:** Activity Feed, File Uploads, Site Status Management
**Researched:** 2026-05-01
**Confidence:** HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| **axum::extract::Multipart** | 0.8 (existing) | File upload handling | Built into Axum, streaming support, zero new dependencies |
| **axum::extract::ws** | 0.8 (existing) | Real-time activity updates | Built into Axum, integrates with existing tokio runtime |
| **image** | 0.25 | Image processing & thumbnails | Pure Rust, no native deps, resize/thumbnail ops, multiple format support |
| **aws-sdk-s3** | 1.x | Object storage client | S3-compatible, works with MinIO in Kubernetes, official AWS SDK |
| **react-dropzone** | 14.x | Frontend file uploads | Industry standard, React 18 compatible, integrates with shadcn/ui |
| **xstate** | 5.x | Frontend state machine | TypeScript-first, visual debugging, strict state transitions |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| **statig** | 0.4 | Backend state machine | If site status becomes complex (hierarchical states, side effects) |
| **tokio-util** | 0.7 | Codec for WebSocket messages | When implementing real-time activity feed |
| **mime** | 0.3 | MIME type detection | File upload validation |
| **uuid** | 1.x (existing) | Unique file identifiers | Already in use for entity IDs |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| **MinIO** | Local S3-compatible storage | Deploy in Kubernetes, same API as AWS S3 |
| **Stately.ai** | XState visualizer | Debug state machine transitions (optional) |

## Installation

### Backend (Rust)

```toml
# Cargo.toml additions

[dependencies]
# File uploads - already in axum 0.8
# axum::extract::Multipart is built-in with "multipart" feature

# Image processing
image = "0.25"

# S3-compatible storage (MinIO in Kubernetes)
aws-config = { version = "1", features = ["behavior-version-latest"] }
aws-sdk-s3 = "1"

# MIME type detection for uploads
mime = "0.3"

# State machine (only if simple enum insufficient)
statig = { version = "0.4", features = ["macro"] }
```

### Frontend (React/TypeScript)

```bash
# File uploads
npm install react-dropzone

# State machine
npm install xstate
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| **MinIO (S3 API)** | PostgreSQL bytea | Only for <10MB files total, no Kubernetes. bytea bloats DB, no CDN option. |
| **MinIO (S3 API)** | Local filesystem | Single-server deployments only. K8s pods are ephemeral, requires PersistentVolume complexity. |
| **image-rs** | ImageMagick | When you need advanced operations (watermarks, complex filters). image-rs is pure Rust, no native deps. |
| **image-rs** | Sharp (Node) | Not applicable - backend is Rust |
| **xstate** | Custom useReducer | For trivial state (2-3 transitions). xstate adds ~15KB but prevents invalid states. |
| **xstate** | zustand (existing) | For client state, not state machines. zustand is for stores, xstate for workflows. |
| **Axum WebSocket** | Server-Sent Events | When only server→client push needed. SSE simpler but can't do client→server. |
| **Axum WebSocket** | Polling (existing) | MVP phase. Polling already works, WebSockets for scale (>100 concurrent users). |
| **statig** | Simple enum | For straightforward geplant→aktiv→abgeschlossen. statig overkill for 3 linear states. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| **PostgreSQL bytea for files** | Bloats DB, slow backups, no CDN, max 1GB per row | MinIO S3-compatible storage |
| **Local filesystem in K8s** | Pods are ephemeral, requires PVs, multi-tenant isolation harder | MinIO with tenant-prefixed buckets |
| **actix-multipart** | Different framework, would require rewrite | Axum's built-in multipart |
| **multer** | Lower-level, Axum wraps it already | axum::extract::Multipart |
| **File input without dropzone** | Poor UX on mobile, no drag-drop | react-dropzone |
| **Complex state machine for simple status** | Over-engineering for 3-state linear workflow | Enum with transition validation |
| **External image service** | Adds network latency, deployment complexity | image-rs in-process |

## Storage Architecture

### Recommended: MinIO (S3-Compatible)

```
┌─────────────────────────────────────────────────────────────┐
│  Kubernetes Cluster                                         │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  MinIO (S3-Compatible Storage)                      │   │
│  │  ┌─────────────────────────────────────────────┐    │   │
│  │  │  Bucket: schreinerei-{tenant_id}            │    │   │
│  │  │  ├── notes/                                │    │   │
│  │  │  │   ├── {note_id}/                        │    │   │
│  │  │  │   │   ├── image_1.jpg                   │    │   │
│  │  │  │   │   └── document_1.pdf                │    │   │
│  │  │  └── thumbnails/                           │    │   │
│  │  │      └── {image_hash}_thumb.jpg            │    │   │
│  │  └─────────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

**Multi-tenant isolation:**
- Bucket per tenant: `schreinerei-{tenant_id}`
- OR single bucket with prefix: `schreinerei/{tenant_id}/notes/...`

**Why MinIO:**
- S3 API compatibility (use aws-sdk-s3)
- Self-hosted in Kubernetes (no external dependency)
- Works offline/airgapped
- Presigned URLs for direct client upload (optional optimization)
- Built-in versioning support

### File Upload Flow

```
┌──────────┐    1. Upload     ┌──────────┐    2. Store      ┌──────────┐
│ Frontend │ ──────────────▶ │  Backend │ ───────────────▶ │  MinIO   │
│          │                  │  (Axum)  │                   │  (S3)    │
│          │    4. Response   │          │    3. Metadata   │          │
│          │ ◀────────────── │          │ ◀─────────────── │          │
└──────────┘                  └──────────┘                   └──────────┘
                                    │
                                    ▼ 3b. File metadata
                              ┌──────────┐
                              │PostgreSQL│
                              │ (JSONB)  │
                              └──────────┘
```

### Database Schema for Attachments

```sql
-- Add to existing schema
CREATE TABLE note_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    note_id UUID NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL,
    
    -- File metadata
    original_filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    size_bytes BIGINT NOT NULL,
    
    -- Storage reference
    storage_key VARCHAR(500) NOT NULL,  -- S3 key
    
    -- Image-specific (nullable)
    width INT,
    height INT,
    thumbnail_key VARCHAR(500),
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

-- Store in JSONB for flexibility
-- notes.content can include embedded attachment references
```

## State Machine Approach

### Site Status: Simple Enum (Recommended for v1.8)

For the linear workflow `geplant → aktiv → abgeschlossen`, use a simple enum with validation:

```rust
// Backend: src/modules/sites/domain/site_status.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "site_status", rename_all = "lowercase")]
pub enum SiteStatus {
    Geplant,
    Aktiv,
    Abgeschlossen,
    Archiviert,
}

impl SiteStatus {
    pub fn can_transition_to(&self, target: Self) -> bool {
        match (self, target) {
            (Self::Geplant, Self::Aktiv) => true,
            (Self::Aktiv, Self::Abgeschlossen) => true,
            (Self::Abgeschlossen, Self::Archiviert) => true,
            // Allow reopening
            (Self::Abgeschlossen, Self::Aktiv) => true,
            _ => false,
        }
    }
}
```

```typescript
// Frontend: Status badge with transitions
type SiteStatus = 'geplant' | 'aktiv' | 'abgeschlossen' | 'archiviert';

const ALLOWED_TRANSITIONS: Record<SiteStatus, SiteStatus[]> = {
  geplant: ['aktiv'],
  aktiv: ['abgeschlossen'],
  abgeschlossen: ['aktiv', 'archiviert'],
  archiviert: [],
};
```

### When to Use statig (Future)

If site status becomes complex:
- Hierarchical states (aktiv has substates: vorbereitung, in_arbeit, abschluss)
- Side effects on transitions (send notifications, archive materials)
- State-local storage (different data per state)

### When to Use xstate (Frontend)

For the activity feed interaction:
- Complex form state (draft → uploading → uploaded → error)
- Multi-step note creation with attachments
- Offline sync queue state

## Real-Time Updates

### Current: Polling (Sufficient for MVP)

- React Query already polls for data freshness
- Works offline (cached data available)
- Simple to implement and debug

### Future: WebSockets (When Needed)

Enable when:
- >100 concurrent users on same site
- Sub-second update latency required
- Real-time collaboration features

```rust
// Backend: WebSocket endpoint
use axum::extract::ws::{WebSocketUpgrade, WebSocket};

async fn site_activity_ws(
    ws: WebSocketUpgrade,
    Path(site_id): Path<Uuid>,
    Extension(broadcaster): Extension<ActivityBroadcaster>,
) -> Response {
    ws.on_upgrade(move |socket| {
        broadcaster.subscribe(site_id, socket)
    })
}
```

```typescript
// Frontend: WebSocket hook (future)
const useSiteActivityStream = (siteId: string) => {
  useEffect(() => {
    const ws = new WebSocket(`${WS_URL}/sites/${siteId}/activity`);
    ws.onmessage = (event) => {
      queryClient.invalidateQueries(['activities', siteId]);
    };
    return () => ws.close();
  }, [siteId]);
};
```

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| axum 0.8 | tokio 1.x | Already in use |
| image 0.25 | tokio 1.x | Sync API, works with async via spawn_blocking |
| aws-sdk-s3 1.x | tokio 1.x | Native async support |
| react-dropzone 14.x | React 18/19 | Works with both versions |
| xstate 5.x | React 18/19 | Use @xstate/react package |

## Offline Considerations

### File Uploads (Offline-First)

1. **Frontend**: Queue uploads in IndexedDB when offline
2. **Sync**: Process queue when connection restored
3. **Backend**: Deduplicate by content hash (SHA-256)

```typescript
// Offline upload queue with Dexie
const offlineUploads = db.table('pendingUploads');
await offlineUploads.add({
  id: uuid(),
  file: await file.arrayBuffer(),
  noteId,
  createdAt: Date.now(),
  synced: false,
});
```

### Activity Feed (Offline-First)

1. **Read**: Load from IndexedDB cache
2. **Write**: Store locally, sync when online
3. **Merge**: Last-write-wins with server timestamps

## Sources

- **Context7 /image-rs/image** — Image resize, thumbnail, format support
- **Context7 /awslabs/aws-sdk-rust** — S3 upload, ByteStream API
- **Context7 /websites/rs_axum** — Multipart handling, WebSocket support
- **Context7 /react-dropzone/react-dropzone** — File upload hooks, validation
- **Context7 /statelyai/xstate** — State machine patterns, TypeScript integration
- **docs.rs/statig** — Hierarchical state machines in Rust
- **min.io/docs** — MinIO Kubernetes deployment, S3 compatibility

---

*Stack research for: Activity Feed & Site Status (v1.8)*
*Researched: 2026-05-01*
