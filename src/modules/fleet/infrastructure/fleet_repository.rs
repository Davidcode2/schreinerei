use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::events::{DomainEvent, EventBus};
use crate::common::types::{
    AssetId, AssetKind, MachineId, MaintenanceDueId, MaintenanceDueStatus, MaintenanceScheduleId,
    ReservationId, ReservationStatus, ResourceStatus, ResourceType, SiteId, TenantId, ToolId,
    UserId, VehicleId, VehicleType,
};
use crate::modules::fleet::domain::{
    Asset, CreateMachine, CreateMaintenanceSchedule, CreateReservation, CreateTool, CreateVehicle,
    Machine, MaintenanceDue, MaintenanceSchedule, Reservation, ReservationHolder,
    ReservationWithDetails, ResolveMaintenanceDue, Tool, UpdateMachine, UpdateReservation,
    UpdateTool, UpdateVehicle, Vehicle,
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

    fn map_asset_write_error(error: sqlx::Error, duplicate_message: &str) -> AppError {
        let message = error.to_string();
        if message.contains("vehicle_display_colors_tenant_id_display_color_key") {
            return AppError::Validation(
                "Vehicle color is already used in this tenant".to_string(),
            );
        }
        if message.contains("unique") || message.contains("duplicate") {
            AppError::Validation(duplicate_message.to_string())
        } else {
            AppError::Database(message)
        }
    }

    async fn generate_unique_vehicle_display_color(
        &self,
        tenant_id: TenantId,
        vehicle_id: Uuid,
    ) -> Result<String, AppError> {
        for attempt in 0..1024_u32 {
            let color = vehicle_display_color_from_seed(vehicle_id, attempt);
            let exists: bool = sqlx::query_scalar(
                r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM vehicle_display_colors
                    WHERE tenant_id = $1 AND display_color = $2
                )
                "#,
            )
            .bind(tenant_id.0)
            .bind(&color)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

            if !exists {
                return Ok(color);
            }
        }

        Err(AppError::Internal(
            "Could not allocate a unique vehicle color".to_string(),
        ))
    }

    pub async fn find_asset_by_id(
        &self,
        tenant_id: TenantId,
        id: AssetId,
    ) -> Result<Option<Asset>, AppError> {
        let asset = sqlx::query_as::<_, AssetRow>(
            r#"
            SELECT id, tenant_id, asset_kind, name, description, status, location, qr_code, created_at, updated_at
            FROM assets
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(asset.map(|row| row.into_asset()))
    }

    pub async fn find_reservable_asset(
        &self,
        tenant_id: TenantId,
        resource_type: ResourceType,
        resource_id: Uuid,
    ) -> Result<Option<Asset>, AppError> {
        let Some(asset) = self
            .find_asset_by_id(tenant_id, AssetId(resource_id))
            .await?
        else {
            return Ok(None);
        };

        let expected_kind = AssetKind::from(resource_type);
        if asset.kind == expected_kind && asset.can_be_reserved() {
            Ok(Some(asset))
        } else {
            Ok(None)
        }
    }

    pub async fn project_exists(
        &self,
        tenant_id: TenantId,
        project_id: SiteId,
    ) -> Result<bool, AppError> {
        let exists: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM sites
                WHERE id = $1
                  AND tenant_id = $2
                  AND deleted_at IS NULL
            )
            "#,
        )
        .bind(project_id.0)
        .bind(tenant_id.0)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(exists)
    }

    // === Maintenance operations ===

    pub async fn create_maintenance_schedule(
        &self,
        create: &CreateMaintenanceSchedule,
        tenant_id: TenantId,
    ) -> Result<(MaintenanceSchedule, MaintenanceDue), AppError> {
        let now = Utc::now();
        let schedule_id = Uuid::new_v4();
        let due_id = Uuid::new_v4();

        let row = sqlx::query_as::<_, MaintenanceScheduleWithDueRow>(
            r#"
            WITH inserted_schedule AS (
                INSERT INTO maintenance_schedules (
                    id, tenant_id, asset_id, task_description, interval_days, is_active, created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, true, $6, $7)
                RETURNING id, tenant_id, asset_id, task_description, interval_days, is_active, created_at, updated_at
            ),
            inserted_due AS (
                INSERT INTO maintenance_due (
                    id, tenant_id, schedule_id, asset_id, due_date, status, created_at, updated_at
                )
                SELECT $8, tenant_id, id, asset_id, $9, 'open', created_at, updated_at
                FROM inserted_schedule
                RETURNING id, tenant_id, schedule_id, asset_id, due_date, status, resolved_at, resolved_by, resolution_notes, created_at, updated_at
            )
            SELECT
                s.id AS schedule_id,
                s.tenant_id,
                s.asset_id,
                s.task_description,
                s.interval_days,
                s.is_active,
                s.created_at AS schedule_created_at,
                s.updated_at AS schedule_updated_at,
                d.id AS due_id,
                d.due_date,
                d.status AS due_status,
                d.resolved_at,
                d.resolved_by,
                d.resolution_notes,
                d.created_at AS due_created_at,
                d.updated_at AS due_updated_at,
                a.asset_kind AS resource_type,
                a.name AS resource_name
            FROM inserted_schedule s
            JOIN inserted_due d ON d.schedule_id = s.id
            JOIN assets a ON a.id = s.asset_id AND a.tenant_id = s.tenant_id
            "#
        )
        .bind(schedule_id)
        .bind(tenant_id.0)
        .bind(create.asset_id.0)
        .bind(create.task_description.trim())
        .bind(create.interval_days)
        .bind(now)
        .bind(now)
        .bind(due_id)
        .bind(create.next_due_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.into_schedule_and_due())
    }

    pub async fn list_maintenance_schedules(
        &self,
        tenant_id: TenantId,
        asset_id: Option<AssetId>,
    ) -> Result<Vec<MaintenanceSchedule>, AppError> {
        let rows = sqlx::query_as::<_, MaintenanceScheduleRow>(
            r#"
            SELECT id, tenant_id, asset_id, task_description, interval_days, is_active, created_at, updated_at
            FROM maintenance_schedules
            WHERE tenant_id = $1
              AND ($2::uuid IS NULL OR asset_id = $2)
            ORDER BY is_active DESC, task_description
            "#,
        )
        .bind(tenant_id.0)
        .bind(asset_id.map(|value| value.0))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|row| row.into_schedule()).collect())
    }

    pub async fn list_maintenance_due(
        &self,
        tenant_id: TenantId,
        asset_id: Option<AssetId>,
        status: Option<MaintenanceDueStatus>,
    ) -> Result<Vec<MaintenanceDue>, AppError> {
        let rows = sqlx::query_as::<_, MaintenanceDueRow>(
            r#"
            SELECT
                d.id,
                d.tenant_id,
                d.schedule_id,
                d.asset_id,
                a.asset_kind AS resource_type,
                a.name AS resource_name,
                s.task_description,
                d.due_date,
                d.status,
                d.resolved_at,
                d.resolved_by,
                d.resolution_notes,
                d.created_at,
                d.updated_at
            FROM maintenance_due d
            JOIN maintenance_schedules s ON s.id = d.schedule_id AND s.tenant_id = d.tenant_id
            JOIN assets a ON a.id = d.asset_id AND a.tenant_id = d.tenant_id
            WHERE d.tenant_id = $1
              AND ($2::uuid IS NULL OR d.asset_id = $2)
              AND ($3::text IS NULL OR d.status = $3)
              AND a.deleted_at IS NULL
            ORDER BY d.due_date ASC, a.name ASC
            "#,
        )
        .bind(tenant_id.0)
        .bind(asset_id.map(|value| value.0))
        .bind(status.as_ref().map(|value| value.as_str()))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|row| row.into_due()).collect())
    }

    pub async fn resolve_maintenance_due(
        &self,
        tenant_id: TenantId,
        due_id: MaintenanceDueId,
        resolved_by: UserId,
        resolve: &ResolveMaintenanceDue,
    ) -> Result<MaintenanceDue, AppError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let row = sqlx::query_as::<_, MaintenanceDueRow>(
            r#"
            WITH resolved_due AS (
                UPDATE maintenance_due
                SET
                    status = 'resolved',
                    resolved_at = NOW(),
                    resolved_by = $3,
                    resolution_notes = $4,
                    updated_at = NOW()
                WHERE id = $1
                  AND tenant_id = $2
                  AND status = 'open'
                RETURNING id, tenant_id, schedule_id, asset_id, due_date, status, resolved_at, resolved_by, resolution_notes, created_at, updated_at
            )
            SELECT
                d.id,
                d.tenant_id,
                d.schedule_id,
                d.asset_id,
                a.asset_kind AS resource_type,
                a.name AS resource_name,
                s.task_description,
                d.due_date,
                d.status,
                d.resolved_at,
                d.resolved_by,
                d.resolution_notes,
                d.created_at,
                d.updated_at
            FROM resolved_due d
            JOIN maintenance_schedules s ON s.id = d.schedule_id AND s.tenant_id = d.tenant_id
            JOIN assets a ON a.id = d.asset_id AND a.tenant_id = d.tenant_id
            "#,
        )
        .bind(due_id.0)
        .bind(tenant_id.0)
        .bind(resolved_by.0)
        .bind(&resolve.resolution_notes)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Open maintenance reminder not found".to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO maintenance_due (
                id, tenant_id, schedule_id, asset_id, due_date, status, created_at, updated_at
            )
            SELECT
                uuid_generate_v4(),
                tenant_id,
                id,
                asset_id,
                ($3::date + (interval_days || ' days')::interval)::date,
                'open',
                NOW(),
                NOW()
            FROM maintenance_schedules
            WHERE tenant_id = $1
              AND id = $2
              AND is_active = true
              AND NOT EXISTS (
                  SELECT 1
                  FROM maintenance_due
                  WHERE tenant_id = $1
                    AND schedule_id = $2
                    AND status = 'open'
              )
            "#,
        )
        .bind(tenant_id.0)
        .bind(row.schedule_id)
        .bind(row.due_date)
        .execute(&mut *transaction)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        transaction
            .commit()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.into_due())
    }

    // === Vehicle operations ===

    pub async fn create_vehicle(
        &self,
        create: &CreateVehicle,
        tenant_id: TenantId,
    ) -> Result<Vehicle, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        let display_color = match &create.display_color {
            Some(color) => color.to_ascii_lowercase(),
            None => {
                self.generate_unique_vehicle_display_color(tenant_id, id)
                    .await?
            }
        };

        let vehicle = sqlx::query_as::<_, VehicleRow>(
            r#"
            WITH inserted_asset AS (
                INSERT INTO assets (id, tenant_id, asset_kind, name, description, status, location, qr_code, created_at, updated_at)
                VALUES ($1, $2, 'vehicle', $3, $4, $5, $6, $7, $8, $9)
                RETURNING id, tenant_id, name, description, status, location, qr_code, created_at, updated_at
            ),
            inserted_detail AS (
                INSERT INTO vehicle_details (asset_id, tenant_id, license_plate, vehicle_type, created_at, updated_at)
                SELECT id, tenant_id, $10, $11, created_at, updated_at
                FROM inserted_asset
                RETURNING asset_id, license_plate, vehicle_type
            ),
            inserted_color AS (
                INSERT INTO vehicle_display_colors (asset_id, tenant_id, display_color, created_at, updated_at)
                SELECT id, tenant_id, $12, created_at, updated_at
                FROM inserted_asset
                RETURNING asset_id, display_color
            )
            SELECT
                a.id,
                a.tenant_id,
                a.name,
                d.license_plate,
                d.vehicle_type,
                a.description,
                a.status,
                a.location,
                a.qr_code,
                c.display_color,
                a.created_at,
                a.updated_at
            FROM inserted_asset a
            JOIN inserted_detail d ON d.asset_id = a.id
            JOIN inserted_color c ON c.asset_id = a.id
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create.name)
        .bind(&create.description)
        .bind(ResourceStatus::Available.as_str())
        .bind(&create.location)
        .bind(&create.qr_code)
        .bind(now)
        .bind(now)
        .bind(&create.license_plate)
        .bind(create.vehicle_type.as_str())
        .bind(&display_color)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Self::map_asset_write_error(e, "Vehicle with this name already exists"))?;

        Ok(vehicle.into_vehicle())
    }

    pub async fn find_vehicle_by_id(
        &self,
        tenant_id: TenantId,
        id: VehicleId,
    ) -> Result<Option<Vehicle>, AppError> {
        let vehicle = sqlx::query_as::<_, VehicleRow>(
            r#"
            SELECT assets.id, assets.tenant_id, assets.name, vehicle_details.license_plate, vehicle_details.vehicle_type, assets.description, assets.status, assets.location, assets.qr_code, vehicle_display_colors.display_color, assets.created_at, assets.updated_at
            FROM assets
            JOIN vehicle_details ON vehicle_details.asset_id = assets.id
            JOIN vehicle_display_colors ON vehicle_display_colors.asset_id = assets.id
            WHERE assets.id = $1
              AND assets.tenant_id = $2
              AND assets.asset_kind = 'vehicle'
              AND assets.deleted_at IS NULL
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
                    SELECT assets.id, assets.tenant_id, assets.name, vehicle_details.license_plate, vehicle_details.vehicle_type, assets.description, assets.status, assets.location, assets.qr_code, vehicle_display_colors.display_color, assets.created_at, assets.updated_at
                    FROM assets
                    JOIN vehicle_details ON vehicle_details.asset_id = assets.id
                    JOIN vehicle_display_colors ON vehicle_display_colors.asset_id = assets.id
                    WHERE assets.tenant_id = $1
                      AND assets.status = $2
                      AND assets.asset_kind = 'vehicle'
                      AND assets.deleted_at IS NULL
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
                    SELECT assets.id, assets.tenant_id, assets.name, vehicle_details.license_plate, vehicle_details.vehicle_type, assets.description, assets.status, assets.location, assets.qr_code, vehicle_display_colors.display_color, assets.created_at, assets.updated_at
                    FROM assets
                    JOIN vehicle_details ON vehicle_details.asset_id = assets.id
                    JOIN vehicle_display_colors ON vehicle_display_colors.asset_id = assets.id
                    WHERE assets.tenant_id = $1
                      AND assets.asset_kind = 'vehicle'
                      AND assets.deleted_at IS NULL
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
        let current = self
            .find_vehicle_by_id(tenant_id, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Vehicle not found".to_string()))?;

        // Validate status transition if status is being changed
        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(format!(
                    "Invalid status transition from {} to {}",
                    current.status, new_status
                )));
            }
        }

        let vehicle = sqlx::query_as::<_, VehicleRow>(
            r#"
            WITH updated_asset AS (
                UPDATE assets
                SET
                    name = COALESCE($1, name),
                    description = COALESCE($2, description),
                    status = COALESCE($3, status),
                    location = COALESCE($4, location),
                    qr_code = COALESCE($5, qr_code),
                    updated_at = NOW()
                WHERE id = $6
                  AND tenant_id = $7
                  AND asset_kind = 'vehicle'
                  AND deleted_at IS NULL
                RETURNING id, tenant_id, name, description, status, location, qr_code, created_at, updated_at
            ),
            updated_detail AS (
                UPDATE vehicle_details
                SET
                    license_plate = COALESCE($8, license_plate),
                    vehicle_type = COALESCE($9, vehicle_type),
                    updated_at = NOW()
                WHERE asset_id = $6 AND tenant_id = $7
                RETURNING asset_id, license_plate, vehicle_type
            ),
            updated_color AS (
                UPDATE vehicle_display_colors
                SET
                    display_color = COALESCE($10, display_color),
                    updated_at = NOW()
                WHERE asset_id = $6 AND tenant_id = $7
                RETURNING asset_id, display_color
            )
            SELECT
                a.id,
                a.tenant_id,
                a.name,
                d.license_plate,
                d.vehicle_type,
                a.description,
                a.status,
                a.location,
                a.qr_code,
                c.display_color,
                a.created_at,
                a.updated_at
            FROM updated_asset a
            JOIN updated_detail d ON d.asset_id = a.id
            JOIN updated_color c ON c.asset_id = a.id
            "#
        )
        .bind(&update.name)
        .bind(&update.description)
        .bind(update.status.as_ref().map(|s| s.as_str()))
        .bind(&update.location)
        .bind(&update.qr_code)
        .bind(id.0)
        .bind(tenant_id.0)
        .bind(&update.license_plate)
        .bind(update.vehicle_type.as_ref().map(|v| v.as_str()))
        .bind(update.display_color.as_ref().map(|color| color.to_ascii_lowercase()))
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Self::map_asset_write_error(e, "Vehicle with this name already exists"))?
        .ok_or_else(|| AppError::NotFound("Vehicle not found".to_string()))?;

        Ok(vehicle.into_vehicle())
    }

    pub async fn delete_vehicle(&self, tenant_id: TenantId, id: VehicleId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE assets
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1
              AND tenant_id = $2
              AND asset_kind = 'vehicle'
              AND deleted_at IS NULL
            "#,
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

    /// Count active reservations for a resource (for delete dependency check)
    pub async fn count_active_reservations(
        &self,
        tenant_id: TenantId,
        asset_id: Uuid,
    ) -> Result<i64, AppError> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM reservations
            WHERE tenant_id = $1 
              AND asset_id = $2
              AND status NOT IN ('cancelled', 'completed')
              AND end_time > NOW()
            "#,
        )
        .bind(tenant_id.0)
        .bind(asset_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(count)
    }

    pub async fn find_vehicle_by_qr_code(
        &self,
        tenant_id: TenantId,
        qr_code: &str,
    ) -> Result<Option<Vehicle>, AppError> {
        let vehicle = sqlx::query_as::<_, VehicleRow>(
            r#"
            SELECT assets.id, assets.tenant_id, assets.name, vehicle_details.license_plate, vehicle_details.vehicle_type, assets.description, assets.status, assets.location, assets.qr_code, vehicle_display_colors.display_color, assets.created_at, assets.updated_at
            FROM assets
            JOIN vehicle_details ON vehicle_details.asset_id = assets.id
            JOIN vehicle_display_colors ON vehicle_display_colors.asset_id = assets.id
            WHERE assets.qr_code = $1
              AND assets.tenant_id = $2
              AND assets.asset_kind = 'vehicle'
              AND assets.deleted_at IS NULL
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
            WITH inserted_asset AS (
                INSERT INTO assets (id, tenant_id, asset_kind, name, description, status, location, qr_code, created_at, updated_at)
                VALUES ($1, $2, 'tool', $3, $4, $5, $6, $7, $8, $9)
                RETURNING id, tenant_id, name, description, status, location, qr_code, created_at, updated_at
            ),
            inserted_detail AS (
                INSERT INTO tool_details (asset_id, tenant_id, category, created_at, updated_at)
                SELECT id, tenant_id, $10, created_at, updated_at
                FROM inserted_asset
                RETURNING asset_id, category
            )
            SELECT
                a.id,
                a.tenant_id,
                a.name,
                d.category,
                a.description,
                a.status,
                a.location,
                a.qr_code,
                a.created_at,
                a.updated_at
            FROM inserted_asset a
            JOIN inserted_detail d ON d.asset_id = a.id
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create.name)
        .bind(&create.description)
        .bind(ResourceStatus::Available.as_str())
        .bind(&create.location)
        .bind(&create.qr_code)
        .bind(now)
        .bind(now)
        .bind(&create.category)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Self::map_asset_write_error(e, "Tool with this name already exists"))?;

        Ok(tool.into_tool())
    }

    pub async fn find_tool_by_id(
        &self,
        tenant_id: TenantId,
        id: ToolId,
    ) -> Result<Option<Tool>, AppError> {
        let tool = sqlx::query_as::<_, ToolRow>(
            r#"
            SELECT assets.id, assets.tenant_id, assets.name, tool_details.category, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
            FROM assets
            JOIN tool_details ON tool_details.asset_id = assets.id
            WHERE assets.id = $1
              AND assets.tenant_id = $2
              AND assets.asset_kind = 'tool'
              AND assets.deleted_at IS NULL
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
                    SELECT assets.id, assets.tenant_id, assets.name, tool_details.category, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
                    FROM assets
                    JOIN tool_details ON tool_details.asset_id = assets.id
                    WHERE assets.tenant_id = $1
                      AND assets.status = $2
                      AND tool_details.category = $3
                      AND assets.asset_kind = 'tool'
                      AND assets.deleted_at IS NULL
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
                    SELECT assets.id, assets.tenant_id, assets.name, tool_details.category, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
                    FROM assets
                    JOIN tool_details ON tool_details.asset_id = assets.id
                    WHERE assets.tenant_id = $1
                      AND assets.status = $2
                      AND assets.asset_kind = 'tool'
                      AND assets.deleted_at IS NULL
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
                    SELECT assets.id, assets.tenant_id, assets.name, tool_details.category, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
                    FROM assets
                    JOIN tool_details ON tool_details.asset_id = assets.id
                    WHERE assets.tenant_id = $1
                      AND tool_details.category = $2
                      AND assets.asset_kind = 'tool'
                      AND assets.deleted_at IS NULL
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
                    SELECT assets.id, assets.tenant_id, assets.name, tool_details.category, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
                    FROM assets
                    JOIN tool_details ON tool_details.asset_id = assets.id
                    WHERE assets.tenant_id = $1
                      AND assets.asset_kind = 'tool'
                      AND assets.deleted_at IS NULL
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
        let current = self
            .find_tool_by_id(tenant_id, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tool not found".to_string()))?;

        // Validate status transition if status is being changed
        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(format!(
                    "Invalid status transition from {} to {}",
                    current.status, new_status
                )));
            }
        }

        let tool = sqlx::query_as::<_, ToolRow>(
            r#"
            WITH updated_asset AS (
                UPDATE assets
                SET
                    name = COALESCE($1, name),
                    description = COALESCE($2, description),
                    status = COALESCE($3, status),
                    location = COALESCE($4, location),
                    qr_code = COALESCE($5, qr_code),
                    updated_at = NOW()
                WHERE id = $6
                  AND tenant_id = $7
                  AND asset_kind = 'tool'
                  AND deleted_at IS NULL
                RETURNING id, tenant_id, name, description, status, location, qr_code, created_at, updated_at
            ),
            updated_detail AS (
                UPDATE tool_details
                SET
                    category = COALESCE($8, category),
                    updated_at = NOW()
                WHERE asset_id = $6 AND tenant_id = $7
                RETURNING asset_id, category
            )
            SELECT
                a.id,
                a.tenant_id,
                a.name,
                d.category,
                a.description,
                a.status,
                a.location,
                a.qr_code,
                a.created_at,
                a.updated_at
            FROM updated_asset a
            JOIN updated_detail d ON d.asset_id = a.id
            "#
        )
        .bind(&update.name)
        .bind(&update.description)
        .bind(update.status.as_ref().map(|s| s.as_str()))
        .bind(&update.location)
        .bind(&update.qr_code)
        .bind(id.0)
        .bind(tenant_id.0)
        .bind(&update.category)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Self::map_asset_write_error(e, "Tool with this name already exists"))?
        .ok_or_else(|| AppError::NotFound("Tool not found".to_string()))?;

        Ok(tool.into_tool())
    }

    pub async fn delete_tool(&self, tenant_id: TenantId, id: ToolId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE assets
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1
              AND tenant_id = $2
              AND asset_kind = 'tool'
              AND deleted_at IS NULL
            "#,
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
            SELECT assets.id, assets.tenant_id, assets.name, tool_details.category, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
            FROM assets
            JOIN tool_details ON tool_details.asset_id = assets.id
            WHERE assets.qr_code = $1
              AND assets.tenant_id = $2
              AND assets.asset_kind = 'tool'
              AND assets.deleted_at IS NULL
            "#
        )
        .bind(qr_code)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(tool.map(|t| t.into_tool()))
    }

    // === Machine operations ===

    pub async fn create_machine(
        &self,
        create: &CreateMachine,
        tenant_id: TenantId,
    ) -> Result<Machine, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let machine = sqlx::query_as::<_, MachineRow>(
            r#"
            WITH inserted_asset AS (
                INSERT INTO assets (id, tenant_id, asset_kind, name, description, status, location, qr_code, created_at, updated_at)
                VALUES ($1, $2, 'machine', $3, $4, $5, $6, $7, $8, $9)
                RETURNING id, tenant_id, name, description, status, location, qr_code, created_at, updated_at
            ),
            inserted_detail AS (
                INSERT INTO machine_details (asset_id, tenant_id, machine_type, created_at, updated_at)
                SELECT id, tenant_id, $10, created_at, updated_at
                FROM inserted_asset
                RETURNING asset_id, machine_type
            )
            SELECT
                a.id,
                a.tenant_id,
                a.name,
                d.machine_type,
                a.description,
                a.status,
                a.location,
                a.qr_code,
                a.created_at,
                a.updated_at
            FROM inserted_asset a
            JOIN inserted_detail d ON d.asset_id = a.id
            "#,
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create.name)
        .bind(&create.description)
        .bind(ResourceStatus::Available.as_str())
        .bind(&create.location)
        .bind(&create.qr_code)
        .bind(now)
        .bind(now)
        .bind(&create.machine_type)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Self::map_asset_write_error(e, "Machine with this name already exists"))?;

        Ok(machine.into_machine())
    }

    pub async fn find_machine_by_id(
        &self,
        tenant_id: TenantId,
        id: MachineId,
    ) -> Result<Option<Machine>, AppError> {
        let machine = sqlx::query_as::<_, MachineRow>(
            r#"
            SELECT assets.id, assets.tenant_id, assets.name, machine_details.machine_type, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
            FROM assets
            JOIN machine_details ON machine_details.asset_id = assets.id
            WHERE assets.id = $1
              AND assets.tenant_id = $2
              AND assets.asset_kind = 'machine'
              AND assets.deleted_at IS NULL
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(machine.map(|m| m.into_machine()))
    }

    pub async fn list_machines(
        &self,
        tenant_id: TenantId,
        status: Option<String>,
    ) -> Result<Vec<Machine>, AppError> {
        let machines = match status {
            Some(s) => {
                sqlx::query_as::<_, MachineRow>(
                    r#"
                    SELECT assets.id, assets.tenant_id, assets.name, machine_details.machine_type, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
                    FROM assets
                    JOIN machine_details ON machine_details.asset_id = assets.id
                    WHERE assets.tenant_id = $1
                      AND assets.status = $2
                      AND assets.asset_kind = 'machine'
                      AND assets.deleted_at IS NULL
                    ORDER BY name
                    "#,
                )
                .bind(tenant_id.0)
                .bind(&s)
                .fetch_all(&self.pool)
                .await
            }
            None => {
                sqlx::query_as::<_, MachineRow>(
                    r#"
                    SELECT assets.id, assets.tenant_id, assets.name, machine_details.machine_type, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
                    FROM assets
                    JOIN machine_details ON machine_details.asset_id = assets.id
                    WHERE assets.tenant_id = $1
                      AND assets.asset_kind = 'machine'
                      AND assets.deleted_at IS NULL
                    ORDER BY name
                    "#,
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(machines.into_iter().map(|m| m.into_machine()).collect())
    }

    pub async fn update_machine(
        &self,
        tenant_id: TenantId,
        id: MachineId,
        update: &UpdateMachine,
    ) -> Result<Machine, AppError> {
        let current = self
            .find_machine_by_id(tenant_id, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Machine not found".to_string()))?;

        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(format!(
                    "Invalid status transition from {} to {}",
                    current.status, new_status
                )));
            }
        }

        let machine = sqlx::query_as::<_, MachineRow>(
            r#"
            WITH updated_asset AS (
                UPDATE assets
                SET
                    name = COALESCE($1, name),
                    description = COALESCE($2, description),
                    status = COALESCE($3, status),
                    location = COALESCE($4, location),
                    qr_code = COALESCE($5, qr_code),
                    updated_at = NOW()
                WHERE id = $6
                  AND tenant_id = $7
                  AND asset_kind = 'machine'
                  AND deleted_at IS NULL
                RETURNING id, tenant_id, name, description, status, location, qr_code, created_at, updated_at
            ),
            updated_detail AS (
                UPDATE machine_details
                SET
                    machine_type = COALESCE($8, machine_type),
                    updated_at = NOW()
                WHERE asset_id = $6 AND tenant_id = $7
                RETURNING asset_id, machine_type
            )
            SELECT
                a.id,
                a.tenant_id,
                a.name,
                d.machine_type,
                a.description,
                a.status,
                a.location,
                a.qr_code,
                a.created_at,
                a.updated_at
            FROM updated_asset a
            JOIN updated_detail d ON d.asset_id = a.id
            "#,
        )
        .bind(&update.name)
        .bind(&update.description)
        .bind(update.status.as_ref().map(|s| s.as_str()))
        .bind(&update.location)
        .bind(&update.qr_code)
        .bind(id.0)
        .bind(tenant_id.0)
        .bind(&update.machine_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Self::map_asset_write_error(e, "Machine with this name already exists"))?
        .ok_or_else(|| AppError::NotFound("Machine not found".to_string()))?;

        Ok(machine.into_machine())
    }

    pub async fn delete_machine(&self, tenant_id: TenantId, id: MachineId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE assets
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1
              AND tenant_id = $2
              AND asset_kind = 'machine'
              AND deleted_at IS NULL
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Machine not found".to_string()));
        }

        Ok(())
    }

    pub async fn find_machine_by_qr_code(
        &self,
        tenant_id: TenantId,
        qr_code: &str,
    ) -> Result<Option<Machine>, AppError> {
        let machine = sqlx::query_as::<_, MachineRow>(
            r#"
            SELECT assets.id, assets.tenant_id, assets.name, machine_details.machine_type, assets.description, assets.status, assets.location, assets.qr_code, assets.created_at, assets.updated_at
            FROM assets
            JOIN machine_details ON machine_details.asset_id = assets.id
            WHERE assets.qr_code = $1
              AND assets.tenant_id = $2
              AND assets.asset_kind = 'machine'
              AND assets.deleted_at IS NULL
            "#,
        )
        .bind(qr_code)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(machine.map(|m| m.into_machine()))
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
            INSERT INTO reservations (id, tenant_id, resource_type, resource_id, asset_id, user_id, site_id, project_id, start_time, end_time, status, purpose, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id, tenant_id, resource_type, asset_id AS resource_id, user_id, site_id, project_id, start_time, end_time, status, purpose, notes, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(create.resource_type.as_str())
        .bind(create.resource_id)
        .bind(user_id.0)
        .bind(create.site_id.map(|s| s.0))
        .bind(create.project_id.map(|s| s.0))
        .bind(create.start_time)
        .bind(create.end_time)
        .bind(ReservationStatus::Confirmed.as_str())
        .bind(&create.purpose)
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
            SELECT id, tenant_id, resource_type, asset_id AS resource_id, user_id, site_id, project_id, start_time, end_time, status, purpose, notes, created_at, updated_at
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
        site_id: Option<crate::common::types::SiteId>,
    ) -> Result<Vec<Reservation>, AppError> {
        let reservations = sqlx::query_as::<_, ReservationRow>(
            r#"
            SELECT id, tenant_id, resource_type, asset_id AS resource_id, user_id, site_id, project_id, start_time, end_time, status, purpose, notes, created_at, updated_at
            FROM reservations
            WHERE tenant_id = $1
              AND ($2::uuid IS NULL OR user_id = $2)
              AND ($3::text IS NULL OR resource_type = $3)
              AND ($4::uuid IS NULL OR asset_id = $4)
              AND ($5::uuid IS NULL OR site_id = $5)
            ORDER BY start_time DESC
            "#,
        )
        .bind(tenant_id.0)
        .bind(user_id.map(|value| value.0))
        .bind(resource_type.as_ref().map(|value| value.as_str()))
        .bind(resource_id)
        .bind(site_id.map(|value| value.0))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(reservations
            .into_iter()
            .map(|r| r.into_reservation())
            .collect())
    }

    pub async fn update_reservation(
        &self,
        tenant_id: TenantId,
        id: ReservationId,
        update: &UpdateReservation,
    ) -> Result<Reservation, AppError> {
        // First get the current reservation
        let current = self
            .find_reservation_by_id(tenant_id, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Reservation not found".to_string()))?;

        // Validate status transition if status is being changed
        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(format!(
                    "Invalid status transition from {} to {}",
                    current.status, new_status
                )));
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
                project_id = COALESCE($4, project_id),
                purpose = COALESCE($5, purpose),
                notes = COALESCE($6, notes),
                status = COALESCE($7, status),
                updated_at = NOW()
            WHERE id = $8 AND tenant_id = $9
            RETURNING id, tenant_id, resource_type, asset_id AS resource_id, user_id, site_id, project_id, start_time, end_time, status, purpose, notes, created_at, updated_at
            "#
        )
        .bind(update.start_time)
        .bind(update.end_time)
        .bind(update.site_id.map(|s| s.0))
        .bind(update.project_id.map(|s| s.0))
        .bind(&update.purpose)
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
            "#,
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
        asset_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        exclude_reservation_id: Option<ReservationId>,
    ) -> Result<bool, AppError> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM reservations
            WHERE tenant_id = $1 
              AND asset_id = $2
              AND status != 'cancelled'
              AND (start_time, end_time) OVERLAPS ($3, $4)
              AND ($5::uuid IS NULL OR id != $5)
            "#,
        )
        .bind(tenant_id.0)
        .bind(asset_id)
        .bind(start_time)
        .bind(end_time)
        .bind(exclude_reservation_id.map(|id| id.0))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(count == 0)
    }

    /// Find all reservations that conflict with the given time range
    /// Returns detailed information about conflicting reservations
    pub async fn find_conflicts(
        &self,
        tenant_id: TenantId,
        asset_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<ReservationSummary>, AppError> {
        let conflicts = sqlx::query_as::<_, ReservationSummaryRow>(
            r#"
            SELECT 
                r.id, 
                r.start_time, 
                r.end_time, 
                COALESCE(NULLIF(u.name, ''), u.email, r.user_id::text) as user_name,
                r.site_id,
                s.name as site_name,
                r.status
            FROM reservations r
            LEFT JOIN users u ON r.user_id = u.id AND r.tenant_id = u.tenant_id
            LEFT JOIN sites s ON r.site_id = s.id AND r.tenant_id = s.tenant_id
            WHERE r.tenant_id = $1 
              AND r.asset_id = $2
              AND r.status != 'cancelled'
              AND (r.start_time, r.end_time) OVERLAPS ($3, $4)
            ORDER BY r.start_time
            "#,
        )
        .bind(tenant_id.0)
        .bind(asset_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(conflicts.into_iter().map(|r| r.into_summary()).collect())
    }

    /// Get calendar data for a date range
    pub async fn get_calendar_data(
        &self,
        tenant_id: TenantId,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        resource_type: Option<ResourceType>,
        site_id: Option<crate::common::types::SiteId>,
    ) -> Result<Vec<CalendarEntry>, AppError> {
        let rows = sqlx::query_as::<_, CalendarRow>(
            r#"
            SELECT 
                a.asset_kind as resource_type,
                a.id as resource_id,
                a.name as resource_name,
                vc.display_color as resource_display_color,
                COALESCE(
                    json_agg(
                        json_build_object(
                            'id', res.id,
                            'start_time', res.start_time,
                            'end_time', res.end_time,
                            'user_name', COALESCE(NULLIF(u.name, ''), u.email, res.user_id::text),
                            'site_id', res.site_id,
                            'site_name', s.name,
                            'status', res.status
                        )
                    ) FILTER (WHERE res.id IS NOT NULL), 
                    '[]'::json
                ) as reservations
            FROM assets a
            LEFT JOIN vehicle_display_colors vc ON vc.asset_id = a.id
            LEFT JOIN reservations res ON res.asset_id = a.id
                 AND res.tenant_id = $1 
                 AND res.status != 'cancelled'
                 AND (res.start_time, res.end_time) OVERLAPS ($2, $3)
                 AND ($5::uuid IS NULL OR res.site_id = $5)
             LEFT JOIN users u ON u.id = res.user_id
             LEFT JOIN sites s ON s.id = res.site_id
             WHERE a.tenant_id = $1
               AND a.deleted_at IS NULL
               AND ($4::text IS NULL OR a.asset_kind = $4)
            GROUP BY a.asset_kind, a.id, a.name, vc.display_color
            ORDER BY a.asset_kind, a.name
            "#,
        )
        .bind(tenant_id.0)
        .bind(start_date)
        .bind(end_date)
        .bind(resource_type.as_ref().map(|rt| rt.as_str()))
        .bind(site_id.map(|value| value.0))
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
        let asset = sqlx::query_as::<_, AssetRow>(
            r#"
            SELECT id, tenant_id, asset_kind, name, description, status, location, qr_code, created_at, updated_at
            FROM assets
            WHERE qr_code = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(qr_code)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        match asset.map(|row| row.into_asset()) {
            Some(asset) => {
                self.build_resource_status(
                    tenant_id,
                    asset.resource_type(),
                    asset.id.0,
                    asset.name,
                    asset.status,
                )
                .await
            }
            None => Ok(None),
        }
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
            SELECT res.id, res.start_time, res.end_time,
                   COALESCE(NULLIF(u.name, ''), u.email, res.user_id::text) as user_name,
                   res.site_id,
                   s.name as site_name, res.status
            FROM reservations res
            LEFT JOIN users u ON u.id = res.user_id
            LEFT JOIN sites s ON s.id = res.site_id
            WHERE res.tenant_id = $1 
              AND res.asset_id = $2
              AND res.status != 'cancelled'
              AND res.start_time <= $3 
              AND res.end_time >= $3
            LIMIT 1
            "#,
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
            SELECT res.id, res.start_time, res.end_time,
                   COALESCE(NULLIF(u.name, ''), u.email, res.user_id::text) as user_name,
                   res.site_id,
                   s.name as site_name, res.status
            FROM reservations res
            LEFT JOIN users u ON u.id = res.user_id
            LEFT JOIN sites s ON s.id = res.site_id
            WHERE res.tenant_id = $1 
              AND res.asset_id = $2
              AND res.status != 'cancelled'
              AND res.start_time > $3
            ORDER BY res.start_time
            LIMIT 3
            "#,
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
                res.id, res.tenant_id, a.asset_kind as resource_type, res.asset_id as resource_id,
                res.user_id, res.site_id, res.project_id, res.start_time, res.end_time,
                res.status, res.purpose, res.notes, res.created_at, res.updated_at,
                a.name as resource_name,
                COALESCE(NULLIF(u.name, ''), u.email, res.user_id::text) as user_name,
                s.name as site_name,
                p.name as project_name,
                CASE
                    WHEN res.status IN ('confirmed', 'in_use')
                     AND res.start_time <= NOW()
                     AND res.end_time >= NOW()
                    THEN res.user_id
                    ELSE NULL
                END AS current_holder_user_id,
                CASE
                    WHEN res.status IN ('confirmed', 'in_use')
                     AND res.start_time <= NOW()
                     AND res.end_time >= NOW()
                    THEN COALESCE(NULLIF(u.name, ''), u.email, res.user_id::text)
                    ELSE NULL
                END AS current_holder_user_name
            FROM reservations res
            JOIN assets a ON a.id = res.asset_id AND a.tenant_id = res.tenant_id
            LEFT JOIN users u ON u.id = res.user_id AND u.tenant_id = res.tenant_id
            LEFT JOIN sites s ON s.id = res.site_id AND s.tenant_id = res.tenant_id
            LEFT JOIN sites p ON p.id = res.project_id AND p.tenant_id = res.tenant_id
            WHERE res.id = $1 AND res.tenant_id = $2
            "#,
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
struct AssetRow {
    id: Uuid,
    tenant_id: Uuid,
    asset_kind: String,
    name: String,
    description: Option<String>,
    status: String,
    location: Option<String>,
    qr_code: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl AssetRow {
    fn into_asset(self) -> Asset {
        Asset {
            id: AssetId(self.id),
            tenant_id: TenantId(self.tenant_id),
            kind: self.asset_kind.parse().unwrap_or(AssetKind::Tool),
            name: self.name,
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
struct MaintenanceScheduleRow {
    id: Uuid,
    tenant_id: Uuid,
    asset_id: Uuid,
    task_description: String,
    interval_days: i32,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MaintenanceScheduleRow {
    fn into_schedule(self) -> MaintenanceSchedule {
        MaintenanceSchedule {
            id: MaintenanceScheduleId(self.id),
            tenant_id: TenantId(self.tenant_id),
            asset_id: AssetId(self.asset_id),
            task_description: self.task_description,
            interval_days: self.interval_days,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct MaintenanceDueRow {
    id: Uuid,
    tenant_id: Uuid,
    schedule_id: Uuid,
    asset_id: Uuid,
    resource_type: String,
    resource_name: String,
    task_description: String,
    due_date: chrono::NaiveDate,
    status: String,
    resolved_at: Option<DateTime<Utc>>,
    resolved_by: Option<Uuid>,
    resolution_notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MaintenanceDueRow {
    fn into_due(self) -> MaintenanceDue {
        MaintenanceDue {
            id: MaintenanceDueId(self.id),
            tenant_id: TenantId(self.tenant_id),
            schedule_id: MaintenanceScheduleId(self.schedule_id),
            asset_id: AssetId(self.asset_id),
            resource_type: self.resource_type.parse().unwrap_or(ResourceType::Tool),
            resource_name: self.resource_name,
            task_description: self.task_description,
            due_date: self.due_date,
            status: self.status.parse().unwrap_or(MaintenanceDueStatus::Open),
            resolved_at: self.resolved_at,
            resolved_by: self.resolved_by.map(UserId),
            resolution_notes: self.resolution_notes,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct MaintenanceScheduleWithDueRow {
    schedule_id: Uuid,
    tenant_id: Uuid,
    asset_id: Uuid,
    task_description: String,
    interval_days: i32,
    is_active: bool,
    schedule_created_at: DateTime<Utc>,
    schedule_updated_at: DateTime<Utc>,
    due_id: Uuid,
    due_date: chrono::NaiveDate,
    due_status: String,
    resolved_at: Option<DateTime<Utc>>,
    resolved_by: Option<Uuid>,
    resolution_notes: Option<String>,
    due_created_at: DateTime<Utc>,
    due_updated_at: DateTime<Utc>,
    resource_type: String,
    resource_name: String,
}

impl MaintenanceScheduleWithDueRow {
    fn into_schedule_and_due(self) -> (MaintenanceSchedule, MaintenanceDue) {
        let schedule = MaintenanceSchedule {
            id: MaintenanceScheduleId(self.schedule_id),
            tenant_id: TenantId(self.tenant_id),
            asset_id: AssetId(self.asset_id),
            task_description: self.task_description.clone(),
            interval_days: self.interval_days,
            is_active: self.is_active,
            created_at: self.schedule_created_at,
            updated_at: self.schedule_updated_at,
        };

        let due = MaintenanceDue {
            id: MaintenanceDueId(self.due_id),
            tenant_id: TenantId(self.tenant_id),
            schedule_id: MaintenanceScheduleId(self.schedule_id),
            asset_id: AssetId(self.asset_id),
            resource_type: self.resource_type.parse().unwrap_or(ResourceType::Tool),
            resource_name: self.resource_name,
            task_description: self.task_description,
            due_date: self.due_date,
            status: self
                .due_status
                .parse()
                .unwrap_or(MaintenanceDueStatus::Open),
            resolved_at: self.resolved_at,
            resolved_by: self.resolved_by.map(UserId),
            resolution_notes: self.resolution_notes,
            created_at: self.due_created_at,
            updated_at: self.due_updated_at,
        };

        (schedule, due)
    }
}

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
    display_color: String,
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
            display_color: self.display_color,
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
struct MachineRow {
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    machine_type: Option<String>,
    description: Option<String>,
    status: String,
    location: Option<String>,
    qr_code: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MachineRow {
    fn into_machine(self) -> Machine {
        Machine {
            id: MachineId(self.id),
            tenant_id: TenantId(self.tenant_id),
            name: self.name,
            machine_type: self.machine_type,
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
    project_id: Option<Uuid>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    status: String,
    purpose: Option<String>,
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
            project_id: self.project_id.map(SiteId),
            start_time: self.start_time,
            end_time: self.end_time,
            status: self.status.parse().unwrap_or(ReservationStatus::Pending),
            purpose: self.purpose,
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
    pub resource_display_color: Option<String>,
    pub reservations: Vec<ReservationSummary>,
}

/// Summary of a reservation for calendar display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationSummary {
    pub id: ReservationId,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub user_name: Option<String>,
    pub site_id: Option<crate::common::types::SiteId>,
    pub site_name: Option<String>,
    pub status: ReservationStatus,
}

/// Availability information with conflict details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityInfo {
    pub available: bool,
    pub conflicts: Vec<ReservationSummary>,
}

#[derive(Debug, FromRow)]
struct CalendarRow {
    resource_type: String,
    resource_id: Uuid,
    resource_name: String,
    resource_display_color: Option<String>,
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
            resource_display_color: self.resource_display_color,
            reservations,
        }
    }
}

fn vehicle_display_color_from_seed(vehicle_id: Uuid, attempt: u32) -> String {
    let mut hash = 2_166_136_261_u32;

    for byte in vehicle_id.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(16_777_619);
    }

    hash ^= attempt.wrapping_mul(2_654_435_761);

    let red = 48 + ((hash >> 16) & 0x7f);
    let green = 48 + ((hash >> 8) & 0x7f);
    let blue = 48 + (hash & 0x7f);

    format!("#{red:02x}{green:02x}{blue:02x}")
}

#[derive(Debug, FromRow)]
struct ReservationSummaryRow {
    id: Uuid,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    user_name: Option<String>,
    site_id: Option<Uuid>,
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
            site_id: self.site_id.map(crate::common::types::SiteId),
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
    project_id: Option<Uuid>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    status: String,
    purpose: Option<String>,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    resource_name: String,
    user_name: Option<String>,
    site_name: Option<String>,
    project_name: Option<String>,
    current_holder_user_id: Option<Uuid>,
    current_holder_user_name: Option<String>,
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
                project_id: self.project_id.map(SiteId),
                start_time: self.start_time,
                end_time: self.end_time,
                status: self.status.parse().unwrap_or(ReservationStatus::Pending),
                purpose: self.purpose,
                notes: self.notes,
                created_at: self.created_at,
                updated_at: self.updated_at,
            },
            resource_name: self.resource_name,
            user_name: self.user_name,
            site_name: self.site_name,
            project_name: self.project_name,
            current_holder: self
                .current_holder_user_id
                .map(|user_id| ReservationHolder {
                    user_id: UserId(user_id),
                    user_name: self.current_holder_user_name,
                }),
        }
    }
}
