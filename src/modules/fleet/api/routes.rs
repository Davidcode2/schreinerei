use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
    routing::{get, post, patch, delete},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{VehicleId, ToolId, VehicleType, ResourceStatus};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::fleet::application::fleet_service::FleetService;
use crate::modules::fleet::domain::{CreateVehicle, UpdateVehicle, CreateTool, UpdateTool};
use crate::modules::fleet::infrastructure::fleet_repository::FleetRepository;
use crate::AppState;

/// Create the fleet API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Vehicles
        .route("/api/v1/fleet/vehicles", get(list_vehicles).post(create_vehicle))
        .route("/api/v1/fleet/vehicles/{id}", get(get_vehicle).patch(update_vehicle).delete(delete_vehicle))
        
        // Tools
        .route("/api/v1/fleet/tools", get(list_tools).post(create_tool))
        .route("/api/v1/fleet/tools/{id}", get(get_tool).patch(update_tool).delete(delete_tool))
}

// === DTOs ===

#[derive(Debug, Serialize)]
pub struct VehicleResponse {
    pub id: String,
    pub name: String,
    pub license_plate: Option<String>,
    pub vehicle_type: String,
    pub description: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::modules::fleet::domain::Vehicle> for VehicleResponse {
    fn from(vehicle: crate::modules::fleet::domain::Vehicle) -> Self {
        Self {
            id: vehicle.id.to_string(),
            name: vehicle.name,
            license_plate: vehicle.license_plate,
            vehicle_type: vehicle.vehicle_type.to_string(),
            description: vehicle.description,
            status: vehicle.status.to_string(),
            location: vehicle.location,
            qr_code: vehicle.qr_code,
            created_at: vehicle.created_at.to_rfc3339(),
            updated_at: vehicle.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateVehicleRequest {
    pub name: String,
    pub license_plate: Option<String>,
    pub vehicle_type: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVehicleRequest {
    pub name: Option<String>,
    pub license_plate: Option<String>,
    pub vehicle_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListVehiclesQuery {
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ToolResponse {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::modules::fleet::domain::Tool> for ToolResponse {
    fn from(tool: crate::modules::fleet::domain::Tool) -> Self {
        Self {
            id: tool.id.to_string(),
            name: tool.name,
            category: tool.category,
            description: tool.description,
            status: tool.status.to_string(),
            location: tool.location,
            qr_code: tool.qr_code,
            created_at: tool.created_at.to_rfc3339(),
            updated_at: tool.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateToolRequest {
    pub name: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateToolRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListToolsQuery {
    pub status: Option<String>,
    pub category: Option<String>,
}

// === Vehicle Handlers ===

pub async fn list_vehicles(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Query(query): Query<ListVehiclesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let vehicles = service.list_vehicles(query.status, &ctx).await?;
    let response: Vec<VehicleResponse> = vehicles.into_iter().map(VehicleResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn create_vehicle(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateVehicleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let vehicle_type = request.vehicle_type.parse::<VehicleType>()
        .map_err(|e: String| AppError::Validation(e))?;
    
    let create = CreateVehicle {
        name: request.name,
        license_plate: request.license_plate,
        vehicle_type,
        description: request.description,
        location: request.location,
        qr_code: request.qr_code,
    };
    
    let vehicle = service.create_vehicle(create, &ctx).await?;
    
    Ok((StatusCode::CREATED, Json(VehicleResponse::from(vehicle))))
}

pub async fn get_vehicle(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let vehicle_id = Uuid::parse_str(&id)
        .map(VehicleId)
        .map_err(|_| AppError::Validation("Invalid vehicle ID".to_string()))?;
    
    let vehicle = service.get_vehicle(vehicle_id, &ctx).await?;
    
    Ok(Json(VehicleResponse::from(vehicle)))
}

pub async fn update_vehicle(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateVehicleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let vehicle_id = Uuid::parse_str(&id)
        .map(VehicleId)
        .map_err(|_| AppError::Validation("Invalid vehicle ID".to_string()))?;
    
    let vehicle_type = request.vehicle_type
        .map(|s| s.parse::<VehicleType>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;
    
    let status = request.status
        .map(|s| s.parse::<ResourceStatus>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;
    
    let update = UpdateVehicle {
        name: request.name,
        license_plate: request.license_plate,
        vehicle_type,
        description: request.description,
        status,
        location: request.location,
        qr_code: request.qr_code,
    };
    
    let vehicle = service.update_vehicle(vehicle_id, update, &ctx).await?;
    
    Ok(Json(VehicleResponse::from(vehicle)))
}

pub async fn delete_vehicle(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let vehicle_id = Uuid::parse_str(&id)
        .map(VehicleId)
        .map_err(|_| AppError::Validation("Invalid vehicle ID".to_string()))?;
    
    service.delete_vehicle(vehicle_id, &ctx).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

// === Tool Handlers ===

pub async fn list_tools(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Query(query): Query<ListToolsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let tools = service.list_tools(query.status, query.category, &ctx).await?;
    let response: Vec<ToolResponse> = tools.into_iter().map(ToolResponse::from).collect();
    
    Ok(Json(response))
}

pub async fn create_tool(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<CreateToolRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let create = CreateTool {
        name: request.name,
        category: request.category,
        description: request.description,
        location: request.location,
        qr_code: request.qr_code,
    };
    
    let tool = service.create_tool(create, &ctx).await?;
    
    Ok((StatusCode::CREATED, Json(ToolResponse::from(tool))))
}

pub async fn get_tool(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let tool_id = Uuid::parse_str(&id)
        .map(ToolId)
        .map_err(|_| AppError::Validation("Invalid tool ID".to_string()))?;
    
    let tool = service.get_tool(tool_id, &ctx).await?;
    
    Ok(Json(ToolResponse::from(tool)))
}

pub async fn update_tool(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateToolRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let tool_id = Uuid::parse_str(&id)
        .map(ToolId)
        .map_err(|_| AppError::Validation("Invalid tool ID".to_string()))?;
    
    let status = request.status
        .map(|s| s.parse::<ResourceStatus>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;
    
    let update = UpdateTool {
        name: request.name,
        category: request.category,
        description: request.description,
        status,
        location: request.location,
        qr_code: request.qr_code,
    };
    
    let tool = service.update_tool(tool_id, update, &ctx).await?;
    
    Ok(Json(ToolResponse::from(tool)))
}

pub async fn delete_tool(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);
    
    let tool_id = Uuid::parse_str(&id)
        .map(ToolId)
        .map_err(|_| AppError::Validation("Invalid tool ID".to_string()))?;
    
    service.delete_tool(tool_id, &ctx).await?;
    
    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}
