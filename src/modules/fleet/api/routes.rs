use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{
    AssetId, MachineId, MaintenanceDueId, MaintenanceDueStatus, ReservationId, ReservationStatus,
    ResourceStatus, ResourceType, ToolId, VehicleId, VehicleType,
};
use crate::modules::fleet::application::fleet_service::FleetService;
use crate::modules::fleet::domain::{
    CreateMachine, CreateMaintenanceSchedule, CreateReservation, CreateTool, CreateVehicle,
    ResolveMaintenanceDue, UpdateMachine, UpdateReservation, UpdateTool, UpdateVehicle,
};
use crate::modules::fleet::infrastructure::fleet_repository::{
    FleetRepository, ReservationSummary,
};
use crate::modules::iam::application::user_service::TenantContext;
use crate::AppState;

/// Create the fleet API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Vehicles
        .route(
            "/api/v1/fleet/vehicles",
            get(list_vehicles).post(create_vehicle),
        )
        .route(
            "/api/v1/fleet/vehicles/{id}",
            get(get_vehicle)
                .patch(update_vehicle)
                .delete(delete_vehicle),
        )
        // Tools
        .route("/api/v1/fleet/tools", get(list_tools).post(create_tool))
        .route(
            "/api/v1/fleet/tools/{id}",
            get(get_tool).patch(update_tool).delete(delete_tool),
        )
        // Machines
        .route(
            "/api/v1/fleet/machines",
            get(list_machines).post(create_machine),
        )
        .route(
            "/api/v1/fleet/machines/{id}",
            get(get_machine)
                .patch(update_machine)
                .delete(delete_machine),
        )
        // Reservations
        .route(
            "/api/v1/fleet/reservations",
            get(list_reservations).post(create_reservation),
        )
        .route("/api/v1/fleet/reservations/my", get(list_my_reservations))
        .route(
            "/api/v1/fleet/reservations/{id}",
            get(get_reservation)
                .patch(update_reservation)
                .delete(cancel_reservation),
        )
        // Calendar
        .route("/api/v1/fleet/calendar", get(get_calendar))
        // Maintenance
        .route(
            "/api/v1/fleet/maintenance/schedules",
            get(list_maintenance_schedules).post(create_maintenance_schedule),
        )
        .route("/api/v1/fleet/maintenance/due", get(list_maintenance_due))
        .route(
            "/api/v1/fleet/maintenance/due/{id}/resolve",
            axum::routing::post(resolve_maintenance_due),
        )
        // Availability
        .route("/api/v1/fleet/availability", get(check_availability))
        // QR Code
        .route("/api/v1/fleet/qr/{code}", get(get_status_by_qr))
}

// === DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct VehicleResponse {
    pub id: String,
    pub name: String,
    pub license_plate: Option<String>,
    pub vehicle_type: String,
    pub description: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub display_color: String,
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
            display_color: vehicle.display_color,
            created_at: vehicle.created_at.to_rfc3339(),
            updated_at: vehicle.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CreateVehicleRequest {
    pub name: String,
    pub license_plate: Option<String>,
    pub vehicle_type: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub display_color: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdateVehicleRequest {
    pub name: Option<String>,
    pub license_plate: Option<String>,
    pub vehicle_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub display_color: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ListVehiclesQuery {
    pub status: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
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

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CreateToolRequest {
    pub name: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdateToolRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ListToolsQuery {
    pub status: Option<String>,
    pub category: Option<String>,
}

// === Machine DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct MachineResponse {
    pub id: String,
    pub name: String,
    pub machine_type: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub qr_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::modules::fleet::domain::Machine> for MachineResponse {
    fn from(machine: crate::modules::fleet::domain::Machine) -> Self {
        Self {
            id: machine.id.to_string(),
            name: machine.name,
            machine_type: machine.machine_type,
            description: machine.description,
            status: machine.status.to_string(),
            location: machine.location,
            qr_code: machine.qr_code,
            created_at: machine.created_at.to_rfc3339(),
            updated_at: machine.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CreateMachineRequest {
    pub name: String,
    pub machine_type: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdateMachineRequest {
    pub name: Option<String>,
    pub machine_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ListMachinesQuery {
    pub status: Option<String>,
}

// === Maintenance DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct MaintenanceScheduleResponse {
    pub id: String,
    pub asset_id: String,
    pub task_description: String,
    pub interval_days: i32,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::modules::fleet::domain::MaintenanceSchedule> for MaintenanceScheduleResponse {
    fn from(schedule: crate::modules::fleet::domain::MaintenanceSchedule) -> Self {
        Self {
            id: schedule.id.to_string(),
            asset_id: schedule.asset_id.to_string(),
            task_description: schedule.task_description,
            interval_days: schedule.interval_days,
            is_active: schedule.is_active,
            created_at: schedule.created_at.to_rfc3339(),
            updated_at: schedule.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct MaintenanceDueResponse {
    pub id: String,
    pub schedule_id: String,
    pub asset_id: String,
    pub resource_type: String,
    pub resource_name: String,
    pub task_description: String,
    pub due_date: String,
    pub status: String,
    pub severity: String,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
    pub resolution_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::modules::fleet::domain::MaintenanceDue> for MaintenanceDueResponse {
    fn from(due: crate::modules::fleet::domain::MaintenanceDue) -> Self {
        let today = Utc::now().date_naive();
        let severity = due.severity(today).to_string();

        Self {
            id: due.id.to_string(),
            schedule_id: due.schedule_id.to_string(),
            asset_id: due.asset_id.to_string(),
            resource_type: due.resource_type.to_string(),
            resource_name: due.resource_name,
            task_description: due.task_description,
            due_date: due.due_date.to_string(),
            status: due.status.to_string(),
            severity,
            resolved_at: due.resolved_at.map(|value| value.to_rfc3339()),
            resolved_by: due.resolved_by.map(|value| value.to_string()),
            resolution_notes: due.resolution_notes,
            created_at: due.created_at.to_rfc3339(),
            updated_at: due.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CreateMaintenanceScheduleResponse {
    pub schedule: MaintenanceScheduleResponse,
    pub due: MaintenanceDueResponse,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CreateMaintenanceScheduleRequest {
    pub asset_id: String,
    pub task_description: String,
    pub interval_days: i32,
    pub next_due_date: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ListMaintenanceSchedulesQuery {
    pub asset_id: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ListMaintenanceDueQuery {
    pub asset_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ResolveMaintenanceDueRequest {
    pub resolution_notes: Option<String>,
}

// === Reservation DTOs ===

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ReservationResponse {
    pub id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub resource_name: String,
    pub user_id: String,
    pub user_name: Option<String>,
    pub site_id: Option<String>,
    pub site_name: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub current_holder: Option<ReservationHolderResponse>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::modules::fleet::domain::ReservationWithDetails> for ReservationResponse {
    fn from(details: crate::modules::fleet::domain::ReservationWithDetails) -> Self {
        Self {
            id: details.reservation.id.to_string(),
            resource_type: details.reservation.resource_type.to_string(),
            resource_id: details.reservation.resource_id.to_string(),
            resource_name: details.resource_name,
            user_id: details.reservation.user_id.to_string(),
            user_name: details.user_name,
            site_id: details.reservation.site_id.map(|s| s.to_string()),
            site_name: details.site_name,
            project_id: details.reservation.project_id.map(|s| s.to_string()),
            project_name: details.project_name,
            start_time: details.reservation.start_time.to_rfc3339(),
            end_time: details.reservation.end_time.to_rfc3339(),
            status: details.reservation.status.to_string(),
            purpose: details.reservation.purpose,
            notes: details.reservation.notes,
            current_holder: details.current_holder.map(ReservationHolderResponse::from),
            created_at: details.reservation.created_at.to_rfc3339(),
            updated_at: details.reservation.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ReservationHolderResponse {
    pub user_id: String,
    pub user_name: Option<String>,
}

impl From<crate::modules::fleet::domain::ReservationHolder> for ReservationHolderResponse {
    fn from(holder: crate::modules::fleet::domain::ReservationHolder) -> Self {
        Self {
            user_id: holder.user_id.to_string(),
            user_name: holder.user_name,
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CreateReservationRequest {
    pub resource_type: String,
    pub resource_id: String,
    pub site_id: Option<String>,
    pub project_id: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub purpose: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdateReservationRequest {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub site_id: Option<String>,
    pub project_id: Option<String>,
    pub purpose: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ListReservationsQuery {
    pub user_id: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub site_id: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CalendarResponse {
    pub resources: Vec<CalendarEntryResponse>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CalendarEntryResponse {
    pub resource_type: String,
    pub resource_id: String,
    pub resource_name: String,
    pub resource_display_color: Option<String>,
    pub reservations: Vec<ReservationSummaryResponse>,
}

impl From<crate::modules::fleet::infrastructure::fleet_repository::CalendarEntry>
    for CalendarEntryResponse
{
    fn from(entry: crate::modules::fleet::infrastructure::fleet_repository::CalendarEntry) -> Self {
        Self {
            resource_type: entry.resource_type.to_string(),
            resource_id: entry.resource_id.to_string(),
            resource_name: entry.resource_name,
            resource_display_color: entry.resource_display_color,
            reservations: entry
                .reservations
                .into_iter()
                .map(ReservationSummaryResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ReservationSummaryResponse {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
    pub user_name: Option<String>,
    pub site_id: Option<String>,
    pub site_name: Option<String>,
    pub status: String,
}

impl From<ReservationSummary> for ReservationSummaryResponse {
    fn from(summary: ReservationSummary) -> Self {
        Self {
            id: summary.id.to_string(),
            start_time: summary.start_time.to_rfc3339(),
            end_time: summary.end_time.to_rfc3339(),
            user_name: summary.user_name,
            site_id: summary.site_id.map(|value| value.to_string()),
            site_name: summary.site_name,
            status: summary.status.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct CalendarQuery {
    pub start_date: String,
    pub end_date: String,
    pub resource_type: Option<String>,
    pub site_id: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct AvailabilityResponse {
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<ConflictDetail>>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct ConflictDetail {
    pub id: String,
    pub user_name: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct AvailabilityQuery {
    pub resource_type: String,
    pub resource_id: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct QrStatusResponse {
    pub resource_type: String,
    pub resource_id: String,
    pub resource_name: String,
    pub status: String,
    pub current_reservation: Option<ReservationSummaryResponse>,
    pub upcoming_reservations: Vec<ReservationSummaryResponse>,
}

impl From<crate::modules::fleet::infrastructure::fleet_repository::ResourceStatusInfo>
    for QrStatusResponse
{
    fn from(
        info: crate::modules::fleet::infrastructure::fleet_repository::ResourceStatusInfo,
    ) -> Self {
        Self {
            resource_type: info.resource_type.to_string(),
            resource_id: info.resource_id.to_string(),
            resource_name: info.resource_name,
            status: info.status.to_string(),
            current_reservation: info
                .current_reservation
                .map(ReservationSummaryResponse::from),
            upcoming_reservations: info
                .upcoming_reservations
                .into_iter()
                .map(ReservationSummaryResponse::from)
                .collect(),
        }
    }
}

// === Vehicle Handlers ===

pub async fn list_vehicles(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<ListVehiclesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let vehicles = service.list_vehicles(query.status, &ctx).await?;
    let response: Vec<VehicleResponse> = vehicles.into_iter().map(VehicleResponse::from).collect();

    Ok(Json(response))
}

pub async fn create_vehicle(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<CreateVehicleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let vehicle_type = request
        .vehicle_type
        .parse::<VehicleType>()
        .map_err(|e: String| AppError::Validation(e))?;

    let create = CreateVehicle {
        name: request.name,
        license_plate: request.license_plate,
        vehicle_type,
        description: request.description,
        location: request.location,
        qr_code: request.qr_code,
        display_color: request
            .display_color
            .map(|color| color.to_ascii_lowercase()),
    };

    let vehicle = service.create_vehicle(create, &ctx).await?;

    Ok((StatusCode::CREATED, Json(VehicleResponse::from(vehicle))))
}

pub async fn get_vehicle(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let vehicle_id = Uuid::parse_str(&id)
        .map(VehicleId)
        .map_err(|_| AppError::Validation("Invalid vehicle ID".to_string()))?;

    let vehicle = service.get_vehicle(vehicle_id, &ctx).await?;

    Ok(Json(VehicleResponse::from(vehicle)))
}

pub async fn update_vehicle(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateVehicleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let vehicle_id = Uuid::parse_str(&id)
        .map(VehicleId)
        .map_err(|_| AppError::Validation("Invalid vehicle ID".to_string()))?;

    let vehicle_type = request
        .vehicle_type
        .map(|s| s.parse::<VehicleType>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let status = request
        .status
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
        display_color: request
            .display_color
            .map(|color| color.to_ascii_lowercase()),
    };

    let vehicle = service.update_vehicle(vehicle_id, update, &ctx).await?;

    Ok(Json(VehicleResponse::from(vehicle)))
}

pub async fn delete_vehicle(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let vehicle_id = Uuid::parse_str(&id)
        .map(VehicleId)
        .map_err(|_| AppError::Validation("Invalid vehicle ID".to_string()))?;

    service.delete_vehicle(vehicle_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

// === Tool Handlers ===

pub async fn list_tools(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<ListToolsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let tools = service
        .list_tools(query.status, query.category, &ctx)
        .await?;
    let response: Vec<ToolResponse> = tools.into_iter().map(ToolResponse::from).collect();

    Ok(Json(response))
}

pub async fn create_tool(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<CreateToolRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

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
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let tool_id = Uuid::parse_str(&id)
        .map(ToolId)
        .map_err(|_| AppError::Validation("Invalid tool ID".to_string()))?;

    let tool = service.get_tool(tool_id, &ctx).await?;

    Ok(Json(ToolResponse::from(tool)))
}

pub async fn update_tool(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateToolRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let tool_id = Uuid::parse_str(&id)
        .map(ToolId)
        .map_err(|_| AppError::Validation("Invalid tool ID".to_string()))?;

    let status = request
        .status
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
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let tool_id = Uuid::parse_str(&id)
        .map(ToolId)
        .map_err(|_| AppError::Validation("Invalid tool ID".to_string()))?;

    service.delete_tool(tool_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

// === Machine Handlers ===

pub async fn list_machines(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<ListMachinesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let machines = service.list_machines(query.status, &ctx).await?;
    let response: Vec<MachineResponse> = machines.into_iter().map(MachineResponse::from).collect();

    Ok(Json(response))
}

pub async fn create_machine(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<CreateMachineRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let create = CreateMachine {
        name: request.name,
        machine_type: request.machine_type,
        description: request.description,
        location: request.location,
        qr_code: request.qr_code,
    };

    let machine = service.create_machine(create, &ctx).await?;

    Ok((StatusCode::CREATED, Json(MachineResponse::from(machine))))
}

pub async fn get_machine(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let machine_id = Uuid::parse_str(&id)
        .map(MachineId)
        .map_err(|_| AppError::Validation("Invalid machine ID".to_string()))?;

    let machine = service.get_machine(machine_id, &ctx).await?;

    Ok(Json(MachineResponse::from(machine)))
}

pub async fn update_machine(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateMachineRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let machine_id = Uuid::parse_str(&id)
        .map(MachineId)
        .map_err(|_| AppError::Validation("Invalid machine ID".to_string()))?;

    let status = request
        .status
        .map(|s| s.parse::<ResourceStatus>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let update = UpdateMachine {
        name: request.name,
        machine_type: request.machine_type,
        description: request.description,
        status,
        location: request.location,
        qr_code: request.qr_code,
    };

    let machine = service.update_machine(machine_id, update, &ctx).await?;

    Ok(Json(MachineResponse::from(machine)))
}

pub async fn delete_machine(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let machine_id = Uuid::parse_str(&id)
        .map(MachineId)
        .map_err(|_| AppError::Validation("Invalid machine ID".to_string()))?;

    service.delete_machine(machine_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

// === Maintenance Handlers ===

pub async fn create_maintenance_schedule(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<CreateMaintenanceScheduleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let asset_id = Uuid::parse_str(&request.asset_id)
        .map(AssetId)
        .map_err(|_| AppError::Validation("Invalid asset ID".to_string()))?;
    let next_due_date = NaiveDate::parse_from_str(&request.next_due_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid due date format. Use YYYY-MM-DD".to_string()))?;

    let create = CreateMaintenanceSchedule {
        asset_id,
        task_description: request.task_description,
        interval_days: request.interval_days,
        next_due_date,
    };

    let (schedule, due) = service.create_maintenance_schedule(create, &ctx).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateMaintenanceScheduleResponse {
            schedule: MaintenanceScheduleResponse::from(schedule),
            due: MaintenanceDueResponse::from(due),
        }),
    ))
}

pub async fn list_maintenance_schedules(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<ListMaintenanceSchedulesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let asset_id = query
        .asset_id
        .map(|value| Uuid::parse_str(&value))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid asset ID".to_string()))?
        .map(AssetId);

    let schedules = service.list_maintenance_schedules(asset_id, &ctx).await?;
    let response: Vec<MaintenanceScheduleResponse> = schedules
        .into_iter()
        .map(MaintenanceScheduleResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn list_maintenance_due(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<ListMaintenanceDueQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let asset_id = query
        .asset_id
        .map(|value| Uuid::parse_str(&value))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid asset ID".to_string()))?
        .map(AssetId);
    let status = query
        .status
        .map(|value| value.parse::<MaintenanceDueStatus>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let due_records = service.list_maintenance_due(asset_id, status, &ctx).await?;
    let response: Vec<MaintenanceDueResponse> = due_records
        .into_iter()
        .map(MaintenanceDueResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn resolve_maintenance_due(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<ResolveMaintenanceDueRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));
    let due_id = Uuid::parse_str(&id)
        .map(MaintenanceDueId)
        .map_err(|_| AppError::Validation("Invalid maintenance due ID".to_string()))?;

    let due = service
        .resolve_maintenance_due(
            due_id,
            ResolveMaintenanceDue {
                resolution_notes: request.resolution_notes,
            },
            &ctx,
        )
        .await?;

    Ok(Json(MaintenanceDueResponse::from(due)))
}

// === Reservation Handlers ===

pub async fn list_reservations(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<ListReservationsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let user_id = query
        .user_id
        .map(|s| Uuid::parse_str(&s))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?
        .map(crate::common::types::UserId);

    let resource_type = query
        .resource_type
        .map(|s| s.parse::<ResourceType>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let resource_id = query
        .resource_id
        .map(|s| Uuid::parse_str(&s))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid resource ID".to_string()))?;

    let site_id = query
        .site_id
        .map(|s| Uuid::parse_str(&s))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?
        .map(crate::common::types::SiteId);

    let reservations = service
        .list_reservations(user_id, resource_type, resource_id, site_id, &ctx)
        .await?;
    let response: Vec<ReservationResponse> = reservations
        .into_iter()
        .map(ReservationResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn list_my_reservations(
    State(state): State<AppState>,
    ctx: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let reservations = service.list_my_reservations(&ctx).await?;
    let response: Vec<ReservationResponse> = reservations
        .into_iter()
        .map(ReservationResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn create_reservation(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<CreateReservationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let resource_type = request
        .resource_type
        .parse::<ResourceType>()
        .map_err(|e: String| AppError::Validation(e))?;

    let resource_id = Uuid::parse_str(&request.resource_id)
        .map_err(|_| AppError::Validation("Invalid resource ID".to_string()))?;

    let site_id = request
        .site_id
        .as_deref()
        .map(Uuid::parse_str)
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?
        .map(crate::common::types::SiteId);

    let project_id = request
        .project_id
        .map(|s| Uuid::parse_str(&s))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid project ID".to_string()))?
        .map(crate::common::types::SiteId);

    let start_time = DateTime::parse_from_rfc3339(&request.start_time)
        .map_err(|_| AppError::Validation("Invalid start time format".to_string()))?
        .with_timezone(&Utc);

    let end_time = DateTime::parse_from_rfc3339(&request.end_time)
        .map_err(|_| AppError::Validation("Invalid end time format".to_string()))?
        .with_timezone(&Utc);

    let create = CreateReservation {
        resource_type,
        resource_id,
        site_id,
        project_id: project_id.or(site_id),
        start_time,
        end_time,
        purpose: request.purpose,
        notes: request.notes,
    };

    let reservation = service.create_reservation(create, &ctx).await?;

    // Get the full details for the response
    let details = service.get_reservation(reservation.id, &ctx).await?;

    Ok((
        StatusCode::CREATED,
        Json(ReservationResponse::from(details)),
    ))
}

pub async fn get_reservation(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let reservation_id = Uuid::parse_str(&id)
        .map(ReservationId)
        .map_err(|_| AppError::Validation("Invalid reservation ID".to_string()))?;

    let reservation = service.get_reservation(reservation_id, &ctx).await?;

    Ok(Json(ReservationResponse::from(reservation)))
}

pub async fn update_reservation(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateReservationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let reservation_id = Uuid::parse_str(&id)
        .map(ReservationId)
        .map_err(|_| AppError::Validation("Invalid reservation ID".to_string()))?;

    let start_time = request
        .start_time
        .map(|s| DateTime::parse_from_rfc3339(&s))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid start time format".to_string()))?
        .map(|dt| dt.with_timezone(&Utc));

    let end_time = request
        .end_time
        .map(|s| DateTime::parse_from_rfc3339(&s))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid end time format".to_string()))?
        .map(|dt| dt.with_timezone(&Utc));

    let site_id = request
        .site_id
        .as_deref()
        .map(Uuid::parse_str)
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?
        .map(crate::common::types::SiteId);

    let project_id = request
        .project_id
        .map(|s| Uuid::parse_str(&s))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid project ID".to_string()))?
        .map(crate::common::types::SiteId);

    let status = request
        .status
        .map(|s| s.parse::<ReservationStatus>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let update = UpdateReservation {
        start_time,
        end_time,
        site_id,
        project_id,
        purpose: request.purpose,
        notes: request.notes,
        status,
    };

    let reservation = service
        .update_reservation(reservation_id, update, &ctx)
        .await?;

    // Get the full details for the response
    let details = service.get_reservation(reservation.id, &ctx).await?;

    Ok(Json(ReservationResponse::from(details)))
}

pub async fn cancel_reservation(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let reservation_id = Uuid::parse_str(&id)
        .map(ReservationId)
        .map_err(|_| AppError::Validation("Invalid reservation ID".to_string()))?;

    service.cancel_reservation(reservation_id, &ctx).await?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "success": true }))))
}

// === Calendar Handler ===

pub async fn get_calendar(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<CalendarQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let start_date = parse_date_time(&query.start_date, 0, 0, 0).map_err(|_| {
        AppError::Validation("Invalid start_date format. Use YYYY-MM-DD or RFC3339".to_string())
    })?;

    let end_date = parse_date_time(&query.end_date, 23, 59, 59).map_err(|_| {
        AppError::Validation("Invalid end_date format. Use YYYY-MM-DD or RFC3339".to_string())
    })?;

    let resource_type = query
        .resource_type
        .map(|s| s.parse::<ResourceType>())
        .transpose()
        .map_err(|e: String| AppError::Validation(e))?;

    let site_id = query
        .site_id
        .map(|s| Uuid::parse_str(&s))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?
        .map(crate::common::types::SiteId);

    let entries = service
        .get_calendar(start_date, end_date, resource_type, site_id, &ctx)
        .await?;
    let response = CalendarResponse {
        resources: entries
            .into_iter()
            .map(CalendarEntryResponse::from)
            .collect(),
    };

    Ok(Json(response))
}

fn parse_date_time(
    s: &str,
    default_hour: u32,
    default_minute: u32,
    default_second: u32,
) -> Result<DateTime<Utc>, ()> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| {
                d.and_hms_opt(default_hour, default_minute, default_second)
                    .unwrap()
                    .and_utc()
            })
        })
        .map_err(|_| ())
}

// === Availability Handler ===

pub async fn check_availability(
    State(state): State<AppState>,
    ctx: TenantContext,
    Query(query): Query<AvailabilityQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let resource_type = query
        .resource_type
        .parse::<ResourceType>()
        .map_err(|e: String| AppError::Validation(e))?;

    let resource_id = Uuid::parse_str(&query.resource_id)
        .map_err(|_| AppError::Validation("Invalid resource ID".to_string()))?;

    let start_time = DateTime::parse_from_rfc3339(&query.start_time)
        .map_err(|_| AppError::Validation("Invalid start_time format".to_string()))?
        .with_timezone(&Utc);

    let end_time = DateTime::parse_from_rfc3339(&query.end_time)
        .map_err(|_| AppError::Validation("Invalid end_time format".to_string()))?
        .with_timezone(&Utc);

    let info = service
        .check_availability_with_conflicts(resource_type, resource_id, start_time, end_time, &ctx)
        .await?;

    let conflicts = if info.conflicts.is_empty() {
        None
    } else {
        Some(
            info.conflicts
                .into_iter()
                .map(|c| ConflictDetail {
                    id: c.id.to_string(),
                    user_name: c.user_name,
                    start_time: c.start_time.to_rfc3339(),
                    end_time: c.end_time.to_rfc3339(),
                    status: c.status.to_string(),
                })
                .collect(),
        )
    };

    Ok(Json(AvailabilityResponse {
        available: info.available,
        conflicts,
    }))
}

// === QR Code Handler ===

pub async fn get_status_by_qr(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = FleetService::new(FleetRepository::new(state.pool));

    let status_info = service.get_status_by_qr(&code, &ctx).await?;

    Ok(Json(QrStatusResponse::from(status_info)))
}
