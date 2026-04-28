use crate::common::error::AppError;
use crate::common::types::{VehicleId, ToolId, ResourceStatus, ResourceType};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::fleet::domain::{
    Vehicle, Tool, CreateVehicle, UpdateVehicle, CreateTool, UpdateTool,
    VehicleCreatedPayload, ToolCreatedPayload, ResourceStatusChangedPayload,
};
use crate::modules::fleet::infrastructure::fleet_repository::FleetRepository;

/// Service for fleet business logic
pub struct FleetService {
    fleet_repo: FleetRepository,
}

impl FleetService {
    pub fn new(fleet_repo: FleetRepository) -> Self {
        Self { fleet_repo }
    }

    // === Vehicle operations ===

    pub async fn create_vehicle(
        &self,
        create: CreateVehicle,
        ctx: &TenantContext,
    ) -> Result<Vehicle, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        create.validate()?;

        let vehicle = self.fleet_repo.create_vehicle(&create, ctx.tenant_id).await?;

        // Emit VehicleCreated event
        let event = VehicleCreatedPayload {
            vehicle_id: vehicle.id,
            name: vehicle.name.clone(),
            vehicle_type: vehicle.vehicle_type.to_string(),
        }.into_event(ctx.tenant_id);

        self.fleet_repo.publish_event(&event).await?;

        Ok(vehicle)
    }

    pub async fn update_vehicle(
        &self,
        vehicle_id: VehicleId,
        update: UpdateVehicle,
        ctx: &TenantContext,
    ) -> Result<Vehicle, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        let old_vehicle = self.fleet_repo.find_vehicle_by_id(ctx.tenant_id, vehicle_id).await?
            .ok_or_else(|| AppError::NotFound("Vehicle not found".to_string()))?;

        let vehicle = self.fleet_repo.update_vehicle(ctx.tenant_id, vehicle_id, &update).await?;

        // Emit ResourceStatusChanged event if status changed
        if let Some(new_status) = &update.status {
            if old_vehicle.status != *new_status {
                let event = ResourceStatusChangedPayload {
                    resource_type: ResourceType::Vehicle,
                    resource_id: vehicle.id.to_string(),
                    old_status: old_vehicle.status.to_string(),
                    new_status: new_status.to_string(),
                }.into_event(ctx.tenant_id);

                self.fleet_repo.publish_event(&event).await?;
            }
        }

        Ok(vehicle)
    }

    pub async fn get_vehicle(
        &self,
        vehicle_id: VehicleId,
        ctx: &TenantContext,
    ) -> Result<Vehicle, AppError> {
        self.fleet_repo
            .find_vehicle_by_id(ctx.tenant_id, vehicle_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Vehicle not found".to_string()))
    }

    pub async fn list_vehicles(
        &self,
        status: Option<String>,
        ctx: &TenantContext,
    ) -> Result<Vec<Vehicle>, AppError> {
        self.fleet_repo.list_vehicles(ctx.tenant_id, status).await
    }

    pub async fn delete_vehicle(
        &self,
        vehicle_id: VehicleId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        self.fleet_repo.delete_vehicle(ctx.tenant_id, vehicle_id).await
    }

    // === Tool operations ===

    pub async fn create_tool(
        &self,
        create: CreateTool,
        ctx: &TenantContext,
    ) -> Result<Tool, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        create.validate()?;

        let tool = self.fleet_repo.create_tool(&create, ctx.tenant_id).await?;

        // Emit ToolCreated event
        let event = ToolCreatedPayload {
            tool_id: tool.id,
            name: tool.name.clone(),
            category: tool.category.clone(),
        }.into_event(ctx.tenant_id);

        self.fleet_repo.publish_event(&event).await?;

        Ok(tool)
    }

    pub async fn update_tool(
        &self,
        tool_id: ToolId,
        update: UpdateTool,
        ctx: &TenantContext,
    ) -> Result<Tool, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        let old_tool = self.fleet_repo.find_tool_by_id(ctx.tenant_id, tool_id).await?
            .ok_or_else(|| AppError::NotFound("Tool not found".to_string()))?;

        let tool = self.fleet_repo.update_tool(ctx.tenant_id, tool_id, &update).await?;

        // Emit ResourceStatusChanged event if status changed
        if let Some(new_status) = &update.status {
            if old_tool.status != *new_status {
                let event = ResourceStatusChangedPayload {
                    resource_type: ResourceType::Tool,
                    resource_id: tool.id.to_string(),
                    old_status: old_tool.status.to_string(),
                    new_status: new_status.to_string(),
                }.into_event(ctx.tenant_id);

                self.fleet_repo.publish_event(&event).await?;
            }
        }

        Ok(tool)
    }

    pub async fn get_tool(
        &self,
        tool_id: ToolId,
        ctx: &TenantContext,
    ) -> Result<Tool, AppError> {
        self.fleet_repo
            .find_tool_by_id(ctx.tenant_id, tool_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tool not found".to_string()))
    }

    pub async fn list_tools(
        &self,
        status: Option<String>,
        category: Option<String>,
        ctx: &TenantContext,
    ) -> Result<Vec<Tool>, AppError> {
        self.fleet_repo.list_tools(ctx.tenant_id, status, category).await
    }

    pub async fn delete_tool(
        &self,
        tool_id: ToolId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        self.fleet_repo.delete_tool(ctx.tenant_id, tool_id).await
    }
}
