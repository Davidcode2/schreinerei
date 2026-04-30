# Technology Stack

**Project:** Schreinerei SaaS - v1.6 User Experience & Missing Functionality
**Researched:** 2026-04-30
**Milestone Focus:** Delete buttons, edit capabilities, reservation workflow, UX improvements, E2E tests

## Executive Summary

This milestone requires **minimal new stack additions**. The existing infrastructure supports all planned features:

- **Delete confirmations:** Add shadcn/ui AlertDialog component (CLI install only)
- **Backend gaps:** Add missing DELETE/PATCH routes for sites, materials, time entries
- **Form validation:** No new libraries needed — use existing controlled input pattern with inline error state
- **Calendar click-to-create:** No library needed — add onClick handlers to custom calendar
- **Low stock alerts:** Use existing Badge component + sonner toast
- **E2E tests:** Use existing Playwright patterns

**Critical finding:** Backend is missing routes for site/material/time-entry deletion and editing. Frontend has all required UI libraries.

---

## Required Stack Additions

### Frontend Components

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| `@radix-ui/react-alert-dialog` | ^1.1.2 | Delete confirmation dialogs | Standard shadcn/ui pattern, consistent with existing dialog usage |

**Installation:**
```bash
cd frontend
npx shadcn@latest add alert-dialog
```

This adds the AlertDialog component to `frontend/src/components/ui/alert-dialog.tsx`. No npm package installation needed — shadcn/ui copies component code into your project.

### Backend Routes (Rust)

| Route | Method | Module | Status |
|-------|--------|--------|--------|
| `/api/v1/sites/{id}` | DELETE | sites | ❌ MISSING |
| `/api/v1/inventory/materials/{id}` | DELETE | inventory | ❌ MISSING |
| `/api/v1/inventory/materials/{id}` | PATCH | inventory | ❌ MISSING |
| `/api/v1/time-entries/{id}` | DELETE | sites | ❌ MISSING |
| `/api/v1/time-entries/{id}` | PATCH | sites | ❌ MISSING |
| `/api/v1/fleet/vehicles/{id}` | DELETE | fleet | ✅ EXISTS |
| `/api/v1/fleet/tools/{id}` | DELETE | fleet | ✅ EXISTS |
| `/api/v1/fleet/reservations/{id}` | PATCH | fleet | ✅ EXISTS |
| `/api/v1/fleet/reservations/{id}` | DELETE | fleet | ✅ EXISTS |

---

## Existing Stack (No Changes Required)

The following are **already in place** and support the milestone features:

### Core Frontend

| Technology | Version | Purpose |
|------------|---------|---------|
| React | ^19.2.5 | UI framework |
| Vite | ^7.3.2 | Build tool |
| TypeScript | ~6.0.2 | Type safety |
| Tailwind CSS | ^4.2.4 | Styling |

### UI Components (shadcn/ui)

| Component | Location | Use Case |
|-----------|----------|----------|
| Dialog | `components/ui/dialog.tsx` | Edit/create dialogs |
| Button | `components/ui/button.tsx` | Actions, status transitions |
| Badge | `components/ui/badge.tsx` | Low stock indicators, status badges |
| Input | `components/ui/input.tsx` | Form fields |
| Label | `components/ui/label.tsx` | Form labels |
| DropdownMenu | `components/ui/dropdown-menu.tsx` | Actions menus |

### Notifications & Feedback

| Technology | Version | Purpose |
|------------|---------|---------|
| sonner | ^2.0.7 | Toast notifications (success/error feedback) |
| lucide-react | ^1.12.0 | Icons (AlertCircle, Trash, Edit, etc.) |

### State & Data

| Technology | Version | Purpose |
|------------|---------|---------|
| @tanstack/react-query | ^5.100.6 | Server state, mutations, cache invalidation |
| zustand | ^5.0.12 | Client state |

### E2E Testing

| Technology | Version | Purpose |
|------------|---------|---------|
| @playwright/test | ^1.59.1 | E2E testing |
| @testing-library/react | ^16.3.2 | Component testing |
| @testing-library/user-event | ^14.6.1 | User interaction simulation |

### QR Code

| Technology | Version | Purpose |
|------------|---------|---------|
| html5-qrcode | ^2.3.8 | QR scanning (already installed) |

---

## NOT Recommended (Explicitly Avoid)

| Technology | Why Not |
|------------|---------|
| react-hook-form | Overkill for existing simple dialogs. Current pattern uses controlled inputs with useState. Adding would require refactoring 7+ existing dialogs without clear benefit. |
| zod | Not needed without react-hook-form. Existing validation is backend-driven with toast errors. Add inline validation state locally instead. |
| Full calendar library (react-big-calendar, fullcalendar) | Custom calendar already built. Just needs onClick handlers for empty slots. No need to replace working code. |
| Additional testing libraries | Playwright patterns already established. Use existing helpers in `tests/helpers/`. |

---

## Integration Points

### 1. Delete Flow Pattern

```
User clicks delete button
    ↓
Open AlertDialog (confirmation)
    ↓
User confirms
    ↓
Call DELETE mutation (react-query)
    ↓
On success: toast.success + invalidate queries
On error: toast.error
```

**Example implementation:**
```tsx
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { useDeleteSite } from "@/lib/api/hooks"
import { toast } from "sonner"

function DeleteSiteButton({ siteId }: { siteId: string }) {
  const deleteMutation = useDeleteSite()
  
  const handleDelete = () => {
    deleteMutation.mutate(siteId, {
      onSuccess: () => toast.success("Baustelle gelöscht"),
      onError: () => toast.error("Löschen fehlgeschlagen"),
    })
  }
  
  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button variant="ghost" size="icon">
          <Trash2 className="h-4 w-4" />
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Baustelle löschen?</AlertDialogTitle>
          <AlertDialogDescription>
            Diese Aktion kann nicht rückgängig gemacht werden.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Abbrechen</AlertDialogCancel>
          <AlertDialogAction onClick={handleDelete}>Löschen</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
```

### 2. Inline Validation Pattern

Use local state for validation, display errors below inputs:

```tsx
const [hoursError, setHoursError] = useState<string | null>(null)

const validateHours = (value: string) => {
  const num = parseFloat(value)
  if (isNaN(num) || num <= 0) {
    setHoursError("Stunden müssen größer als 0 sein")
    return false
  }
  setHoursError(null)
  return true
}

return (
  <div className="space-y-2">
    <Label>Stunden</Label>
    <Input
      type="number"
      value={hours}
      onChange={(e) => {
        setHours(parseFloat(e.target.value) || 0)
        validateHours(e.target.value)
      }}
    />
    {hoursError && (
      <p className="text-sm text-destructive">{hoursError}</p>
    )}
  </div>
)
```

### 3. Calendar Click-to-Create

Add onClick to calendar grid cells:

```tsx
// In CalendarView.tsx, modify the day cell div:
<div
  key={i}
  className="p-2 min-h-[60px] border-l last:border-r cursor-pointer hover:bg-muted/50"
  onClick={() => {
    // Open reservation dialog for this date/resource
    setDialogDate(dateStr)
    setDialogResourceId(entry.resource_id)
    setDialogResourceType(entry.resource_type)
    setReservationDialogOpen(true)
  }}
>
```

### 4. Low Stock Alert UI

Add badge indicator to material list items:

```tsx
{material.is_low_stock && (
  <Badge variant="destructive" className="ml-2">
    <AlertTriangle className="h-3 w-3 mr-1" />
    Niedrig
  </Badge>
)}
```

Toast notification on dashboard load:
```tsx
const { data: lowStock } = useLowStockMaterials()

useEffect(() => {
  if (lowStock && lowStock.length > 0) {
    toast.warning(`${lowStock.length} Materialien unter Mindestbestand`)
  }
}, [lowStock])
```

---

## Backend Implementation Notes

### Required Route Additions

**Sites module (`src/modules/sites/api/routes.rs`):**

```rust
// Add to create_router():
.route("/api/v1/sites/{id}", delete(delete_site))

// Add handler:
pub async fn delete_site(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = SiteService::new(SiteRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let site_id = Uuid::parse_str(&id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
    
    service.delete_site(site_id, &ctx).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}
```

**Inventory module (`src/modules/inventory/api/routes.rs`):**

```rust
// Add to create_router():
.route("/api/v1/inventory/materials/{id}", patch(update_material).delete(delete_material))

// Add handlers similar to fleet module patterns
```

**Time Entries (sites module):**

```rust
// Add to create_router():
.route("/api/v1/time-entries/{id}", patch(update_time_entry).delete(delete_time_entry))

// Add handlers for update/delete
```

---

## E2E Test Patterns

Use existing helper patterns in `frontend/tests/helpers/api.ts`:

```typescript
// Example: Test site delete
test('should delete site', async ({ page }) => {
  const site = await createSite(page, {
    name: uniqueName('To Delete'),
    customer_name: 'Customer',
    location: 'Location',
  })
  track.site(site.id)
  
  // Navigate to site
  await page.goto(`/sites/${site.id}`)
  
  // Click delete button
  await page.click('button:has([data-testid="delete-site"])')
  
  // Confirm in dialog
  await page.click('button:has-text("Löschen")')
  
  // Verify toast
  await expect(page.locator('[data-sonner-toast]')).toContainText('gelöscht')
  
  // Verify site no longer exists
  const response = await page.request.get(`/api/v1/sites/${site.id}`)
  expect(response.status()).toBe(404)
})
```

---

## Files to Create/Modify

### New Files

| File | Purpose |
|------|---------|
| `frontend/src/components/ui/alert-dialog.tsx` | shadcn/ui AlertDialog component |
| `frontend/tests/reservations.spec.ts` | E2E tests for reservation workflow |
| `frontend/tests/update-delete.spec.ts` | E2E tests for update/delete operations |

### Modified Files

| File | Change |
|------|--------|
| `frontend/src/pages/sites/SitesListPage.tsx` | Add delete button with AlertDialog |
| `frontend/src/pages/inventory/InventoryListPage.tsx` | Add delete button, wire QR button |
| `frontend/src/pages/fleet/FleetPage.tsx` | Add delete buttons for vehicles/tools |
| `frontend/src/pages/fleet/CalendarView.tsx` | Add click-to-create on empty slots |
| `frontend/src/pages/fleet/ReservationDialog.tsx` | Add edit mode, status transition buttons |
| `frontend/src/pages/sites/TimeEntryDialog.tsx` | Add edit mode, inline validation |
| `frontend/src/lib/api/hooks.ts` | Add delete/update mutation hooks |
| `src/modules/sites/api/routes.rs` | Add DELETE route for sites |
| `src/modules/inventory/api/routes.rs` | Add DELETE/PATCH routes for materials |
| `src/modules/sites/domain/mod.rs` | Add DeleteSite, UpdateTimeEntry commands |
| `src/modules/inventory/domain/mod.rs` | Add UpdateMaterial command |

---

## Confidence Assessment

| Area | Confidence | Reason |
|------|------------|--------|
| AlertDialog addition | HIGH | Standard shadcn/ui pattern, matches existing Dialog usage |
| Backend routes | HIGH | Existing patterns in fleet module to follow |
| Form validation | HIGH | Local state pattern sufficient for simple dialogs |
| Calendar click-to-create | HIGH | Simple onClick handler, no library needed |
| E2E test patterns | HIGH | Existing helpers and patterns to follow |

---

## Summary

**What to add:**
1. `shadcn/ui AlertDialog` component (CLI install)
2. Backend DELETE/PATCH routes for sites, materials, time entries

**What NOT to add:**
1. react-hook-form — existing controlled input pattern sufficient
2. zod — backend validation handles errors
3. Calendar library — custom calendar works, just needs onClick
4. Any new testing libraries — Playwright patterns established

**The milestone is primarily about wiring existing infrastructure, not adding new dependencies.**
