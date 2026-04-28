use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::events::{EventBus, DomainEvent};
use crate::common::types::{
    TenantId, VehicleId, ToolId, VehicleType, ResourceStatus,
};
use crate::modules::fleet::domain::{
    Vehicle, Tool, CreateVehicle, UpdateVehicle, CreateTool, UpdateTool,
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
