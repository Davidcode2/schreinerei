# Phase 10: Bug Fixes Round 2 - Context

**Gathered:** 2026-04-29
**Status:** Ready for planning
**Source:** BUG-REPORT.md from automated Playwright testing

<domain>
## Phase Boundary

Fix all 8 bugs discovered during Phase 9 automated frontend testing. These bugs range from critical authentication issues to non-functional UI buttons. All bugs are frontend-focused and do not require backend changes.

</domain>

<decisions>
## Implementation Decisions

### BUG-001: Token Exchange Failure (Critical)
- **D-01**: Add protection against double token exchange in AuthCallback.tsx
- **D-02**: Use a flag in sessionStorage to track if exchange already happened
- **D-03**: Clear PKCE verifier immediately after first use to prevent reuse

### BUG-002: Token Refresh Cascade Failure (Critical)
- **D-04**: Implement retry logic with exponential backoff for token refresh
- **D-05**: Do not logout immediately on first refresh failure - retry up to 3 times
- **D-06**: Show toast notification when session is expiring (warning before logout)

### BUG-003: Wrong API URL (High)
- **D-07**: Update workbox runtimeCaching to only match external URLs, not localhost proxy
- **D-08**: Ensure service worker does not intercept localhost:5175/api/* requests (let Vite proxy handle them)

### BUG-004: Fleet "Neu" Button Non-Functional (High)
- **D-09**: Wire AddVehicleDialog and AddToolDialog to FleetPage.tsx
- **D-10**: Add dropdown menu to "Neu" button for selecting Vehicle or Tool
- **D-11**: Track dialog state (vehicle/tool/none) in FleetPage component

### BUG-005: Redundant API Calls (Medium)
- **D-12**: Add staleTime: 30000 (30 seconds) to all useQuery calls
- **D-13**: Centralize React Query configuration in a shared hook wrapper

### BUG-006: User List Not Displaying (Medium)
- **D-14**: Add explicit enabled condition based on isAuthenticated state
- **D-15**: Fix dependency array in UserManagementSection to prevent re-renders

### BUG-007: No Email Invite Dialog (Medium)
- **D-16**: Create InviteUserDialog component with email input
- **D-17**: Replace "Einladen" button onClick with dialog open
- **D-18**: Keep invite link copy as secondary option in dialog

### BUG-008: Offline Sync Fails Silently (Medium)
- **D-19**: Add toast notifications for sync success/failure
- **D-20**: Add sync status indicator to UI (last sync time, syncing state)
- **D-21**: Add manual retry button when sync fails

### the agent's Discretion
- Exact toast message wording
- UI placement of sync status indicator
- Styling of dropdown menu for "Neu" button

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Bug Report (Primary Source)
- `.planning/BUG-REPORT.md` — Full bug report with network evidence and code locations

### Affected Files
- `frontend/src/lib/auth/keycloak.ts` — Token exchange logic (BUG-001)
- `frontend/src/lib/api/client.ts` — Token refresh logic (BUG-002)
- `frontend/vite.config.ts` — PWA/Workbox config (BUG-003)
- `frontend/src/pages/fleet/FleetPage.tsx` — Neu button (BUG-004)
- `frontend/src/pages/fleet/AddVehicleDialog.tsx` — Existing dialog to wire
- `frontend/src/pages/fleet/AddToolDialog.tsx` — Existing dialog to wire
- `frontend/src/lib/api/hooks/*.ts` — Query hooks (BUG-005)
- `frontend/src/pages/settings/UserManagementSection.tsx` — User list (BUG-006, BUG-007)
- `frontend/src/lib/offline/sync.ts` — Sync logic (BUG-008)

### Patterns to Follow
- `frontend/src/pages/inventory/AddMaterialDialog.tsx` — Reference dialog pattern
- `frontend/src/pages/sites/AddSiteDialog.tsx` — Reference dialog pattern

</canonical_refs>

<specifics>
## Specific Ideas

### BUG-001 Fix Approach
```typescript
// In AuthCallback.tsx, check if exchange already happened
const exchangeKey = `auth_exchanging_${code}`
if (sessionStorage.getItem(exchangeKey)) {
  // Already processing this code, skip
  return
}
sessionStorage.setItem(exchangeKey, 'true')
```

### BUG-002 Fix Approach
```typescript
// In client.ts, add retry with backoff
const MAX_RETRIES = 3
const BASE_DELAY = 1000 // 1 second

async function refreshWithRetry(token: string, attempt = 1): Promise<AuthTokens> {
  try {
    return await refreshAccessToken(token)
  } catch (error) {
    if (attempt >= MAX_RETRIES) throw error
    await new Promise(r => setTimeout(r, BASE_DELAY * Math.pow(2, attempt - 1)))
    return refreshWithRetry(token, attempt + 1)
  }
}
```

### BUG-004 Fix Approach
```tsx
// Add dropdown menu to FleetPage
import { AddVehicleDialog } from "./AddVehicleDialog"
import { AddToolDialog } from "./AddToolDialog"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"

const [dialogType, setDialogType] = useState<"vehicle" | "tool" | null>(null)

// In JSX:
<DropdownMenu>
  <DropdownMenuTrigger asChild>
    <Button className="gap-2">
      <Plus className="h-4 w-4" />
      <span className="hidden sm:inline">Neu</span>
    </Button>
  </DropdownMenuTrigger>
  <DropdownMenuContent>
    <DropdownMenuItem onClick={() => setDialogType("vehicle")}>
      Fahrzeug
    </DropdownMenuItem>
    <DropdownMenuItem onClick={() => setDialogType("tool")}>
      Werkzeug
    </DropdownMenuItem>
  </DropdownMenuContent>
</DropdownMenu>
```

</specifics>

<deferred>
## Deferred Ideas

None — all bugs in scope for this phase.

</deferred>

---

*Phase: 10-bug-fixes-round-2*
*Context gathered: 2026-04-29 from BUG-REPORT.md*
