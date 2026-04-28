use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{MaterialId, CategoryId};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::inventory::application::InventoryService;
use crate::modules::inventory::domain::{CreateCategory, CreateMaterial, WithdrawMaterial, AdjustStock};
use crate::AppState;

/// Create the inventory API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Categories
        .route("/api/v1/inventory/categories", get(list_categories).post(create_category))
        .route("/api/v1/inventory/categories/{id}", get(get_category))
        
        // Materials
        .route("/api/v1/inventory/materials", get(list_materials).post(create_material))
        .route("/api/v1/inventory/materials/{id}", get(get_material))
        .route("/api/v1/inventory/materials/{id}/withdraw", post(withdraw_material))
        .route("/api/v1/inventory/materials/{id}/adjust", post(adjust_stock))
        
        // Low stock
        .route("/api/v1/inventory/low-stock", get(list_low_stock))
        
        // QR lookup
        .route("/api/v1/inventory/qr/{code}", get(get_by_qr_code))
}

// === DTOs ===

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct CreateMaterialRequest {
    pub category_id: String,
    pub name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub min_quantity: i32,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WithdrawRequest {
    pub quantity: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdjustStockRequest {
    pub quantity: i32,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct ListMaterialsQuery {
    pub category_id: Option<String>,
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
