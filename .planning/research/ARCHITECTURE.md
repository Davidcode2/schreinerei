# Architecture Research: Active Project Context

**Domain:** Construction site management SaaS (Schreinerei)
**Feature:** Active Project (Baustelle) Context
**Researched:** 2026-04-30
**Confidence:** HIGH

## Current Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (React PWA)                      │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────────────┐│
│  │   IAM   │  │Inventory│  │  Sites  │  │     Fleet       ││
│  │ Pages   │  │  Pages  │  │  Pages  │  │     Pages       ││
│  └────┬────┘  └────┬────┘  └────┬────┘  └────────┬────────┘│
│       │            │            │                 │         │
│  ┌────┴────────────┴────────────┴─────────────────┴───────┐│
│  │              React Query + API Client                   ││
│  └────────────────────────┬────────────────────────────────┘│
│                           │                                  │
│  ┌────────────────────────┴────────────────────────────────┐│
│  │            Offline Layer (Dexie/IndexedDB)              ││
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  ││
│  │  │ Cached Data │  │ Sync Queue  │  │  Pending Actions│  ││
│  │  └─────────────┘  └─────────────┘  └─────────────────┘  ││
│  └─────────────────────────────────────────────────────────┘│
└───────────────────────────────┬─────────────────────────────┘
                                │ HTTP/REST
                                ▼
┌─────────────────────────────────────────────────────────────┐
│                    Backend (Rust/Axum)                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Auth Layer (JWT/Keycloak)               │    │
│  │  ┌───────────────────────────────────────────────┐  │    │
│  │  │  AuthenticatedUser { user_id, tenant_id, ... }│  │    │
│  │  └───────────────────────────────────────────────┘  │    │
│  └───────────────────────────┬─────────────────────────┘    │
│                              │                               │
│  ┌───────────────────────────┴─────────────────────────┐    │
│  │              Application Services                    │    │
│  │  ┌─────────┐  ┌───────────┐  ┌───────┐  ┌─────────┐ │    │
│  │  │ UserService│InventorySvc│ │SiteSvc│  │FleetSvc │ │    │
│  │  └────┬────┘  └─────┬─────┘  └───┬───┘  └────┬────┘ │    │
│  │       │            │            │            │       │    │
│  │       └────────────┴────────────┴────────────┘       │    │
│  │                      │                               │    │
│  │               TenantContext                          │    │
│  └───────────────────────┴─────────────────────────────┘    │
│                              │                               │
│  ┌───────────────────────────┴─────────────────────────┐    │
│  │              Domain Layer (Pure Business Logic)      │    │
│  │  ┌─────────┐  ┌───────────┐  ┌───────┐  ┌─────────┐ │    │
│  │  │  User   │  │  Material │  │ Site  │  │Reservation│ │   │
│  │  └─────────┘  └───────────┘  └───────┘  └─────────┘ │    │
│  └─────────────────────────────────────────────────────┘    │
│                              │                               │
│  ┌───────────────────────────┴─────────────────────────┐    │
│  │              Infrastructure Layer                    │    │
│  │  ┌─────────────┐  ┌────────────────────────────┐    │    │
│  │  │ Repositories│  │     PostgreSQL Database    │    │    │
│  │  └─────────────┘  └────────────────────────────┘    │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Existing Integration Points

### 1. Reservation → Site (Already Exists)

**Location:** `src/modules/fleet/domain/reservation.rs:17`

```rust
pub struct Reservation {
    pub site_id: Option<SiteId>,  // ← Already supports site association
    // ...
}
```

**Status:** ✅ No changes needed. Auto-assignment can set this field.

### 2. Stock Withdrawal → Site (MISSING)

**Current state:** `src/modules/inventory/domain/material.rs:68-74`

```rust
pub struct WithdrawMaterial {
    pub material_id: MaterialId,
    pub quantity: i32,
    pub notes: Option<String>,
    // NO site_id field!
}
```

**Database:** `stock_entries` table has no `site_id` column.

**Required changes:**
- Add `site_id: Option<SiteId>` to `WithdrawMaterial` command
- Add migration for `stock_entries.site_id` column
- Update `withdraw_stock()` in repository

### 3. TenantContext (Request Context)

**Location:** `src/modules/iam/application/user_service.rs:8-14`

```rust
pub struct TenantContext {
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub email: String,
    pub roles: Vec<Role>,
    // NO active_site_id!
}
```

**Decision:** Do NOT add `active_site_id` to TenantContext. Active site is **user preference**, not request context. It should be stored separately and injected by services.

### 4. Offline Sync Queue

**Location:** `frontend/src/lib/offline/queue.ts:7-43`

```typescript
const actionHandlers = {
  withdraw: async (data) => { /* materialId, quantity, notes */ },
  reservation: async (data) => { /* resourceType, resourceId, siteId, ... */ }
}
```

**Required changes:**
- Inject `activeSiteId` into pending action data when queued
- Already supports siteId for reservations

## Proposed Architecture for Active Project Context

### New Component: UserPreferences (IAM Module)

```
src/modules/iam/
├── domain/
│   ├── user.rs           (existing)
│   └── user_preferences.rs  ← NEW
├── application/
│   ├── user_service.rs   (existing)
│   └── preferences_service.rs  ← NEW
└── infrastructure/
    ├── user_repository.rs     (existing)
    └── preferences_repository.rs  ← NEW
```

### Domain Model: UserPreferences

```rust
// src/modules/iam/domain/user_preferences.rs
pub struct UserPreferences {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub active_site_id: Option<SiteId>,
    pub updated_at: DateTime<Utc>,
}

pub struct SetActiveSite {
    pub site_id: Option<SiteId>,
}
```

### Database Schema

```sql
-- New table in IAM module
CREATE TABLE user_preferences (
    user_id UUID NOT NULL,
    tenant_id UUID NOT NULL,
    active_site_id UUID NULL REFERENCES sites(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, tenant_id)
);

-- Add site_id to stock_entries (migration)
ALTER TABLE stock_entries 
ADD COLUMN site_id UUID NULL REFERENCES sites(id);
```

### API Endpoints

```
GET  /api/v1/preferences              → Get current user preferences
PATCH /api/v1/preferences/active-site → Set active site (or clear)
```

### Frontend State Management

**New Zustand Store:** `frontend/src/stores/preferencesStore.ts`

```typescript
interface PreferencesState {
  activeSiteId: string | null;
  activeSite: Site | null;  // Cached site details
  setActiveSite: (siteId: string | null) => void;
  clearActiveSite: () => void;
}
```

**Persistence Strategy:**
1. **Primary:** Backend `user_preferences` table
2. **Cache:** Zustand with localStorage persistence
3. **Offline:** IndexedDB sync (already exists)

### Data Flow for Auto-Assignment

```
User sets active Baustelle
         │
         ▼
┌─────────────────────────────────────┐
│  Frontend: preferencesStore         │
│  - Update Zustand state             │
│  - Persist to localStorage          │
│  - POST /api/v1/preferences/...     │
└─────────────────┬───────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│  Backend: PreferencesService        │
│  - Validate site belongs to tenant  │
│  - Store in user_preferences table  │
└─────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│  Frontend: Cached for subsequent    │
│  - Inject into WithdrawDialog       │
│  - Inject into reservation forms    │
│  - Queue offline actions with site  │
└─────────────────────────────────────┘
```

### Integration Points Summary

| Component | Current State | Required Change |
|-----------|---------------|-----------------|
| Reservation | ✅ Has `site_id` | Auto-fill from active site |
| WithdrawMaterial | ❌ No `site_id` | Add optional field |
| stock_entries table | ❌ No `site_id` | Add nullable column |
| TenantContext | ❌ Not for preferences | Don't modify |
| User entity | ❌ No preferences | Keep separate |
| Offline queue | ✅ Supports data | Inject siteId |

## Recommended Build Order

### Phase 1: Backend Foundation
1. **Add UserPreferences domain model** (IAM module)
   - Create `user_preferences.rs` in domain
   - Add `SetActiveSite` command with validation
   - Unit tests for domain logic

2. **Create database migration**
   - `user_preferences` table
   - Add `site_id` to `stock_entries`

3. **Implement PreferencesRepository**
   - CRUD for user preferences
   - Tenant-scoped queries

4. **Create PreferencesService**
   - `get_preferences(ctx)`
   - `set_active_site(site_id, ctx)` with validation

5. **Add API routes**
   - GET `/preferences`
   - PATCH `/preferences/active-site`

### Phase 2: Frontend Integration
6. **Create preferencesStore** (Zustand + persist)
   - activeSiteId state
   - setActiveSite/clearActiveSite actions
   - Sync with backend on change

7. **Add ActiveSiteIndicator component**
   - Persistent UI element (top bar or sidebar)
   - Shows current active Baustelle with color
   - Click to change

8. **Create ActiveSiteSelector component**
   - Used on overview page and dashboard
   - Lists active Baustellen (status='active')
   - Clear option

### Phase 3: Auto-Assignment
9. **Modify WithdrawDialog**
   - Read activeSiteId from store
   - Show opt-out checkbox/timer
   - Include site_id in withdrawal request

10. **Modify reservation forms**
    - Auto-fill site_id from active site
    - Allow override

11. **Update offline queue**
    - Inject activeSiteId when queueing actions
    - Handle case where active site changed before sync

### Phase 4: Polish
12. **Add Baustelle colors**
    - Generate consistent color per site (hash-based)
    - Store in Site entity or generate client-side

13. **Opt-out dialog implementation**
    - 5-second auto-confirm timer
    - Change/dismiss options
    - Unobtrusive design

## Architectural Patterns

### Pattern 1: User Preferences as Separate Aggregate

**What:** Store user preferences in a separate table/entity, not in User aggregate.

**Why:**
- Preferences change independently of user identity
- Different lifecycle (preferences can be cleared)
- Follows Single Responsibility Principle
- Avoids bloating User aggregate

**Example:**
```rust
// DON'T add to User
pub struct User {
    // ... identity fields
    pub active_site_id: Option<SiteId>, // ❌ Wrong place
}

// DO create separate aggregate
pub struct UserPreferences {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub active_site_id: Option<SiteId>, // ✅ Right place
}
```

### Pattern 2: Frontend State with Backend Sync

**What:** Maintain frontend state independently with eventual consistency to backend.

**Why:**
- Immediate UI feedback (no loading states)
- Works offline
- Syncs when online

**Example:**
```typescript
const usePreferencesStore = create<PreferencesState>()(
  persist(
    (set, get) => ({
      activeSiteId: null,
      setActiveSite: async (siteId) => {
        set({ activeSiteId: siteId }); // Immediate update
        await api.patch('/preferences/active-site', { site_id: siteId }); // Sync
      },
    }),
    { name: 'preferences-storage' }
  )
);
```

### Pattern 3: Opt-Out with Timeout

**What:** Show confirmation dialog with auto-confirm countdown instead of blocking.

**Why:**
- Reduces friction for common workflow
- Still allows override when needed
- Non-modal, non-blocking

**Example:**
```tsx
function WithdrawDialog({ activeSite }) {
  const [showOptOut, setShowOptOut] = useState(!!activeSite);
  
  // 5-second countdown, then auto-confirm
  useEffect(() => {
    if (showOptOut) {
      const timer = setTimeout(() => confirmWithSite(), 5000);
      return () => clearTimeout(timer);
    }
  }, [showOptOut]);
  
  return (
    <Dialog>
      {showOptOut && (
        <div className="opt-out-banner">
          Material wird Baustelle "{activeSite.name}" zugeordnet
          <button onClick={() => setShowOptOut(false)}>Ändern</button>
          <span className="countdown">5s</span>
        </div>
      )}
    </Dialog>
  );
}
```

## Anti-Patterns to Avoid

### Anti-Pattern 1: Adding Active Site to TenantContext

**What people do:** Add `active_site_id` to the TenantContext struct passed to all services.

**Why it's wrong:**
- TenantContext represents request authentication context
- Active site is user preference, not request metadata
- Would require modifying all service signatures
- Mixes concerns (auth vs preferences)

**Do this instead:** Create separate `UserPreferences` aggregate in IAM module with its own service.

### Anti-Pattern 2: Frontend-Only Storage

**What people do:** Store active site only in localStorage or IndexedDB.

**Why it's wrong:**
- Doesn't persist across devices
- Lost on logout/cache clear
- Cannot enforce business rules
- No audit trail

**Do this instead:** Store in backend `user_preferences` table, cache locally for offline.

### Anti-Pattern 3: Blocking Modal on Every Action

**What people do:** Show a modal asking "Assign to Baustelle X?" every time user withdraws material.

**Why it's wrong:**
- Extremely annoying for common workflow
- Slows down repetitive tasks
- Users will just click "yes" without reading

**Do this instead:** Use opt-out banner with auto-confirm timeout. Show current assignment, allow change.

## Offline Considerations

### Scenario 1: Set Active Site Offline

```
User opens app offline
  → Loads activeSiteId from localStorage/Zustand
  → Can still use auto-assignment
  → Change to active site queued for sync
  → Synced when back online
```

### Scenario 2: Withdraw Material Offline

```
User withdraws material offline
  → activeSiteId from local state
  → Queued action includes site_id
  → Synced when online
  → If site was deleted remotely, error shown during sync
```

### Scenario 3: Active Site Changed Before Sync

```
User withdraws material (site A) offline
User changes active site to B
User goes online
  → Withdrawal action syncs with site A (as queued)
  → Active site preference updates to B
  → No conflict (different data)
```

### IndexedDB Updates

**Current:** `frontend/src/lib/offline/db.ts`

Add new table for preferences cache:

```typescript
this.version(2).stores({
  // ... existing tables
  preferences: 'id'  // Single row for current user preferences
});

export interface CachedPreferences {
  id: 'current';
  activeSiteId: string | null;
  cachedAt: Date;
}
```

## Sources

- Project source code analysis (HIGH confidence)
- Existing domain models: `src/modules/*/domain/*.rs`
- Frontend state patterns: `frontend/src/lib/auth/authStore.ts`
- Offline sync: `frontend/src/lib/offline/queue.ts`, `frontend/src/lib/offline/db.ts`
- Repository patterns: `src/modules/*/infrastructure/*_repository.rs`

---
*Architecture research for: Active Project Context (v1.7)*
*Researched: 2026-04-30*
