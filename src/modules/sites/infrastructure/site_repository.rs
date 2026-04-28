use chrono::{DateTime, Utc, NaiveDate};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::events::{EventBus, DomainEvent};
use crate::common::types::{
    TenantId, SiteId, TimeEntryId, UserId, SiteStatus, AssignmentRole, WorkType,
};
use crate::modules::sites::domain::{
    Site, SiteAssignment, TimeEntry,
    CreateSite, UpdateSite, CreateTimeEntry,
};

/// Repository for site data access with tenant isolation
pub struct SiteRepository {
    pool: PgPool,
    event_bus: EventBus,
}

impl SiteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            event_bus: EventBus::new(),
        }
    }

    // === Site operations ===

    pub async fn create_site(
        &self,
        create: &CreateSite,
        tenant_id: TenantId,
    ) -> Result<Site, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let site = sqlx::query_as::<_, SiteRow>(
            r#"
            INSERT INTO sites (id, tenant_id, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, tenant_id, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create.name)
        .bind(&create.customer_name)
        .bind(&create.location)
        .bind(&create.description)
        .bind(SiteStatus::Planned.as_str())
        .bind(create.start_date)
        .bind(create.end_date)
        .bind(create.estimated_days)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                AppError::Validation("Site with this name already exists".to_string())
            } else {
                AppError::Database(e.to_string())
            }
        })?;

        Ok(site.into_site())
    }

    pub async fn find_site_by_id(
        &self,
        tenant_id: TenantId,
        id: SiteId,
    ) -> Result<Option<Site>, AppError> {
        let site = sqlx::query_as::<_, SiteRow>(
            r#"
            SELECT id, tenant_id, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
            FROM sites
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(site.map(|s| s.into_site()))
    }

    pub async fn list_sites(
        &self,
        tenant_id: TenantId,
        status: Option<String>,
    ) -> Result<Vec<Site>, AppError> {
        let sites = match status {
            Some(s) => {
                sqlx::query_as::<_, SiteRow>(
                    r#"
                    SELECT id, tenant_id, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
                    FROM sites
                    WHERE tenant_id = $1 AND status = $2
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(&s)
                .fetch_all(&self.pool)
                .await
            }
            None => {
                sqlx::query_as::<_, SiteRow>(
                    r#"
                    SELECT id, tenant_id, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
                    FROM sites
                    WHERE tenant_id = $1
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(sites.into_iter().map(|s| s.into_site()).collect())
    }

    pub async fn update_site(
        &self,
        tenant_id: TenantId,
        id: SiteId,
        update: &UpdateSite,
    ) -> Result<Site, AppError> {
        // First get the current site
        let current = self.find_site_by_id(tenant_id, id).await?
            .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        // Validate status transition if status is being changed
        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(
                    format!("Invalid status transition from {} to {}", current.status, new_status)
                ));
            }
        }

        let site = sqlx::query_as::<_, SiteRow>(
            r#"
            UPDATE sites
            SET 
                name = COALESCE($1, name),
                customer_name = COALESCE($2, customer_name),
                location = COALESCE($3, location),
                description = COALESCE($4, description),
                status = COALESCE($5, status),
                start_date = COALESCE($6, start_date),
                end_date = COALESCE($7, end_date),
                estimated_days = COALESCE($8, estimated_days),
                updated_at = NOW()
            WHERE id = $9 AND tenant_id = $10
            RETURNING id, tenant_id, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
            "#
        )
        .bind(&update.name)
        .bind(&update.customer_name)
        .bind(&update.location)
        .bind(&update.description)
        .bind(update.status.as_ref().map(|s| s.as_str()))
        .bind(update.start_date)
        .bind(update.end_date)
        .bind(update.estimated_days)
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        Ok(site.into_site())
    }

    // === Assignment operations ===

    pub async fn assign_user(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
        user_id: UserId,
        role: AssignmentRole,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO site_assignments (id, tenant_id, site_id, user_id, role, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (tenant_id, site_id, user_id) 
            DO UPDATE SET role = $5
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(user_id.0)
        .bind(role.as_str())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn remove_assignment(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
        user_id: UserId,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM site_assignments
            WHERE tenant_id = $1 AND site_id = $2 AND user_id = $3
            "#
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(user_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Assignment not found".to_string()));
        }

        Ok(())
    }

    pub async fn list_assignments(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
    ) -> Result<Vec<SiteAssignment>, AppError> {
        let assignments = sqlx::query_as::<_, AssignmentRow>(
            r#"
            SELECT id, tenant_id, site_id, user_id, role, created_at
            FROM site_assignments
            WHERE tenant_id = $1 AND site_id = $2
            ORDER BY created_at
            "#
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(assignments.into_iter().map(|a| a.into_assignment()).collect())
    }

    // === Time entry operations ===

    pub async fn create_time_entry(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
        create: &CreateTimeEntry,
    ) -> Result<TimeEntry, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let entry = sqlx::query_as::<_, TimeEntryRow>(
            r#"
            INSERT INTO time_entries (id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(create.site_id.map(|s| s.0))
        .bind(user_id.0)
        .bind(create.work_type.as_str())
        .bind(create.hours)
        .bind(create.work_date)
        .bind(&create.notes)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entry.into_time_entry())
    }

    pub async fn list_time_entries(
        &self,
        tenant_id: TenantId,
        site_id: Option<SiteId>,
        user_id: Option<UserId>,
    ) -> Result<Vec<TimeEntry>, AppError> {
        let entries = match (site_id, user_id) {
            (Some(site), Some(user)) => {
                sqlx::query_as::<_, TimeEntryRow>(
                    r#"
                    SELECT id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
                    FROM time_entries
                    WHERE tenant_id = $1 AND site_id = $2 AND user_id = $3
                    ORDER BY work_date DESC, created_at DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(site.0)
                .bind(user.0)
                .fetch_all(&self.pool)
                .await
            }
            (Some(site), None) => {
                sqlx::query_as::<_, TimeEntryRow>(
                    r#"
                    SELECT id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
                    FROM time_entries
                    WHERE tenant_id = $1 AND site_id = $2
                    ORDER BY work_date DESC, created_at DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(site.0)
                .fetch_all(&self.pool)
                .await
            }
            (None, Some(user)) => {
                sqlx::query_as::<_, TimeEntryRow>(
                    r#"
                    SELECT id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
                    FROM time_entries
                    WHERE tenant_id = $1 AND user_id = $2
                    ORDER BY work_date DESC, created_at DESC
                    "#
                )
                .bind(tenant_id.0)
                .bind(user.0)
                .fetch_all(&self.pool)
                .await
            }
            (None, None) => {
                sqlx::query_as::<_, TimeEntryRow>(
                    r#"
                    SELECT id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
                    FROM time_entries
                    WHERE tenant_id = $1
                    ORDER BY work_date DESC, created_at DESC
                    "#
                )
                .bind(tenant_id.0)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entries.into_iter().map(|e| e.into_time_entry()).collect())
    }

    // === Event publishing ===

    pub async fn publish_event(&self, event: &DomainEvent) -> Result<(), AppError> {
        self.event_bus.publish(event, &self.pool).await
    }
}

// === Database row types ===

#[derive(Debug, FromRow)]
struct SiteRow {
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    customer_name: String,
    location: Option<String>,
    description: Option<String>,
    status: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    estimated_days: Option<i32>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SiteRow {
    fn into_site(self) -> Site {
        Site {
            id: SiteId(self.id),
            tenant_id: TenantId(self.tenant_id),
            name: self.name,
            customer_name: self.customer_name,
            location: self.location,
            description: self.description,
            status: self.status.parse().unwrap_or(SiteStatus::Planned),
            start_date: self.start_date,
            end_date: self.end_date,
            estimated_days: self.estimated_days,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct AssignmentRow {
    id: Uuid,
    tenant_id: Uuid,
    site_id: Uuid,
    user_id: Uuid,
    role: String,
    created_at: DateTime<Utc>,
}

impl AssignmentRow {
    fn into_assignment(self) -> SiteAssignment {
        SiteAssignment {
            id: self.id,
            tenant_id: TenantId(self.tenant_id),
            site_id: SiteId(self.site_id),
            user_id: UserId(self.user_id),
            role: self.role.parse().unwrap_or(AssignmentRole::Worker),
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct TimeEntryRow {
    id: Uuid,
    tenant_id: Uuid,
    site_id: Option<Uuid>,
    user_id: Uuid,
    work_type: String,
    hours: f64,
    work_date: NaiveDate,
    notes: Option<String>,
    created_at: DateTime<Utc>,
}

impl TimeEntryRow {
    fn into_time_entry(self) -> TimeEntry {
        TimeEntry {
            id: TimeEntryId(self.id),
            tenant_id: TenantId(self.tenant_id),
            site_id: self.site_id.map(SiteId),
            user_id: UserId(self.user_id),
            work_type: self.work_type.parse().unwrap_or(WorkType::SiteWork),
            hours: self.hours,
            work_date: self.work_date,
            notes: self.notes,
            created_at: self.created_at,
        }
    }
}
