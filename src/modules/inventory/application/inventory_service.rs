use crate::common::error::AppError;
use crate::common::types::{CategoryId, MaterialId, OrderRequestId, Role, UserId};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::modules::inventory::domain::{
    AdjustStock, ApproveOrderRequest, Category, CreateCategory, CreateMaterial, CreateOrderRequest,
    EnrichedStockEntry, FulfillOrderRequest, LocationChangedPayload, MarkOrderedRequest, Material,
    MaterialAddedPayload, MaterialCreatedPayload, MinQuantityChangedPayload, OrderRequest,
    OrderRequestCreatedPayload, OrderRequestKind, OrderStatus, StockAdjustedPayload, StockIn,
    StockLowPayload, StockWithdrawnPayload, UpdateCategory, UpdateMaterial, WithdrawMaterial,
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
    const DISABLE_EXPIRY_WITH_LIVE_STOCK_ERROR: &'static str =
        "Expiry tracking can only be disabled after all live stock in this category is depleted";
    const LOW_STOCK_REASON: &'static str = "Automatisch: Mindestbestand unterschritten";
    const LAST_PACKAGE_REASON: &'static str = "Automatisch: Letzte Packung entnommen";
    const AUTO_REPLENISHMENT_RESOLVED_NOTE: &'static str =
        "Automatisch nach Einlagerung oder Bestandskorrektur aufgelost";

    pub fn new(material_repo: MaterialRepository) -> Self {
        let pool = material_repo.pool();
        Self {
            material_repo,
            pool,
        }
    }

    async fn resolve_local_user_id(&self, ctx: &TenantContext) -> Result<UserId, AppError> {
        let user_repo = UserRepository::new(self.pool.clone());
        let user = user_repo
            .find_or_create_by_keycloak_id(
                &ctx.user_id.to_string(),
                ctx.tenant_id,
                &ctx.email,
                if ctx.is_admin() {
                    Role::Admin
                } else {
                    Role::Employee
                },
            )
            .await?;
        Ok(user.id)
    }

    fn suggested_replenishment_quantity(material: &Material) -> i32 {
        (material.min_quantity * 2).max(1)
    }

    async fn ensure_replenishment_signal(
        &self,
        material: &Material,
        triggered_by: UserId,
        kind: OrderRequestKind,
        tenant_id: crate::common::types::TenantId,
    ) -> Result<(), AppError> {
        let existing = self
            .material_repo
            .find_active_order_request_for_material(material.id, tenant_id)
            .await?;

        if existing.is_some() {
            return Ok(());
        }

        self.material_repo
            .create_order_request(
                &CreateOrderRequest {
                    material_id: material.id,
                    quantity: Self::suggested_replenishment_quantity(material),
                    reason: Some(
                        match kind {
                            OrderRequestKind::LastPackage => Self::LAST_PACKAGE_REASON,
                            OrderRequestKind::MinimumBreach | OrderRequestKind::Manual => {
                                Self::LOW_STOCK_REASON
                            }
                        }
                        .to_string(),
                    ),
                    request_kind: kind,
                },
                triggered_by,
                tenant_id,
            )
            .await?;

        Ok(())
    }

    async fn resolve_auto_replenishment_signals_if_restocked(
        &self,
        material: &Material,
        tenant_id: crate::common::types::TenantId,
    ) -> Result<(), AppError> {
        if material.is_low_stock() {
            return Ok(());
        }

        self.material_repo
            .resolve_active_auto_order_requests(
                material.id,
                Some(Self::AUTO_REPLENISHMENT_RESOLVED_NOTE.to_string()),
                tenant_id,
            )
            .await
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
        self.material_repo
            .create_category(&create, ctx.tenant_id)
            .await
    }

    pub async fn list_categories(&self, ctx: &TenantContext) -> Result<Vec<Category>, AppError> {
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

        let existing = self.get_category(id, ctx).await?;
        let has_live_stock = if matches!(update.can_expire, Some(false)) {
            self.material_repo
                .category_has_live_stock(id, ctx.tenant_id)
                .await?
        } else {
            false
        };

        Self::validate_category_expiry_toggle(&existing, &update, has_live_stock)?;

        self.material_repo
            .update_category(id, &update, ctx.tenant_id)
            .await
    }

    fn validate_category_expiry_toggle(
        existing: &Category,
        update: &UpdateCategory,
        has_live_stock: bool,
    ) -> Result<(), AppError> {
        if existing.can_expire && matches!(update.can_expire, Some(false)) && has_live_stock {
            return Err(AppError::Validation(
                Self::DISABLE_EXPIRY_WITH_LIVE_STOCK_ERROR.to_string(),
            ));
        }

        Ok(())
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

        let category = self
            .material_repo
            .find_category_by_id(create.category_id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::Validation("Category not found".to_string()))?;

        if category.can_expire && create.quantity > 0 && create.expires_on.is_none() {
            return Err(AppError::Validation(
                "MHD is required for expiring categories".to_string(),
            ));
        }

        let material = self
            .material_repo
            .create_material(&create, ctx.tenant_id, category.can_expire)
            .await?;

        // Emit MaterialCreated event
        let event = MaterialCreatedPayload {
            material_id: material.id,
            material_name: material.name.clone(),
            category_id: create.category_id.to_string(),
            initial_quantity: create.quantity,
            created_by: ctx.user_id,
        }
        .into_event(ctx.tenant_id);

        self.material_repo.publish_event(&event).await?;

        Ok(material)
    }

    pub async fn list_materials(
        &self,
        category_id: Option<CategoryId>,
        ctx: &TenantContext,
    ) -> Result<Vec<Material>, AppError> {
        self.material_repo
            .list_materials(ctx.tenant_id, category_id)
            .await
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

        let material = self
            .material_repo
            .withdraw_stock(
                withdraw.material_id,
                withdraw.quantity,
                local_user_id,
                withdraw.notes.clone(),
                withdraw.site_id, // Pass site_id from command
                withdraw.disposal,
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
        }
        .into_event(ctx.tenant_id);

        self.material_repo.publish_event(&event).await?;

        // Emit StockLow event if below threshold
        if material.is_low_stock() {
            let low_event = StockLowPayload {
                material_id: material.id,
                material_name: material.name.clone(),
                current_quantity: material.quantity,
                min_quantity: material.min_quantity,
                triggered_by_user_id: Some(local_user_id),
            }
            .into_event(ctx.tenant_id);

            self.material_repo.publish_event(&low_event).await?;

            self.ensure_replenishment_signal(
                &material,
                local_user_id,
                OrderRequestKind::MinimumBreach,
                ctx.tenant_id,
            )
            .await?;
        }

        if withdraw.last_package_taken {
            self.ensure_replenishment_signal(
                &material,
                local_user_id,
                OrderRequestKind::LastPackage,
                ctx.tenant_id,
            )
            .await?;
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

        let material = self
            .material_repo
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
        }
        .into_event(ctx.tenant_id);

        self.material_repo.publish_event(&event).await?;

        // Emit StockLow event if below threshold after negative adjustment
        if material.is_low_stock() && adjust.quantity < 0 {
            let low_event = StockLowPayload {
                material_id: material.id,
                material_name: material.name.clone(),
                current_quantity: material.quantity,
                min_quantity: material.min_quantity,
                triggered_by_user_id: Some(local_user_id),
            }
            .into_event(ctx.tenant_id);

            self.material_repo.publish_event(&low_event).await?;

            self.ensure_replenishment_signal(
                &material,
                local_user_id,
                OrderRequestKind::MinimumBreach,
                ctx.tenant_id,
            )
            .await?;
        }

        if adjust.quantity > 0 {
            self.resolve_auto_replenishment_signals_if_restocked(&material, ctx.tenant_id)
                .await?;
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
        let old_material = self
            .material_repo
            .find_material_by_id(id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        let updated = self
            .material_repo
            .update_material(id, &update, ctx.tenant_id)
            .await?;

        // Emit events for changed fields
        let local_user_id = self.resolve_local_user_id(ctx).await?;

        if old_material.location != updated.location {
            let event = LocationChangedPayload {
                material_id: updated.id,
                material_name: updated.name.clone(),
                old_location: old_material.location.clone(),
                new_location: updated.location.clone(),
                changed_by: local_user_id,
            }
            .into_event(ctx.tenant_id);
            self.material_repo.publish_event(&event).await?;
        }

        if old_material.min_quantity != updated.min_quantity {
            let event = MinQuantityChangedPayload {
                material_id: updated.id,
                material_name: updated.name.clone(),
                old_min_quantity: old_material.min_quantity,
                new_min_quantity: updated.min_quantity,
                changed_by: local_user_id,
            }
            .into_event(ctx.tenant_id);
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

        let existing_material = self
            .material_repo
            .find_material_by_id(stock_in.material_id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        if existing_material.can_expire && stock_in.expires_on.is_none() {
            return Err(AppError::Validation(
                "MHD is required for expiring categories".to_string(),
            ));
        }

        let material = self
            .material_repo
            .stock_in(&stock_in, local_user_id, ctx.tenant_id)
            .await?;

        // Emit MaterialAdded event
        let event = MaterialAddedPayload {
            material_id: material.id,
            material_name: material.name.clone(),
            quantity_added: stock_in.quantity,
            quantity_after: material.quantity,
            added_by: local_user_id,
            notes: stock_in.notes,
        }
        .into_event(ctx.tenant_id);

        self.material_repo.publish_event(&event).await?;

        self.resolve_auto_replenishment_signals_if_restocked(&material, ctx.tenant_id)
            .await?;

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

        self.material_repo
            .list_enriched_stock_entries(material_id, ctx.tenant_id, 50)
            .await
    }

    pub async fn list_low_stock(&self, ctx: &TenantContext) -> Result<Vec<Material>, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        self.material_repo
            .list_low_stock_materials(ctx.tenant_id)
            .await
    }

    pub async fn list_inventory_alerts(
        &self,
        ctx: &TenantContext,
    ) -> Result<Vec<Material>, AppError> {
        self.material_repo
            .list_inventory_alert_materials(ctx.tenant_id)
            .await
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
        let pending_count = self
            .material_repo
            .count_pending_order_requests(material_id, ctx.tenant_id)
            .await?;
        if pending_count > 0 {
            return Err(AppError::Conflict(format!(
                "Cannot delete: {} pending order request(s) exist",
                pending_count
            )));
        }

        // Verify material exists
        self.material_repo
            .find_material_by_id(material_id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

        // Perform soft delete
        self.material_repo
            .delete_material(material_id, ctx.tenant_id)
            .await
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
        let qr_identifier = format!(
            "MAT-{}-{}",
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

        let qr_code = material.qr_code.ok_or_else(|| {
            AppError::NotFound("QR code not generated for this material".to_string())
        })?;

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

        let order = self
            .material_repo
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
        }
        .into_event(ctx.tenant_id);

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
        self.material_repo
            .list_order_requests(ctx.tenant_id, status)
            .await
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

    pub async fn mark_order_request_ordered(
        &self,
        id: OrderRequestId,
        mark_ordered: MarkOrderedRequest,
        ctx: &TenantContext,
    ) -> Result<OrderRequest, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        self.material_repo
            .mark_order_request_ordered(id, mark_ordered.notes, ctx.tenant_id)
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

    pub async fn cancel_order_request(
        &self,
        id: OrderRequestId,
        notes: Option<String>,
        ctx: &TenantContext,
    ) -> Result<OrderRequest, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        self.material_repo
            .cancel_order_request(id, notes, ctx.tenant_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::InventoryService;
    use crate::common::types::{CategoryId, TenantId};
    use crate::modules::inventory::domain::{Category, UpdateCategory};
    use chrono::Utc;

    fn category(can_expire: bool) -> Category {
        Category {
            id: CategoryId::new(),
            tenant_id: TenantId::new(),
            name: "Lacke".to_string(),
            description: None,
            can_expire,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn validate_category_expiry_toggle_blocks_disabling_with_live_stock() {
        let error = InventoryService::validate_category_expiry_toggle(
            &category(true),
            &UpdateCategory {
                name: None,
                description: None,
                can_expire: Some(false),
            },
            true,
        )
        .expect_err("live stock should block disabling expiry");

        assert_eq!(
            error.to_string(),
            format!(
                "Validation error: {}",
                InventoryService::DISABLE_EXPIRY_WITH_LIVE_STOCK_ERROR
            )
        );
    }

    #[test]
    fn validate_category_expiry_toggle_allows_enabling_with_live_legacy_stock() {
        let result = InventoryService::validate_category_expiry_toggle(
            &category(false),
            &UpdateCategory {
                name: None,
                description: None,
                can_expire: Some(true),
            },
            true,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn validate_category_expiry_toggle_allows_disabling_after_stock_is_depleted() {
        let result = InventoryService::validate_category_expiry_toggle(
            &category(true),
            &UpdateCategory {
                name: None,
                description: None,
                can_expire: Some(false),
            },
            false,
        );

        assert!(result.is_ok());
    }
}
