# Architecture Research

**Domain:** Schreinerei SaaS - User Experience & Missing Functionality
**Researched:** 2026-04-30
**Confidence:** HIGH

## Current Architecture Overview

The existing Schreinerei SaaS uses **Hexagonal Architecture (Ports & Adapters)** with **Modular Monolith** and **DDD Bounded Contexts**. Each module follows a three-layer structure:

```
┌─────────────────────────────────────────────────────────────┐
│                        API Layer (routes.rs)                 │
│  - HTTP handlers, DTOs, validation                           │
│  - Maps HTTP to domain commands                              │
├─────────────────────────────────────────────────────────────┤
│                    Application Layer (service.rs)            │
│  - Use cases, business orchestration                         │
│  - Emits domain events                                       │
├─────────────────────────────────────────────────────────────┤
│                    Domain Layer (domain/*.rs)                │
│  - Entities, Value Objects, State Machines                   │
│  - Pure business logic, no dependencies                      │
├─────────────────────────────────────────────────────────────┤
│                  Infrastructure Layer (repository.rs)        │
│  - Database access, external APIs                            │
│  - Implements repository traits                              │
└─────────────────────────────────────────────────────────────┘
```

### Module Structure

```
src/modules/
├── iam/           # Identity & Access Management
├── inventory/     # Material management
├── sites/         # Construction site management
└── fleet/         # Vehicle & tool management
```

## Gap Analysis: What's Missing

### Backend API Gaps

| Module | Delete | Update | Status | Notes |
|--------|--------|--------|--------|-------|
| **Inventory** | ❌ Missing | ❌ Missing | N/A | No delete_material, no update_material |
| **Sites** | ❌ Missing | ✅ Exists | ✅ State machine | No delete_site route |
| **Fleet** | ✅ Exists | ✅ Exists | ✅ State machine | All operations exist |
| **Time Entries** | ❌ Missing | ❌ Missing | N/A | No update/delete routes |

### Frontend Gaps

| Feature | Backend | Frontend Hook | UI Component | Status |
|---------|---------|---------------|--------------|--------|
| Delete Material | ❌ | ❌ | ❌ | Full gap |
| Delete Site | ❌ | ❌ | ❌ | Full gap |
| Delete Vehicle | ✅ | ✅ (`useDeleteVehicle` missing) | ❌ | Hook exists as pattern, UI missing |
| Delete Tool | ✅ | ✅ (`useDeleteTool` missing) | ❌ | Hook exists as pattern, UI missing |
| Edit Time Entry | ❌ | ❌ | ❌ | Full gap |
| Delete Time Entry | ❌ | ❌ | ❌ | Full gap |
| Edit Reservation | ✅ | ✅ | ❌ | UI missing |
| Cancel Reservation | ✅ | ✅ | ❌ | UI missing |
| Status Transitions | ✅ | ✅ | ❌ | State machine in domain, UI missing |
| Low Stock Alerts | ✅ | ✅ | ⚠️ Partial | Badge exists, no prominent alert |
| QR Button | N/A | N/A | ❌ | onClick missing |
| Calendar Click-to-Create | ✅ | ✅ | ❌ | Handler missing |

## Integration Points

### 1. Delete Operations

**Pattern (from Fleet):**
```rust
// Backend route (routes.rs)
.route("/api/v1/fleet/vehicles/{id}", 
    get(get_vehicle)
    .patch(update_vehicle)
    .delete(delete_vehicle)  // ← Pattern to follow
)

// Handler
pub async fn delete_vehicle(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    let vehicle_id = Uuid::parse_str(&id).map(VehicleId)...;
    service.delete_vehicle(vehicle_id, &ctx).await?;
    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

// Service (service.rs)
pub async fn delete_vehicle(&self, vehicle_id: VehicleId, ctx: &TenantContext) 
    -> Result<(), AppError> {
    if !ctx.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    self.fleet_repo.delete_vehicle(ctx.tenant_id, vehicle_id).await
}
```

**Required Additions:**

| Module | Route | Handler | Service | Repository |
|--------|-------|---------|---------|------------|
| Inventory | `DELETE /api/v1/inventory/materials/{id}` | `delete_material` | `delete_material` | `delete_material` |
| Sites | `DELETE /api/v1/sites/{id}` | `delete_site` | `delete_site` | `delete_site` |
| Time Entries | `DELETE /api/v1/time-entries/{id}` | `delete_time_entry` | `delete_time_entry` | `delete_time_entry` |

### 2. Update Operations

**Pattern (from Sites):**
```rust
// Backend route
.route("/api/v1/sites/{id}", get(get_site).patch(update_site))

// Update struct (domain)
pub struct UpdateSite {
    pub name: Option<String>,
    pub customer_name: Option<String>,
    pub status: Option<SiteStatus>,  // ← Status via update
    // ...other optional fields
}

// Validation in service
if let Some(new_status) = &update.status {
    if !old_site.can_transition_to(*new_status) {
        return Err(AppError::Validation("Invalid status transition".to_string()));
    }
}
```

**Required Additions:**

| Module | Route | Handler | DTO |
|--------|-------|---------|-----|
| Inventory | `PATCH /api/v1/inventory/materials/{id}` | `update_material` | `UpdateMaterialRequest` |
| Time Entries | `PATCH /api/v1/time-entries/{id}` | `update_time_entry` | `UpdateTimeEntryRequest` |

### 3. Status Transitions

**Existing State Machines:**

```rust
// Site Status: Planned → Active → Completed → Archived
impl Site {
    pub fn can_transition_to(&self, new_status: SiteStatus) -> bool {
        match (&self.status, &new_status) {
            (SiteStatus::Planned, SiteStatus::Active) => true,
            (SiteStatus::Active, SiteStatus::Completed) => true,
            (SiteStatus::Completed, SiteStatus::Archived) => true,
            _ if self.status == new_status => true,
            _ => false,
        }
    }
}

// Reservation Status: Pending → Confirmed → InUse → Completed, any → Cancelled
impl Reservation {
    pub fn can_transition_to(&self, new_status: ReservationStatus) -> bool {
        match (&self.status, &new_status) {
            (ReservationStatus::Pending, ReservationStatus::Confirmed) => true,
            (ReservationStatus::Pending, ReservationStatus::Cancelled) => true,
            (ReservationStatus::Confirmed, ReservationStatus::InUse) => true,
            (ReservationStatus::Confirmed, ReservationStatus::Cancelled) => true,
            (ReservationStatus::InUse, ReservationStatus::Completed) => true,
            (_, ReservationStatus::Cancelled) => self.status != ReservationStatus::Cancelled,
            _ if self.status == new_status => true,
            _ => false,
        }
    }
}

// Resource Status: Available, Reserved, InUse, Maintenance
// No state machine - any transition allowed (business rule)
```

**Integration:** Status transitions are already handled via the update endpoints. Frontend just needs UI to trigger them.

### 4. Low Stock Alerts

**Existing Backend:**
```rust
// Route exists
.route("/api/v1/inventory/low-stock", get(list_low_stock))

// Material has computed field
is_low_stock: mat.quantity <= mat.min_quantity
```

**Frontend Hook Exists:**
```typescript
export function useLowStockMaterials() {
  return useQuery({
    queryKey: ["low-stock"],
    queryFn: () => apiClient.get<Material[]>("/api/v1/inventory/low-stock"),
    staleTime: 30000,
  })
}
```

**Integration:** Just needs UI component to display alerts prominently (e.g., on dashboard or as a banner).

### 5. Validation Patterns

**Existing Pattern (from domain):**
```rust
impl CreateSite {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Site name is required".to_string());
        }
        // ... more validation
        Ok(())
    }
}
```

**Frontend Pattern (needed for TimeEntryDialog):**
```typescript
// Current (BUG-TIME-001):
const [hours, setHours] = useState(1)
onChange={(e) => setHours(parseFloat(e.target.value) || 0)}  // ← Bug: allows 0

// Fixed:
const [hours, setHours] = useState(1)
const [errors, setErrors] = useState<Record<string, string>>({})

const validate = () => {
  const newErrors: Record<string, string> = {}
  if (hours <= 0) newErrors.hours = "Stunden müssen größer als 0 sein"
  setErrors(newErrors)
  return Object.keys(newErrors).length === 0
}

// Disable submit when invalid
<Button disabled={createMutation.isPending || hours <= 0}>
```

## New Components Needed

### Backend

| Component | Location | Purpose |
|-----------|----------|---------|
| `delete_material` route | `inventory/api/routes.rs` | Delete material endpoint |
| `update_material` route | `inventory/api/routes.rs` | Update material endpoint |
| `UpdateMaterialRequest` DTO | `inventory/api/routes.rs` | Update request struct |
| `delete_material` service | `inventory/application/inventory_service.rs` | Business logic |
| `update_material` service | `inventory/application/inventory_service.rs` | Business logic |
| `delete_material` repo | `inventory/infrastructure/material_repository.rs` | DB operation |
| `delete_site` route | `sites/api/routes.rs` | Delete site endpoint |
| `delete_site` service | `sites/application/site_service.rs` | Business logic |
| `delete_site` repo | `sites/infrastructure/site_repository.rs` | DB operation |
| `update_time_entry` route | `sites/api/routes.rs` | Update time entry endpoint |
| `delete_time_entry` route | `sites/api/routes.rs` | Delete time entry endpoint |
| `UpdateTimeEntryRequest` DTO | `sites/api/routes.rs` | Update request struct |
| `update_time_entry` service | `sites/application/site_service.rs` | Business logic |
| `delete_time_entry` service | `sites/application/site_service.rs` | Business logic |

### Frontend Hooks

| Hook | Location | Purpose |
|------|----------|---------|
| `useDeleteMaterial` | `useInventory.ts` | Delete material mutation |
| `useUpdateMaterial` | `useInventory.ts` | Update material mutation |
| `useDeleteSite` | `useSites.ts` | Delete site mutation |
| `useDeleteTimeEntry` | `useSites.ts` | Delete time entry mutation |
| `useUpdateTimeEntry` | `useSites.ts` | Update time entry mutation |

### Frontend UI Components

| Component | Location | Purpose |
|-----------|----------|---------|
| `DeleteConfirmDialog` | `components/shared/` | Reusable delete confirmation |
| `MaterialDeleteButton` | `MaterialCard.tsx` (add) | Delete button with confirmation |
| `SiteDeleteButton` | `SiteCard.tsx` (add) | Delete button with confirmation |
| `VehicleDeleteButton` | `ResourceCard.tsx` (add) | Delete button with confirmation |
| `ToolDeleteButton` | `ResourceCard.tsx` (add) | Delete button with confirmation |
| `EditTimeEntryDialog` | `pages/sites/` | Edit existing time entry |
| `EditReservationDialog` | `pages/fleet/` | Edit existing reservation |
| `StatusTransitionMenu` | `components/shared/` | Dropdown for status changes |
| `LowStockAlert` | `components/inventory/` | Prominent alert component |
| `CalendarSlot` | `CalendarView.tsx` (modify) | Clickable empty slot |

## Data Flow

### Delete Operation Flow

```
[User clicks delete]
    ↓
[DeleteConfirmDialog opens]
    ↓
[User confirms]
    ↓
[useDeleteXxx mutation called]
    ↓
[DELETE /api/v1/xxx/{id}]
    ↓
[Service checks permissions]
    ↓
[Service checks business rules] (e.g., no active reservations)
    ↓
[Repository deletes from DB]
    ↓
[QueryClient invalidates cache]
    ↓
[UI updates]
```

### Status Transition Flow

```
[User clicks status button]
    ↓
[StatusTransitionMenu shows valid transitions]
    ↓
[User selects new status]
    ↓
[useUpdateXxx mutation called with status]
    ↓
[PATCH /api/v1/xxx/{id}]
    ↓
[Service validates transition via can_transition_to()]
    ↓
[Repository updates status]
    ↓
[Domain event emitted]
    ↓
[QueryClient invalidates cache]
    ↓
[UI updates]
```

## Architectural Patterns to Follow

### 1. Soft Delete vs Hard Delete

**Recommendation:** Use **soft delete** for audit trail and recovery.

```sql
-- Add deleted_at column to all tables
ALTER TABLE materials ADD COLUMN deleted_at TIMESTAMPTZ DEFAULT NULL;
ALTER TABLE sites ADD COLUMN deleted_at TIMESTAMPTZ DEFAULT NULL;
-- etc.

-- Repository filters out deleted rows by default
SELECT * FROM materials WHERE tenant_id = $1 AND deleted_at IS NULL
```

### 2. Delete Business Rules

Before deleting, check:

| Entity | Rule | Error |
|--------|------|-------|
| Material | No pending order requests | "Material has pending orders" |
| Site | No active time entries (last 30 days) | "Site has recent time entries" |
| Vehicle | No active reservations | "Vehicle has active reservations" |
| Tool | No active reservations | "Tool has active reservations" |
| Time Entry | Only owner or admin can delete | "Can only delete own entries" |
| Reservation | Only owner or admin can cancel | "Can only cancel own reservations" |

### 3. Domain Events for Deletions

```rust
pub struct MaterialDeletedPayload {
    pub material_id: MaterialId,
    pub name: String,
}

pub struct SiteDeletedPayload {
    pub site_id: SiteId,
    pub name: String,
}
```

### 4. Frontend State Management

Use React Query's optimistic updates for better UX:

```typescript
export function useDeleteMaterial() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: string) => 
      apiClient.delete(`/api/v1/inventory/materials/${id}`),
    onMutate: async (id) => {
      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: ['materials'] })
      
      // Snapshot previous value
      const previous = queryClient.getQueryData(['materials'])
      
      // Optimistically update
      queryClient.setQueryData(['materials'], (old: Material[]) =>
        old.filter(m => m.id !== id)
      )
      
      return { previous }
    },
    onError: (err, id, context) => {
      // Rollback on error
      queryClient.setQueryData(['materials'], context.previous)
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ['materials'] })
    },
  })
}
```

## Anti-Patterns to Avoid

### 1. Delete Without Business Rule Checks

**Don't:**
```rust
pub async fn delete_site(&self, site_id: SiteId, ctx: &TenantContext) -> Result<(), AppError> {
    self.site_repo.delete(ctx.tenant_id, site_id).await
}
```

**Do:**
```rust
pub async fn delete_site(&self, site_id: SiteId, ctx: &TenantContext) -> Result<(), AppError> {
    // Check for recent time entries
    let recent_entries = self.site_repo.count_recent_time_entries(ctx.tenant_id, site_id, 30).await?;
    if recent_entries > 0 {
        return Err(AppError::Validation(
            "Cannot delete site with recent time entries".to_string()
        ));
    }
    
    self.site_repo.delete(ctx.tenant_id, site_id).await
}
```

### 2. Status Transitions Without Validation

**Don't:**
```rust
// Frontend directly sets any status
updateMutation.mutate({ id, status: "completed" })
```

**Do:**
```rust
// Backend validates transition
if !current.can_transition_to(new_status) {
    return Err(AppError::Validation(format!(
        "Invalid status transition from {} to {}",
        current.status, new_status
    )));
}
```

### 3. Inline Delete Buttons Without Confirmation

**Don't:**
```tsx
<Button onClick={() => deleteMutation.mutate(id)}>Delete</Button>
```

**Do:**
```tsx
<Button onClick={() => setConfirmOpen(true)}>Delete</Button>
<DeleteConfirmDialog
  open={confirmOpen}
  onOpenChange={setConfirmOpen}
  onConfirm={() => deleteMutation.mutate(id)}
  itemName={material.name}
/>
```

### 4. Missing Tenant Scoping

**Don't:**
```rust
pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM materials WHERE id = $1")
        .bind(id)
        .execute(&self.pool)
        .await?;
    Ok(())
}
```

**Do:**
```rust
pub async fn delete(&self, tenant_id: TenantId, id: MaterialId) -> Result<(), AppError> {
    sqlx::query("DELETE FROM materials WHERE tenant_id = $1 AND id = $2")
        .bind(tenant_id)
        .bind(id)
        .execute(&self.pool)
        .await?;
    Ok(())
}
```

## Build Order

Based on dependencies and risk:

### Phase 1: Bug Fixes (Low Risk)
1. Fix BUG-TIME-001 (hours validation)
2. Add input validation feedback to TimeEntryDialog
3. Wire QR button onClick handler

**Reasoning:** No backend changes, minimal risk, immediate UX improvement.

### Phase 2: Backend Additions (Medium Risk)
1. Add `delete_material` backend route + service + repo
2. Add `update_material` backend route + service + repo
3. Add `delete_site` backend route + service + repo
4. Add `update_time_entry` backend route + service + repo
5. Add `delete_time_entry` backend route + service + repo

**Reasoning:** Backend first allows parallel frontend work later. Each module is independent.

### Phase 3: Frontend Delete UI (Low Risk)
1. Create shared `DeleteConfirmDialog` component
2. Add `useDeleteMaterial` hook
3. Add delete button to MaterialCard
4. Add `useDeleteSite` hook
5. Add delete button to SiteCard
6. Add delete buttons to Vehicle/Tool cards
7. Add `useDeleteTimeEntry` hook
8. Add delete to time entries

**Reasoning:** Reusable components first, then specific implementations.

### Phase 4: Edit Dialogs (Medium Risk)
1. Create `EditTimeEntryDialog` (refactor TimeEntryDialog for edit mode)
2. Create `EditReservationDialog` (refactor ReservationDialog for edit mode)
3. Add `useUpdateMaterial` hook
4. Add `useUpdateTimeEntry` hook
5. Add edit buttons to relevant list/detail pages

**Reasoning:** Refactor existing dialogs rather than create new ones.

### Phase 5: Status Transitions (Medium Risk)
1. Create shared `StatusTransitionMenu` component
2. Add status menu to SiteDetailPage
3. Add status menu to ReservationDialog
4. Add status buttons to Vehicle/Tool cards

**Reasoning:** Requires understanding of state machines, but backend already validates.

### Phase 6: Calendar Click-to-Create (Medium Risk)
1. Modify `CalendarView.tsx` to detect slot clicks
2. Pre-fill `ReservationDialog` with slot time and resource
3. Wire `ReservationDialog` open state to calendar click

**Reasoning:** Calendar interaction needs careful testing.

### Phase 7: Low Stock Alerts (Low Risk)
1. Create `LowStockAlert` component
2. Add to DashboardPage
3. Consider adding as banner to InventoryPage

**Reasoning:** Backend and hooks exist, just UI needed.

### Phase 8: E2E Tests (Medium Risk)
1. Add reservation tests (create, edit, cancel)
2. Add update/delete tests for inventory
3. Add update/delete tests for sites
4. Add update/delete tests for fleet
5. Add time entry edit/delete tests

**Reasoning:** Tests after implementation ensures coverage of new features.

## Sources

- Existing codebase analysis (HIGH confidence)
- Domain models: `src/modules/*/domain/*.rs`
- API routes: `src/modules/*/api/routes.rs`
- Services: `src/modules/*/application/*_service.rs`
- Frontend hooks: `frontend/src/lib/api/hooks/*.ts`
- Frontend components: `frontend/src/pages/**/*.tsx`
- E2E tests: `frontend/tests/*.spec.ts`

---
*Architecture research for: v1.6 User Experience & Missing Functionality*
*Researched: 2026-04-30*
