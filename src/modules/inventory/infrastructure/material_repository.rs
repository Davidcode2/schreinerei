use chrono::{DateTime, Duration, NaiveDate, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::events::{DomainEvent, EventBus};
use crate::common::types::{
    CategoryId, MaterialId, OrderRequestId, SiteId, TenantId, Unit, UserId,
};
use crate::modules::inventory::domain::{
    Category, CreateCategory, CreateMaterial, CreateOrderRequest, EnrichedStockEntry, EntryType,
    Material, MaterialBatchSummary, OrderRequest, OrderStatus, SiteStockHistoryEntry,
    StockEntryWithSite, UpdateCategory, UpdateMaterial,
};

/// Repository for material data access with tenant isolation
pub struct MaterialRepository {
    pool: PgPool,
    event_bus: EventBus,
}

impl MaterialRepository {
    const EXPIRY_WARNING_DAYS: i64 = 10;

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
            INSERT INTO categories (id, tenant_id, name, description, can_expire, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, name, description, can_expire, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create.name)
        .bind(&create.description)
        .bind(create.can_expire)
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

    pub async fn list_categories(&self, tenant_id: TenantId) -> Result<Vec<Category>, AppError> {
        let categories = sqlx::query_as::<_, CategoryRow>(
            r#"
            SELECT id, tenant_id, name, description, can_expire, created_at, updated_at
            FROM categories
            WHERE tenant_id = $1
            ORDER BY name
            "#,
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
            SELECT id, tenant_id, name, description, can_expire, created_at, updated_at
            FROM categories
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(category.map(|c| c.into_category()))
    }

    pub async fn update_category(
        &self,
        id: CategoryId,
        update: &UpdateCategory,
        tenant_id: TenantId,
    ) -> Result<Category, AppError> {
        let category = sqlx::query_as::<_, CategoryRow>(
            r#"
            UPDATE categories
            SET name = COALESCE($2, name),
                description = CASE
                    WHEN $3 IS NULL THEN description
                    WHEN $3 = '' THEN NULL
                    ELSE $3
                END,
                can_expire = COALESCE($4, can_expire),
                updated_at = NOW()
            WHERE id = $1 AND tenant_id = $5
            RETURNING id, tenant_id, name, description, can_expire, created_at, updated_at
            "#,
        )
        .bind(id.0)
        .bind(update.name.as_deref())
        .bind(update.description.as_deref())
        .bind(update.can_expire)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                AppError::Validation("Category with this name already exists".to_string())
            } else {
                AppError::Database(e.to_string())
            }
        })?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

        Ok(category.into_category())
    }

    pub async fn delete_category(
        &self,
        id: CategoryId,
        tenant_id: TenantId,
    ) -> Result<(), AppError> {
        // Check if any materials reference this category
        let count: i64 = sqlx::query_scalar(Self::delete_category_conflict_count_query())
            .bind(id.0)
            .bind(tenant_id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if count > 0 {
            return Err(AppError::Conflict(
                "Cannot delete category: material history must be preserved".to_string(),
            ));
        }

        let result = sqlx::query(
            r#"
            DELETE FROM categories
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503") => {
                AppError::Conflict(
                    "Cannot delete category: materials still reference it".to_string(),
                )
            }
            _ => AppError::Database(e.to_string()),
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Category not found".to_string()));
        }

        Ok(())
    }

    fn delete_category_conflict_count_query() -> &'static str {
        r#"
            SELECT COUNT(*) FROM materials
            WHERE category_id = $1 AND tenant_id = $2
            "#
    }

    // === Material operations ===

    pub async fn create_material(
        &self,
        create: &CreateMaterial,
        tenant_id: TenantId,
        can_expire: bool,
    ) -> Result<Material, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO materials (
                id, tenant_id, category_id, name, description, unit, quantity,
                min_quantity, legacy_quantity, location, qr_code, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(create.category_id.0)
        .bind(&create.name)
        .bind(&create.description)
        .bind(create.unit.to_string())
        .bind(create.quantity)
        .bind(create.min_quantity)
        .bind(if can_expire { 0 } else { create.quantity })
        .bind(&create.location)
        .bind(&Option::<String>::None)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
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

        if can_expire && create.quantity > 0 {
            self.insert_batch(
                &mut tx,
                tenant_id,
                MaterialId(id),
                create.quantity,
                create.expires_on.expect("validated expiry date"),
                now,
            )
            .await?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_material_by_id(MaterialId(id), tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))
    }

    pub async fn list_materials(
        &self,
        tenant_id: TenantId,
        category_id: Option<CategoryId>,
    ) -> Result<Vec<Material>, AppError> {
        let materials = match category_id {
            Some(cat_id) => {
                sqlx::query_as::<_, MaterialRow>(&Self::material_list_query(true))
                    .bind(tenant_id.0)
                    .bind(cat_id.0)
                    .fetch_all(&self.pool)
                    .await
            }
            None => {
                sqlx::query_as::<_, MaterialRow>(&Self::material_list_query(false))
                    .bind(tenant_id.0)
                    .fetch_all(&self.pool)
                    .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(materials
            .into_iter()
            .map(MaterialRow::into_material_without_batches)
            .collect())
    }

    pub async fn find_material_by_id(
        &self,
        id: MaterialId,
        tenant_id: TenantId,
    ) -> Result<Option<Material>, AppError> {
        let material = sqlx::query_as::<_, MaterialRow>(&Self::material_detail_query(
            "m.id = $1 AND m.tenant_id = $2 AND m.deleted_at IS NULL",
        ))
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let Some(material) = material else {
            return Ok(None);
        };

        let batches = self.list_expiry_batches(id, tenant_id).await?;
        Ok(Some(material.into_material_with_batches(batches)))
    }

    pub async fn find_material_by_qr_code(
        &self,
        qr_code: &str,
        tenant_id: TenantId,
    ) -> Result<Option<Material>, AppError> {
        let material = sqlx::query_as::<_, MaterialRow>(&Self::material_detail_query(
            "m.qr_code = $1 AND m.tenant_id = $2 AND m.deleted_at IS NULL",
        ))
        .bind(qr_code)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let Some(material) = material else {
            return Ok(None);
        };

        let batches = self
            .list_expiry_batches(MaterialId(material.id), tenant_id)
            .await?;
        Ok(Some(material.into_material_with_batches(batches)))
    }

    pub async fn update_material(
        &self,
        id: MaterialId,
        update: &UpdateMaterial,
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        // Handle clear_location separately from location set
        let (location_param, clear_location) = match (&update.location, update.clear_location) {
            // clear_location is true: explicitly set location to NULL
            (_, Some(true)) => (None::<String>, true),
            // location is Some: set location to new value
            (Some(loc), _) => (Some(loc.clone()), false),
            // Neither: don't change location
            (None, _) => (None, false),
        };

        let updated_id = if clear_location {
            sqlx::query_scalar::<_, Uuid>(
                r#"
                UPDATE materials
                SET location = NULL,
                    min_quantity = COALESCE($2, min_quantity),
                    updated_at = NOW()
                WHERE id = $3 AND tenant_id = $4 AND deleted_at IS NULL
                RETURNING id
                "#,
            )
            .bind(location_param)
            .bind(update.min_quantity)
            .bind(id.0)
            .bind(tenant_id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?
        } else {
            sqlx::query_scalar::<_, Uuid>(
                r#"
                UPDATE materials
                SET location = COALESCE($1, location),
                    min_quantity = COALESCE($2, min_quantity),
                    updated_at = NOW()
                WHERE id = $3 AND tenant_id = $4 AND deleted_at IS NULL
                RETURNING id
                "#,
            )
            .bind(location_param)
            .bind(update.min_quantity)
            .bind(id.0)
            .bind(tenant_id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?
        };

        self.find_material_by_id(MaterialId(updated_id), tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))
    }

    // === Stock operations ===

    #[allow(clippy::too_many_arguments)]
    pub async fn withdraw_stock(
        &self,
        material_id: MaterialId,
        quantity: i32,
        user_id: UserId,
        notes: Option<String>,
        site_id: Option<SiteId>, // Optional link to Baustelle
        disposal: bool,
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let current = sqlx::query_as::<_, MaterialLockRow>(
            r#"
            SELECT m.quantity, m.legacy_quantity, c.can_expire
            FROM materials m
            INNER JOIN categories c ON c.id = m.category_id
            WHERE m.id = $1 AND m.tenant_id = $2 AND m.deleted_at IS NULL
            FOR UPDATE
            "#,
        )
        .bind(material_id.0)
        .bind(tenant_id.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        if current.quantity < quantity {
            return Err(AppError::Validation(format!(
                "Insufficient stock. Current: {}, Requested: {}",
                current.quantity, quantity
            )));
        }

        let new_quantity = current.quantity - quantity;

        if disposal {
            let expired_total = self
                .total_batch_quantity_for_update(&mut tx, material_id, tenant_id, true)
                .await?;
            if expired_total < quantity {
                return Err(AppError::Validation(format!(
                    "Insufficient expired stock. Current expired: {}, Requested disposal: {}",
                    expired_total, quantity
                )));
            }

            self.consume_batches(&mut tx, material_id, tenant_id, quantity, true)
                .await?;
            self.update_material_quantities(
                &mut tx,
                material_id,
                tenant_id,
                new_quantity,
                current.legacy_quantity,
            )
            .await?;

            sqlx::query(
                r#"
                INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, site_id, entry_type, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, NULL, 'disposed', $8)
                "#
            )
            .bind(Uuid::new_v4())
            .bind(tenant_id.0)
            .bind(material_id.0)
            .bind(user_id.0)
            .bind(-quantity)
            .bind(new_quantity)
            .bind(&notes)
            .bind(Utc::now())
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        } else {
            let consumed_from_legacy = current.legacy_quantity.min(quantity);
            let remaining = quantity - consumed_from_legacy;
            let new_legacy_quantity = current.legacy_quantity - consumed_from_legacy;

            if remaining > 0 {
                self.consume_batches(&mut tx, material_id, tenant_id, remaining, false)
                    .await?;
            }

            self.update_material_quantities(
                &mut tx,
                material_id,
                tenant_id,
                new_quantity,
                new_legacy_quantity,
            )
            .await?;

            sqlx::query(
                r#"
                INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, site_id, entry_type, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'withdrawn', $9)
                "#
            )
            .bind(Uuid::new_v4())
            .bind(tenant_id.0)
            .bind(material_id.0)
            .bind(user_id.0)
            .bind(-quantity)
            .bind(new_quantity)
            .bind(&notes)
            .bind(site_id.map(|s| s.0))
            .bind(Utc::now())
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        }
        tx.commit()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_material_by_id(material_id, tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))
    }

    pub async fn adjust_stock(
        &self,
        material_id: MaterialId,
        quantity_change: i32,
        reason: &str,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let current = sqlx::query_as::<_, MaterialLockRow>(
            r#"
            SELECT m.quantity, m.legacy_quantity, c.can_expire
            FROM materials m
            INNER JOIN categories c ON c.id = m.category_id
            WHERE m.id = $1 AND m.tenant_id = $2 AND m.deleted_at IS NULL
            FOR UPDATE
            "#,
        )
        .bind(material_id.0)
        .bind(tenant_id.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        let abs_change = quantity_change.abs();
        if quantity_change < 0 && current.quantity < abs_change {
            return Err(AppError::Validation(format!(
                "Insufficient stock. Current: {}, Requested adjustment: {}",
                current.quantity, abs_change
            )));
        }

        let new_quantity = current.quantity + quantity_change;
        let new_legacy_quantity = if quantity_change >= 0 {
            current.legacy_quantity + quantity_change
        } else {
            let consumed_from_legacy = current.legacy_quantity.min(abs_change);
            let remaining = abs_change - consumed_from_legacy;
            let new_legacy = current.legacy_quantity - consumed_from_legacy;
            if remaining > 0 {
                self.consume_batches(&mut tx, material_id, tenant_id, remaining, false)
                    .await?;
            }
            new_legacy
        };

        self.update_material_quantities(
            &mut tx,
            material_id,
            tenant_id,
            new_quantity,
            new_legacy_quantity,
        )
        .await?;

        sqlx::query(
            r#"
            INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, entry_type, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'adjusted', $8)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id.0)
        .bind(material_id.0)
        .bind(user_id.0)
        .bind(quantity_change)
        .bind(new_quantity)
        .bind(reason)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_material_by_id(material_id, tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))
    }

    pub async fn stock_in(
        &self,
        material_id: MaterialId,
        quantity: i32,
        notes: Option<String>,
        user_id: UserId,
        expires_on: Option<NaiveDate>,
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let current = sqlx::query_as::<_, MaterialLockRow>(
            r#"
            SELECT m.quantity, m.legacy_quantity, c.can_expire
            FROM materials m
            INNER JOIN categories c ON c.id = m.category_id
            WHERE m.id = $1 AND m.tenant_id = $2 AND m.deleted_at IS NULL
            FOR UPDATE
            "#,
        )
        .bind(material_id.0)
        .bind(tenant_id.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        let new_quantity = current.quantity + quantity;
        let new_legacy_quantity = if current.can_expire {
            current.legacy_quantity
        } else {
            current.legacy_quantity + quantity
        };

        self.update_material_quantities(
            &mut tx,
            material_id,
            tenant_id,
            new_quantity,
            new_legacy_quantity,
        )
        .await?;

        if current.can_expire {
            self.insert_batch(
                &mut tx,
                tenant_id,
                material_id,
                quantity,
                expires_on.expect("validated expiry date"),
                Utc::now(),
            )
            .await?;
        }

        sqlx::query(
            r#"
            INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, entry_type, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'material_added', $8)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id.0)
        .bind(material_id.0)
        .bind(user_id.0)
        .bind(quantity)
        .bind(new_quantity)
        .bind(&notes)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_material_by_id(material_id, tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))
    }

    pub async fn list_enriched_stock_entries(
        &self,
        material_id: MaterialId,
        tenant_id: TenantId,
        limit: i32,
    ) -> Result<Vec<EnrichedStockEntry>, AppError> {
        let entries = sqlx::query_as::<_, EnrichedStockEntryRow>(
            r#"
            SELECT
                se.id, se.tenant_id, se.material_id, se.user_id,
                COALESCE(u.name, u.email, se.user_id::text) AS user_name,
                se.entry_type, se.quantity_change, se.quantity_after,
                se.notes, se.site_id, s.name AS site_name,
                c.name AS category_name, se.created_at
            FROM stock_entries se
            INNER JOIN materials m ON se.material_id = m.id
            INNER JOIN categories c ON m.category_id = c.id
            LEFT JOIN users u ON se.user_id = u.id
            LEFT JOIN sites s ON se.site_id = s.id
            WHERE se.material_id = $1 AND se.tenant_id = $2
            ORDER BY se.created_at DESC
            LIMIT $3
            "#,
        )
        .bind(material_id.0)
        .bind(tenant_id.0)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entries
            .into_iter()
            .map(|row| row.into_enriched_stock_entry())
            .collect())
    }

    pub async fn update_qr_code(
        &self,
        material_id: MaterialId,
        qr_code: &str,
        tenant_id: TenantId,
    ) -> Result<Material, AppError> {
        let updated_id = sqlx::query_scalar::<_, Uuid>(
            r#"
            UPDATE materials
            SET qr_code = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id
            "#,
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

        self.find_material_by_id(MaterialId(updated_id), tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))
    }

    pub async fn list_low_stock_materials(
        &self,
        tenant_id: TenantId,
    ) -> Result<Vec<Material>, AppError> {
        let materials = sqlx::query_as::<_, MaterialRow>(&Self::low_stock_materials_query())
            .bind(tenant_id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(materials
            .into_iter()
            .map(MaterialRow::into_material_without_batches)
            .collect())
    }

    fn material_list_query(filter_by_category: bool) -> String {
        let category_clause = if filter_by_category {
            "AND m.category_id = $2"
        } else {
            ""
        };

        format!(
            r#"
            SELECT
                m.id, m.tenant_id, m.category_id, m.name, m.description, m.unit,
                m.quantity, m.min_quantity, m.legacy_quantity, m.location, m.qr_code,
                m.created_at, m.updated_at, c.can_expire,
                COALESCE(summary.expired_quantity, 0) AS expired_quantity,
                COALESCE(summary.expiring_soon_quantity, 0) AS expiring_soon_quantity,
                summary.next_expiry_on
            FROM materials m
            INNER JOIN categories c ON c.id = m.category_id
            LEFT JOIN LATERAL (
                SELECT
                    COALESCE(SUM(CASE WHEN mb.expires_on < CURRENT_DATE THEN mb.remaining_quantity ELSE 0 END), 0)::INT4 AS expired_quantity,
                    COALESCE(SUM(CASE WHEN mb.expires_on >= CURRENT_DATE AND mb.expires_on <= CURRENT_DATE + {days} THEN mb.remaining_quantity ELSE 0 END), 0)::INT4 AS expiring_soon_quantity,
                    MIN(mb.expires_on) FILTER (WHERE mb.remaining_quantity > 0) AS next_expiry_on
                FROM material_batches mb
                WHERE mb.material_id = m.id AND mb.tenant_id = m.tenant_id AND mb.remaining_quantity > 0
            ) summary ON TRUE
            WHERE m.tenant_id = $1 AND m.deleted_at IS NULL {category_clause}
            ORDER BY m.name
            "#,
            days = Self::EXPIRY_WARNING_DAYS,
            category_clause = category_clause,
        )
    }

    fn low_stock_materials_query() -> String {
        format!(
            r#"
            SELECT
                m.id, m.tenant_id, m.category_id, m.name, m.description, m.unit,
                m.quantity, m.min_quantity, m.legacy_quantity, m.location, m.qr_code,
                m.created_at, m.updated_at, c.can_expire,
                COALESCE(summary.expired_quantity, 0) AS expired_quantity,
                COALESCE(summary.expiring_soon_quantity, 0) AS expiring_soon_quantity,
                summary.next_expiry_on
            FROM materials m
            INNER JOIN categories c ON c.id = m.category_id
            LEFT JOIN LATERAL (
                SELECT
                    COALESCE(SUM(CASE WHEN mb.expires_on < CURRENT_DATE THEN mb.remaining_quantity ELSE 0 END), 0)::INT4 AS expired_quantity,
                    COALESCE(SUM(CASE WHEN mb.expires_on >= CURRENT_DATE AND mb.expires_on <= CURRENT_DATE + {days} THEN mb.remaining_quantity ELSE 0 END), 0)::INT4 AS expiring_soon_quantity,
                    MIN(mb.expires_on) FILTER (WHERE mb.remaining_quantity > 0) AS next_expiry_on
                FROM material_batches mb
                WHERE mb.material_id = m.id AND mb.tenant_id = m.tenant_id AND mb.remaining_quantity > 0
            ) summary ON TRUE
            WHERE m.tenant_id = $1 AND m.deleted_at IS NULL AND m.quantity <= m.min_quantity
            ORDER BY m.quantity ASC
            "#,
            days = Self::EXPIRY_WARNING_DAYS,
        )
    }

    fn material_detail_query(filter_clause: &str) -> String {
        format!(
            r#"
            SELECT
                m.id, m.tenant_id, m.category_id, m.name, m.description, m.unit,
                m.quantity, m.min_quantity, m.legacy_quantity, m.location, m.qr_code,
                m.created_at, m.updated_at, c.can_expire,
                COALESCE(summary.expired_quantity, 0) AS expired_quantity,
                COALESCE(summary.expiring_soon_quantity, 0) AS expiring_soon_quantity,
                summary.next_expiry_on
            FROM materials m
            INNER JOIN categories c ON c.id = m.category_id
            LEFT JOIN LATERAL (
                SELECT
                    COALESCE(SUM(CASE WHEN mb.expires_on < CURRENT_DATE THEN mb.remaining_quantity ELSE 0 END), 0)::INT4 AS expired_quantity,
                    COALESCE(SUM(CASE WHEN mb.expires_on >= CURRENT_DATE AND mb.expires_on <= CURRENT_DATE + {days} THEN mb.remaining_quantity ELSE 0 END), 0)::INT4 AS expiring_soon_quantity,
                    MIN(mb.expires_on) FILTER (WHERE mb.remaining_quantity > 0) AS next_expiry_on
                FROM material_batches mb
                WHERE mb.material_id = m.id AND mb.tenant_id = m.tenant_id AND mb.remaining_quantity > 0
            ) summary ON TRUE
            WHERE {filter_clause}
            "#,
            days = Self::EXPIRY_WARNING_DAYS,
            filter_clause = filter_clause,
        )
    }

    async fn list_expiry_batches(
        &self,
        material_id: MaterialId,
        tenant_id: TenantId,
    ) -> Result<Vec<MaterialBatchSummary>, AppError> {
        let rows = sqlx::query_as::<_, MaterialBatchRow>(
            r#"
            SELECT expires_on, remaining_quantity
            FROM material_batches
            WHERE material_id = $1 AND tenant_id = $2 AND remaining_quantity > 0
            ORDER BY expires_on ASC
            "#,
        )
        .bind(material_id.0)
        .bind(tenant_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let today = Utc::now().date_naive();
        let warning_cutoff = today + Duration::days(Self::EXPIRY_WARNING_DAYS);
        Ok(rows
            .into_iter()
            .map(|row| MaterialBatchSummary {
                expires_on: row.expires_on,
                quantity: row.remaining_quantity,
                is_expired: row.expires_on < today,
                is_expiring_soon: row.expires_on >= today && row.expires_on <= warning_cutoff,
            })
            .collect())
    }

    async fn update_material_quantities(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        material_id: MaterialId,
        tenant_id: TenantId,
        quantity: i32,
        legacy_quantity: i32,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE materials
            SET quantity = $1, legacy_quantity = $2, updated_at = NOW()
            WHERE id = $3 AND tenant_id = $4
            "#,
        )
        .bind(quantity)
        .bind(legacy_quantity)
        .bind(material_id.0)
        .bind(tenant_id.0)
        .execute(&mut **tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn insert_batch(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        tenant_id: TenantId,
        material_id: MaterialId,
        quantity: i32,
        expires_on: NaiveDate,
        created_at: DateTime<Utc>,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO material_batches (id, tenant_id, material_id, expires_on, initial_quantity, remaining_quantity, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id.0)
        .bind(material_id.0)
        .bind(expires_on)
        .bind(quantity)
        .bind(quantity)
        .bind(created_at)
        .execute(&mut **tx)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn total_batch_quantity_for_update(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        material_id: MaterialId,
        tenant_id: TenantId,
        expired_only: bool,
    ) -> Result<i32, AppError> {
        let filter = if expired_only {
            "AND expires_on < CURRENT_DATE"
        } else {
            ""
        };
        let sql = format!(
            "SELECT COALESCE(SUM(remaining_quantity), 0) FROM material_batches WHERE material_id = $1 AND tenant_id = $2 AND remaining_quantity > 0 {filter}",
            filter = filter,
        );
        sqlx::query_scalar::<_, i64>(&sql)
            .bind(material_id.0)
            .bind(tenant_id.0)
            .fetch_one(&mut **tx)
            .await
            .map(|value| value as i32)
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn consume_batches(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        material_id: MaterialId,
        tenant_id: TenantId,
        quantity: i32,
        expired_only: bool,
    ) -> Result<(), AppError> {
        let filter = if expired_only {
            "AND expires_on < CURRENT_DATE"
        } else {
            ""
        };
        let sql = format!(
            r#"
            SELECT id, remaining_quantity
            FROM material_batches
            WHERE material_id = $1 AND tenant_id = $2 AND remaining_quantity > 0 {filter}
            ORDER BY expires_on ASC
            FOR UPDATE
            "#,
            filter = filter,
        );

        let batches = sqlx::query_as::<_, MaterialBatchStateRow>(&sql)
            .bind(material_id.0)
            .bind(tenant_id.0)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut remaining = quantity;
        for batch in batches {
            if remaining == 0 {
                break;
            }

            let consumed = batch.remaining_quantity.min(remaining);
            sqlx::query(
                r#"
                UPDATE material_batches
                SET remaining_quantity = $1
                WHERE id = $2
                "#,
            )
            .bind(batch.remaining_quantity - consumed)
            .bind(batch.id)
            .execute(&mut **tx)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            remaining -= consumed;
        }

        if remaining > 0 {
            return Err(AppError::Validation(
                "Insufficient tracked stock for withdrawal".to_string(),
            ));
        }

        Ok(())
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
            "#,
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
            "#,
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
                se.site_id, se.entry_type, se.created_at,
                s.name as site_name
            FROM stock_entries se
            LEFT JOIN sites s ON se.site_id = s.id
            WHERE se.material_id = $1 AND se.tenant_id = $2
            ORDER BY se.created_at DESC
            LIMIT $3
            "#,
        )
        .bind(material_id.0)
        .bind(tenant_id.0)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entries
            .into_iter()
            .map(|row| row.into_stock_entry_with_site())
            .collect())
    }

    pub async fn list_stock_entries_for_site(
        &self,
        site_id: SiteId,
        tenant_id: TenantId,
        limit: i32,
    ) -> Result<Vec<SiteStockHistoryEntry>, AppError> {
        let entries = sqlx::query_as::<_, SiteStockHistoryRow>(Self::site_history_query())
            .bind(site_id.0)
            .bind(tenant_id.0)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entries
            .into_iter()
            .map(SiteStockHistoryRow::into_site_stock_history_entry)
            .collect())
    }

    fn site_history_query() -> &'static str {
        r#"
        SELECT
            se.id,
            se.tenant_id,
            se.material_id,
            m.name AS material_name,
            c.name AS category_name,
            se.user_id,
            COALESCE(u.name, u.email, se.user_id::text) AS extracted_by,
            se.quantity_change,
            se.quantity_after,
            se.notes,
            se.site_id,
            s.name AS site_name,
            se.created_at
        FROM stock_entries se
        INNER JOIN materials m ON se.material_id = m.id
        INNER JOIN categories c ON m.category_id = c.id
        LEFT JOIN users u ON se.user_id = u.id
        LEFT JOIN sites s ON se.site_id = s.id
        WHERE se.site_id = $1 AND se.tenant_id = $2
        ORDER BY se.created_at DESC
        LIMIT $3
        "#
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
        let mut tx = self
            .pool
            .begin()
            .await
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
            "#,
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

        tx.commit()
            .await
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
    can_expire: bool,
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
            can_expire: self.can_expire,
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
    legacy_quantity: i32,
    location: Option<String>,
    qr_code: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    can_expire: bool,
    expired_quantity: i32,
    expiring_soon_quantity: i32,
    next_expiry_on: Option<NaiveDate>,
}

impl MaterialRow {
    fn into_material_without_batches(self) -> Material {
        self.into_material_with_batches(Vec::new())
    }

    fn into_material_with_batches(self, expiry_batches: Vec<MaterialBatchSummary>) -> Material {
        Material {
            id: MaterialId(self.id),
            tenant_id: TenantId(self.tenant_id),
            category_id: CategoryId(self.category_id),
            name: self.name,
            description: self.description,
            unit: self.unit.parse().unwrap_or(Unit::Piece),
            quantity: self.quantity,
            min_quantity: self.min_quantity,
            legacy_quantity: self.legacy_quantity,
            can_expire: self.can_expire,
            expired_quantity: self.expired_quantity,
            expiring_soon_quantity: self.expiring_soon_quantity,
            next_expiry_on: self.next_expiry_on,
            expiry_batches,
            location: self.location,
            qr_code: self.qr_code,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct MaterialLockRow {
    quantity: i32,
    legacy_quantity: i32,
    can_expire: bool,
}

#[derive(Debug, FromRow)]
struct MaterialBatchRow {
    expires_on: NaiveDate,
    remaining_quantity: i32,
}

#[derive(Debug, FromRow)]
struct MaterialBatchStateRow {
    id: Uuid,
    remaining_quantity: i32,
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
    entry_type: String,
    created_at: DateTime<Utc>,
    site_name: Option<String>,
}

#[derive(Debug, FromRow)]
struct EnrichedStockEntryRow {
    id: Uuid,
    tenant_id: Uuid,
    material_id: Uuid,
    user_id: Uuid,
    user_name: String,
    entry_type: String,
    quantity_change: i32,
    quantity_after: i32,
    notes: Option<String>,
    site_id: Option<Uuid>,
    site_name: Option<String>,
    category_name: String,
    created_at: DateTime<Utc>,
}

impl EnrichedStockEntryRow {
    fn into_enriched_stock_entry(self) -> EnrichedStockEntry {
        EnrichedStockEntry {
            id: self.id,
            tenant_id: TenantId(self.tenant_id),
            material_id: MaterialId(self.material_id),
            user_id: UserId(self.user_id),
            user_name: self.user_name,
            entry_type: self.entry_type.parse().unwrap_or(EntryType::Adjusted),
            quantity_change: self.quantity_change,
            quantity_after: self.quantity_after,
            notes: self.notes,
            site_id: self.site_id.map(SiteId),
            site_name: self.site_name,
            category_name: self.category_name,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct SiteStockHistoryRow {
    id: Uuid,
    tenant_id: Uuid,
    material_id: Uuid,
    material_name: String,
    category_name: String,
    user_id: Uuid,
    extracted_by: String,
    quantity_change: i32,
    quantity_after: i32,
    notes: Option<String>,
    site_id: Option<Uuid>,
    site_name: Option<String>,
    created_at: DateTime<Utc>,
}

impl SiteStockHistoryRow {
    fn into_site_stock_history_entry(self) -> SiteStockHistoryEntry {
        SiteStockHistoryEntry {
            id: self.id,
            tenant_id: TenantId(self.tenant_id),
            material_id: MaterialId(self.material_id),
            material_name: self.material_name,
            category_name: self.category_name,
            user_id: UserId(self.user_id),
            extracted_by: self.extracted_by,
            quantity_change: self.quantity_change,
            quantity_after: self.quantity_after,
            notes: self.notes,
            site_id: self.site_id.map(SiteId),
            site_name: self.site_name,
            created_at: self.created_at,
        }
    }
}

impl StockEntryRow {
    fn into_stock_entry_with_site(self) -> StockEntryWithSite {
        StockEntryWithSite {
            id: self.id,
            tenant_id: TenantId(self.tenant_id),
            material_id: MaterialId(self.material_id),
            user_id: UserId(self.user_id),
            entry_type: self.entry_type.parse().unwrap_or(EntryType::Adjusted),
            quantity_change: self.quantity_change,
            quantity_after: self.quantity_after,
            notes: self.notes,
            site_id: self.site_id.map(SiteId),
            site_name: self.site_name,
            created_at: self.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MaterialRepository;

    #[test]
    fn site_history_query_enforces_tenant_and_site_filters() {
        let sql = MaterialRepository::site_history_query();
        assert!(sql.contains("WHERE se.site_id = $1 AND se.tenant_id = $2"));
        assert!(sql.contains("COALESCE(u.name, u.email, se.user_id::text)"));
        assert!(sql.contains("INNER JOIN materials m"));
        assert!(sql.contains("INNER JOIN categories c"));
    }

    #[test]
    fn delete_category_query_checks_material_count() {
        // Verify deletion is blocked when any material row still exists for history preservation.
        let sql = MaterialRepository::delete_category_conflict_count_query();
        assert!(sql.contains("category_id"));
        assert!(!sql.contains("deleted_at IS NULL"));
    }

    #[test]
    fn delete_category_query_keeps_tenant_scoping() {
        let sql = MaterialRepository::delete_category_conflict_count_query();
        assert!(sql.contains("tenant_id = $2"));
        assert!(sql.contains("COUNT(*)"));
    }
}
