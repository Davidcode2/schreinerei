use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::events::{EventBus, DomainEvent};
use crate::common::types::{TenantId, MaterialId, CategoryId, UserId, Unit, OrderRequestId, SiteId};
use crate::modules::inventory::domain::{
    Category, Material, CreateCategory, CreateMaterial,
    OrderRequest, OrderStatus, CreateOrderRequest,
    StockEntryWithSite,
};

/// Repository for material data access with tenant isolation
pub struct MaterialRepository {
    pool: PgPool,
    event_bus: EventBus,
}

impl MaterialRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { 
            pool,
            event_bus: EventBus::new(),
        }
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    // === Category operations ===

    pub async fn create_category(
        &self,
        create: &CreateCategory,
        tenant_id: TenantId,
    ) -> Result<Category, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let category = sqlx::query_as::<_, CategoryRow>(
            r#"
            INSERT INTO categories (id, tenant_id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, name, description, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create.name)
        .bind(&create.description)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                AppError::Validation("Category with this name already exists".to_string())
            } else {
                AppError::Database(e.to_string())
            }
        })?;

        Ok(category.into_category())
    }

    pub async fn list_categories(
        &self,
        tenant_id: TenantId,
    ) -> Result<Vec<Category>, AppError> {
        let categories = sqlx::query_as::<_, CategoryRow>(
            r#"
            SELECT id, tenant_id, name, description, created_at, updated_at
            FROM categories
            WHERE tenant_id = $1
            ORDER BY name
            "#
        )
        .bind(tenant_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(categories.into_iter().map(|c| c.into_category()).collect())
    }

    pub async fn find_category_by_id(
        &self,
        id: CategoryId,
        tenant_id: TenantId,
    ) -> Result<Option<Category>, AppError> {
        let category = sqlx::query_as::<_, CategoryRow>(
            r#"
            SELECT id, tenant_id, name, description, created_at, updated_at
            FROM categories
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(category.map(|c| c.into_category()))
    }

    // === Material operations ===

    pub async fn create_material(
        &self,
        create: &CreateMaterial,
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let material = sqlx::query_as::<_, MaterialRow>(
            r#"
            INSERT INTO materials (id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(create.category_id.0)
        .bind(&create.name)
        .bind(&create.description)
        .bind(create.unit.to_string())
        .bind(create.quantity)
        .bind(create.min_quantity)
        .bind(&create.location)
        .bind(&Option::<String>::None)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("unique") || err_str.contains("duplicate") {
                AppError::Validation("Material with this name already exists".to_string())
            } else if err_str.contains("foreign key") {
                AppError::Validation("Category not found".to_string())
            } else {
                AppError::Database(err_str)
            }
        })?;

        Ok(material.into_material())
    }

    pub async fn list_materials(
        &self,
        tenant_id: TenantId,
        category_id: Option<CategoryId>,
    ) -> Result<Vec<Material>, AppError> {
        let materials = match category_id {
            Some(cat_id) => {
                sqlx::query_as::<_, MaterialRow>(
                    r#"
                    SELECT id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
                    FROM materials
                    WHERE tenant_id = $1 AND category_id = $2 AND deleted_at IS NULL
                    ORDER BY name
                    "#
                )
                .bind(tenant_id.0)
                .bind(cat_id.0)
                .fetch_all(&self.pool)
                .await
            }
            None => {
                sqlx::query_as::<_, MaterialRow>(
                    r#"
                    SELECT id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
                    FROM materials
                    WHERE tenant_id = $1 AND deleted_at IS NULL
                    ORDER BY name
                    "#
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(materials.into_iter().map(|m| m.into_material()).collect())
    }

    pub async fn find_material_by_id(
        &self,
        id: MaterialId,
        tenant_id: TenantId,
    ) -> Result<Option<Material>, AppError> {
        let material = sqlx::query_as::<_, MaterialRow>(
            r#"
            SELECT id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
            FROM materials
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(material.map(|m| m.into_material()))
    }

    pub async fn find_material_by_qr_code(
        &self,
        qr_code: &str,
        tenant_id: TenantId,
    ) -> Result<Option<Material>, AppError> {
        let material = sqlx::query_as::<_, MaterialRow>(
            r#"
            SELECT id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
            FROM materials
            WHERE qr_code = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#
        )
        .bind(qr_code)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(material.map(|m| m.into_material()))
    }

    // === Stock operations ===

    pub async fn withdraw_stock(
        &self,
        material_id: MaterialId,
        quantity: i32,
        user_id: UserId,
        notes: Option<String>,
        site_id: Option<SiteId>,  // Optional link to Baustelle
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        // Use transaction for atomic update + audit log
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Get current material with lock
        let current = sqlx::query_as::<_, MaterialRow>(
            r#"
            SELECT id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
            FROM materials
            WHERE id = $1 AND tenant_id = $2
            FOR UPDATE
            "#
        )
        .bind(material_id.0)
        .bind(tenant_id.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        let current_material = current.into_material();

        if !current_material.can_withdraw(quantity) {
            return Err(AppError::Validation(
                format!("Insufficient stock. Current: {}, Requested: {}", current_material.quantity, quantity)
            ));
        }

        let new_quantity = current_material.quantity - quantity;

        // Update material stock
        let updated = sqlx::query_as::<_, MaterialRow>(
            r#"
            UPDATE materials
            SET quantity = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
            "#
        )
        .bind(new_quantity)
        .bind(material_id.0)
        .bind(tenant_id.0)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Create audit entry
        sqlx::query(
            r#"
            INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, site_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id.0)
        .bind(material_id.0)
        .bind(user_id.0)
        .bind(-quantity)
        .bind(new_quantity)
        .bind(&notes)
        .bind(site_id.map(|s| s.0))  // Optional site_id
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(updated.into_material())
    }

    pub async fn adjust_stock(
        &self,
        material_id: MaterialId,
        quantity_change: i32,
        reason: &str,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Update stock (can be positive or negative)
        let updated = sqlx::query_as::<_, MaterialRow>(
            r#"
            UPDATE materials
            SET quantity = quantity + $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
            "#
        )
        .bind(quantity_change)
        .bind(material_id.0)
        .bind(tenant_id.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        let material = updated.into_material();

        // Create audit entry
        sqlx::query(
            r#"
            INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id.0)
        .bind(material_id.0)
        .bind(user_id.0)
        .bind(quantity_change)
        .bind(material.quantity)
        .bind(reason)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(material)
    }

    pub async fn update_qr_code(
        &self,
        material_id: MaterialId,
        qr_code: &str,
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        let material = sqlx::query_as::<_, MaterialRow>(
            r#"
            UPDATE materials
            SET qr_code = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
            "#
        )
        .bind(qr_code)
        .bind(material_id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                AppError::Validation("QR code already in use".to_string())
            } else {
                AppError::Database(e.to_string())
            }
        })?
        .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        Ok(material.into_material())
    }

    pub async fn list_low_stock_materials(
        &self,
        tenant_id: TenantId,
    ) -> Result<Vec<Material>, AppError> {
        let materials = sqlx::query_as::<_, MaterialRow>(
            r#"
            SELECT id, tenant_id, category_id, name, description, unit, quantity, min_quantity, location, qr_code, created_at, updated_at
            FROM materials
            WHERE tenant_id = $1 AND quantity <= min_quantity AND deleted_at IS NULL
            ORDER BY quantity ASC
            "#
        )
        .bind(tenant_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(materials.into_iter().map(|m| m.into_material()).collect())
    }

    /// Count pending order requests for a material (for delete dependency check)
    pub async fn count_pending_order_requests(
        &self,
        material_id: MaterialId,
        tenant_id: TenantId,
    ) -> Result<i64, AppError> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM order_requests
            WHERE tenant_id = $1 AND material_id = $2 AND status = 'pending'
            "#
        )
        .bind(tenant_id.0)
        .bind(material_id.0)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(count)
    }

    /// Soft delete a material by setting deleted_at timestamp
    pub async fn delete_material(
        &self,
        id: MaterialId,
        tenant_id: TenantId,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE materials
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Material not found".to_string()));
        }

        Ok(())
    }

    /// List stock entries for a material with optional site name
    pub async fn list_stock_entries_with_site(
        &self,
        material_id: MaterialId,
        tenant_id: TenantId,
        limit: i32,
    ) -> Result<Vec<StockEntryWithSite>, AppError> {
        let entries = sqlx::query_as::<_, StockEntryRow>(
            r#"
            SELECT 
                se.id, se.tenant_id, se.material_id, se.user_id, 
                se.quantity_change, se.quantity_after, se.notes, 
                se.site_id, se.created_at,
                s.name as site_name
            FROM stock_entries se
            LEFT JOIN sites s ON se.site_id = s.id
            WHERE se.material_id = $1 AND se.tenant_id = $2
            ORDER BY se.created_at DESC
            LIMIT $3
            "#
        )
        .bind(material_id.0)
        .bind(tenant_id.0)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entries.into_iter().map(|row| row.into_stock_entry_with_site()).collect())
    }

    // === Order Request operations ===

    pub async fn create_order_request(
        &self,
        create: &CreateOrderRequest,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<OrderRequest, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let order = sqlx::query_as::<_, OrderRequestRow>(
            r#"
            INSERT INTO order_requests (id, tenant_id, material_id, quantity, requested_by, status, reason, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, material_id, quantity, requested_by, status, reason, approved_by, approved_at, fulfilled_at, notes, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(create.material_id.0)
        .bind(create.quantity)
        .bind(user_id.0)
        .bind("pending")
        .bind(&create.reason)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(order.into_order_request())
    }

    pub async fn list_order_requests(
        &self,
        tenant_id: TenantId,
        status: Option<OrderStatus>,
    ) -> Result<Vec<OrderRequest>, AppError> {
        let orders = match status {
            Some(s) => {
                sqlx::query_as::<_, OrderRequestRow>(
                    r#"
                    SELECT id, tenant_id, material_id, quantity, requested_by, status, reason, approved_by, approved_at, fulfilled_at, notes, created_at, updated_at
                    FROM order_requests
                    WHERE tenant_id = $1 AND status = $2
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(s.as_str())
                .fetch_all(&self.pool)
                .await
            }
            None => {
                sqlx::query_as::<_, OrderRequestRow>(
                    r#"
                    SELECT id, tenant_id, material_id, quantity, requested_by, status, reason, approved_by, approved_at, fulfilled_at, notes, created_at, updated_at
                    FROM order_requests
                    WHERE tenant_id = $1
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(orders.into_iter().map(|o| o.into_order_request()).collect())
    }

    pub async fn approve_order_request(
        &self,
        id: OrderRequestId,
        user_id: UserId,
        notes: Option<String>,
        tenant_id: TenantId,
    ) -> Result<OrderRequest, AppError> {
        let now = Utc::now();

        let order = sqlx::query_as::<_, OrderRequestRow>(
            r#"
            UPDATE order_requests
            SET status = 'approved', approved_by = $1, approved_at = $2, notes = COALESCE($3, notes), updated_at = $2
            WHERE id = $4 AND tenant_id = $5 AND status = 'pending'
            RETURNING id, tenant_id, material_id, quantity, requested_by, status, reason, approved_by, approved_at, fulfilled_at, notes, created_at, updated_at
            "#
        )
        .bind(user_id.0)
        .bind(now)
        .bind(&notes)
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Order request not found or already processed".to_string()))?;

        Ok(order.into_order_request())
    }

    pub async fn fulfill_order_request(
        &self,
        id: OrderRequestId,
        actual_quantity: i32,
        notes: Option<String>,
        tenant_id: TenantId,
    ) -> Result<OrderRequest, AppError> {
        let now = Utc::now();

        // Use transaction to update both order and material stock
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Get the order request
        let order: OrderRequestRow = sqlx::query_as(
            r#"
            SELECT id, tenant_id, material_id, quantity, requested_by, status, reason, approved_by, approved_at, fulfilled_at, notes, created_at, updated_at
            FROM order_requests
            WHERE id = $1 AND tenant_id = $2 AND status = 'approved'
            FOR UPDATE
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Order request not found or not approved".to_string()))?;

        // Update material stock
        sqlx::query(
            r#"
            UPDATE materials
            SET quantity = quantity + $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            "#
        )
        .bind(actual_quantity)
        .bind(order.material_id)
        .bind(tenant_id.0)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // Update order status
        let updated = sqlx::query_as::<_, OrderRequestRow>(
            r#"
            UPDATE order_requests
            SET status = 'fulfilled', fulfilled_at = $1, notes = COALESCE($2, notes), updated_at = $1
            WHERE id = $3 AND tenant_id = $4
            RETURNING id, tenant_id, material_id, quantity, requested_by, status, reason, approved_by, approved_at, fulfilled_at, notes, created_at, updated_at
            "#
        )
        .bind(now)
        .bind(&notes)
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(updated.into_order_request())
    }

    pub async fn find_order_request_by_id(
        &self,
        id: OrderRequestId,
        tenant_id: TenantId,
    ) -> Result<Option<OrderRequest>, AppError> {
        let order = sqlx::query_as::<_, OrderRequestRow>(
            r#"
            SELECT id, tenant_id, material_id, quantity, requested_by, status, reason, approved_by, approved_at, fulfilled_at, notes, created_at, updated_at
            FROM order_requests
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(order.map(|o| o.into_order_request()))
    }

    // === Event publishing ===

    pub async fn publish_event(&self, event: &DomainEvent) -> Result<(), AppError> {
        self.event_bus.publish(event, &self.pool).await
    }
}

// === Database row types ===

#[derive(Debug, FromRow)]
struct CategoryRow {
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CategoryRow {
    fn into_category(self) -> Category {
        Category {
            id: CategoryId(self.id),
            tenant_id: TenantId(self.tenant_id),
            name: self.name,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct MaterialRow {
    id: Uuid,
    tenant_id: Uuid,
    category_id: Uuid,
    name: String,
    description: Option<String>,
    unit: String,
    quantity: i32,
    min_quantity: i32,
    location: Option<String>,
    qr_code: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MaterialRow {
    fn into_material(self) -> Material {
        Material {
            id: MaterialId(self.id),
            tenant_id: TenantId(self.tenant_id),
            category_id: CategoryId(self.category_id),
            name: self.name,
            description: self.description,
            unit: self.unit.parse().unwrap_or(Unit::Piece),
            quantity: self.quantity,
            min_quantity: self.min_quantity,
            location: self.location,
            qr_code: self.qr_code,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct OrderRequestRow {
    id: Uuid,
    tenant_id: Uuid,
    material_id: Uuid,
    quantity: i32,
    requested_by: Uuid,
    status: String,
    reason: Option<String>,
    approved_by: Option<Uuid>,
    approved_at: Option<DateTime<Utc>>,
    fulfilled_at: Option<DateTime<Utc>>,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl OrderRequestRow {
    fn into_order_request(self) -> OrderRequest {
        OrderRequest {
            id: OrderRequestId(self.id),
            tenant_id: TenantId(self.tenant_id),
            material_id: MaterialId(self.material_id),
            quantity: self.quantity,
            requested_by: UserId(self.requested_by),
            status: self.status.parse().unwrap_or(OrderStatus::Pending),
            reason: self.reason,
            approved_by: self.approved_by.map(UserId),
            approved_at: self.approved_at,
            fulfilled_at: self.fulfilled_at,
            notes: self.notes,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct StockEntryRow {
    id: Uuid,
    tenant_id: Uuid,
    material_id: Uuid,
    user_id: Uuid,
    quantity_change: i32,
    quantity_after: i32,
    notes: Option<String>,
    site_id: Option<Uuid>,
    created_at: DateTime<Utc>,
    site_name: Option<String>,
}

impl StockEntryRow {
    fn into_stock_entry_with_site(self) -> StockEntryWithSite {
        StockEntryWithSite {
            id: self.id,
            tenant_id: TenantId(self.tenant_id),
            material_id: MaterialId(self.material_id),
            user_id: UserId(self.user_id),
            quantity_change: self.quantity_change,
            quantity_after: self.quantity_after,
            notes: self.notes,
            site_id: self.site_id.map(SiteId),
            site_name: self.site_name,
            created_at: self.created_at,
        }
    }
}
