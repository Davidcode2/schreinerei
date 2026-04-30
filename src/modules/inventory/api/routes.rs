use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{MaterialId, CategoryId, OrderRequestId};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::inventory::application::InventoryService;
use crate::modules::inventory::domain::{
    CreateCategory, CreateMaterial, WithdrawMaterial, AdjustStock,
    CreateOrderRequest, ApproveOrderRequest, FulfillOrderRequest,
};
use crate::AppState;

/// Create the inventory API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Categories
        .route("/api/v1/inventory/categories", get(list_categories).post(create_category))
        .route("/api/v1/inventory/categories/{id}", get(get_category))
        
        // Materials
        .route("/api/v1/inventory/materials", get(list_materials).post(create_material))
        .route("/api/v1/inventory/materials/{id}", get(get_material).delete(delete_material))
        .route("/api/v1/inventory/materials/{id}/withdraw", post(withdraw_material))
        .route("/api/v1/inventory/materials/{id}/adjust", post(adjust_stock))
        .route("/api/v1/inventory/materials/{id}/qr", post(generate_qr_code))
        .route("/api/v1/inventory/materials/{id}/qr/svg", get(get_qr_svg))
        
        // Low stock
        .route("/api/v1/inventory/low-stock", get(list_low_stock))
        
        // QR lookup
        .route("/api/v1/inventory/qr/{code}", get(get_by_qr_code))
        
        // Order requests
        .route("/api/v1/inventory/orders", get(list_order_requests).post(create_order_request))
        .route("/api/v1/inventory/orders/{id}/approve", post(approve_order_request))
        .route("/api/v1/inventory/orders/{id}/fulfill", post(fulfill_order_request))
}

// === DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

impl From<crate::modules::inventory::domain::Category> for CategoryResponse {
    fn from(cat: crate::modules::inventory::domain::Category) -> Self {
        Self {
            id: cat.id.to_string(),
            name: cat.name,
            description: cat.description,
            created_at: cat.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
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
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct WithdrawRequest {
    pub quantity: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct AdjustStockRequest {
    pub quantity: i32,
    pub reason: String,
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
    pub fn from_order(order: crate::modules::inventory::domain::OrderRequest, material_name: String) -> Self {
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

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct OrderStatusQuery {
    pub status: Option<String>,
}

// === Handlers ===

pub async fn list_categories(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let categories = service.list_categories(&ctx).await?;
    let response: Vec<CategoryResponse> = categories.into_iter().map(CategoryResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn create_category(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let create = CreateCategory {
        name: request.name,
        description: request.description,
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let category_id = Uuid::parse_str(&id)
        .map(CategoryId)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;
    
    let category = service.get_category(category_id, &ctx).await?;
    
    Ok(Json(CategoryResponse::from(category)))
}

pub async fn list_materials(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Query(query): Query<ListMaterialsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let category_id = query.category_id
        .map(|s| Uuid::parse_str(&s).map(CategoryId))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;
    
    let materials = service.list_materials(category_id, &ctx).await?;
    let response: Vec<MaterialResponse> = materials.into_iter().map(MaterialResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn create_material(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateMaterialRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let category_id = Uuid::parse_str(&request.category_id)
        .map(CategoryId)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;
    
    let unit = request.unit.parse()
        .map_err(|e: String| AppError::Validation(e))?;
    
    let create = CreateMaterial {
        category_id,
        name: request.name,
        description: request.description,
        unit,
        quantity: request.quantity,
        min_quantity: request.min_quantity,
        location: request.location,
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;
    
    let material = service.get_material(material_id, &ctx).await?;
    
    Ok(Json(MaterialResponse::from(material)))
}

pub async fn delete_material(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let material_id = Uuid::parse_str(&id)
        .map(MaterialId)
        .map_err(|_| AppError::Validation("Invalid material ID".to_string()))?;
    
    let withdraw = WithdrawMaterial {
        material_id,
        quantity: request.quantity,
        notes: request.notes,
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
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

pub async fn list_low_stock(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let materials = service.list_low_stock(&ctx).await?;
    let response: Vec<MaterialResponse> = materials.into_iter().map(MaterialResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn get_by_qr_code(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
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
    
    Ok((StatusCode::CREATED, Json(OrderRequestResponse::from_order(order, material.name))))
}

pub async fn list_order_requests(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Query(query): Query<OrderStatusQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = InventoryService::new(
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let status = query.status
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let order_id = Uuid::parse_str(&id)
        .map(OrderRequestId)
        .map_err(|_| AppError::Validation("Invalid order ID".to_string()))?;
    
    let approve = ApproveOrderRequest {
        notes: request.notes,
    };
    
    let order = service.approve_order_request(order_id, approve, &ctx).await?;
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
        crate::modules::inventory::infrastructure::MaterialRepository::new(state.pool)
    );
    let ctx = TenantContext::from_auth(&auth);
    
    let order_id = Uuid::parse_str(&id)
        .map(OrderRequestId)
        .map_err(|_| AppError::Validation("Invalid order ID".to_string()))?;
    
    let fulfill = FulfillOrderRequest {
        actual_quantity: request.actual_quantity,
        notes: request.notes,
    };
    
    let order = service.fulfill_order_request(order_id, fulfill, &ctx).await?;
    let material = service.get_material(order.material_id, &ctx).await?;
    
    Ok(Json(OrderRequestResponse::from_order(order, material.name)))
}
