# Feature Landscape

**Domain:** Construction Management SaaS (Schreinerei)
**Researched:** 2026-05-01
**Context:** Subsequent milestone - Adding activity feed, site status workflow, and file attachments to existing app

## Table Stakes

Features users expect in construction management apps. Missing = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Activity Feed** | Project history is fundamental. Teams need to see what happened on site. | Medium | Already partially built (photos, notes). Need tabbed UI and file attachments. |
| **Status Badges** | Visual progress indication is standard in all PM tools. | Low | Already implemented. Need modal for status changes. |
| **Status Transitions** | Controlled state machine prevents errors (can't complete before starting). | Low | State machine already exists in backend. Need UI integration. |
| **Material Tracking** | Construction teams expect to know where materials went. | Medium | Already has site_id link in StockEntry. Need display in feed. |
| **Photo Upload** | Mobile-first apps must support camera capture. | Medium | Already has photo_url field. Need upload UI and storage. |
| **Time Tracking** | Every construction app tracks hours. | Low | Already built and working. |
| **User Attribution** | Teams need to know who did what. | Low | Already has user_id on activities and stock entries. |
| **Relative Timestamps** | "vor 2 Stunden" is expected UX pattern. | Low | Already implemented in ActivityFeed component. |
| **Offline Support** | Construction sites often have poor connectivity. | High | PWA with IndexedDB already exists. Files need offline strategy. |

## Differentiators

Features that set the product apart. Not expected, but valued.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Unified Activity Feed** | One place to see notes, documents, photos, material usage, and status changes. Most tools fragment this across tabs. | Medium | Competitive advantage: simplicity. |
| **Material Context in Feed** | Link material withdrawals to specific sites with category context. Shows "5x Schrauben (Befestigungsmaterial) → Baustelle Müller". | Low | Differentiator: most tools separate inventory from project views. |
| **Offline File Capture** | Capture photos on site without connectivity, auto-sync when online. | High | Critical for field workers. Most competitors are online-only. |
| **Status Change Audit Trail** | Automatic activity entries when status changes. "Max Mustermann setzte Status auf 'Aktiv'". | Low | Transparency differentiator. |
| **Category Context** | Show material category in feed entries. Helps understand what was used at a glance. | Low | Small UX improvement, high value. |
| **Hash-Based Colors** | Deterministic colors for sites based on name hash. Visual recognition without configuration. | Low | Already implemented in v1.7. Polish differentiator. |
| **Auto-Prefill from Active Site** | New entries automatically link to user's active Baustelle. Reduces friction. | Low | Already implemented in v1.7. UX differentiator. |

## Anti-Features

Features to explicitly NOT build.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Reversible Status Transitions** | Business logic complexity. Can't "un-complete" a site without audit implications. | Allow only forward transitions (planned→active→completed→archived). Already enforced by backend. |
| **Rich Text Editor for Notes** | Overkill for field notes. Adds complexity, storage, and mobile UX issues. | Plain text with optional photos. Simple, fast, offline-friendly. |
| **Document Version Control** | Not a document management system. Over-engineering for the use case. | Simple file attachments with upload date. Replace (not version) if needed. |
| **Real-Time Activity Sync** | WebSocket complexity for small teams. Not critical. | Pull-to-refresh and background polling (already implemented). |
| **Comment Threads on Activities** | Adds notification complexity, threading, permissions. Keep it simple. | Notes are standalone. Use separate notes for follow-up. |
| **File Preview Generation** | Server-side preview generation adds infrastructure complexity. | Client-side preview for images. Documents download directly. |
| **External File Storage Links** | Google Drive/Dropbox integration adds OAuth, permissions, sync complexity. | Local file storage (S3 or filesystem) owned by the app. |
| **Activity Filtering by User/Type** | Small team size doesn't justify. Creates empty states. | Simple chronological feed. Search if needed later. |

## Feature Dependencies

```
Site Status Workflow
├── Depends on: Site entity (✓ exists)
├── Depends on: SiteStatus enum (✓ exists)
├── Depends on: State machine validation (✓ exists)
└── Produces: Status change activities

Activity Feed Enhancement
├── Depends on: Activity entity (✓ exists)
├── Depends on: ActivityFeed component (✓ exists)
├── Depends on: Site detail page (✓ exists)
├── New: File attachment storage
├── New: Document upload UI
└── Produces: Tabbed feed view

Material Extraction History
├── Depends on: StockEntry entity (✓ exists)
├── Depends on: StockEntryWithSite (✓ exists)
├── Depends on: Site link in withdrawals (✓ exists)
├── New: Category display in feed
└── Produces: Material activities in feed

File Attachments
├── New: Storage backend (S3/filesystem)
├── New: Upload API endpoints
├── New: File entity/table
├── New: Offline queue for uploads
└── Produces: Attachment URLs in activities
```

## Existing Codebase Analysis

### Already Built (Strong Foundation)

| Component | Status | Location |
|-----------|--------|----------|
| Activity entity | ✅ Complete | `src/modules/sites/domain/activity.rs` |
| ActivityFeed component | ✅ Complete | `frontend/src/pages/sites/ActivityFeed.tsx` |
| Site status state machine | ✅ Complete | `src/modules/sites/domain/site.rs:25-37` |
| Status transitions validation | ✅ Complete | `can_transition_to()` with tests |
| StockEntry with site_id | ✅ Complete | `src/modules/inventory/domain/stock_entry.rs` |
| StockEntryWithSite | ✅ Complete | Resolves site_name for display |
| WithdrawMaterial command | ✅ Complete | Has optional site_id field |
| ActivityType enum | ✅ Complete | Photo, Note, StatusChange |
| Time tracking | ✅ Complete | Working with UI |
| Active site indicator | ✅ Complete | v1.7 with hash-based colors |
| Auto-prefill | ✅ Complete | v1.7 user preferences |

### Gap Analysis

| Gap | Impact | Effort |
|-----|--------|--------|
| File attachment storage | HIGH - Required for photos/documents | Medium - S3 integration or filesystem |
| Document tab in feed | MEDIUM - UX completeness | Low - New tab component |
| Status change modal | MEDIUM - Required for workflow UI | Low - Dialog component |
| Status change activities | LOW - Audit trail | Low - Hook into status update |
| Category in material feed | LOW - UX enhancement | Low - Join query |
| "Auswählen" button rename | LOW - UX clarity | Trivial - Copy change |

## MVP Recommendation

Prioritize for v1.8:

1. **Status Change Modal** (Low complexity, high visibility)
   - Dialog with current status
   - Show valid next states only
   - Confirmation for destructive changes
   - Auto-create activity entry

2. **Tabbed Activity Feed** (Low complexity, organizes content)
   - "Notizen/Dokumente" tab (existing activities + documents)
   - "Material" tab (stock entries linked to site)
   - Preserve existing ActivityFeed component

3. **Material History Tab** (Low complexity, leverages existing data)
   - Query StockEntryWithSite by site_id
   - Show: date, material name, category, quantity, user
   - Link to material detail page

4. **Photo Upload** (Medium complexity, critical for field use)
   - Camera capture on mobile
   - File selection fallback
   - Upload to backend (storage decision needed)
   - Offline queue for pending uploads

Defer:
- **Document attachments**: Start with photos only. Documents add preview complexity.
- **Category optimization**: Simple join query is sufficient for now.

## Storage Decision Required

File attachments need a storage backend. Options:

| Option | Pros | Cons | Recommendation |
|--------|------|------|----------------|
| Local filesystem | Simple, no cost | Not scalable, no CDN | ❌ Not for production |
| S3-compatible (AWS/MinIO) | Scalable, CDN, existing pattern | Cost, infrastructure | ✅ Recommended |
| Database BLOB | Simple queries, transactional | DB bloat, no CDN, poor for files | ❌ Anti-pattern |

**Recommendation:** S3-compatible storage (AWS S3 or self-hosted MinIO for the existing Kubernetes cluster).

## Sources

- **Codebase Analysis**: Existing Rust backend and React frontend (HIGH confidence - direct inspection)
- **MDN File API**: File upload patterns, object URLs for previews (HIGH confidence - official docs)
- **Buildertrend Features**: Construction PM feature patterns (MEDIUM confidence - competitor analysis)
- **Procore Library**: Construction industry context (LOW confidence - marketing content, not technical docs)

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Existing codebase | HIGH | Direct code inspection |
| Status workflow patterns | HIGH | State machine already implemented |
| Activity feed patterns | MEDIUM | Component exists, needs enhancement |
| File upload patterns | MEDIUM | Standard web patterns, need storage decision |
| Material tracking | HIGH | Already built with site links |
| Construction industry norms | LOW | Limited to competitor feature pages |
