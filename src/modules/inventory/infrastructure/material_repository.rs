use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{TenantId, MaterialId, CategoryId, UserId, Unit};
use crate::modules::inventory::domain::{
    Category, Material, CreateCategory, CreateMaterial,
};

/// Repository for material data access with tenant isolation
pub struct MaterialRepository {
    pool: PgPool,
}

impl MaterialRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
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
                    WHERE tenant_id = $1 AND category_id = $2
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
                    WHERE tenant_id = $1
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
            WHERE id = $1 AND tenant_id = $2
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
            WHERE qr_code = $1 AND tenant_id = $2
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
            INSERT INTO stock_entries (id, tenant_id, material_id, user_id, quantity_change, quantity_after, notes, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
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
            WHERE tenant_id = $1 AND quantity <= min_quantity
            ORDER BY quantity ASC
            "#
        )
        .bind(tenant_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(materials.into_iter().map(|m| m.into_material()).collect())
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
