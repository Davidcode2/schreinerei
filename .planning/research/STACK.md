# Stack Research: Active Project Context

**Domain:** User-scoped active Baustelle state with auto-assignment
**Researched:** 2026-04-30
**Confidence:** HIGH

## Summary

The Active Project Context feature requires **NO new libraries**. All capabilities are already present in the existing stack:

- **Zustand with persist middleware** — Already used for auth, same pattern for active site
- **Tailwind CSS color palette** — Deterministic color assignment from site ID hash
- **TanStack Query** — Already used for API calls and cache invalidation
- **Backend stack** — Axum 0.8, SQLx 0.8 already support the needed endpoints

---

## Recommended Stack

### Frontend State Management

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Zustand | 5.0.12 | Active site state store | Already in use for auth with `persist` middleware. Same pattern applies. |
| Zustand persist | (built-in) | LocalStorage persistence | Built into Zustand, used by authStore.ts. Survives page refresh. |

### Color Generation

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Tailwind CSS colors | 4.2.4 (existing) | Deterministic site colors | Use predefined palette (rose, orange, amber, emerald, teal, cyan, blue, indigo, violet). Hash site ID → index into palette. No new library. |

### Backend

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Axum | 0.8 | API endpoints | Already in use. Add GET/PUT `/api/v1/users/me/active-site` |
| SQLx | 0.8 | Database queries | Already in use. Add migration for user preferences |
| ts-rs | 12 | Type generation | Already in use. Add DTOs for active site preference |

---

## No New Dependencies Required

**The feature can be implemented entirely with existing libraries:**

```bash
# No npm install needed
# No cargo add needed
```

---

## Implementation Components

### 1. Frontend: Active Site Store

Create a new Zustand store following the existing `authStore.ts` pattern:

```typescript
// frontend/src/lib/stores/activeSiteStore.ts
import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface ActiveSiteState {
  activeSiteId: string | null
  setActiveSite: (id: string | null) => void
}

export const useActiveSiteStore = create<ActiveSiteState>()(
  persist(
    (set) => ({
      activeSiteId: null,
      setActiveSite: (id) => set({ activeSiteId: id }),
    }),
    {
      name: 'active-site-storage',
      partialize: (state) => ({
        activeSiteId: state.activeSiteId,
      }),
    }
  )
)
```

### 2. Frontend: Color Generation

Use a simple hash function with Tailwind's color palette:

```typescript
// frontend/src/lib/utils/siteColor.ts

// Tailwind color palette indices (matching Tailwind 500 shades for good contrast)
const SITE_COLORS = [
  'bg-rose-500',
  'bg-orange-500', 
  'bg-amber-500',
  'bg-emerald-500',
  'bg-teal-500',
  'bg-cyan-500',
  'bg-blue-500',
  'bg-indigo-500',
  'bg-violet-500',
] as const

// Deterministic color from UUID
export function getSiteColor(siteId: string): string {
  // Simple hash: sum char codes, mod length
  const hash = siteId.split('').reduce((sum, char) => sum + char.charCodeAt(0), 0)
  return SITE_COLORS[hash % SITE_COLORS.length]
}

// Also export text variants for accessibility
export function getSiteColorText(siteId: string): string {
  return getSiteColor(siteId).replace('bg-', 'text-')
}
```

### 3. Frontend: Active Site Status Component

```typescript
// frontend/src/components/active-site/ActiveSiteIndicator.tsx
import { useActiveSiteStore } from '@/lib/stores/activeSiteStore'
import { useSite } from '@/lib/api/hooks/useSites'
import { Building2, X } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { getSiteColor } from '@/lib/utils/siteColor'

export function ActiveSiteIndicator() {
  const { activeSiteId, setActiveSite } = useActiveSiteStore()
  const { data: site } = useSite(activeSiteId || '')

  if (!activeSiteId || !site) return null

  return (
    <div className="flex items-center gap-2 p-2 bg-muted rounded-lg">
      <div className={`w-3 h-3 rounded-full ${getSiteColor(site.id)}`} />
      <Building2 className="h-4 w-4" />
      <span className="font-medium truncate">{site.name}</span>
      <Badge variant="outline" className="text-xs">Aktiv</Badge>
      <button onClick={() => setActiveSite(null)} className="ml-auto">
        <X className="h-4 w-4" />
      </button>
    </div>
  )
}
```

### 4. Backend: Migration

```sql
-- migrations/011_add_user_active_site.sql
ALTER TABLE users ADD COLUMN active_site_id UUID REFERENCES sites(id) ON DELETE SET NULL;
CREATE INDEX idx_users_active_site ON users(active_site_id);
```

### 5. Backend: API Endpoint

```rust
// Add to src/modules/iam/application/user_service.rs

pub async fn get_active_site(
    user_id: UserId,
    repo: &dyn UserRepository,
) -> Result<Option<SiteId>, IamError> {
    repo.get_active_site(user_id).await
}

pub async fn set_active_site(
    user_id: UserId,
    site_id: Option<SiteId>,
    repo: &dyn UserRepository,
) -> Result<(), IamError> {
    repo.set_active_site(user_id, site_id).await
}
```

### 6. Frontend: Auto-Assignment Hook

```typescript
// frontend/src/hooks/useAutoAssignSite.ts
import { useActiveSiteStore } from '@/lib/stores/activeSiteStore'
import { useSite } from '@/lib/api/hooks/useSites'

export function useAutoAssignSite() {
  const { activeSiteId } = useActiveSiteStore()
  const { data: activeSite } = useSite(activeSiteId || '')

  return {
    activeSiteId,
    activeSite,
    shouldAutoAssign: !!activeSiteId,
  }
}
```

---

## Integration Points

### Material Withdrawal

```typescript
// In WithdrawDialog.tsx
const { activeSiteId, shouldAutoAssign } = useAutoAssignSite()
const [showConfirmDialog, setShowConfirmDialog] = useState(false)

// Pre-fill notes with active site
const defaultNotes = shouldAutoAssign 
  ? `Baustelle: ${activeSite?.name}` 
  : ''

// On submit, show opt-out dialog if auto-assigning
const handleSubmit = () => {
  if (shouldAutoAssign) {
    setShowConfirmDialog(true)
  } else {
    onConfirm(quantity, notes)
  }
}
```

### Tool Reservation

```typescript
// In ReservationDialog.tsx
const { activeSiteId } = useActiveSiteStore()

// Pre-select site in dropdown if active
useEffect(() => {
  if (mode === 'create' && activeSiteId && !siteId) {
    setSiteId(activeSiteId)
  }
}, [mode, activeSiteId])
```

---

## Alternatives Considered

| Recommended | Alternative | Why Not |
|-------------|-------------|---------|
| Zustand persist | React Context | Zustand already in use, Context requires more boilerplate for persistence |
| Zustand persist | localStorage directly | Zustand handles serialization, hydration, and subscriptions automatically |
| Tailwind colors | color2k library | No need for color manipulation, just palette selection |
| Tailwind colors | tinycolor2 | Heavier (5kB vs 0kB), overkill for simple palette assignment |
| Deterministic hash | Random color | Colors must be consistent across page loads and devices |
| localStorage only | Backend preference | Cross-device sync not required for v1.7, can add later |

---

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Redux | Overkill for single value state | Zustand (already in use) |
| IndexedDB for active site | Auth uses localStorage, consistency matters | Zustand persist (localStorage) |
| Color generation libraries | Palette selection is trivial | Hash function + Tailwind colors |
| Server-sent events | Real-time not needed for user preference | TanStack Query cache invalidation |
| Complex state machines | Two states: null or site ID | Simple Zustand store |

---

## Version Compatibility

All existing versions are compatible:

| Package | Version | Compatible With | Notes |
|---------|---------|-----------------|-------|
| zustand | 5.0.12 | React 19.2.5 | Already in use |
| @tanstack/react-query | 5.100.6 | React 19.2.5 | Already in use |
| tailwindcss | 4.2.4 | Vite 7.3.2 | Already in use |
| axum | 0.8 | tokio 1, sqlx 0.8 | Already in use |
| sqlx | 0.8 | PostgreSQL | Already in use |

---

## Database Schema Addition

```sql
-- Single column addition to users table
ALTER TABLE users ADD COLUMN active_site_id UUID REFERENCES sites(id) ON DELETE SET NULL;
```

**Alternative (user_preferences table):**
```sql
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    active_site_id UUID REFERENCES sites(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Recommendation:** Single column on `users` table is simpler for v1.7. Can migrate to separate table later if preferences grow.

---

## Sources

- **Zustand docs** — Context7 `/pmndrs/zustand` — persist middleware with custom storage
- **Existing codebase** — `frontend/src/lib/auth/authStore.ts` — Pattern for persisted Zustand store
- **Tailwind CSS colors** — https://tailwindcss.com/docs/customizing-colors — Default color palette
- **color2k comparison** — https://github.com/ricokahler/color2k — Not needed for this use case

---

## Confidence Assessment

| Area | Level | Reason |
|------|-------|--------|
| State Management | HIGH | Zustand persist already used for auth, identical pattern |
| Color Generation | HIGH | Hash + palette is trivial, no library needed |
| Backend Integration | HIGH | Standard Axum + SQLx patterns already established |
| Auto-assignment UI | HIGH | Dialog patterns exist, opt-out is simple countdown |

---

*Stack research for: Active Project Context (v1.7)*
*Researched: 2026-04-30*
