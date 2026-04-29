# Phase 5: PWA & Mobile — Context & Decisions

**Phase:** 5 — PWA & Mobile
**Status:** Ready to Plan
**Requirements:** PWA-01, PWA-02, PWA-03, PWA-04

---

## Overview

This is the **final phase** before V1 milestone. All backend modules are complete (IAM, Inventory, Sites, Fleet). The frontend does **not exist** yet — this phase builds it from scratch as a Progressive Web App.

---

## Backend API Summary

The Rust backend provides a complete REST API:

### Authentication (Keycloak)
- `GET /api/v1/auth/me` — Current user profile
- `PATCH /api/v1/users/me` — Update own profile
- `GET /api/v1/users` — List users (admin)
- `POST /api/v1/users/invite` — Invite user (admin)
- `PATCH /api/v1/users/{id}/role` — Update role (admin)

### Inventory Module
- `GET|POST /api/v1/inventory/categories`
- `GET /api/v1/inventory/categories/{id}`
- `GET|POST /api/v1/inventory/materials`
- `GET /api/v1/inventory/materials/{id}`
- `POST /api/v1/inventory/materials/{id}/withdraw`
- `POST /api/v1/inventory/materials/{id}/adjust`
- `POST /api/v1/inventory/materials/{id}/qr` — Generate QR code
- `GET /api/v1/inventory/materials/{id}/qr/svg` — Get QR SVG
- `GET /api/v1/inventory/low-stock`
- `GET /api/v1/inventory/qr/{code}` — Lookup by QR code
- `GET|POST /api/v1/inventory/orders` — Order requests
- `POST /api/v1/inventory/orders/{id}/approve`
- `POST /api/v1/inventory/orders/{id}/fulfill`

### Sites Module
- `GET|POST /api/v1/sites`
- `GET|PATCH /api/v1/sites/{id}`
- `POST /api/v1/sites/{id}/assign`
- `DELETE /api/v1/sites/{id}/assign/{user_id}`
- `GET /api/v1/sites/{id}/assignments`
- `GET /api/v1/sites/{id}/time-entries`
- `GET|POST /api/v1/time-entries`
- `GET /api/v1/time-entries/my`
- `GET|POST /api/v1/sites/{id}/activities`
- `GET /api/v1/dashboard/sites`

### Fleet Module
- `GET|POST /api/v1/fleet/vehicles`
- `GET|PATCH|DELETE /api/v1/fleet/vehicles/{id}`
- `GET|POST /api/v1/fleet/tools`
- `GET|PATCH|DELETE /api/v1/fleet/tools/{id}`
- `GET|POST /api/v1/fleet/reservations`
- `GET /api/v1/fleet/reservations/my`
- `GET|PATCH|DELETE /api/v1/fleet/reservations/{id}`
- `GET /api/v1/fleet/calendar`
- `GET /api/v1/fleet/availability`
- `GET /api/v1/fleet/qr/{code}` — Status by QR code

---

## Key Decisions for Discussion

### D-01: Frontend Framework & Build Tool

**Options:**
- A: **Vite + React** (PROJECT.md default)
- B: Next.js (SSR, more complex)
- C: SvelteKit (lighter, but different ecosystem)

**Recommendation:** Vite + React
- Already documented in PROJECT.md
- Fast dev server, optimized builds
- React ecosystem for component libraries
- Works well with PWA plugins

**Question:** Should we use React 18 with concurrent features, or stick to stable patterns?

---

### D-02: State Management Strategy

**Options:**
- A: **React Query + Zustand** (server state + client state)
- B: Redux Toolkit (traditional, more boilerplate)
- C: Jotai/Recoil (atomic state)

**Recommendation:** React Query + Zustand
- React Query handles API caching, background refetch, optimistic updates
- Zustand for simple client state (auth, UI preferences)
- Minimal boilerplate, excellent DX
- Works well with offline-first patterns

**Critical for offline:** React Query's `useMutation` with `onSettled` for sync queue

---

### D-03: UI Component Library

**Options:**
- A: **Tailwind CSS + Headless UI** (utility-first, accessible)
- B: shadcn/ui (Tailwind + Radix primitives)
- C: MUI/Mantine (full component library)
- D: Custom components with Tailwind

**Recommendation:** Tailwind CSS + shadcn/ui
- shadcn/ui provides copy-paste components (not a dependency)
- Accessible by default (Radix primitives)
- Fully customizable with Tailwind
- Small bundle size (only what you use)

**Responsive design:** Tailwind's `sm:`, `md:`, `lg:` breakpoints cover tablet/phone

---

### D-04: PWA Implementation Strategy

**Core Requirements:**
- Service Worker for offline caching
- Web App Manifest for installability
- IndexedDB for offline data storage
- Background sync for pending mutations

**Service Worker Approach:**
- A: **Workbox** (Google's PWA library)
- B: Vite PWA Plugin (wraps Workbox)
- C: Custom service worker

**Recommendation:** Vite PWA Plugin + Workbox
- `vite-plugin-pwa` handles manifest + service worker generation
- Workbox strategies for runtime caching
- Auto-updates service worker on build

**Offline Data Strategy:**
```
Online Flow:
  User action → API call → Update React Query cache → Sync to IndexedDB

Offline Flow:
  User action → Queue in IndexedDB → Update React Query cache optimistically
  On reconnect → Process queue → Sync with server
```

---

### D-05: Authentication Flow

**Keycloak Integration:**
- Backend validates JWT tokens
- Frontend needs to obtain tokens from Keycloak

**Options:**
- A: **OAuth2 Authorization Code + PKCE** (redirect flow)
- B: Keycloak JS adapter
- C: Implicit flow (deprecated)

**Recommendation:** OAuth2 Authorization Code + PKCE
- Most secure for SPAs
- Keycloak supports PKCE natively
- Token refresh handled by interceptor

**Implementation:**
1. User visits app → Redirect to Keycloak login
2. Keycloak redirects back with auth code
3. Frontend exchanges code for tokens
4. Store tokens in memory (not localStorage for security)
5. Refresh token rotation via silent refresh

---

### D-06: QR Code Scanner

**Options:**
- A: **jsQR** (pure JS, lightweight)
- B: QuaggaJS (more features, larger)
- C: ZXing browser library

**Recommendation:** jsQR or html5-qrcode
- `html5-qrcode` is more maintained and handles camera permissions well
- Works on mobile browsers (iOS Safari, Chrome)
- Fallback: Upload photo for QR detection

**Camera Access:**
```
1. Request camera permission
2. Stream video to <video> element
3. Capture frames to canvas
4. Decode QR from canvas imageData
5. Handle result (navigate to resource)
```

---

### D-07: Responsive Design Breakpoints

**Target Devices:**
- Smartphone: 320px - 480px (portrait)
- Tablet: 768px - 1024px
- Desktop: 1024px+

**Tailwind Breakpoints:**
```css
sm: 640px   /* Small tablets */
md: 768px   /* Tablets */
lg: 1024px  /* Laptops */
xl: 1280px  /* Desktops */
2xl: 1536px /* Large screens */
```

**Design Patterns:**
- Mobile-first CSS (default styles for mobile)
- Bottom navigation on mobile
- Sidebar navigation on tablet/desktop
- Touch-friendly tap targets (min 44px)
- Swipe gestures for common actions

---

### D-08: Offline-First Data Architecture

**Entities to Cache Offline:**
| Entity | Cache Duration | Offline Create? |
|--------|---------------|-----------------|
| User profile | Session | No |
| Sites | 24h | No (admin only) |
| Time entries | Sync immediately | **Yes** |
| Materials | 1h | No |
| Withdrawals | Sync immediately | **Yes** |
| Reservations | Sync immediately | **Yes** |
| Activities (photos/notes) | Sync immediately | **Yes** |

**IndexedDB Schema:**
```
stores: {
  'sync-queue': { keyPath: 'id', autoIncrement: true },
  'sites': { keyPath: 'id' },
  'materials': { keyPath: 'id' },
  'time-entries': { keyPath: 'id' },
  'reservations': { keyPath: 'id' },
  'activities': { keyPath: 'id' }
}
```

**Sync Queue Structure:**
```typescript
interface SyncQueueItem {
  id: number;
  action: 'create' | 'update' | 'delete';
  entity: string; // 'time-entry', 'withdrawal', etc.
  data: object;
  timestamp: number;
  retryCount: number;
}
```

---

## Architecture Questions

### Q-01: How to handle photo uploads offline?

**Options:**
- A: Store as Base64 in IndexedDB (blobs can be stored directly)
- B: Store in Cache API
- C: Compress before storing

**Recommendation:** Store original blob in IndexedDB, compress on sync

---

### Q-02: Conflict resolution for offline edits?

**Scenarios:**
- Time entry created offline, another user modifies same site
- Material withdrawal created offline, stock changed by another user

**Strategy:**
- Last-write-wins for most entities
- Server returns conflict error if data stale
- Frontend shows notification with options (overwrite / discard)

---

### Q-03: What features work offline vs require connectivity?

**Offline-Capable:**
- View cached sites, materials, reservations
- Create time entries
- Create activities (photos/notes)
- Scan QR codes (camera works offline)
- View own reservations

**Requires Connectivity:**
- Login/logout
- Admin functions (create sites, invite users)
- View other users' data
- Real-time availability checks

---

## Technical Decisions Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Framework | Vite + React 18 | Fast, modern, well-documented |
| State | React Query + Zustand | Server state + client state separation |
| UI | Tailwind + shadcn/ui | Accessible, customizable, small bundle |
| PWA | vite-plugin-pwa + Workbox | Best practices built-in |
| Auth | OAuth2 + PKCE | Most secure SPA flow |
| QR Scanner | html5-qrcode | Works on mobile, maintained |
| Offline DB | IndexedDB (idb library) | Native browser API, good DX wrapper |

---

## Project Structure (Proposed)

```
frontend/
├── public/
│   ├── icons/              # PWA icons (192x192, 512x512)
│   └── manifest.json       # Generated by vite-plugin-pwa
├── src/
│   ├── api/                # API client functions
│   │   ├── client.ts       # Fetch wrapper with auth
│   │   ├── inventory.ts
│   │   ├── sites.ts
│   │   ├── fleet.ts
│   │   └── auth.ts
│   ├── components/
│   │   ├── ui/             # shadcn/ui components
│   │   ├── layout/         # App layout, navigation
│   │   └── features/       # Feature-specific components
│   ├── hooks/
│   │   ├── useAuth.ts
│   │   ├── useOffline.ts
│   │   └── useQRScanner.ts
│   ├── lib/
│   │   ├── db.ts           # IndexedDB wrapper
│   │   ├── sync.ts         # Offline sync queue
│   │   └── utils.ts
│   ├── pages/
│   │   ├── Dashboard.tsx
│   │   ├── Sites/
│   │   ├── Inventory/
│   │   ├── Fleet/
│   │   ├── TimeTracking/
│   │   └── Scanner/
│   ├── stores/             # Zustand stores
│   ├── types/              # TypeScript types
│   ├── App.tsx
│   ├── main.tsx
│   └── vite-env.d.ts
├── index.html
├── tailwind.config.js
├── vite.config.ts
├── tsconfig.json
└── package.json
```

---

## Open Questions for User

1. **Design preferences:** Any existing brand guidelines (colors, fonts, logo)?
2. **Language:** German UI confirmed? (PROJECT.md mentions "Deutsch first")
3. **Testing device:** What tablet/phone models will the pilot customer use?
4. **Deployment:** Same Kubernetes cluster as backend? Separate domain?

---

## Next Steps

After decisions are confirmed:
1. Create detailed PLAN.md files
2. Split into logical waves:
   - Wave 1: Project setup, auth flow, basic layout
   - Wave 2: Core features (Dashboard, Sites, Inventory)
   - Wave 3: Fleet & Reservations
   - Wave 4: PWA features (offline, QR scanner)
   - Wave 5: Polish & responsive design

---

*Created: 2026-04-28*
*Phase: 5 — PWA & Mobile*
