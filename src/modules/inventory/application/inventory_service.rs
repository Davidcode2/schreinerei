use crate::common::error::AppError;
use crate::common::types::{MaterialId, CategoryId};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::inventory::domain::{
    Category, Material, CreateCategory, CreateMaterial, WithdrawMaterial, AdjustStock,
};
use crate::modules::inventory::infrastructure::MaterialRepository;

/// Service for inventory business logic
pub struct InventoryService {
    material_repo: MaterialRepository,
}

impl InventoryService {
    pub fn new(material_repo: MaterialRepository) -> Self {
        Self { material_repo }
    }

    // === Category operations ===

    pub async fn create_category(
        &self,
        create: CreateCategory,
        ctx: &TenantContext,
    ) -> Result<Category, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        create.validate()?;
        self.material_repo.create_category(&create, ctx.tenant_id).await
    }

    pub async fn list_categories(
        &self,
        ctx: &TenantContext,
    ) -> Result<Vec<Category>, AppError> {
        self.material_repo.list_categories(ctx.tenant_id).await
    }

    pub async fn get_category(
        &self,
        id: CategoryId,
        ctx: &TenantContext,
    ) -> Result<Category, AppError> {
        self.material_repo
            .find_category_by_id(id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))
    }

    // === Material operations ===

    pub async fn create_material(
        &self,
        create: CreateMaterial,
        ctx: &TenantContext,
    ) -> Result<Material, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        create.validate()?;

        // Verify category exists
        self.material_repo
            .find_category_by_id(create.category_id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::Validation("Category not found".to_string()))?;

        self.material_repo.create_material(&create, ctx.tenant_id).await
    }

    pub async fn list_materials(
        &self,
        category_id: Option<CategoryId>,
        ctx: &TenantContext,
    ) -> Result<Vec<Material>, AppError> {
        self.material_repo.list_materials(ctx.tenant_id, category_id).await
    }

    pub async fn get_material(
        &self,
        id: MaterialId,
        ctx: &TenantContext,
    ) -> Result<Material, AppError> {
        self.material_repo
            .find_material_by_id(id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))
    }

    pub async fn get_material_by_qr(
        &self,
        qr_code: &str,
        ctx: &TenantContext,
    ) -> Result<Material, AppError> {
        self.material_repo
            .find_material_by_qr_code(qr_code, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found for this QR code".to_string()))
    }

    // === Stock operations ===

    pub async fn withdraw_material(
        &self,
        withdraw: WithdrawMaterial,
        ctx: &TenantContext,
    ) -> Result<Material, AppError> {
        withdraw.validate()?;

        let material = self.material_repo
            .withdraw_stock(
                withdraw.material_id,
                withdraw.quantity,
                ctx.user_id,
                withdraw.notes,
                ctx.tenant_id,
            )
            .await?;

        // Check for low stock warning (will be emitted as event in Plan 02)
        if material.is_low_stock() {
            tracing::warn!(
                "Low stock alert: {} has {} units remaining (min: {})",
                material.name,
                material.quantity,
                material.min_quantity
            );
        }

        Ok(material)
    }

    pub async fn adjust_stock(
        &self,
        adjust: AdjustStock,
        ctx: &TenantContext,
    ) -> Result<Material, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        adjust.validate()?;

        self.material_repo
            .adjust_stock(
                adjust.material_id,
                adjust.quantity,
                &adjust.reason,
                ctx.user_id,
                ctx.tenant_id,
            )
            .await
    }

    pub async fn list_low_stock(
        &self,
        ctx: &TenantContext,
    ) -> Result<Vec<Material>, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        self.material_repo.list_low_stock_materials(ctx.tenant_id).await
    }
}
