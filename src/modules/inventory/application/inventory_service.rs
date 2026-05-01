use crate::common::error::AppError;
use crate::common::types::{MaterialId, CategoryId, OrderRequestId, UserId, Role};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::modules::inventory::domain::{
    Category, Material, CreateCategory, CreateMaterial, WithdrawMaterial, AdjustStock,
    UpdateCategory, UpdateMaterial, StockIn,
    OrderRequest, OrderStatus, CreateOrderRequest, ApproveOrderRequest, FulfillOrderRequest,
    StockLowPayload, StockWithdrawnPayload, MaterialCreatedPayload, StockAdjustedPayload,
    OrderRequestCreatedPayload, MaterialAddedPayload, LocationChangedPayload, MinQuantityChangedPayload,
    EnrichedStockEntry,
};
use crate::modules::inventory::infrastructure::MaterialRepository;
use qrcode::render::svg;
use qrcode::QrCode;
use sqlx::PgPool;

/// Service for inventory business logic
pub struct InventoryService {
    material_repo: MaterialRepository,
    pool: PgPool,
}

impl InventoryService {
    pub fn new(material_repo: MaterialRepository) -> Self {
        let pool = material_repo.pool();
        Self { material_repo, pool }
    }

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

    pub async fn update_category(
        &self,
        id: CategoryId,
        update: UpdateCategory,
        ctx: &TenantContext,
    ) -> Result<Category, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        update.validate()?;
        self.material_repo.update_category(id, &update, ctx.tenant_id).await
    }

    pub async fn delete_category(
        &self,
        id: CategoryId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        self.material_repo.delete_category(id, ctx.tenant_id).await
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

        let material = self.material_repo.create_material(&create, ctx.tenant_id).await?;

        // Emit MaterialCreated event
        let event = MaterialCreatedPayload {
            material_id: material.id,
            material_name: material.name.clone(),
            category_id: create.category_id.to_string(),
            initial_quantity: create.quantity,
            created_by: ctx.user_id,
        }.into_event(ctx.tenant_id);

        self.material_repo.publish_event(&event).await?;

        Ok(material)
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

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        let material = self.material_repo
            .withdraw_stock(
                withdraw.material_id,
                withdraw.quantity,
                local_user_id,
                withdraw.notes.clone(),
                withdraw.site_id,  // Pass site_id from command
                ctx.tenant_id,
            )
            .await?;

        // Emit StockWithdrawn event
        let event = StockWithdrawnPayload {
            material_id: material.id,
            material_name: material.name.clone(),
            quantity_withdrawn: withdraw.quantity,
            quantity_after: material.quantity,
            withdrawn_by: local_user_id,
            notes: withdraw.notes,
            is_last_unit: material.is_last_unit(),
        }.into_event(ctx.tenant_id);

        self.material_repo.publish_event(&event).await?;

        // Emit StockLow event if below threshold
        if material.is_low_stock() {
            let low_event = StockLowPayload {
                material_id: material.id,
                material_name: material.name.clone(),
                current_quantity: material.quantity,
                min_quantity: material.min_quantity,
                triggered_by_user_id: Some(local_user_id),
            }.into_event(ctx.tenant_id);

            self.material_repo.publish_event(&low_event).await?;
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

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        let material = self.material_repo
            .adjust_stock(
                adjust.material_id,
                adjust.quantity,
                &adjust.reason,
                local_user_id,
                ctx.tenant_id,
            )
            .await?;

        // Emit StockAdjusted event
        let event = StockAdjustedPayload {
            material_id: material.id,
            material_name: material.name.clone(),
            quantity_change: adjust.quantity,
            quantity_after: material.quantity,
            reason: adjust.reason,
            adjusted_by: local_user_id,
        }.into_event(ctx.tenant_id);

        self.material_repo.publish_event(&event).await?;

        // Emit StockLow event if below threshold after negative adjustment
        if material.is_low_stock() && adjust.quantity < 0 {
            let low_event = StockLowPayload {
                material_id: material.id,
                material_name: material.name.clone(),
                current_quantity: material.quantity,
                min_quantity: material.min_quantity,
                triggered_by_user_id: Some(local_user_id),
            }.into_event(ctx.tenant_id);

            self.material_repo.publish_event(&low_event).await?;
        }

        Ok(material)
    }

    pub async fn update_material(
        &self,
        id: MaterialId,
        update: UpdateMaterial,
        ctx: &TenantContext,
    ) -> Result<Material, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        update.validate()?;

        // Record old values for events before update
        let old_material = self.material_repo
            .find_material_by_id(id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        let updated = self.material_repo.update_material(id, &update, ctx.tenant_id).await?;

        // Emit events for changed fields
        let local_user_id = self.resolve_local_user_id(ctx).await?;

        if old_material.location != updated.location {
            let event = LocationChangedPayload {
                material_id: updated.id,
                material_name: updated.name.clone(),
                old_location: old_material.location.clone(),
                new_location: updated.location.clone(),
                changed_by: local_user_id,
            }.into_event(ctx.tenant_id);
            self.material_repo.publish_event(&event).await?;
        }

        if old_material.min_quantity != updated.min_quantity {
            let event = MinQuantityChangedPayload {
                material_id: updated.id,
                material_name: updated.name.clone(),
                old_min_quantity: old_material.min_quantity,
                new_min_quantity: updated.min_quantity,
                changed_by: local_user_id,
            }.into_event(ctx.tenant_id);
            self.material_repo.publish_event(&event).await?;
        }

        Ok(updated)
    }

    pub async fn stock_in(
        &self,
        stock_in: StockIn,
        ctx: &TenantContext,
    ) -> Result<Material, AppError> {
        // StockIn available to all users — no admin check
        stock_in.validate()?;

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        let material = self.material_repo
            .stock_in(
                stock_in.material_id,
                stock_in.quantity,
                stock_in.notes.clone(),
                local_user_id,
                ctx.tenant_id,
            )
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

    pub async fn list_enriched_history(
        &self,
        material_id: MaterialId,
        ctx: &TenantContext,
    ) -> Result<Vec<EnrichedStockEntry>, AppError> {
        self.material_repo
            .find_material_by_id(material_id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        self.material_repo.list_enriched_stock_entries(material_id, ctx.tenant_id, 50).await
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

    /// Delete a material (soft delete)
    /// Returns Conflict error if there are pending order requests
    pub async fn delete_material(
        &self,
        material_id: MaterialId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // Check for pending order requests
        let pending_count = self.material_repo.count_pending_order_requests(material_id, ctx.tenant_id).await?;
        if pending_count > 0 {
            return Err(AppError::Conflict(
                format!("Cannot delete: {} pending order request(s) exist", pending_count)
            ));
        }

        // Verify material exists
        self.material_repo.find_material_by_id(material_id, ctx.tenant_id).await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        // Perform soft delete
        self.material_repo.delete_material(material_id, ctx.tenant_id).await
    }

    // === QR Code operations ===

    pub async fn generate_qr_code(
        &self,
        material_id: MaterialId,
        ctx: &TenantContext,
    ) -> Result<String, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // Get material to verify it exists
        let _material = self.get_material(material_id, ctx).await?;

        // Generate QR code identifier
        let qr_identifier = format!("MAT-{}-{}",
            &ctx.tenant_id.to_string()[..8].to_uppercase(),
            &material_id.to_string()[..8].to_uppercase()
        );

        // Update material with QR code
        self.material_repo
            .update_qr_code(material_id, &qr_identifier, ctx.tenant_id)
            .await?;

        Ok(qr_identifier)
    }

    pub async fn get_qr_code_svg(
        &self,
        material_id: MaterialId,
        ctx: &TenantContext,
    ) -> Result<String, AppError> {
        let material = self.get_material(material_id, ctx).await?;

        let qr_code = material.qr_code
            .ok_or_else(|| AppError::NotFound("QR code not generated for this material".to_string()))?;

        // Generate QR code SVG
        let code = QrCode::new(&qr_code)
            .map_err(|e| AppError::Validation(format!("Failed to generate QR code: {}", e)))?;

        let svg_string = code
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        Ok(svg_string)
    }

    // === Order Request operations ===

    pub async fn create_order_request(
        &self,
        create: CreateOrderRequest,
        ctx: &TenantContext,
    ) -> Result<OrderRequest, AppError> {
        create.validate()?;

        let material = self.get_material(create.material_id, ctx).await?;

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        let order = self.material_repo
            .create_order_request(&create, local_user_id, ctx.tenant_id)
            .await?;

        // Emit OrderRequestCreated event
        let event = OrderRequestCreatedPayload {
            order_request_id: order.id.to_string(),
            material_id: material.id,
            material_name: material.name,
            quantity_requested: create.quantity,
            requested_by: local_user_id,
            reason: create.reason.clone(),
        }.into_event(ctx.tenant_id);

        self.material_repo.publish_event(&event).await?;

        Ok(order)
    }

    pub async fn list_order_requests(
        &self,
        status: Option<OrderStatus>,
        ctx: &TenantContext,
    ) -> Result<Vec<OrderRequest>, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        self.material_repo.list_order_requests(ctx.tenant_id, status).await
    }

    pub async fn get_order_request(
        &self,
        id: OrderRequestId,
        ctx: &TenantContext,
    ) -> Result<OrderRequest, AppError> {
        self.material_repo
            .find_order_request_by_id(id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Order request not found".to_string()))
    }

    pub async fn approve_order_request(
        &self,
        id: OrderRequestId,
        approve: ApproveOrderRequest,
        ctx: &TenantContext,
    ) -> Result<OrderRequest, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        let local_user_id = self.resolve_local_user_id(ctx).await?;
        self.material_repo
            .approve_order_request(id, local_user_id, approve.notes, ctx.tenant_id)
            .await
    }

    pub async fn fulfill_order_request(
        &self,
        id: OrderRequestId,
        fulfill: FulfillOrderRequest,
        ctx: &TenantContext,
    ) -> Result<OrderRequest, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        self.material_repo
            .fulfill_order_request(id, fulfill.actual_quantity, fulfill.notes, ctx.tenant_id)
            .await
    }
}
