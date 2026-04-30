use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::events::{EventBus, DomainEvent};
use crate::common::types::{
    TenantId, VehicleId, ToolId, VehicleType, ResourceStatus,
    ReservationId, ReservationStatus, ResourceType, UserId, SiteId,
};
use crate::modules::fleet::domain::{
    Vehicle, Tool, CreateVehicle, UpdateVehicle, CreateTool, UpdateTool,
    Reservation, CreateReservation, UpdateReservation, ReservationWithDetails,
};

/// Repository for fleet data access with tenant isolation
pub struct FleetRepository {
    pool: PgPool,
    event_bus: EventBus,
}

impl FleetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            event_bus: EventBus::new(),
        }
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    // === Vehicle operations ===

    pub async fn create_vehicle(
        &self,
        create: &CreateVehicle,
        tenant_id: TenantId,
    ) -> Result<Vehicle, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let vehicle = sqlx::query_as::<_, VehicleRow>(
            r#"
            INSERT INTO vehicles (id, tenant_id, name, license_plate, vehicle_type, description, status, location, qr_code, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, tenant_id, name, license_plate, vehicle_type, description, status, location, qr_code, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create.name)
        .bind(&create.license_plate)
        .bind(create.vehicle_type.as_str())
        .bind(&create.description)
        .bind(ResourceStatus::Available.as_str())
        .bind(&create.location)
        .bind(&create.qr_code)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                AppError::Validation("Vehicle with this name already exists".to_string())
            } else {
                AppError::Database(e.to_string())
            }
        })?;

        Ok(vehicle.into_vehicle())
    }

    pub async fn find_vehicle_by_id(
        &self,
        tenant_id: TenantId,
        id: VehicleId,
    ) -> Result<Option<Vehicle>, AppError> {
        let vehicle = sqlx::query_as::<_, VehicleRow>(
            r#"
            SELECT id, tenant_id, name, license_plate, vehicle_type, description, status, location, qr_code, created_at, updated_at
            FROM vehicles
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(vehicle.map(|v| v.into_vehicle()))
    }

    pub async fn list_vehicles(
        &self,
        tenant_id: TenantId,
        status: Option<String>,
    ) -> Result<Vec<Vehicle>, AppError> {
        let vehicles = match status {
            Some(s) => {
                sqlx::query_as::<_, VehicleRow>(
                    r#"
                    SELECT id, tenant_id, name, license_plate, vehicle_type, description, status, location, qr_code, created_at, updated_at
                    FROM vehicles
                    WHERE tenant_id = $1 AND status = $2
                    ORDER BY name
                    "#
                )
                .bind(tenant_id.0)
                .bind(&s)
                .fetch_all(&self.pool)
                .await
            }
            None => {
                sqlx::query_as::<_, VehicleRow>(
                    r#"
                    SELECT id, tenant_id, name, license_plate, vehicle_type, description, status, location, qr_code, created_at, updated_at
                    FROM vehicles
                    WHERE tenant_id = $1
                    ORDER BY name
                    "#
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(vehicles.into_iter().map(|v| v.into_vehicle()).collect())
    }

    pub async fn update_vehicle(
        &self,
        tenant_id: TenantId,
        id: VehicleId,
        update: &UpdateVehicle,
    ) -> Result<Vehicle, AppError> {
        // First get the current vehicle
        let current = self.find_vehicle_by_id(tenant_id, id).await?
            .ok_or_else(|| AppError::NotFound("Vehicle not found".to_string()))?;

        // Validate status transition if status is being changed
        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(
                    format!("Invalid status transition from {} to {}", current.status, new_status)
                ));
            }
        }

        let vehicle = sqlx::query_as::<_, VehicleRow>(
            r#"
            UPDATE vehicles
            SET 
                name = COALESCE($1, name),
                license_plate = COALESCE($2, license_plate),
                vehicle_type = COALESCE($3, vehicle_type),
                description = COALESCE($4, description),
                status = COALESCE($5, status),
                location = COALESCE($6, location),
                qr_code = COALESCE($7, qr_code),
                updated_at = NOW()
            WHERE id = $8 AND tenant_id = $9
            RETURNING id, tenant_id, name, license_plate, vehicle_type, description, status, location, qr_code, created_at, updated_at
            "#
        )
        .bind(&update.name)
        .bind(&update.license_plate)
        .bind(update.vehicle_type.as_ref().map(|v| v.as_str()))
        .bind(&update.description)
        .bind(update.status.as_ref().map(|s| s.as_str()))
        .bind(&update.location)
        .bind(&update.qr_code)
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Vehicle not found".to_string()))?;

        Ok(vehicle.into_vehicle())
    }

    pub async fn delete_vehicle(
        &self,
        tenant_id: TenantId,
        id: VehicleId,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM vehicles
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Vehicle not found".to_string()));
        }

        Ok(())
    }

    pub async fn find_vehicle_by_qr_code(
        &self,
        tenant_id: TenantId,
        qr_code: &str,
    ) -> Result<Option<Vehicle>, AppError> {
        let vehicle = sqlx::query_as::<_, VehicleRow>(
            r#"
            SELECT id, tenant_id, name, license_plate, vehicle_type, description, status, location, qr_code, created_at, updated_at
            FROM vehicles
            WHERE qr_code = $1 AND tenant_id = $2
            "#
        )
        .bind(qr_code)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(vehicle.map(|v| v.into_vehicle()))
    }

    // === Tool operations ===

    pub async fn create_tool(
        &self,
        create: &CreateTool,
        tenant_id: TenantId,
    ) -> Result<Tool, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let tool = sqlx::query_as::<_, ToolRow>(
            r#"
            INSERT INTO tools (id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create.name)
        .bind(&create.category)
        .bind(&create.description)
        .bind(ResourceStatus::Available.as_str())
        .bind(&create.location)
        .bind(&create.qr_code)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                AppError::Validation("Tool with this name already exists".to_string())
            } else {
                AppError::Database(e.to_string())
            }
        })?;

        Ok(tool.into_tool())
    }

    pub async fn find_tool_by_id(
        &self,
        tenant_id: TenantId,
        id: ToolId,
    ) -> Result<Option<Tool>, AppError> {
        let tool = sqlx::query_as::<_, ToolRow>(
            r#"
            SELECT id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at
            FROM tools
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(tool.map(|t| t.into_tool()))
    }

    pub async fn list_tools(
        &self,
        tenant_id: TenantId,
        status: Option<String>,
        category: Option<String>,
    ) -> Result<Vec<Tool>, AppError> {
        let tools = match (&status, &category) {
            (Some(s), Some(c)) => {
                sqlx::query_as::<_, ToolRow>(
                    r#"
                    SELECT id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at
                    FROM tools
                    WHERE tenant_id = $1 AND status = $2 AND category = $3
                    ORDER BY name
                    "#
                )
                .bind(tenant_id.0)
                .bind(s)
                .bind(c)
                .fetch_all(&self.pool)
                .await
            }
            (Some(s), None) => {
                sqlx::query_as::<_, ToolRow>(
                    r#"
                    SELECT id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at
                    FROM tools
                    WHERE tenant_id = $1 AND status = $2
                    ORDER BY name
                    "#
                )
                .bind(tenant_id.0)
                .bind(s)
                .fetch_all(&self.pool)
                .await
            }
            (None, Some(c)) => {
                sqlx::query_as::<_, ToolRow>(
                    r#"
                    SELECT id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at
                    FROM tools
                    WHERE tenant_id = $1 AND category = $2
                    ORDER BY name
                    "#
                )
                .bind(tenant_id.0)
                .bind(c)
                .fetch_all(&self.pool)
                .await
            }
            (None, None) => {
                sqlx::query_as::<_, ToolRow>(
                    r#"
                    SELECT id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at
                    FROM tools
                    WHERE tenant_id = $1
                    ORDER BY name
                    "#
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(tools.into_iter().map(|t| t.into_tool()).collect())
    }

    pub async fn update_tool(
        &self,
        tenant_id: TenantId,
        id: ToolId,
        update: &UpdateTool,
    ) -> Result<Tool, AppError> {
        // First get the current tool
        let current = self.find_tool_by_id(tenant_id, id).await?
            .ok_or_else(|| AppError::NotFound("Tool not found".to_string()))?;

        // Validate status transition if status is being changed
        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(
                    format!("Invalid status transition from {} to {}", current.status, new_status)
                ));
            }
        }

        let tool = sqlx::query_as::<_, ToolRow>(
            r#"
            UPDATE tools
            SET 
                name = COALESCE($1, name),
                category = COALESCE($2, category),
                description = COALESCE($3, description),
                status = COALESCE($4, status),
                location = COALESCE($5, location),
                qr_code = COALESCE($6, qr_code),
                updated_at = NOW()
            WHERE id = $7 AND tenant_id = $8
            RETURNING id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at
            "#
        )
        .bind(&update.name)
        .bind(&update.category)
        .bind(&update.description)
        .bind(update.status.as_ref().map(|s| s.as_str()))
        .bind(&update.location)
        .bind(&update.qr_code)
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Tool not found".to_string()))?;

        Ok(tool.into_tool())
    }

    pub async fn delete_tool(
        &self,
        tenant_id: TenantId,
        id: ToolId,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM tools
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Tool not found".to_string()));
        }

        Ok(())
    }

    pub async fn find_tool_by_qr_code(
        &self,
        tenant_id: TenantId,
        qr_code: &str,
    ) -> Result<Option<Tool>, AppError> {
        let tool = sqlx::query_as::<_, ToolRow>(
            r#"
            SELECT id, tenant_id, name, category, description, status, location, qr_code, created_at, updated_at
            FROM tools
            WHERE qr_code = $1 AND tenant_id = $2
            "#
        )
        .bind(qr_code)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(tool.map(|t| t.into_tool()))
    }

    // === Reservation operations ===

    pub async fn create_reservation(
        &self,
        create: &CreateReservation,
        tenant_id: TenantId,
        user_id: UserId,
    ) -> Result<Reservation, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let reservation = sqlx::query_as::<_, ReservationRow>(
            r#"
            INSERT INTO reservations (id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(create.resource_type.as_str())
        .bind(create.resource_id)
        .bind(user_id.0)
        .bind(create.site_id.map(|s| s.0))
        .bind(create.start_time)
        .bind(create.end_time)
        .bind(ReservationStatus::Confirmed.as_str())
        .bind(&create.notes)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(reservation.into_reservation())
    }

    pub async fn find_reservation_by_id(
        &self,
        tenant_id: TenantId,
        id: ReservationId,
    ) -> Result<Option<Reservation>, AppError> {
        let reservation = sqlx::query_as::<_, ReservationRow>(
            r#"
            SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
            FROM reservations
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(reservation.map(|r| r.into_reservation()))
    }

    pub async fn list_reservations(
        &self,
        tenant_id: TenantId,
        user_id: Option<UserId>,
        resource_type: Option<ResourceType>,
        resource_id: Option<Uuid>,
    ) -> Result<Vec<Reservation>, AppError> {
        let reservations = match (&user_id, &resource_type, &resource_id) {
            (Some(uid), Some(rt), Some(rid)) => {
                sqlx::query_as::<_, ReservationRow>(
                    r#"
                    SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
                    FROM reservations
                    WHERE tenant_id = $1 AND user_id = $2 AND resource_type = $3 AND resource_id = $4
                    ORDER BY start_time DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(uid.0)
                .bind(rt.as_str())
                .bind(*rid)
                .fetch_all(&self.pool)
                .await
            }
            (Some(uid), Some(rt), None) => {
                sqlx::query_as::<_, ReservationRow>(
                    r#"
                    SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
                    FROM reservations
                    WHERE tenant_id = $1 AND user_id = $2 AND resource_type = $3
                    ORDER BY start_time DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(uid.0)
                .bind(rt.as_str())
                .fetch_all(&self.pool)
                .await
            }
            (Some(uid), None, None) => {
                sqlx::query_as::<_, ReservationRow>(
                    r#"
                    SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
                    FROM reservations
                    WHERE tenant_id = $1 AND user_id = $2
                    ORDER BY start_time DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(uid.0)
                .fetch_all(&self.pool)
                .await
            }
            (None, Some(rt), Some(rid)) => {
                sqlx::query_as::<_, ReservationRow>(
                    r#"
                    SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
                    FROM reservations
                    WHERE tenant_id = $1 AND resource_type = $2 AND resource_id = $3
                    ORDER BY start_time DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(rt.as_str())
                .bind(*rid)
                .fetch_all(&self.pool)
                .await
            }
            (None, Some(rt), None) => {
                sqlx::query_as::<_, ReservationRow>(
                    r#"
                    SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
                    FROM reservations
                    WHERE tenant_id = $1 AND resource_type = $2
                    ORDER BY start_time DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(rt.as_str())
                .fetch_all(&self.pool)
                .await
            }
            (None, None, Some(rid)) => {
                sqlx::query_as::<_, ReservationRow>(
                    r#"
                    SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
                    FROM reservations
                    WHERE tenant_id = $1 AND resource_id = $2
                    ORDER BY start_time DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(*rid)
                .fetch_all(&self.pool)
                .await
            }
            (None, None, None) => {
                sqlx::query_as::<_, ReservationRow>(
                    r#"
                    SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
                    FROM reservations
                    WHERE tenant_id = $1
                    ORDER BY start_time DESC
                    "#
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
            // Other combinations (uid, None, rid) - not matched above
            _ => {
                sqlx::query_as::<_, ReservationRow>(
                    r#"
                    SELECT id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
                    FROM reservations
                    WHERE tenant_id = $1
                    ORDER BY start_time DESC
                    "#
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(reservations.into_iter().map(|r| r.into_reservation()).collect())
    }

    pub async fn update_reservation(
        &self,
        tenant_id: TenantId,
        id: ReservationId,
        update: &UpdateReservation,
    ) -> Result<Reservation, AppError> {
        // First get the current reservation
        let current = self.find_reservation_by_id(tenant_id, id).await?
            .ok_or_else(|| AppError::NotFound("Reservation not found".to_string()))?;

        // Validate status transition if status is being changed
        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(
                    format!("Invalid status transition from {} to {}", current.status, new_status)
                ));
            }
        }

        // Validate time range if times are being changed
        update.validate(&current)?;

        let reservation = sqlx::query_as::<_, ReservationRow>(
            r#"
            UPDATE reservations
            SET 
                start_time = COALESCE($1, start_time),
                end_time = COALESCE($2, end_time),
                site_id = COALESCE($3, site_id),
                notes = COALESCE($4, notes),
                status = COALESCE($5, status),
                updated_at = NOW()
            WHERE id = $6 AND tenant_id = $7
            RETURNING id, tenant_id, resource_type, resource_id, user_id, site_id, start_time, end_time, status, notes, created_at, updated_at
            "#
        )
        .bind(&update.start_time)
        .bind(&update.end_time)
        .bind(update.site_id.map(|s| s.0))
        .bind(&update.notes)
        .bind(update.status.as_ref().map(|s| s.as_str()))
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Reservation not found".to_string()))?;

        Ok(reservation.into_reservation())
    }

    pub async fn delete_reservation(
        &self,
        tenant_id: TenantId,
        id: ReservationId,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM reservations
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Reservation not found".to_string()));
        }

        Ok(())
    }

    /// Check if a resource is available for the given time range
    /// Returns true if available (no overlapping reservations)
    pub async fn check_availability(
        &self,
        tenant_id: TenantId,
        resource_type: ResourceType,
        resource_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        exclude_reservation_id: Option<ReservationId>,
    ) -> Result<bool, AppError> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM reservations
            WHERE tenant_id = $1 
              AND resource_type = $2 
              AND resource_id = $3 
              AND status != 'cancelled'
              AND (start_time, end_time) OVERLAPS ($4, $5)
              AND ($6::uuid IS NULL OR id != $6)
            "#
        )
        .bind(tenant_id.0)
        .bind(resource_type.as_str())
        .bind(resource_id)
        .bind(start_time)
        .bind(end_time)
        .bind(exclude_reservation_id.map(|id| id.0))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(count == 0)
    }

    /// Get calendar data for a date range
    pub async fn get_calendar_data(
        &self,
        tenant_id: TenantId,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        resource_type: Option<ResourceType>,
    ) -> Result<Vec<CalendarEntry>, AppError> {
        // First get all resources with their reservations
        let rows = sqlx::query_as::<_, CalendarRow>(
            r#"
            WITH resources AS (
                SELECT 'vehicle'::text as resource_type, id, name 
                FROM vehicles 
                WHERE tenant_id = $1
                UNION ALL
                SELECT 'tool'::text as resource_type, id, name 
                FROM tools 
                WHERE tenant_id = $1
            )
            SELECT 
                r.resource_type,
                r.id as resource_id,
                r.name as resource_name,
                COALESCE(
                    json_agg(
                        json_build_object(
                            'id', res.id,
                            'start_time', res.start_time,
                            'end_time', res.end_time,
                            'user_name', u.name,
                            'site_name', s.name,
                            'status', res.status
                        )
                    ) FILTER (WHERE res.id IS NOT NULL), 
                    '[]'::json
                ) as reservations
            FROM resources r
            LEFT JOIN reservations res ON res.resource_id = r.id 
                AND res.tenant_id = $1 
                AND res.status != 'cancelled'
                AND (res.start_time, res.end_time) OVERLAPS ($2, $3)
            LEFT JOIN users u ON u.id = res.user_id
            LEFT JOIN sites s ON s.id = res.site_id
            WHERE ($4::text IS NULL OR r.resource_type = $4)
            GROUP BY r.resource_type, r.id, r.name
            ORDER BY r.resource_type, r.name
            "#
        )
        .bind(tenant_id.0)
        .bind(start_date)
        .bind(end_date)
        .bind(resource_type.as_ref().map(|rt| rt.as_str()))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into_calendar_entry()).collect())
    }

    /// Get resource status by QR code
    pub async fn get_resource_status_by_qr(
        &self,
        tenant_id: TenantId,
        qr_code: &str,
    ) -> Result<Option<ResourceStatusInfo>, AppError> {
        // Try to find vehicle first
        if let Some(vehicle) = self.find_vehicle_by_qr_code(tenant_id, qr_code).await? {
            return self.build_resource_status(tenant_id, ResourceType::Vehicle, vehicle.id.0, vehicle.name, vehicle.status).await;
        }

        // Try to find tool
        if let Some(tool) = self.find_tool_by_qr_code(tenant_id, qr_code).await? {
            return self.build_resource_status(tenant_id, ResourceType::Tool, tool.id.0, tool.name, tool.status).await;
        }

        Ok(None)
    }

    async fn build_resource_status(
        &self,
        tenant_id: TenantId,
        resource_type: ResourceType,
        resource_id: Uuid,
        resource_name: String,
        status: ResourceStatus,
    ) -> Result<Option<ResourceStatusInfo>, AppError> {
        let now = Utc::now();

        // Get current reservation (if any)
        let current: Option<ReservationSummary> = sqlx::query_as::<_, ReservationSummaryRow>(
            r#"
            SELECT res.id, res.start_time, res.end_time, u.name as user_name, s.name as site_name, res.status
            FROM reservations res
            LEFT JOIN users u ON u.id = res.user_id
            LEFT JOIN sites s ON s.id = res.site_id
            WHERE res.tenant_id = $1 
              AND res.resource_id = $2 
              AND res.status != 'cancelled'
              AND res.start_time <= $3 
              AND res.end_time >= $3
            LIMIT 1
            "#
        )
        .bind(tenant_id.0)
        .bind(resource_id)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .map(|r| r.into_summary());

        // Get upcoming reservations (next 3)
        let upcoming: Vec<ReservationSummary> = sqlx::query_as::<_, ReservationSummaryRow>(
            r#"
            SELECT res.id, res.start_time, res.end_time, u.name as user_name, s.name as site_name, res.status
            FROM reservations res
            LEFT JOIN users u ON u.id = res.user_id
            LEFT JOIN sites s ON s.id = res.site_id
            WHERE res.tenant_id = $1 
              AND res.resource_id = $2 
              AND res.status != 'cancelled'
              AND res.start_time > $3
            ORDER BY res.start_time
            LIMIT 3
            "#
        )
        .bind(tenant_id.0)
        .bind(resource_id)
        .bind(now)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .into_iter()
        .map(|r| r.into_summary())
        .collect();

        Ok(Some(ResourceStatusInfo {
            resource_type,
            resource_id,
            resource_name,
            status,
            current_reservation: current,
            upcoming_reservations: upcoming,
        }))
    }

    /// Get reservation with user and site details
    pub async fn get_reservation_with_details(
        &self,
        tenant_id: TenantId,
        id: ReservationId,
    ) -> Result<Option<ReservationWithDetails>, AppError> {
        let row = sqlx::query_as::<_, ReservationDetailsRow>(
            r#"
            SELECT 
                res.id, res.tenant_id, res.resource_type, res.resource_id, 
                res.user_id, res.site_id, res.start_time, res.end_time, 
                res.status, res.notes, res.created_at, res.updated_at,
                COALESCE(v.name, t.name) as resource_name,
                u.name as user_name,
                s.name as site_name
            FROM reservations res
            LEFT JOIN vehicles v ON v.id = res.resource_id AND res.resource_type = 'vehicle'
            LEFT JOIN tools t ON t.id = res.resource_id AND res.resource_type = 'tool'
            LEFT JOIN users u ON u.id = res.user_id
            LEFT JOIN sites s ON s.id = res.site_id
            WHERE res.id = $1 AND res.tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.map(|r| r.into_details()))
    }

    // === Event publishing ===

    pub async fn publish_event(&self, event: &DomainEvent) -> Result<(), AppError> {
        self.event_bus.publish(event, &self.pool).await
    }
}

// === Database row types ===

#[derive(Debug, FromRow)]
struct VehicleRow {
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    license_plate: Option<String>,
    vehicle_type: String,
    description: Option<String>,
    status: String,
    location: Option<String>,
    qr_code: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl VehicleRow {
    fn into_vehicle(self) -> Vehicle {
        Vehicle {
            id: VehicleId(self.id),
            tenant_id: TenantId(self.tenant_id),
            name: self.name,
            license_plate: self.license_plate,
            vehicle_type: self.vehicle_type.parse().unwrap_or(VehicleType::Other),
            description: self.description,
            status: self.status.parse().unwrap_or(ResourceStatus::Available),
            location: self.location,
            qr_code: self.qr_code,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct ToolRow {
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    category: Option<String>,
    description: Option<String>,
    status: String,
    location: Option<String>,
    qr_code: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ToolRow {
    fn into_tool(self) -> Tool {
        Tool {
            id: ToolId(self.id),
            tenant_id: TenantId(self.tenant_id),
            name: self.name,
            category: self.category,
            description: self.description,
            status: self.status.parse().unwrap_or(ResourceStatus::Available),
            location: self.location,
            qr_code: self.qr_code,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct ReservationRow {
    id: Uuid,
    tenant_id: Uuid,
    resource_type: String,
    resource_id: Uuid,
    user_id: Uuid,
    site_id: Option<Uuid>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    status: String,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ReservationRow {
    fn into_reservation(self) -> Reservation {
        Reservation {
            id: ReservationId(self.id),
            tenant_id: TenantId(self.tenant_id),
            resource_type: self.resource_type.parse().unwrap_or(ResourceType::Vehicle),
            resource_id: self.resource_id,
            user_id: UserId(self.user_id),
            site_id: self.site_id.map(SiteId),
            start_time: self.start_time,
            end_time: self.end_time,
            status: self.status.parse().unwrap_or(ReservationStatus::Pending),
            notes: self.notes,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// === Calendar types ===

/// Calendar entry for a resource with its reservations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEntry {
    pub resource_type: ResourceType,
    pub resource_id: Uuid,
    pub resource_name: String,
    pub reservations: Vec<ReservationSummary>,
}

/// Summary of a reservation for calendar display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationSummary {
    pub id: ReservationId,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub user_name: Option<String>,
    pub site_name: Option<String>,
    pub status: ReservationStatus,
}

#[derive(Debug, FromRow)]
struct CalendarRow {
    resource_type: String,
    resource_id: Uuid,
    resource_name: String,
    reservations: serde_json::Value,
}

impl CalendarRow {
    fn into_calendar_entry(self) -> CalendarEntry {
        let reservations: Vec<ReservationSummary> = 
            serde_json::from_value(self.reservations).unwrap_or_default();
        
        CalendarEntry {
            resource_type: self.resource_type.parse().unwrap_or(ResourceType::Vehicle),
            resource_id: self.resource_id,
            resource_name: self.resource_name,
            reservations,
        }
    }
}

#[derive(Debug, FromRow)]
struct ReservationSummaryRow {
    id: Uuid,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    user_name: Option<String>,
    site_name: Option<String>,
    status: String,
}

impl ReservationSummaryRow {
    fn into_summary(self) -> ReservationSummary {
        ReservationSummary {
            id: ReservationId(self.id),
            start_time: self.start_time,
            end_time: self.end_time,
            user_name: self.user_name,
            site_name: self.site_name,
            status: self.status.parse().unwrap_or(ReservationStatus::Pending),
        }
    }
}

// === Resource status types ===

/// Resource status information for QR code lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStatusInfo {
    pub resource_type: ResourceType,
    pub resource_id: Uuid,
    pub resource_name: String,
    pub status: ResourceStatus,
    pub current_reservation: Option<ReservationSummary>,
    pub upcoming_reservations: Vec<ReservationSummary>,
}

#[derive(Debug, FromRow)]
struct ReservationDetailsRow {
    id: Uuid,
    tenant_id: Uuid,
    resource_type: String,
    resource_id: Uuid,
    user_id: Uuid,
    site_id: Option<Uuid>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    status: String,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    resource_name: String,
    user_name: Option<String>,
    site_name: Option<String>,
}

impl ReservationDetailsRow {
    fn into_details(self) -> ReservationWithDetails {
        ReservationWithDetails {
            reservation: Reservation {
                id: ReservationId(self.id),
                tenant_id: TenantId(self.tenant_id),
                resource_type: self.resource_type.parse().unwrap_or(ResourceType::Vehicle),
                resource_id: self.resource_id,
                user_id: UserId(self.user_id),
                site_id: self.site_id.map(SiteId),
                start_time: self.start_time,
                end_time: self.end_time,
                status: self.status.parse().unwrap_or(ReservationStatus::Pending),
                notes: self.notes,
                created_at: self.created_at,
                updated_at: self.updated_at,
            },
            resource_name: self.resource_name,
            user_name: self.user_name,
            site_name: self.site_name,
        }
    }
}
