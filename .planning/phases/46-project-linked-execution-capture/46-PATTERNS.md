# Phase 46: Project-Linked Execution Capture - Pattern Map

**Mapped:** 2026-05-07
**Files analyzed:** 17
**Analogs found:** 17 / 17

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `src/modules/inventory/api/routes.rs` | route | request-response | `src/modules/inventory/api/routes.rs` | exact |
| `src/modules/inventory/application/inventory_service.rs` | service | CRUD | `src/modules/inventory/application/inventory_service.rs` | exact |
| `src/modules/inventory/domain/material.rs` | model | CRUD | `src/modules/inventory/domain/material.rs` | exact |
| `src/modules/inventory/infrastructure/material_repository.rs` | service | CRUD | `src/modules/inventory/infrastructure/material_repository.rs` | exact |
| `src/modules/inventory/domain/events.rs` | model | event-driven | `src/modules/inventory/domain/events.rs` | exact |
| `src/modules/sites/api/routes.rs` | route | request-response | `src/modules/sites/api/routes.rs` | exact |
| `src/modules/sites/application/site_service.rs` | service | CRUD | `src/modules/sites/application/site_service.rs` | exact |
| `src/modules/sites/domain/time_entry.rs` | model | CRUD | `src/modules/sites/domain/time_entry.rs` | exact |
| `src/modules/sites/infrastructure/site_repository.rs` | service | CRUD | `src/modules/sites/infrastructure/site_repository.rs` | exact |
| `src/modules/sites/domain/events.rs` | model | event-driven | `src/modules/sites/domain/events.rs` | exact |
| `frontend/src/pages/inventory/WithdrawDialog.tsx` | component | request-response | `frontend/src/pages/inventory/WithdrawDialog.tsx` | exact |
| `frontend/src/pages/inventory/InventoryDetailPage.tsx` | component | request-response | `frontend/src/pages/inventory/InventoryDetailPage.tsx` | exact |
| `frontend/src/lib/api/hooks/useInventory.ts` | hook | request-response | `frontend/src/lib/api/hooks/useInventory.ts` | exact |
| `frontend/src/pages/inventory/InventoryDetailPage.test.tsx` | test | request-response | `frontend/src/pages/inventory/InventoryDetailPage.test.tsx` | exact |
| `frontend/src/pages/sites/TimeEntryDialog.tsx` | component | request-response | `frontend/src/pages/sites/TimeEntryDialog.tsx` | exact |
| `frontend/src/lib/api/hooks/useSites.ts` | hook | request-response | `frontend/src/lib/api/hooks/useSites.ts` | exact |
| `frontend/src/pages/sites/TimeEntryDialog.test.tsx` | test | request-response | `frontend/src/pages/sites/TimeEntryDialog.test.tsx` | exact |

## Pattern Assignments

### `src/modules/inventory/api/routes.rs` (route, request-response)

**Analog:** `src/modules/inventory/api/routes.rs`

**Imports + DTO export pattern** (lines 1-22, 223-230):
```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct WithdrawRequest {
    pub quantity: i32,
    pub notes: Option<String>,
    pub site_id: Option<String>,
    pub disposal: Option<bool>,
}
```

**Request parsing + validation pattern** (lines 750-780):
```rust
pub async fn withdraw_material(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<WithdrawRequest>,
) -> Result<impl IntoResponse, AppError> {
    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let site_id = request
        .site_id
        .map(|s| Uuid::parse_str(&s).map(SiteId))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;
```

**Service delegation pattern** (lines 770-780):
```rust
    let withdraw = WithdrawMaterial {
        material_id,
        quantity: request.quantity,
        notes: request.notes,
        site_id,
        disposal: request.disposal.unwrap_or(false),
    };

    let material = service.withdraw_material(withdraw, &ctx).await?;
    Ok(Json(MaterialResponse::from(material)))
}
```

### `src/modules/inventory/application/inventory_service.rs` (service, CRUD)

**Analog:** `src/modules/inventory/application/inventory_service.rs`

**Auth + tenant resolution pattern** (lines 32-47):
```rust
async fn resolve_local_user_id(&self, ctx: &TenantContext) -> Result<UserId, AppError> {
    let user_repo = UserRepository::new(self.pool.clone());
    let user = user_repo
        .find_or_create_by_keycloak_id(
            &ctx.user_id.to_string(),
            ctx.tenant_id,
            &ctx.email,
            if ctx.is_admin() { Role::Admin } else { Role::Employee },
        )
        .await?;
    Ok(user.id)
}
```

**Validation-before-repository pattern** (lines 184-205):
```rust
pub async fn withdraw_material(
    &self,
    withdraw: WithdrawMaterial,
    ctx: &TenantContext,
) -> Result<Material, AppError> {
    withdraw.validate()?;

    let local_user_id = self.resolve_local_user_id(ctx).await?;

    let material = self
        .material_repo
        .withdraw_stock(
            withdraw.material_id,
            withdraw.quantity,
            local_user_id,
            withdraw.notes.clone(),
            withdraw.site_id,
            withdraw.disposal,
            ctx.tenant_id,
        )
        .await?;
```

**Event publishing pattern** (lines 206-232):
```rust
    let event = StockWithdrawnPayload {
        material_id: material.id,
        material_name: material.name.clone(),
        quantity_withdrawn: withdraw.quantity,
        quantity_after: material.quantity,
        withdrawn_by: local_user_id,
        notes: withdraw.notes,
        is_last_unit: material.is_last_unit(),
    }
    .into_event(ctx.tenant_id);

    self.material_repo.publish_event(&event).await?;
```

### `src/modules/inventory/domain/material.rs` (model, CRUD)

**Analog:** `src/modules/inventory/domain/material.rs`

**Command object pattern** (lines 83-91):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawMaterial {
    pub material_id: MaterialId,
    pub quantity: i32,
    pub notes: Option<String>,
    pub site_id: Option<SiteId>,
    pub disposal: bool,
}
```

**Short validation function pattern** (lines 93-100):
```rust
impl WithdrawMaterial {
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= 0 {
            return Err("Withdrawal quantity must be positive".to_string());
        }
        Ok(())
    }
}
```

### `src/modules/inventory/infrastructure/material_repository.rs` (service, CRUD)

**Analog:** `src/modules/inventory/infrastructure/material_repository.rs`

**Transaction + row lock pattern** (lines 423-443):
```rust
let mut tx = self.pool.begin().await.map_err(|e| AppError::Database(e.to_string()))?;

let current = sqlx::query_as::<_, MaterialLockRow>(
    r#"
    SELECT m.quantity, m.legacy_quantity, c.can_expire
    FROM materials m
    INNER JOIN categories c ON c.id = m.category_id
    WHERE m.id = $1 AND m.tenant_id = $2 AND m.deleted_at IS NULL
    FOR UPDATE
    "#,
)
```

**Atomic history-write pattern** (lines 512-533):
```rust
sqlx::query(
    r#"
    INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, site_id, entry_type, created_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'withdrawn', $9)
    "#,
)
.bind(site_id.map(|s| s.0))
.execute(&mut *tx)
.await
.map_err(|e| AppError::Database(e.to_string()))?;

tx.commit().await.map_err(|e| AppError::Database(e.to_string()))?;
```

**Error mapping pattern** (lines 445-449):
```rust
if current.quantity < quantity {
    return Err(AppError::Validation(format!(
        "Insufficient stock. Current: {}, Requested: {}",
        current.quantity, quantity
    )));
}
```

### `src/modules/inventory/domain/events.rs` (model, event-driven)

**Analog:** `src/modules/inventory/domain/events.rs`

**Payload + `into_event` pattern** (lines 29-50):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockWithdrawnPayload {
    pub material_id: MaterialId,
    pub material_name: String,
    pub quantity_withdrawn: i32,
    pub quantity_after: i32,
    pub withdrawn_by: UserId,
    pub notes: Option<String>,
    pub is_last_unit: bool,
}

impl StockWithdrawnPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(EventType::StockWithdrawn, tenant_id, "Material", self.material_id.to_string(), json!(self))
    }
}
```

### `src/modules/sites/api/routes.rs` (route, request-response)

**Analog:** `src/modules/sites/api/routes.rs`

**DTO contract pattern** (lines 243-261):
```rust
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateTimeEntryRequest {
    pub site_id: Option<String>,
    pub work_type: String,
    pub hours: f64,
    pub work_date: String,
    pub notes: Option<String>,
}
```

**Typed parse pipeline pattern** (lines 480-513):
```rust
let site_id = request
    .site_id
    .map(|s| Uuid::parse_str(&s).map(SiteId))
    .transpose()
    .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

let work_type = request
    .work_type
    .parse::<WorkType>()
    .map_err(|e: String| AppError::Validation(e))?;

let work_date = NaiveDate::parse_from_str(&request.work_date, "%Y-%m-%d")
    .map_err(|_| AppError::Validation("Invalid work date format (expected YYYY-MM-DD)".to_string()))?;
```

**Patch/null semantics pattern** (lines 560-592):
```rust
let site_id = request
    .site_id
    .map(|s| {
        if s.is_empty() {
            Ok(None)
        } else {
            Uuid::parse_str(&s).map(|u| Some(SiteId(u)))
        }
    })
    .transpose()?;

let update = UpdateTimeEntry {
    site_id,
    work_type,
    hours: request.hours,
    work_date,
    notes: request.notes.map(Some),
};
```

### `src/modules/sites/application/site_service.rs` (service, CRUD)

**Analog:** `src/modules/sites/application/site_service.rs`

**Existence check + tenant scope pattern** (lines 363-380):
```rust
pub async fn create_time_entry(
    &self,
    create: CreateTimeEntry,
    ctx: &TenantContext,
) -> Result<TimeEntry, AppError> {
    create.validate()?;

    if let Some(site_id) = create.site_id {
        let _site = self.get_site(site_id, ctx).await?;
    }

    let local_user_id = self.resolve_local_user_id(ctx).await?;
    let entry = self.site_repo.create_time_entry(ctx.tenant_id, local_user_id, &create).await?;
```

**Ownership/authorization pattern** (lines 434-466):
```rust
let existing = self
    .site_repo
    .find_time_entry_by_id(ctx.tenant_id, entry_id)
    .await?
    .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))?;

let local_user_id = self.resolve_local_user_id(ctx).await?;
if existing.user_id != local_user_id && !ctx.is_admin() {
    return Err(AppError::Forbidden("Can only edit own time entries".to_string()));
}
```

**Event payload emission pattern** (lines 382-392):
```rust
let event = TimeEntryCreatedPayload {
    site_id: create.site_id,
    user_id: ctx.user_id,
    hours: create.hours,
    work_type: create.work_type.to_string(),
    work_date: create.work_date.to_string(),
}
.into_event(ctx.tenant_id);

self.site_repo.publish_event(&event).await?;
```

### `src/modules/sites/domain/time_entry.rs` (model, CRUD)

**Analog:** `src/modules/sites/domain/time_entry.rs`

**Create command pattern** (lines 27-35):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimeEntry {
    pub site_id: Option<SiteId>,
    pub work_type: WorkType,
    pub hours: f64,
    pub work_date: NaiveDate,
    pub notes: Option<String>,
}
```

**Update nullability pattern** (lines 55-64):
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTimeEntry {
    pub site_id: Option<Option<SiteId>>, // None = not provided, Some(None) = set to null
    pub work_type: Option<WorkType>,
    pub hours: Option<f64>,
    pub work_date: Option<NaiveDate>,
    pub notes: Option<Option<String>>,
}
```

**Short validation pattern** (lines 67-84):
```rust
if let Some(hours) = self.hours {
    if hours <= 0.0 {
        return Err("Hours must be positive".to_string());
    }
    if hours > 24.0 {
        return Err("Hours cannot exceed 24 per day".to_string());
    }
}
```

### `src/modules/sites/infrastructure/site_repository.rs` (service, CRUD)

**Analog:** `src/modules/sites/infrastructure/site_repository.rs`

**Simple insert pattern** (lines 327-356):
```rust
let entry = sqlx::query_as::<_, TimeEntryRow>(
    r#"
    INSERT INTO time_entries (id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    RETURNING id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
    "#,
)
.bind(create.site_id.map(|s| s.0))
```

**Current update pattern to copy and then harden** (lines 455-488):
```rust
let notes_update = update.notes.clone();
let should_update_notes = notes_update.is_some();
let notes_value = notes_update.flatten();

let entry = sqlx::query_as::<_, TimeEntryRow>(
    r#"
    UPDATE time_entries
    SET 
        site_id = COALESCE($1, site_id),
        work_type = COALESCE($2, work_type),
        hours = COALESCE($3, hours),
        work_date = COALESCE($4, work_date),
        notes = CASE WHEN $5::boolean THEN $6 ELSE notes END
    WHERE id = $7 AND tenant_id = $8
    RETURNING id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
    "#,
)
```

**Read-model aggregation pattern** (lines 865-895):
```rust
SELECT 
    s.id, s.tenant_id, s.project_type, s.name, s.customer_name, s.location,
    s.status, s.start_date, s.end_date, s.estimated_days,
    COUNT(DISTINCT sa.user_id) as assigned_users,
    COALESCE(SUM(te.hours), 0)::FLOAT as total_hours
FROM sites s
LEFT JOIN site_assignments sa ON s.id = sa.site_id AND s.tenant_id = sa.tenant_id
LEFT JOIN time_entries te ON s.id = te.site_id AND s.tenant_id = te.tenant_id
WHERE s.tenant_id = $1 AND s.status IN ('planned', 'active')
```

### `src/modules/sites/domain/events.rs` (model, event-driven)

**Analog:** `src/modules/sites/domain/events.rs`

**Time-entry event payload pattern** (lines 69-88):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryCreatedPayload {
    pub site_id: Option<SiteId>,
    pub user_id: UserId,
    pub hours: f64,
    pub work_type: String,
    pub work_date: String,
}

impl TimeEntryCreatedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(EventType::TimeEntryCreated, tenant_id, "TimeEntry", tenant_id.to_string(), json!(self))
    }
}
```

### `frontend/src/pages/inventory/WithdrawDialog.tsx` (component, request-response)

**Analog:** `frontend/src/pages/inventory/WithdrawDialog.tsx`

**Controlled-form state pattern** (lines 46-49, 67-76):
```tsx
const [quantity, setQuantity] = useState(1)
const [notes, setNotes] = useState("")
const [siteId, setSiteId] = useState(initialSiteId ?? "")
const [disposal, setDisposal] = useState(false)

useEffect(() => {
  if (open) {
    setSiteId(initialSiteId ?? "")
    setDisposal(false)
  }
}, [open, initialSiteId])
```

**Payload shaping pattern** (lines 58-63):
```tsx
const handleSubmit = () => {
  onConfirm(quantity, notes || undefined, disposal ? null : siteId || null, disposal)
  setQuantity(1)
  setNotes("")
  setDisposal(false)
}
```

**Project select UX pattern** (lines 202-218):
```tsx
{!disposal && (
  <div className="space-y-2 rounded-xl border border-border/70 bg-card/70 p-4 shadow-sm">
    <Label htmlFor="site">Baustelle (optional)</Label>
    <select value={siteId} onChange={(event) => setSiteId(event.target.value)}>
      <option value="">Keine Zuordnung</option>
      {sites.map((site) => (
        <option key={site.id} value={site.id}>{site.name}</option>
      ))}
    </select>
  </div>
)}
```

### `frontend/src/pages/inventory/InventoryDetailPage.tsx` (component, request-response)

**Analog:** `frontend/src/pages/inventory/InventoryDetailPage.tsx`

**Preference/default wiring pattern** (lines 52-59, 372-387):
```tsx
const { data: preferences } = usePreferences()
const { data: sites } = useSites()
const withdrawSiteIdFromQuery = searchParams.get("siteId")

<WithdrawDialog
  material={material}
  onConfirm={handleWithdraw}
  isLoading={withdrawMutation.isPending}
  sites={(sites ?? []).map((site) => ({ id: site.id, name: site.name }))}
  initialSiteId={withdrawSiteIdFromQuery ?? preferences?.active_site_id ?? null}
/>
```

**Mutation + toast pattern** (lines 91-114):
```tsx
const handleWithdraw = async (quantity: number, notes?: string, siteId?: string | null, disposal?: boolean) => {
  try {
    await withdrawMutation.mutateAsync({
      id: material.id,
      quantity,
      notes: notes ?? null,
      site_id: siteId ?? null,
      disposal: disposal ?? false,
    })
    toast.success(disposal ? `${quantity} ${material.unit} entsorgt` : `${quantity} ${material.unit} entnommen`)
    closeWithdrawDialog()
  } catch {
    toast.error("Entnahme fehlgeschlagen")
  }
}
```

### `frontend/src/lib/api/hooks/useInventory.ts` (hook, request-response)

**Analog:** `frontend/src/lib/api/hooks/useInventory.ts`

**Mutation wrapper pattern** (lines 180-199):
```tsx
export function useWithdrawMaterial() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, ...data }: WithdrawRequest & { id: string }) =>
      apiClient.post<Material>(`/api/v1/inventory/materials/${id}/withdraw`, {
        quantity: data.quantity,
        notes: data.notes,
        site_id: data.site_id ?? null,
        disposal: data.disposal ?? false,
      }),
```

**Invalidation pattern** (lines 194-198):
```tsx
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["materials"] })
      queryClient.invalidateQueries({ queryKey: ["material"] })
      queryClient.invalidateQueries({ queryKey: ["low-stock"] })
    },
```

### `frontend/src/pages/inventory/InventoryDetailPage.test.tsx` (test, request-response)

**Analog:** `frontend/src/pages/inventory/InventoryDetailPage.test.tsx`

**MSW payload assertion pattern** (lines 196-221):
```tsx
let stockInPayload: unknown = null

server.use(
  http.post(apiPath("/inventory/materials/mat-123/stock-in"), async ({ request }) => {
    stockInPayload = await request.json()
    return HttpResponse.json({ ...materialResponse, quantity: 51 })
  })
)

await waitFor(() => {
  expect(stockInPayload).toEqual({ quantity: 3, notes: "Lieferung HolzLand", expires_on: null })
})
```

### `frontend/src/pages/sites/TimeEntryDialog.tsx` (component, request-response)

**Analog:** `frontend/src/pages/sites/TimeEntryDialog.tsx`

**Dialog-reset + defaulting pattern** (lines 85-105):
```tsx
useEffect(() => {
  if (open) {
    if (mode === "edit" && initialData) {
      setWorkType(initialData.work_type)
      setHours(initialData.hours)
      setWorkDate(initialData.work_date)
      setNotes(initialData.notes ?? "")
      setSelectedSiteId(initialData.site_id ?? "")
    } else {
      setWorkType("site")
      setHours(0.5)
      setWorkDate(new Date().toISOString().split("T")[0] ?? "")
      setNotes("")
      setSelectedSiteId(siteId ?? preferences?.active_site_id ?? "")
    }
  }
}, [open, mode, initialData, preferences?.active_site_id, siteId])
```

**Mutation payload pattern** (lines 126-156):
```tsx
if (mode === "edit" && initialData) {
  await updateMutation.mutateAsync({
    id: initialData.id,
    site_id: workType === "site" ? selectedSiteId || null : null,
    work_type: workType,
    hours,
    work_date: workDate,
    ...(notes ? { notes } : {}),
  })
} else {
  await createMutation.mutateAsync({
    site_id: workType === "site" ? selectedSiteId || null : null,
    work_type: workType,
    hours,
    work_date: workDate,
    ...(notes ? { notes } : {}),
  })
}
```

**Work-type selector pattern** (lines 44-49, 191-207):
```tsx
const workTypes: { value: WorkType; label: string }[] = [
  { value: "site", label: "Projekt vor Ort" },
  { value: "workshop", label: "Werkstattprojekt" },
  { value: "travel", label: "Fahrt" },
  { value: "other", label: "Sonstiges" },
]
```

### `frontend/src/lib/api/hooks/useSites.ts` (hook, request-response)

**Analog:** `frontend/src/lib/api/hooks/useSites.ts`

**Create/update mutation pattern** (lines 161-186):
```tsx
export function useCreateTimeEntry() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (data: CreateTimeEntryRequest) =>
      apiClient.post<TimeEntry>("/api/v1/time-entries", data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["time-entries"] })
      queryClient.invalidateQueries({ queryKey: ["my-time-entries"] })
      queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
    },
  })
}
```

**Query key pattern** (lines 140-158):
```tsx
export function useTimeEntries(siteId?: string) {
  return useQuery({
    queryKey: ["time-entries", siteId],
    queryFn: () => {
      if (siteId) {
        return apiClient.get<TimeEntry[]>(`/api/v1/sites/${siteId}/time-entries`)
      }
      return apiClient.get<TimeEntry[]>("/api/v1/time-entries/my")
    },
  })
}
```

### `frontend/src/pages/sites/TimeEntryDialog.test.tsx` (test, request-response)

**Analog:** `frontend/src/pages/sites/TimeEntryDialog.test.tsx`

**Mock data setup pattern** (lines 8-39):
```tsx
mockData.preferences = { active_site_id: 'site-2' }
mockData.sites = [
  { id: 'site-1', project_type: 'external_site', name: 'Villa Müller', ... },
  { id: 'site-2', project_type: 'internal_workshop', name: 'CNC Vorbereitung', ... },
]

render(<TimeEntryDialog open={true} onOpenChange={() => {}} />)
```

**Rendered-option assertion pattern** (lines 41-46):
```tsx
await waitFor(() => {
  expect(screen.getByText('Projekt (optional)')).toBeInTheDocument()
  const options = screen.getAllByRole('option').map((option) => option.textContent)
  expect(options).toContain('Villa Müller (Extern)')
  expect(options).toContain('CNC Vorbereitung (Werkstatt)')
})
```

## Shared Patterns

### Tenant-scoped request handling
**Sources:** `src/modules/inventory/api/routes.rs` lines 750-768, `src/modules/sites/application/site_service.rs` lines 370-380
**Apply to:** All backend request, service, and repository changes
```rust
Path(id): Path<String>,
ctx: TenantContext,

let site_id = request
    .site_id
    .map(|s| Uuid::parse_str(&s).map(SiteId))
    .transpose()
    .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

if let Some(site_id) = create.site_id {
    let _site = self.get_site(site_id, ctx).await?;
}
```

### AppError-first validation
**Sources:** `src/modules/inventory/api/routes.rs` lines 462-472, 759-768; `src/modules/sites/api/routes.rs` lines 494-500
**Apply to:** All new parsing and rule-enforcement paths
```rust
NaiveDate::parse_from_str(&date, "%Y-%m-%d")
    .map_err(|_| AppError::Validation(format!("Invalid {}", field_name)))

Uuid::parse_str(&id)
    .map(MaterialId)
    .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;
```

### Atomic persistence at repository boundary
**Source:** `src/modules/inventory/infrastructure/material_repository.rs` lines 423-533
**Apply to:** Any inventory rule hardening that changes both stock state and attribution history
```rust
let mut tx = self.pool.begin().await.map_err(|e| AppError::Database(e.to_string()))?;
// SELECT ... FOR UPDATE
// UPDATE materials
// INSERT stock_entries
tx.commit().await.map_err(|e| AppError::Database(e.to_string()))?;
```

### Active-project defaulting
**Sources:** `frontend/src/lib/api/hooks/usePreferences.ts` lines 8-13; `frontend/src/pages/inventory/InventoryDetailPage.tsx` lines 52-59, 372-387; `frontend/src/pages/sites/TimeEntryDialog.tsx` lines 95-100
**Apply to:** All frontend capture flows in this phase
```tsx
const { data: preferences } = usePreferences()
setSelectedSiteId(siteId ?? preferences?.active_site_id ?? "")
initialSiteId={withdrawSiteIdFromQuery ?? preferences?.active_site_id ?? null}
```

### React Query invalidation after capture mutations
**Sources:** `frontend/src/lib/api/hooks/useInventory.ts` lines 194-198; `frontend/src/lib/api/hooks/useSites.ts` lines 167-170, 181-185
**Apply to:** Inventory withdraw and time-entry create/update/delete mutations
```tsx
onSuccess: () => {
  queryClient.invalidateQueries({ queryKey: ["time-entries"] })
  queryClient.invalidateQueries({ queryKey: ["my-time-entries"] })
  queryClient.invalidateQueries({ queryKey: ["dashboard-sites"] })
}
```

### Event payload extension pattern
**Sources:** `src/modules/inventory/domain/events.rs` lines 29-50; `src/modules/sites/domain/events.rs` lines 69-88
**Apply to:** Only if planning chooses to enrich event payloads in Phase 46
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryCreatedPayload {
    pub site_id: Option<SiteId>,
    pub user_id: UserId,
    pub hours: f64,
    pub work_type: String,
    pub work_date: String,
}
```

## No Analog Found

None. Phase 46 is hardening existing capture paths rather than introducing a new subsystem.

## Metadata

**Analog search scope:** `src/modules/inventory/**`, `src/modules/sites/**`, `frontend/src/pages/inventory/**`, `frontend/src/pages/sites/**`, `frontend/src/lib/api/hooks/**`
**Files scanned:** 18
**Pattern extraction date:** 2026-05-07
