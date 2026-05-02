use crate::common::error::AppError;
use crate::common::types::{
    ReservationId, ReservationStatus, ResourceType, Role, ToolId, UserId, VehicleId,
};
use crate::modules::fleet::domain::{
    CreateReservation, CreateTool, CreateVehicle, Reservation, ReservationCancelledPayload,
    ReservationCreatedPayload, ReservationUpdatedPayload, ReservationWithDetails,
    ResourceStatusChangedPayload, Tool, ToolCreatedPayload, UpdateReservation, UpdateTool,
    UpdateVehicle, Vehicle, VehicleCreatedPayload,
};
use crate::modules::fleet::infrastructure::fleet_repository::{
    CalendarEntry, FleetRepository, ResourceStatusInfo,
};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use sqlx::PgPool;

/// Service for fleet business logic
pub struct FleetService {
    fleet_repo: FleetRepository,
    pool: PgPool,
}

impl FleetService {
    pub fn new(fleet_repo: FleetRepository) -> Self {
        let pool = fleet_repo.pool();
        Self { fleet_repo, pool }
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

        let vehicle = self
            .fleet_repo
            .create_vehicle(&create, ctx.tenant_id)
            .await?;

        // Emit VehicleCreated event
        let event = VehicleCreatedPayload {
            vehicle_id: vehicle.id,
            name: vehicle.name.clone(),
            vehicle_type: vehicle.vehicle_type.to_string(),
        }
        .into_event(ctx.tenant_id);

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

        let old_vehicle = self
            .fleet_repo
            .find_vehicle_by_id(ctx.tenant_id, vehicle_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Vehicle not found".to_string()))?;

        let vehicle = self
            .fleet_repo
            .update_vehicle(ctx.tenant_id, vehicle_id, &update)
            .await?;

        // Emit ResourceStatusChanged event if status changed
        if let Some(new_status) = &update.status {
            if old_vehicle.status != *new_status {
                let event = ResourceStatusChangedPayload {
                    resource_type: ResourceType::Vehicle,
                    resource_id: vehicle.id.to_string(),
                    old_status: old_vehicle.status.to_string(),
                    new_status: new_status.to_string(),
                }
                .into_event(ctx.tenant_id);

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

        // Check for active reservations
        let active_count = self
            .fleet_repo
            .count_active_reservations(ctx.tenant_id, ResourceType::Vehicle, vehicle_id.0)
            .await?;

        if active_count > 0 {
            return Err(AppError::Conflict(format!(
                "Cannot delete: {} active reservation(s) exist",
                active_count
            )));
        }

        self.fleet_repo
            .delete_vehicle(ctx.tenant_id, vehicle_id)
            .await
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
        }
        .into_event(ctx.tenant_id);

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

        let old_tool = self
            .fleet_repo
            .find_tool_by_id(ctx.tenant_id, tool_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tool not found".to_string()))?;

        let tool = self
            .fleet_repo
            .update_tool(ctx.tenant_id, tool_id, &update)
            .await?;

        // Emit ResourceStatusChanged event if status changed
        if let Some(new_status) = &update.status {
            if old_tool.status != *new_status {
                let event = ResourceStatusChangedPayload {
                    resource_type: ResourceType::Tool,
                    resource_id: tool.id.to_string(),
                    old_status: old_tool.status.to_string(),
                    new_status: new_status.to_string(),
                }
                .into_event(ctx.tenant_id);

                self.fleet_repo.publish_event(&event).await?;
            }
        }

        Ok(tool)
    }

    pub async fn get_tool(&self, tool_id: ToolId, ctx: &TenantContext) -> Result<Tool, AppError> {
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
        self.fleet_repo
            .list_tools(ctx.tenant_id, status, category)
            .await
    }

    pub async fn delete_tool(&self, tool_id: ToolId, ctx: &TenantContext) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // Check for active reservations
        let active_count = self
            .fleet_repo
            .count_active_reservations(ctx.tenant_id, ResourceType::Tool, tool_id.0)
            .await?;

        if active_count > 0 {
            return Err(AppError::Conflict(format!(
                "Cannot delete: {} active reservation(s) exist",
                active_count
            )));
        }

        self.fleet_repo.delete_tool(ctx.tenant_id, tool_id).await
    }

    // === Reservation operations ===

    /// Create a new reservation (any authenticated user can create)
    pub async fn create_reservation(
        &self,
        create: CreateReservation,
        ctx: &TenantContext,
    ) -> Result<Reservation, AppError> {
        // Validate the reservation
        create.validate()?;

        // Check that the resource exists
        let resource_exists = match create.resource_type {
            ResourceType::Vehicle => self
                .fleet_repo
                .find_vehicle_by_id(ctx.tenant_id, VehicleId(create.resource_id))
                .await?
                .is_some(),
            ResourceType::Tool => self
                .fleet_repo
                .find_tool_by_id(ctx.tenant_id, ToolId(create.resource_id))
                .await?
                .is_some(),
        };

        if !resource_exists {
            return Err(AppError::NotFound(format!(
                "{} not found",
                create.resource_type
            )));
        }

        // Check availability
        let is_available = self
            .fleet_repo
            .check_availability(
                ctx.tenant_id,
                create.resource_type,
                create.resource_id,
                create.start_time,
                create.end_time,
                None,
            )
            .await?;

        if !is_available {
            return Err(AppError::Validation(
                "Resource is not available for the requested time period".to_string(),
            ));
        }

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        // Create the reservation with Confirmed status (skip Pending for V1)
        let reservation = self
            .fleet_repo
            .create_reservation(&create, ctx.tenant_id, local_user_id)
            .await?;

        // Emit ReservationCreated event
        let event = ReservationCreatedPayload {
            reservation_id: reservation.id,
            resource_type: reservation.resource_type,
            resource_id: reservation.resource_id,
            user_id: ctx.user_id.to_string(),
            site_id: reservation.site_id.map(|s| s.to_string()),
            start_time: reservation.start_time.to_rfc3339(),
            end_time: reservation.end_time.to_rfc3339(),
        }
        .into_event(ctx.tenant_id);

        self.fleet_repo.publish_event(&event).await?;

        Ok(reservation)
    }

    /// Update a reservation (owner or admin only)
    pub async fn update_reservation(
        &self,
        reservation_id: ReservationId,
        update: UpdateReservation,
        ctx: &TenantContext,
    ) -> Result<Reservation, AppError> {
        // Get existing reservation
        let current = self
            .fleet_repo
            .find_reservation_by_id(ctx.tenant_id, reservation_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Reservation not found".to_string()))?;

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        // Check ownership or admin role
        if current.user_id != local_user_id && !ctx.is_admin() {
            return Err(AppError::Forbidden(
                "You can only update your own reservations".to_string(),
            ));
        }

        // If time range changed, check availability
        if update.start_time.is_some() || update.end_time.is_some() {
            let new_start = update.start_time.as_ref().unwrap_or(&current.start_time);
            let new_end = update.end_time.as_ref().unwrap_or(&current.end_time);

            let is_available = self
                .fleet_repo
                .check_availability(
                    ctx.tenant_id,
                    current.resource_type,
                    current.resource_id,
                    *new_start,
                    *new_end,
                    Some(reservation_id), // Exclude current reservation
                )
                .await?;

            if !is_available {
                return Err(AppError::Validation(
                    "Resource is not available for the requested time period".to_string(),
                ));
            }
        }

        // Track what changed
        let mut changes = Vec::new();
        if update.start_time.is_some() {
            changes.push("start_time".to_string());
        }
        if update.end_time.is_some() {
            changes.push("end_time".to_string());
        }
        if update.site_id.is_some() {
            changes.push("site_id".to_string());
        }
        if update.notes.is_some() {
            changes.push("notes".to_string());
        }
        if update.status.is_some() {
            changes.push("status".to_string());
        }

        let reservation = self
            .fleet_repo
            .update_reservation(ctx.tenant_id, reservation_id, &update)
            .await?;

        // Emit ReservationUpdated event
        if !changes.is_empty() {
            let event = ReservationUpdatedPayload {
                reservation_id: reservation.id,
                changes,
            }
            .into_event(ctx.tenant_id);

            self.fleet_repo.publish_event(&event).await?;
        }

        Ok(reservation)
    }

    /// Cancel a reservation (owner or admin only)
    pub async fn cancel_reservation(
        &self,
        reservation_id: ReservationId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        // Get existing reservation
        let current = self
            .fleet_repo
            .find_reservation_by_id(ctx.tenant_id, reservation_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Reservation not found".to_string()))?;

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        // Check ownership or admin role
        if current.user_id != local_user_id && !ctx.is_admin() {
            return Err(AppError::Forbidden(
                "You can only cancel your own reservations".to_string(),
            ));
        }

        // Update status to Cancelled
        let update = UpdateReservation {
            start_time: None,
            end_time: None,
            site_id: None,
            notes: None,
            status: Some(ReservationStatus::Cancelled),
        };

        self.fleet_repo
            .update_reservation(ctx.tenant_id, reservation_id, &update)
            .await?;

        // Emit ReservationCancelled event
        let event = ReservationCancelledPayload {
            reservation_id: current.id,
            resource_type: current.resource_type,
            resource_id: current.resource_id,
        }
        .into_event(ctx.tenant_id);

        self.fleet_repo.publish_event(&event).await?;

        Ok(())
    }

    /// Get a single reservation with details
    pub async fn get_reservation(
        &self,
        reservation_id: ReservationId,
        ctx: &TenantContext,
    ) -> Result<ReservationWithDetails, AppError> {
        self.fleet_repo
            .get_reservation_with_details(ctx.tenant_id, reservation_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Reservation not found".to_string()))
    }

    /// List reservations with optional filters
    pub async fn list_reservations(
        &self,
        user_id: Option<UserId>,
        resource_type: Option<ResourceType>,
        resource_id: Option<uuid::Uuid>,
        ctx: &TenantContext,
    ) -> Result<Vec<ReservationWithDetails>, AppError> {
        // For non-admin users, only show their own reservations
        let filter_user_id = if !ctx.is_admin() {
            Some(self.resolve_local_user_id(ctx).await?)
        } else {
            user_id
        };

        let reservations = self
            .fleet_repo
            .list_reservations(ctx.tenant_id, filter_user_id, resource_type, resource_id)
            .await?;

        // Get details for each reservation
        let mut results = Vec::new();
        for reservation in reservations {
            if let Some(details) = self
                .fleet_repo
                .get_reservation_with_details(ctx.tenant_id, reservation.id)
                .await?
            {
                results.push(details);
            }
        }

        Ok(results)
    }

    /// List current user's reservations
    pub async fn list_my_reservations(
        &self,
        ctx: &TenantContext,
    ) -> Result<Vec<ReservationWithDetails>, AppError> {
        let local_user_id = self.resolve_local_user_id(ctx).await?;
        self.list_reservations(Some(local_user_id), None, None, ctx)
            .await
    }

    /// Get calendar data for a date range
    pub async fn get_calendar(
        &self,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        resource_type: Option<ResourceType>,
        ctx: &TenantContext,
    ) -> Result<Vec<CalendarEntry>, AppError> {
        // Validate date range
        if end_date <= start_date {
            return Err(AppError::Validation(
                "End date must be after start date".to_string(),
            ));
        }

        self.fleet_repo
            .get_calendar_data(ctx.tenant_id, start_date, end_date, resource_type)
            .await
    }

    /// Check resource availability
    pub async fn check_availability(
        &self,
        resource_type: ResourceType,
        resource_id: uuid::Uuid,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
        ctx: &TenantContext,
    ) -> Result<bool, AppError> {
        // Validate time range
        if end_time <= start_time {
            return Err(AppError::Validation(
                "End time must be after start time".to_string(),
            ));
        }

        self.fleet_repo
            .check_availability(
                ctx.tenant_id,
                resource_type,
                resource_id,
                start_time,
                end_time,
                None,
            )
            .await
    }

    /// Check resource availability with conflict details
    pub async fn check_availability_with_conflicts(
        &self,
        resource_type: ResourceType,
        resource_id: uuid::Uuid,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
        ctx: &TenantContext,
    ) -> Result<crate::modules::fleet::infrastructure::fleet_repository::AvailabilityInfo, AppError>
    {
        // Validate time range
        if end_time <= start_time {
            return Err(AppError::Validation(
                "End time must be after start time".to_string(),
            ));
        }

        let conflicts = self
            .fleet_repo
            .find_conflicts(
                ctx.tenant_id,
                resource_type,
                resource_id,
                start_time,
                end_time,
            )
            .await?;

        Ok(
            crate::modules::fleet::infrastructure::fleet_repository::AvailabilityInfo {
                available: conflicts.is_empty(),
                conflicts,
            },
        )
    }

    /// Get resource status by QR code
    pub async fn get_status_by_qr(
        &self,
        qr_code: &str,
        ctx: &TenantContext,
    ) -> Result<ResourceStatusInfo, AppError> {
        self.fleet_repo
            .get_resource_status_by_qr(ctx.tenant_id, qr_code)
            .await?
            .ok_or_else(|| AppError::NotFound("Resource not found for this QR code".to_string()))
    }
}
