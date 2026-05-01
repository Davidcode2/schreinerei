# Architecture Patterns: Inventory Management v1.9 Improvements

**Domain:** Carpentry SaaS (Schreinerei) — inventory module extension
**Researched:** 2026-05-01
**Overall confidence:** HIGH (codebase-verified)

## Recommended Architecture

All v1.9 features extend the **existing inventory module**. No new modules needed. Follow the established hexagonal pattern with domain → application → infrastructure → API layers.

```
src/
├── common/                          # Unchanged
│   ├── events.rs                    # ADD: MaterialAdded, MaterialLocationChanged, CategoryUpdated, CategoryDeleted
│   └── types.rs                     # Unchanged (all types exist)
├── modules/
│   ├── inventory/
│   │   ├── domain/
│   │   │   ├── material.rs          # EXTEND: add UpdateMaterial command, StockIn command
│   │   │   ├── category.rs          # EXTEND: add UpdateCategory, DeleteCategory commands
│   │   │   ├── events.rs            # EXTEND: MaterialAdded, MaterialLocationChanged, CategoryUpdated payloads
│   │   │   ├── stock_entry.rs       # EXTEND: add entry_type discriminators for history enrichment
│   │   │   └── order_request.rs    # Unchanged
│   │   ├── application/
│   │   │   └── inventory_service.rs # EXTEND: add stock_in, update_material, update_category, delete_category
│   │   ├── infrastructure/
│   │   │   └── material_repository.rs # EXTEND: add update, stock-in, category CRUD queries
│   │   └── api/
│   │       └── routes.rs           # EXTEND: new endpoints (categories PUT/DELETE, materials PATCH, stock-in)
├── modules/sites/                    # Unchanged (reused for ActivityFeed component)
└── frontend/
    └── src/
        ├── pages/
        │   ├── inventory/
        │   │   ├── InventorySettingsPage.tsx    # NEW
        │   │   ├── InventoryDetailPage.tsx       # EXTEND: richer history, edit actions
        │   │   └── StockInDialog.tsx            # NEW
        │   └── settings/
        │       └── SettingsPage.tsx             # EXTEND: add inventory settings section
        ├── components/
        │   ├── inventory/
        │   │   ├── CategoryFilter.tsx            # EXTEND (or replace with settings page version)
        │   │   ├── MaterialHistoryFeed.tsx      # NEW: reusable history component
        │   │   ├── CategoryManager.tsx           # NEW: CRUD grid for categories
        │   │   └── MaterialEditDialog.tsx        # NEW: edit location, min_quantity
        │   └── shared/
        │       └── ActivityFeed.tsx              # EXISTING (reuse pattern, not component)
        └── types/
            ├── inventory.ts                      # EXTEND: add new DTOs
            └── generated.ts                      # AUTO-GENERATED (ts-rs)
```

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| `inventory::domain::material` | Material entity, update/stock-in commands | `events.rs` emits domain events |
| `inventory::domain::category` | Category entity, update/delete commands | `events.rs` emits domain events |
| `inventory::domain::events` | Event payloads for all new history types | `common::events::EventBus` |
| `inventory::domain::stock_entry` | Stock history entry with type discriminator | `infrastructure` persists with `entry_type` |
| `inventory::application::InventoryService` | Orchestrates category CRUD, material update, stock-in | `domain` commands, `infrastructure` repo |
| `inventory::infrastructure::MaterialRepository` | SQL queries for updates, category CRUD, stock-in | PostgreSQL |
| `inventory::api::routes` | New REST endpoints for settings, material edit, stock-in | `InventoryService` |
| `frontend::MaterialHistoryFeed` | Unified history display (replaces basic history list) | Inventory API endpoints |
| `frontend::CategoryManager` | Category CRUD UI in settings | Categories API endpoints |
| `frontend::InventorySettingsPage` | Settings page with category management | Settings + Categories APIs |

### Data Flow

**Category CRUD:**
```
Frontend CategoryManager → PUT/DELETE /api/v1/inventory/categories/{id}
  → routes.rs → InventoryService.update_category/delete_category
  → MaterialRepository → SQL UPDATE/DELETE
  → DomainEvent::CategoryUpdated/CategoryDeleted emitted
```

**Material Edit (location, min_quantity):**
```
Frontend MaterialEditDialog → PATCH /api/v1/inventory/materials/{id}
  → routes.rs → InventoryService.update_material
  → MaterialRepository → SQL UPDATE (location, min_quantity)
  → DomainEvent::MaterialLocationChanged emitted (if location changed)
```

**Material einlagern (stock-in):**
```
Frontend StockInDialog → POST /api/v1/inventory/materials/{id}/stock-in
  → routes.rs → InventoryService.stock_in
  → Material domain: validate positive quantity
  → MaterialRepository.withdraw_stock (reused!) → SQL INSERT stock_entries + UPDATE materials
  → DomainEvent::MaterialAdded emitted
```

**Inventory History (enriched):**
```
Frontend MaterialHistoryFeed → GET /api/v1/inventory/materials/{id}/history
  → routes.rs → MaterialRepository.list_stock_entries_with_site (existing)
  → Enhanced response with entry_type, user_name, category_name
  → Client renders colored badges per type
```

## Patterns to Follow

### Pattern 1: Domain Command + Validation (EXISTING)

**What:** Every mutation goes through a domain command with `validate()`.
**When:** All new write operations (stock-in, update material, update/delete category).
**Example:**
```rust
// domain/material.rs — NEW command
pub struct StockIn {
    pub material_id: MaterialId,
    pub quantity: i32,
    pub notes: Option<String>,
}

impl StockIn {
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= 0 {
            return Err("Stock-in quantity must be positive".to_string());
        }
        Ok(())
    }
}

pub struct UpdateMaterial {
    pub material_id: MaterialId,
    pub location: Option<String>,
    pub min_quantity: Option<i32>,
}

impl UpdateMaterial {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(min) = self.min_quantity {
            if min < 0 {
                return Err("Minimum quantity cannot be negative".to_string());
            }
        }
        Ok(())
    }
}
```

### Pattern 2: Event Payload → EventBus (EXISTING)

**What:** Every state change emits a domain event via the existing `EventBus`.
**When:** All new mutations that alter meaningful state.
**Example:**
```rust
// domain/events.rs — NEW payloads
pub struct MaterialAddedPayload {
    pub material_id: MaterialId,
    pub material_name: String,
    pub quantity_added: i32,
    pub quantity_after: i32,
    pub added_by: UserId,
    pub notes: Option<String>,
}

impl MaterialAddedPayload {
    pub fn into_event(self, tenant_id: TenantId) -> DomainEvent {
        DomainEvent::new(
            EventType::MaterialAdded,
            tenant_id,
            "Material",
            self.material_id.to_string(),
            json!(self),
        )
    }
}

pub struct MaterialLocationChangedPayload {
    pub material_id: MaterialId,
    pub material_name: String,
    pub old_location: Option<String>,
    pub new_location: Option<String>,
    pub changed_by: UserId,
}
```

### Pattern 3: Service Method → Repository → Transaction (EXISTING)

**What:** Application service validates, calls repository in transaction, emits events.
**When:** All new service methods. Follow `withdraw_material` and `adjust_stock` patterns.
**Example:**
```rust
// application/inventory_service.rs — NEW method
pub async fn stock_in(
    &self,
    stock_in: StockIn,
    ctx: &TenantContext,
) -> Result<Material, AppError> {
    stock_in.validate()?;
    let local_user_id = self.resolve_local_user_id(ctx).await?;

    // REUSE adjust_stock pattern: positive quantity adds to stock
    let material = self.material_repo
        .add_stock(stock_in.material_id, stock_in.quantity, local_user_id, &stock_in.notes, ctx.tenant_id)
        .await?;

    // Emit MaterialAdded event
    let event = MaterialAddedPayload {
        material_id: material.id,
        material_name: material.name.clone(),
        quantity_added: stock_in.quantity,
        quantity_after: material.quantity,
        added_by: local_user_id,
        notes: stock_in.notes,
    }.into_event(ctx.tenant_id);

    self.material_repo.publish_event(&event).await?;
    Ok(material)
}
```

### Pattern 4: Frontend Reusable History Feed (NEW — derived from sites ActivityFeed)

**What:** A `MaterialHistoryFeed` component that renders enriched history entries with color-coded badges.
**When:** Rendering inventory history on the detail page.
**Derivation:** The sites `ActivityFeed` already renders typed entries with icons and timestamps. Extract the same visual pattern but with inventory-specific types:

```tsx
// Color mapping for entry types
const entryTypeConfig = {
  withdrawal: { color: "red", label: "Entnahme", icon: ArrowDown },
  addition:   { color: "green", label: "Einlagerung", icon: ArrowUp },
  adjustment:  { color: "blue", label: "Anpassung", icon: Scale },
  location_change: { color: "amber", label: "Standort geändert", icon: MapPin },
}
```

**Key difference from sites ActivityFeed:** The sites feed already has a "Material" tab that renders `SiteMaterialHistoryEntry`. The new `MaterialHistoryFeed` renders `MaterialStockHistoryEntry` (enriched version). Same visual pattern, different data shape. Create a new component rather than sharing because the data shape is different and will diverge further.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Creating a Separate "Settings" Module

**What:** Creating `src/modules/settings/` for category CRUD.
**Why bad:** Categories are an inventory concern. They belong to the inventory bounded context. A separate module creates cross-module coupling and duplicates domain logic.
**Instead:** Add category update/delete to `inventory::domain::category`, `inventory_service`, and `material_repository`.

### Anti-Pattern 2: Bypassing Domain Validation for "Simple" Updates

**What:** Direct SQL UPDATE in repository for location/min_quantity changes without domain command validation.
**Why bad:** Skips the hexagonal architecture. Future validation rules (e.g., location length limits) would need to be added in two places.
**Instead:** Create `UpdateMaterial` command in `domain/material.rs` with `validate()`, call from service.

### Anti-Pattern 3: Stock-In as a Separate Code Path from Stock Adjustment

**What:** Duplicating the transaction logic from `adjust_stock`/`withdraw_stock` for stock-in.
**Why bad:** The existing `stock_entries` table already records all stock changes with `quantity_change` (positive or negative). Stock-in is just a positive `quantity_change` with an `entry_type` of "addition".
**Instead:** Add a new repository method `add_stock()` that follows the same transactional pattern as `withdraw_stock()` but with a positive `quantity_change` and an `entry_type` discriminator. Or better yet, extend `adjust_stock` to record `entry_type`.

### Anti-Pattern 4: Sharing the ActivityFeed Component Directly

**What:** Importing `ActivityFeed` from `sites` and trying to force inventory data into site activity shapes.
**Why bad:** The data shapes are fundamentally different (`Activity` with `activity_type` vs `StockEntryWithSite` with `quantity_change`). Forcing one into the other creates brittle adapter code.
**Instead:** Create `MaterialHistoryFeed` that follows the same visual pattern (cards with icons, timestamps, badges) but reads from the material history API directly. Both can share a `HistoryEntry` base component if visual consistency is important.

### Anti-Pattern 5: Adding entry_type as a String Column Without Enum

**What:** Creating `entry_type VARCHAR` in stock_entries without a Rust enum backing it.
**Why bad:** No compile-time safety. Typos in entry types cause silent failures.
**Instead:** Create a Rust `StockEntryType` enum in `domain/stock_entry.rs` with `FromStr`/`Display` impls, matching the pattern used by `Unit`, `SiteStatus`, etc.

## Scalability Considerations

| Concern | At 100 users | At 10K users | At 1M users |
|---------|--------------|--------------|-------------|
| History query performance | Indexed lookups on `stock_entries(material_id, created_at DESC)` — fast | Add `entry_type` to composite index, consider pagination limit default 50 | Partition by `tenant_id`, add materialized view for recent history |
| Category CRUD on settings page | Simple list + inline edit — straightforward | Consider caching categories in frontend (30s stale time already set) | Materialized view for category counts |
| Event table growth | Small, acceptable | Index optimization on `domain_events(tenant_id, event_type)` | Archive old events, consider event compaction |
| Material list with category string | JOIN on every list query — fine for small data | Consider adding `category_name` denormalized field to materials | Pre-computed category string from material select |

## Phase-Specific Build Order

### Phase 1: Backend Foundation — Category CRUD + Material Update Commands

**Why first:** All frontend features depend on API endpoints existing. Build domain + application + infrastructure + API layers for:

1. **Category update** — `PUT /api/v1/inventory/categories/{id}`, `UpdateCategory` command
2. **Category delete** — `DELETE /api/v1/inventory/categories/{id}`, `DeleteCategory` command (with constraint check: can't delete if materials reference it)
3. **Material update** — `PATCH /api/v1/inventory/materials/{id}`, `UpdateMaterial` command (location, min_quantity)
4. **Stock-in** — `POST /api/v1/inventory/materials/{id}/stock-in`, `StockIn` command
5. **New event types** — `EventType::MaterialAdded`, `EventType::MaterialLocationChanged`, `EventType::CategoryUpdated`, `EventType::CategoryDeleted`
6. **Enhanced history** — Extend `stock_entries` with `entry_type` enum column + migration, enrich `StockEntryResponse` with `user_name`, `category_name`, `entry_type`

### Phase 2: Frontend — Settings Page + Material Edit + Stock-In Dialog

**Why second:** Depends on Phase 1 APIs.

1. **InventorySettingsPage** — New route `/settings/inventory` with `CategoryManager` component
2. **MaterialEditDialog** — Dialog for editing location and min_quantity on material detail page
3. **StockInDialog** — New dialog for "Material einlagern" action, mirrors `WithdrawDialog` pattern
4. **Category display** — Add `category_name` to `MaterialResponse` (backend join or denormalized)

### Phase 3: Frontend — Enriched History + Clickable Baustelle Links

**Why third:** Depends on Phase 1's enhanced history data.

1. **MaterialHistoryFeed** — New component replacing basic history in `InventoryDetailPage`, renders color-coded entry types
2. **Clickable site links** — Link `site_name` entries to `/sites/{site_id}` using React Router `Link`
3. **User attribution** — Show "von {user_name}" in history entries

### Phase 4: ts-rs Type Generation + Tests

**Why last:** Depends on all DTOs being finalized.

1. **ts-rs export** — Add `#[ts(export)]` to all new DTOs, run generation, verify `generated.ts` matches
2. **Backend tests** — Domain command validation tests for `StockIn`, `UpdateMaterial`, `UpdateCategory`, `DeleteCategory`
3. **E2E tests** — Playwright tests for settings page, stock-in dialog, history enrichment

## Database Migration Strategy

### Migration 014: Add entry_type to stock_entries

```sql
-- Add entry_type enum for stock history discrimination
CREATE TYPE stock_entry_type AS ENUM (
    'withdrawal',    -- Material entnommen (negative change)
    'addition',      -- Material eingelagert (positive change via stock-in)
    'adjustment',    -- Stock adjustment (admin correction)
    'location_change' -- Location changed (metadata-only, quantity unchanged)
);

ALTER TABLE stock_entries ADD COLUMN entry_type stock_entry_type NOT NULL DEFAULT 'withdrawal';

-- Update existing rows: negative changes are withdrawals, positive are adjustments
UPDATE stock_entries SET entry_type = 'withdrawal' WHERE quantity_change < 0;
UPDATE stock_entries SET entry_type = 'adjustment' WHERE quantity_change > 0;

CREATE INDEX idx_stock_entries_type ON stock_entries(material_id, entry_type);
```

### Migration 015: Add category_name to materials (denormalization)

```sql
-- Add denormalized category_name for efficient listing
-- (Optional: can also be done as a JOIN in the query)
-- No schema change needed — JOIN is preferred for now.
-- Only add this column if listing queries become slow.
```

**Decision:** Use JOIN in the material listing query to include `category_name` rather than denormalizing. This mirrors the `SiteStockHistoryRow` pattern which already JOINs `categories`.

## Category Delete Constraint

Deleting a category that has materials is a data integrity concern. Two approaches:

| Approach | Implementation | Tradeoff |
|----------|---------------|----------|
| **Block delete** (recommended) | Check `COUNT(*) FROM materials WHERE category_id = ? AND deleted_at IS NULL` in service. Return `AppError::Conflict` if materials exist. | Safe, explicit. Forces admin to re-categorize or move materials first. |
| **Reassign to "Uncategorized"** | Auto-assign orphaned materials to a system category. | More convenient but creates hidden behavior. Risk of "Uncategorized" category being meaningless in Schreinerei context. |

**Recommendation:** Block delete with explicit error message. This follows the pattern already established in `delete_material` which checks for pending order requests.

## Settings Page Routing

The current `SettingsPage.tsx` has `ProfileSection` and `UserManagementSection`. The inventory settings should be:

**Option A (Recommended): Route under Settings**
```
/settings              → existing SettingsPage
/settings/inventory    → new InventorySettingsPage
```

Add a navigation card in `SettingsPage.tsx` linking to `/settings/inventory`. This keeps settings centralized and discoverable.

**Option B: Tab within SettingsPage**
Add a tabbed interface with "Profile", "Benutzer", "Inventar" tabs. More complex, but avoids extra pages.

**Recommendation:** Option A — separate page linked from Settings. Follows the pattern of how sites/inventory/fleet all have their own pages under a nav hierarchy. simpler to build and test.

### Route Addition in App.tsx

```tsx
// Add to protected routes
<Route path="/settings/inventory" element={<InventorySettingsPage />} />
```

And add a link card in `SettingsPage.tsx`:
```tsx
<Card>
  <CardHeader>
    <CardTitle className="flex items-center gap-2">
      <Package className="h-5 w-5" />
      Inventar-Einstellungen
    </CardTitle>
    <CardDescription>Kategorien verwalten</CardDescription>
  </CardHeader>
  <CardContent>
    <Button variant="outline" onClick={() => navigate("/settings/inventory")}>
      Verwalten
    </Button>
  </CardContent>
</Card>
```

## Stock-In: Relationship to Existing Deduction System

The "Material einlagern" (stock-in) action is the **inverse** of "Material entnehmen" (withdraw). Both share the same `stock_entries` audit table. The key architectural insight:

| Aspect | Withdraw (existing) | Stock-In (new) |
|--------|---------------------|-----------------|
| `quantity_change` | Negative (e.g., -5) | Positive (e.g., +10) |
| `entry_type` | `withdrawal` | `addition` |
| Domain command | `WithdrawMaterial` | `StockIn` |
| Validation | `can_withdraw(amount)` | `quantity > 0` |
| Event | `StockWithdrawn` | `MaterialAdded` |
| Admin-only? | No (all users) | No (all users) — any employee can stock in |
| Site link | Optional `site_id` | Optional `site_id` (where did delivery arrive?) |

**Architecture decision:** Create a separate `add_stock()` method in `MaterialRepository` rather than reusing `adjust_stock()`. Reason:
- `adjust_stock` is admin-only and uses a `reason` string
- `stock_in` is available to all users and uses `notes` + optional `site_id`
- The repository pattern splits these concerns cleanly at the SQL level while sharing the same table

The `stock_entries` table already has a `site_id` column (added for withdrawals linked to Baustellen). Stock-in can use this same column to indicate which site received the delivery.

## Extending EventType for New History Entries

The current `EventType` enum in `common/events.rs` already has inventory events. New entries:

```rust
pub enum EventType {
    // Existing
    MaterialCreated,
    StockWithdrawn,
    StockLow,
    StockAdjusted,
    OrderRequestCreated,
    // NEW - inventory v1.9
    MaterialAdded,           // Stock-in: material was restocked
    MaterialLocationChanged, // Location field was updated
    MaterialUpdated,         // Min quantity or other fields changed
    CategoryUpdated,         // Category name/description changed
    CategoryDeleted,         // Category removed
    // Existing - sites
    SiteCreated,
    SiteStatusChanged,
    // ... etc
}
```

These new event types are purely additive — they don't break existing consumers. The `event_type` column in `domain_events` is stored as a string (serialized via `serde_json`), so no migration needed for the table structure itself.

## Sources

- Codebase analysis: `src/modules/inventory/` — all layers examined (domain, application, infrastructure, API)
- Codebase analysis: `src/common/events.rs` — EventType enum and EventBus
- Codebase analysis: `src/common/types.rs` — all domain types
- Codebase analysis: `frontend/src/pages/inventory/` — existing UI patterns
- Codebase analysis: `frontend/src/pages/sites/ActivityFeed.tsx` — reusable visual pattern
- Codebase analysis: `frontend/src/lib/api/hooks/useInventory.ts` — existing React Query hooks
- Codebase analysis: `migrations/002_inventory_schema.sql` — current DB schema