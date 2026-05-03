use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{CategoryId, MaterialId, OrderRequestId, SiteId};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::inventory::application::InventoryService;
use crate::modules::inventory::domain::{
    AdjustStock, ApproveOrderRequest, CreateCategory, CreateMaterial, CreateOrderRequest,
    EnrichedStockEntry, FulfillOrderRequest, SiteStockHistoryEntry, StockEntryWithSite, StockIn,
    UpdateCategory, UpdateMaterial, WithdrawMaterial,
};
use crate::AppState;

/// Create the inventory API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Categories
        .route(
            "/api/v1/inventory/categories",
            get(list_categories).post(create_category),
        )
        .route(
            "/api/v1/inventory/categories/{id}",
            get(get_category)
                .patch(update_category)
                .delete(delete_category),
        )
        // Materials
        .route(
            "/api/v1/inventory/materials",
            get(list_materials).post(create_material),
        )
        .route(
            "/api/v1/inventory/materials/{id}",
            get(get_material)
                .patch(update_material)
                .delete(delete_material),
        )
        .route(
            "/api/v1/inventory/materials/{id}/history",
            get(get_material_history),
        )
        .route(
            "/api/v1/inventory/materials/{id}/history/enriched",
            get(get_enriched_material_history),
        )
        .route(
            "/api/v1/inventory/sites/{site_id}/history",
            get(get_site_material_history),
        )
        .route(
            "/api/v1/inventory/materials/{id}/withdraw",
            post(withdraw_material),
        )
        .route(
            "/api/v1/inventory/materials/{id}/adjust",
            post(adjust_stock),
        )
        .route(
            "/api/v1/inventory/materials/{id}/stock-in",
            post(stock_in_material),
        )
        .route(
            "/api/v1/inventory/materials/{id}/qr",
            post(generate_qr_code),
        )
        .route("/api/v1/inventory/materials/{id}/qr/svg", get(get_qr_svg))
        // Low stock
        .route("/api/v1/inventory/low-stock", get(list_low_stock))
        // QR lookup
        .route("/api/v1/inventory/qr/{code}", get(get_by_qr_code))
        // Order requests
        .route(
            "/api/v1/inventory/orders",
            get(list_order_requests).post(create_order_request),
        )
        .route(
            "/api/v1/inventory/orders/{id}/approve",
            post(approve_order_request),
        )
        .route(
            "/api/v1/inventory/orders/{id}/fulfill",
            post(fulfill_order_request),
        )
}

// === DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub can_expire: bool,
    pub created_at: String,
}

impl From<crate::modules::inventory::domain::Category> for CategoryResponse {
    fn from(cat: crate::modules::inventory::domain::Category) -> Self {
        Self {
            id: cat.id.to_string(),
            name: cat.name,
            description: cat.description,
            can_expire: cat.can_expire,
            created_at: cat.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub can_expire: bool,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UpdateCategoryRequest {
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub description: Option<String>,
    #[ts(optional)]
    pub can_expire: Option<bool>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ExpiryBatchResponse {
    pub expires_on: String,
    pub quantity: i32,
    pub is_expired: bool,
    pub is_expiring_soon: bool,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct MaterialResponse {
    pub id: String,
    pub category_id: String,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub min_quantity: i32,
    pub can_expire: bool,
    pub legacy_quantity: i32,
    pub expired_quantity: i32,
    pub expiring_soon_quantity: i32,
    pub next_expiry_on: Option<String>,
    pub expiry_batches: Vec<ExpiryBatchResponse>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub is_low_stock: bool,
    pub created_at: String,
}

impl From<crate::modules::inventory::domain::Material> for MaterialResponse {
    fn from(mat: crate::modules::inventory::domain::Material) -> Self {
        Self {
            id: mat.id.to_string(),
            category_id: mat.category_id.to_string(),
            name: mat.name,
            description: mat.description,
            unit: mat.unit.to_string(),
            quantity: mat.quantity,
            min_quantity: mat.min_quantity,
            can_expire: mat.can_expire,
            legacy_quantity: mat.legacy_quantity,
            expired_quantity: mat.expired_quantity,
            expiring_soon_quantity: mat.expiring_soon_quantity,
            next_expiry_on: mat.next_expiry_on.map(|date| date.to_string()),
            expiry_batches: mat
                .expiry_batches
                .into_iter()
                .map(ExpiryBatchResponse::from)
                .collect(),
            location: mat.location,
            qr_code: mat.qr_code,
            is_low_stock: mat.quantity <= mat.min_quantity,
            created_at: mat.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateMaterialRequest {
    pub category_id: String,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub min_quantity: i32,
    pub location: Option<String>,
    pub expires_on: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UpdateMaterialRequest {
    #[ts(optional)]
    pub location: Option<String>,
    #[ts(optional)]
    pub min_quantity: Option<i32>,
    #[ts(optional)]
    pub clear_location: Option<bool>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct WithdrawRequest {
    pub quantity: i32,
    pub notes: Option<String>,
    pub site_id: Option<String>, // Optional Baustelle ID
    pub disposal: Option<bool>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct AdjustStockRequest {
    pub quantity: i32,
    pub reason: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct StockInRequest {
    pub quantity: i32,
    pub notes: Option<String>,
    pub expires_on: Option<String>,
}

impl From<crate::modules::inventory::domain::MaterialBatchSummary> for ExpiryBatchResponse {
    fn from(batch: crate::modules::inventory::domain::MaterialBatchSummary) -> Self {
        Self {
            expires_on: batch.expires_on.to_string(),
            quantity: batch.quantity,
            is_expired: batch.is_expired,
            is_expiring_soon: batch.is_expiring_soon,
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ListMaterialsQuery {
    pub category_id: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct QrCodeResponse {
    pub qr_code: String,
    pub material_id: String,
    pub material_name: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct QrSvgResponse {
    pub svg: String,
    pub qr_code: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct OrderRequestResponse {
    pub id: String,
    pub material_id: String,
    pub material_name: String,
    pub quantity: i32,
    pub requested_by: String,
    pub status: String,
    pub reason: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub fulfilled_at: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl OrderRequestResponse {
    pub fn from_order(
        order: crate::modules::inventory::domain::OrderRequest,
        material_name: String,
    ) -> Self {
        Self {
            id: order.id.to_string(),
            material_id: order.material_id.to_string(),
            material_name,
            quantity: order.quantity,
            requested_by: order.requested_by.to_string(),
            status: order.status.to_string(),
            reason: order.reason,
            approved_by: order.approved_by.map(|id| id.to_string()),
            approved_at: order.approved_at.map(|t| t.to_rfc3339()),
            fulfilled_at: order.fulfilled_at.map(|t| t.to_rfc3339()),
            notes: order.notes,
            created_at: order.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateOrderRequestDto {
    pub material_id: String,
    pub quantity: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct ApproveOrderRequestDto {
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct FulfillOrderRequestDto {
    pub actual_quantity: i32,
    pub notes: Option<String>,
}

/// Response DTO for stock entry history
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct StockEntryResponse {
    pub id: String,
    pub quantity_change: i32,
    pub quantity_after: i32,
    pub notes: Option<String>,
    pub site_id: Option<String>,
    pub site_name: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct SiteStockHistoryResponse {
    pub id: String,
    pub material_id: String,
    pub material_name: String,
    pub category_name: String,
    pub quantity_change: i32,
    pub quantity_after: i32,
    pub notes: Option<String>,
    pub site_id: Option<String>,
    pub site_name: Option<String>,
    pub extracted_by: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct EnrichedStockHistoryResponse {
    pub id: String,
    pub material_id: String,
    pub user_id: String,
    pub user_name: String,
    pub entry_type: crate::modules::inventory::domain::EntryType,
    pub quantity_change: i32,
    pub quantity_after: i32,
    pub notes: Option<String>,
    pub site_id: Option<String>,
    pub site_name: Option<String>,
    pub category_name: String,
    pub created_at: String,
}

impl From<SiteStockHistoryEntry> for SiteStockHistoryResponse {
    fn from(entry: SiteStockHistoryEntry) -> Self {
        Self {
            id: entry.id.to_string(),
            material_id: entry.material_id.to_string(),
            material_name: entry.material_name,
            category_name: entry.category_name,
            quantity_change: entry.quantity_change,
            quantity_after: entry.quantity_after,
            notes: entry.notes,
            site_id: entry.site_id.map(|s| s.to_string()),
            site_name: entry.site_name,
            extracted_by: entry.extracted_by,
            created_at: entry.created_at.to_rfc3339(),
        }
    }
}

impl From<EnrichedStockEntry> for EnrichedStockHistoryResponse {
    fn from(entry: EnrichedStockEntry) -> Self {
        Self {
            id: entry.id.to_string(),
            material_id: entry.material_id.to_string(),
            user_id: entry.user_id.to_string(),
            user_name: entry.user_name,
            entry_type: entry.entry_type,
            quantity_change: entry.quantity_change,
            quantity_after: entry.quantity_after,
            notes: entry.notes,
            site_id: entry.site_id.map(|s| s.to_string()),
            site_name: entry.site_name,
            category_name: entry.category_name,
            created_at: entry.created_at.to_rfc3339(),
        }
    }
}

impl From<StockEntryWithSite> for StockEntryResponse {
    fn from(entry: StockEntryWithSite) -> Self {
        Self {
            id: entry.id.to_string(),
            quantity_change: entry.quantity_change,
            quantity_after: entry.quantity_after,
            notes: entry.notes,
            site_id: entry.site_id.map(|s| s.to_string()),
            site_name: entry.site_name,
            created_at: entry.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct OrderStatusQuery {
    pub status: Option<String>,
}

impl From<UpdateCategoryRequest> for UpdateCategory {
    fn from(request: UpdateCategoryRequest) -> Self {
        Self {
            name: request.name,
            description: request.description,
            can_expire: request.can_expire,
        }
    }
}

impl From<UpdateMaterialRequest> for UpdateMaterial {
    fn from(request: UpdateMaterialRequest) -> Self {
        Self {
            location: request.location,
            min_quantity: request.min_quantity,
            clear_location: request.clear_location,
        }
    }
}

fn parse_optional_date(
    value: Option<String>,
    field_name: &str,
) -> Result<Option<NaiveDate>, AppError> {
    value
        .filter(|date| !date.trim().is_empty())
        .map(|date| {
            NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .map_err(|_| AppError::Validation(format!("Invalid {}", field_name)))
        })
        .transpose()
}

// === Handlers ===

pub async fn list_categories(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let categories = service.list_categories(&ctx).await?;
    let response: Vec<CategoryResponse> =
        categories.into_iter().map(CategoryResponse::from).collect();

    Ok(Json(response))
}

pub async fn create_category(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let create = CreateCategory {
        name: request.name,
        description: request.description,
        can_expire: request.can_expire,
    };

    let category = service.create_category(create, &ctx).await?;

    Ok((StatusCode::CREATED, Json(CategoryResponse::from(category))))
}

pub async fn get_category(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let category_id = Uuid::parse_str(&id)
        .map(CategoryId)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;

    let category = service.get_category(category_id, &ctx).await?;

    Ok(Json(CategoryResponse::from(category)))
}

pub async fn update_category(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let category_id = Uuid::parse_str(&id)
        .map(CategoryId)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;

    let category = service
        .update_category(category_id, request.into(), &ctx)
        .await?;

    Ok(Json(CategoryResponse::from(category)))
}

pub async fn delete_category(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let category_id = Uuid::parse_str(&id)
        .map(CategoryId)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;

    service.delete_category(category_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

pub async fn list_materials(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Query(query): Query<ListMaterialsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let category_id = query
        .category_id
        .map(|s| Uuid::parse_str(&s).map(CategoryId))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;

    let materials = service.list_materials(category_id, &ctx).await?;
    let response: Vec<MaterialResponse> =
        materials.into_iter().map(MaterialResponse::from).collect();

    Ok(Json(response))
}

pub async fn create_material(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateMaterialRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let category_id = Uuid::parse_str(&request.category_id)
        .map(CategoryId)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;

    let unit = request
        .unit
        .parse()
        .map_err(|e: String| AppError::Validation(e))?;

    let create = CreateMaterial {
        category_id,
        name: request.name,
        description: request.description,
        unit,
        quantity: request.quantity,
        min_quantity: request.min_quantity,
        location: request.location,
        expires_on: parse_optional_date(request.expires_on, "MHD")?,
    };

    let material = service.create_material(create, &ctx).await?;

    Ok((StatusCode::CREATED, Json(MaterialResponse::from(material))))
}

pub async fn get_material(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let material = service.get_material(material_id, &ctx).await?;

    Ok(Json(MaterialResponse::from(material)))
}

pub async fn update_material(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateMaterialRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let material = service
        .update_material(material_id, request.into(), &ctx)
        .await?;

    Ok(Json(MaterialResponse::from(material)))
}

/// GET /api/v1/inventory/materials/{id}/history - Get stock change history for a material
pub async fn get_material_history(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let repo = crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool);
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    // Verify material exists in tenant
    repo.find_material_by_id(material_id, ctx.tenant_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Material not found".to_string()))?;

    // Get history with site names
    let entries = repo
        .list_stock_entries_with_site(material_id, ctx.tenant_id, 50)
        .await?;
    let response: Vec<StockEntryResponse> =
        entries.into_iter().map(StockEntryResponse::from).collect();

    Ok(Json(response))
}

pub async fn get_enriched_material_history(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let entries = service.list_enriched_history(material_id, &ctx).await?;
    let response: Vec<EnrichedStockHistoryResponse> = entries
        .into_iter()
        .map(EnrichedStockHistoryResponse::from)
        .collect();

    Ok(Json(response))
}

/// GET /api/v1/inventory/sites/{site_id}/history - Site-scoped stock history with enrichments
pub async fn get_site_material_history(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(site_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let repo =
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool.clone());
    let site_repo =
        crate::modules::sites::infrastructure::site_repository::SiteRepository::new(state.pool);
    let ctx = TenantContext::from_auth(&auth);

    let parsed_site_id = Uuid::parse_str(&site_id)
        .map(SiteId)
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    site_repo
        .find_site_by_id(ctx.tenant_id, parsed_site_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

    let entries = repo
        .list_stock_entries_for_site(parsed_site_id, ctx.tenant_id, 50)
        .await?;
    let response: Vec<SiteStockHistoryResponse> = entries
        .into_iter()
        .map(SiteStockHistoryResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn delete_material(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    service.delete_material(material_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

pub async fn withdraw_material(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<WithdrawRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    // Parse site_id if provided
    let site_id = request
        .site_id
        .map(|s| Uuid::parse_str(&s).map(SiteId))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

    let withdraw = WithdrawMaterial {
        material_id,
        quantity: request.quantity,
        notes: request.notes,
        site_id, // Pass parsed site_id
        disposal: request.disposal.unwrap_or(false),
    };

    let material = service.withdraw_material(withdraw, &ctx).await?;

    Ok(Json(MaterialResponse::from(material)))
}

pub async fn adjust_stock(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<AdjustStockRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let adjust = AdjustStock {
        material_id,
        quantity: request.quantity,
        reason: request.reason,
    };

    let material = service.adjust_stock(adjust, &ctx).await?;

    Ok(Json(MaterialResponse::from(material)))
}

pub async fn stock_in_material(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<StockInRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let material = service
        .stock_in(
            StockIn {
                material_id,
                quantity: request.quantity,
                notes: request.notes,
                expires_on: parse_optional_date(request.expires_on, "MHD")?,
            },
            &ctx,
        )
        .await?;

    Ok(Json(MaterialResponse::from(material)))
}

pub async fn list_low_stock(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let materials = service.list_low_stock(&ctx).await?;
    let response: Vec<MaterialResponse> =
        materials.into_iter().map(MaterialResponse::from).collect();

    Ok(Json(response))
}

pub async fn get_by_qr_code(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material = service.get_material_by_qr(&code, &ctx).await?;

    Ok(Json(MaterialResponse::from(material)))
}

pub async fn generate_qr_code(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let qr_code = service.generate_qr_code(material_id, &ctx).await?;
    let material = service.get_material(material_id, &ctx).await?;

    Ok(Json(QrCodeResponse {
        qr_code,
        material_id: material.id.to_string(),
        material_name: material.name,
    }))
}

pub async fn get_qr_svg(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let material = service.get_material(material_id, &ctx).await?;
    let svg = service.get_qr_code_svg(material_id, &ctx).await?;

    Ok(Json(QrSvgResponse {
        svg,
        qr_code: material.qr_code.unwrap_or_default(),
    }))
}

pub async fn create_order_request(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateOrderRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let material_id = Uuid::parse_str(&request.material_id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;

    let create = CreateOrderRequest {
        material_id,
        quantity: request.quantity,
        reason: request.reason,
    };

    let order = service.create_order_request(create, &ctx).await?;

    // Get material name for response
    let material = service.get_material(material_id, &ctx).await?;

    Ok((
        StatusCode::CREATED,
        Json(OrderRequestResponse::from_order(order, material.name)),
    ))
}

pub async fn list_order_requests(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Query(query): Query<OrderStatusQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let status = query
        .status
        .map(|s| s.parse())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let orders = service.list_order_requests(status, &ctx).await?;

    // Fetch material names for response
    let mut responses = Vec::new();
    for order in orders {
        let material = service.get_material(order.material_id, &ctx).await.ok();
        let material_name = material.map(|m| m.name).unwrap_or_default();
        responses.push(OrderRequestResponse::from_order(order, material_name));
    }

    Ok(Json(responses))
}

pub async fn approve_order_request(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<ApproveOrderRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let order_id = Uuid::parse_str(&id)
        .map(OrderRequestId)
        .map_err(|_| AppError::Validation("Invalid order ID".to_string()))?;

    let approve = ApproveOrderRequest {
        notes: request.notes,
    };

    let order = service
        .approve_order_request(order_id, approve, &ctx)
        .await?;
    let material = service.get_material(order.material_id, &ctx).await?;

    Ok(Json(OrderRequestResponse::from_order(order, material.name)))
}

pub async fn fulfill_order_request(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<FulfillOrderRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let order_id = Uuid::parse_str(&id)
        .map(OrderRequestId)
        .map_err(|_| AppError::Validation("Invalid order ID".to_string()))?;

    let fulfill = FulfillOrderRequest {
        actual_quantity: request.actual_quantity,
        notes: request.notes,
    };

    let order = service
        .fulfill_order_request(order_id, fulfill, &ctx)
        .await?;
    let material = service.get_material(order.material_id, &ctx).await?;

    Ok(Json(OrderRequestResponse::from_order(order, material.name)))
}

#[cfg(test)]
mod tests {
    use super::{
        EnrichedStockHistoryResponse, StockInRequest, UpdateCategoryRequest, UpdateMaterialRequest,
    };
    use crate::common::types::{MaterialId, SiteId, TenantId, UserId};
    use crate::modules::inventory::domain::{
        EnrichedStockEntry, EntryType, StockIn, UpdateCategory, UpdateMaterial,
    };
    use chrono::{NaiveDate, Utc};

    #[test]
    fn update_category_request_preserves_patch_semantics() {
        let unchanged: UpdateCategory = UpdateCategoryRequest {
            name: None,
            description: None,
            can_expire: None,
        }
        .into();
        assert_eq!(unchanged.name, None);
        assert_eq!(unchanged.description, None);
        assert_eq!(unchanged.can_expire, None);

        let clear_description: UpdateCategory = UpdateCategoryRequest {
            name: None,
            description: Some(String::new()),
            can_expire: Some(true),
        }
        .into();
        assert_eq!(clear_description.name, None);
        assert_eq!(clear_description.description, Some(String::new()));
        assert_eq!(clear_description.can_expire, Some(true));
    }

    #[test]
    fn update_material_request_preserves_clear_location_without_fabricating_fields() {
        let update: UpdateMaterial = UpdateMaterialRequest {
            location: None,
            min_quantity: None,
            clear_location: Some(true),
        }
        .into();

        assert_eq!(update.location, None);
        assert_eq!(update.min_quantity, None);
        assert_eq!(update.clear_location, Some(true));
    }

    #[test]
    fn stock_in_request_keeps_notes_and_reuses_domain_validation() {
        let request = StockInRequest {
            quantity: 4,
            notes: Some("Lieferschein 1234".to_string()),
            expires_on: Some("2026-05-20".to_string()),
        };
        let stock_in = StockIn {
            material_id: MaterialId::new(),
            quantity: request.quantity,
            notes: request.notes.clone(),
            expires_on: Some(NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()),
        };

        assert_eq!(stock_in.notes, Some("Lieferschein 1234".to_string()));
        assert_eq!(
            stock_in.expires_on,
            Some(NaiveDate::from_ymd_opt(2026, 5, 20).unwrap())
        );
        assert!(stock_in.validate().is_ok());

        let invalid = StockIn {
            material_id: MaterialId::new(),
            quantity: 0,
            notes: None,
            expires_on: None,
        };
        assert_eq!(
            invalid.validate(),
            Err("Stock-in quantity must be positive".to_string())
        );
    }

    #[test]
    fn enriched_history_response_exposes_phase_32_ui_fields() {
        let entry = EnrichedStockEntry {
            id: uuid::Uuid::new_v4(),
            tenant_id: TenantId::new(),
            material_id: MaterialId::new(),
            user_id: UserId::new(),
            user_name: "Max Mustermann".to_string(),
            entry_type: EntryType::Withdrawn,
            quantity_change: -4,
            quantity_after: 12,
            notes: Some("Für Baustelle".to_string()),
            site_id: Some(SiteId::new()),
            site_name: Some("Baustelle Müller".to_string()),
            category_name: "Schrauben".to_string(),
            created_at: Utc::now(),
        };

        let response = EnrichedStockHistoryResponse::from(entry);

        assert_eq!(response.entry_type, EntryType::Withdrawn);
        assert_eq!(response.user_name, "Max Mustermann");
        assert!(response.site_id.is_some());
        assert_eq!(response.site_name, Some("Baustelle Müller".to_string()));
        assert_eq!(response.category_name, "Schrauben");
    }
}
