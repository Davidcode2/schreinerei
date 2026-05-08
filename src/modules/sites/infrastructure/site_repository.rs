use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::events::{DomainEvent, EventBus};
use crate::common::types::{
    ActivityId, AssignmentRole, ProjectType, SiteId, SiteStatus, TenantId, TimeEntryId, UserId,
    WorkType,
};
use crate::modules::sites::domain::{
    Activity, CreateActivity, CreateSite, CreateTimeEntry, Site, SiteActivityAttachment,
    SiteAssignment, TimeEntry, UpdateSite, UpdateTimeEntry,
};

/// Repository for site data access with tenant isolation
pub struct SiteRepository {
    pool: PgPool,
    event_bus: EventBus,
}

#[derive(Debug, Clone)]
pub struct ProjectLaborSummary {
    pub total_hours: f64,
    pub entry_count: i64,
    pub site_hours: f64,
    pub workshop_hours: f64,
    pub last_work_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct SiteHistoryReportRow {
    pub site_id: SiteId,
    pub project_type: ProjectType,
    pub name: String,
    pub customer_name: String,
    pub status: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub estimated_days: Option<i32>,
    pub budget_amount_cents: Option<i64>,
    pub billing_reference: Option<String>,
    pub quote_reference: Option<String>,
    pub total_hours: f64,
    pub worker_count: i64,
    pub distinct_material_count: i64,
    pub withdrawal_count: i64,
    pub cost_basis: String,
}

pub struct SiteHistoryReportFilter {
    pub customer: Option<String>,
    pub project_type: Option<ProjectType>,
    pub worker_id: Option<UserId>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub duration_min_hours: Option<f64>,
    pub duration_max_hours: Option<f64>,
    pub cost_basis: Option<String>,
}

impl SiteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            event_bus: EventBus::new(),
        }
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
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
            INSERT INTO sites (id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(create.project_type.as_str())
        .bind(&create.name)
        .bind(&create.customer_name)
        .bind(&create.location)
        .bind(&create.description)
        .bind(SiteStatus::Planned.as_str())
        .bind(create.start_date)
        .bind(create.end_date)
        .bind(create.estimated_days)
        .bind(create.budget_amount_cents)
        .bind(&create.billing_reference)
        .bind(&create.billing_notes)
        .bind(&create.quote_reference)
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
            SELECT id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference, created_at, updated_at
            FROM sites
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
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
                    SELECT id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference, created_at, updated_at
                    FROM sites
                    WHERE tenant_id = $1 AND status = $2 AND deleted_at IS NULL
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
                    SELECT id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference, created_at, updated_at
                    FROM sites
                    WHERE tenant_id = $1 AND deleted_at IS NULL
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
        let current = self
            .find_site_by_id(tenant_id, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        // Validate status transition if status is being changed
        if let Some(new_status) = &update.status {
            if !current.can_transition_to(*new_status) {
                return Err(AppError::Validation(format!(
                    "Invalid status transition from {} to {}",
                    current.status, new_status
                )));
            }
        }

        let site = sqlx::query_as::<_, SiteRow>(
            r#"
            UPDATE sites
            SET 
                project_type = COALESCE($1, project_type),
                name = COALESCE($2, name),
                customer_name = COALESCE($3, customer_name),
                location = COALESCE($4, location),
                description = COALESCE($5, description),
                status = COALESCE($6, status),
                start_date = COALESCE($7, start_date),
                end_date = COALESCE($8, end_date),
                estimated_days = COALESCE($9, estimated_days),
                budget_amount_cents = CASE WHEN $10 THEN NULL ELSE COALESCE($11, budget_amount_cents) END,
                billing_reference = CASE WHEN $12 THEN NULL ELSE COALESCE($13, billing_reference) END,
                billing_notes = CASE WHEN $14 THEN NULL ELSE COALESCE($15, billing_notes) END,
                quote_reference = CASE WHEN $16 THEN NULL ELSE COALESCE($17, quote_reference) END,
                updated_at = NOW()
            WHERE id = $18 AND tenant_id = $19
            RETURNING id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, budget_amount_cents, billing_reference, billing_notes, quote_reference, created_at, updated_at
            "#
        )
        .bind(update.project_type.as_ref().map(|p| p.as_str()))
        .bind(&update.name)
        .bind(&update.customer_name)
        .bind(&update.location)
        .bind(&update.description)
        .bind(update.status.as_ref().map(|s| s.as_str()))
        .bind(update.start_date)
        .bind(update.end_date)
        .bind(update.estimated_days)
        .bind(update.clear_budget_amount)
        .bind(update.budget_amount_cents)
        .bind(update.clear_billing_reference)
        .bind(&update.billing_reference)
        .bind(update.clear_billing_notes)
        .bind(&update.billing_notes)
        .bind(update.clear_quote_reference)
        .bind(&update.quote_reference)
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        Ok(site.into_site())
    }

    /// Count active reservations for a site (for delete dependency check)
    pub async fn count_active_reservations(
        &self,
        site_id: SiteId,
        tenant_id: TenantId,
    ) -> Result<i64, AppError> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM reservations
            WHERE tenant_id = $1 
              AND site_id = $2 
              AND status NOT IN ('cancelled', 'completed')
              AND end_time > NOW()
            "#,
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(count)
    }

    /// Soft delete a site by setting deleted_at timestamp
    pub async fn delete_site(&self, id: SiteId, tenant_id: TenantId) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sites
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Site not found".to_string()));
        }

        Ok(())
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
            "#,
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
            "#,
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
            "#,
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(assignments
            .into_iter()
            .map(|a| a.into_assignment())
            .collect())
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

    pub async fn find_time_entry_by_id(
        &self,
        tenant_id: TenantId,
        id: TimeEntryId,
    ) -> Result<Option<TimeEntry>, AppError> {
        let entry = sqlx::query_as::<_, TimeEntryRow>(
            r#"
            SELECT id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
            FROM time_entries
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entry.map(|e| e.into_time_entry()))
    }

    pub async fn update_time_entry(
        &self,
        tenant_id: TenantId,
        id: TimeEntryId,
        update: &UpdateTimeEntry,
    ) -> Result<TimeEntry, AppError> {
        let should_update_site_id = update.site_id.is_some();
        let site_id_value = update.site_id.flatten().map(|site_id| site_id.0);
        let notes_update = update.notes.clone();
        let should_update_notes = notes_update.is_some();
        let notes_value = notes_update.flatten();

        let entry = sqlx::query_as::<_, TimeEntryRow>(
            r#"
            UPDATE time_entries
            SET 
                site_id = CASE
                    WHEN $1::boolean THEN $2
                    ELSE site_id
                END,
                work_type = COALESCE($3, work_type),
                hours = COALESCE($4, hours),
                work_date = COALESCE($5, work_date),
                notes = CASE 
                    WHEN $6::boolean THEN $7 
                    ELSE notes 
                END
            WHERE id = $8 AND tenant_id = $9
            RETURNING id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
            "#
        )
        .bind(should_update_site_id)
        .bind(site_id_value)
        .bind(update.work_type.as_ref().map(|wt| wt.as_str()))
        .bind(update.hours)
        .bind(update.work_date)
        .bind(should_update_notes)
        .bind(notes_value)
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))?;

        Ok(entry.into_time_entry())
    }

    pub async fn delete_time_entry(
        &self,
        tenant_id: TenantId,
        id: TimeEntryId,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM time_entries
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Time entry not found".to_string()));
        }

        Ok(())
    }

    // === Activity operations ===

    pub async fn create_activity(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
        create: &CreateActivity,
    ) -> Result<Activity, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let activity = sqlx::query_as::<_, ActivityRow>(
            r#"
            WITH inserted_activity AS (
                INSERT INTO site_activities (id, tenant_id, site_id, user_id, activity_type, content, photo_url, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, tenant_id, site_id, user_id, activity_type, content, photo_url, created_at
            )
            SELECT
                inserted_activity.id,
                inserted_activity.tenant_id,
                inserted_activity.site_id,
                inserted_activity.user_id,
                COALESCE(NULLIF(users.name, ''), users.email, inserted_activity.user_id::text) AS creator_name,
                inserted_activity.activity_type,
                inserted_activity.content,
                inserted_activity.photo_url,
                inserted_activity.created_at
            FROM inserted_activity
            LEFT JOIN users
                ON users.id = inserted_activity.user_id
               AND users.tenant_id = inserted_activity.tenant_id
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(create.site_id.0)
        .bind(user_id.0)
        .bind(create.activity_type.as_str())
        .bind(&create.content)
        .bind(&create.photo_url)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(activity.into_activity())
    }

    pub async fn list_activities(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
        limit: i32,
    ) -> Result<Vec<Activity>, AppError> {
        let activities = sqlx::query_as::<_, ActivityRow>(
            r#"
            SELECT
                site_activities.id,
                site_activities.tenant_id,
                site_activities.site_id,
                site_activities.user_id,
                COALESCE(NULLIF(users.name, ''), users.email, site_activities.user_id::text) AS creator_name,
                site_activities.activity_type,
                site_activities.content,
                site_activities.photo_url,
                site_activities.created_at
            FROM site_activities
            LEFT JOIN users
                ON users.id = site_activities.user_id
               AND users.tenant_id = site_activities.tenant_id
            WHERE site_activities.tenant_id = $1 AND site_activities.site_id = $2
            ORDER BY created_at DESC
            LIMIT $3
            "#
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(activities.into_iter().map(|a| a.into_activity()).collect())
    }

    pub async fn find_activity_by_id(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
        activity_id: ActivityId,
    ) -> Result<Option<Activity>, AppError> {
        let activity = sqlx::query_as::<_, ActivityRow>(
            r#"
            SELECT
                site_activities.id,
                site_activities.tenant_id,
                site_activities.site_id,
                site_activities.user_id,
                COALESCE(NULLIF(users.name, ''), users.email, site_activities.user_id::text) AS creator_name,
                site_activities.activity_type,
                site_activities.content,
                site_activities.photo_url,
                site_activities.created_at
            FROM site_activities
            LEFT JOIN users
                ON users.id = site_activities.user_id
               AND users.tenant_id = site_activities.tenant_id
            WHERE site_activities.tenant_id = $1
              AND site_activities.site_id = $2
              AND site_activities.id = $3
            "#,
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(activity_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(activity.map(ActivityRow::into_activity))
    }

    pub async fn delete_activity(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
        activity_id: ActivityId,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM site_activities
            WHERE tenant_id = $1 AND site_id = $2 AND id = $3
            "#,
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(activity_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Activity not found".to_string()));
        }

        Ok(())
    }

    pub async fn delete_attachment_by_id(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
        attachment_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM site_activity_attachments
            WHERE tenant_id = $1 AND site_id = $2 AND id = $3
            "#,
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(attachment_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn create_activity_attachment(
        &self,
        tenant_id: TenantId,
        attachment: &SiteActivityAttachment,
    ) -> Result<SiteActivityAttachment, AppError> {
        let row = sqlx::query_as::<_, AttachmentRow>(
            r#"
            INSERT INTO site_activity_attachments (
                id, tenant_id, activity_id, site_id, storage_key, thumbnail_key,
                original_filename, mime_type, size_bytes, original_bytes, thumbnail_bytes, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, tenant_id, activity_id, site_id, storage_key, thumbnail_key,
                original_filename, mime_type, size_bytes, original_bytes, thumbnail_bytes, created_at
            "#,
        )
        .bind(attachment.id)
        .bind(tenant_id.0)
        .bind(attachment.activity_id.map(|id| id.0))
        .bind(attachment.site_id.0)
        .bind(&attachment.storage_key)
        .bind(&attachment.thumbnail_key)
        .bind(&attachment.original_filename)
        .bind(&attachment.mime_type)
        .bind(attachment.size_bytes)
        .bind(&attachment.original_bytes)
        .bind(&attachment.thumbnail_bytes)
        .bind(attachment.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.into_attachment())
    }

    pub async fn update_activity_photo_url(
        &self,
        tenant_id: TenantId,
        activity_id: ActivityId,
        site_id: SiteId,
        photo_url: &str,
    ) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            UPDATE site_activities
            SET photo_url = $1
            WHERE id = $2 AND tenant_id = $3 AND site_id = $4 AND activity_type = 'photo'
            "#,
        )
        .bind(photo_url)
        .bind(activity_id.0)
        .bind(tenant_id.0)
        .bind(site_id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Photo activity not found".to_string()));
        }

        Ok(())
    }

    pub async fn link_activity_attachments(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
        activity_id: ActivityId,
        attachment_ids: &[Uuid],
    ) -> Result<Vec<SiteActivityAttachment>, AppError> {
        if attachment_ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as::<_, AttachmentRow>(
            r#"
            UPDATE site_activity_attachments
            SET activity_id = $1
            WHERE tenant_id = $2
              AND site_id = $3
              AND id = ANY($4)
              AND activity_id IS NULL
            RETURNING id, tenant_id, activity_id, site_id, storage_key, thumbnail_key,
                original_filename, mime_type, size_bytes, original_bytes, thumbnail_bytes, created_at
            "#,
        )
        .bind(activity_id.0)
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(attachment_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if rows.len() != attachment_ids.len() {
            return Err(AppError::Validation(
                "One or more attachments could not be linked to this activity".to_string(),
            ));
        }

        Ok(rows
            .into_iter()
            .map(AttachmentRow::into_attachment)
            .collect())
    }

    pub async fn list_activity_attachments(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
        activity_ids: &[ActivityId],
    ) -> Result<HashMap<ActivityId, Vec<SiteActivityAttachment>>, AppError> {
        if activity_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let activity_uuids: Vec<_> = activity_ids.iter().map(|id| id.0).collect();
        let rows = sqlx::query_as::<_, AttachmentRow>(
            r#"
            SELECT id, tenant_id, activity_id, site_id, storage_key, thumbnail_key,
                original_filename, mime_type, size_bytes, original_bytes, thumbnail_bytes, created_at
            FROM site_activity_attachments
            WHERE tenant_id = $1
              AND site_id = $2
              AND activity_id = ANY($3)
            ORDER BY created_at ASC
            "#,
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .bind(&activity_uuids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let mut grouped = HashMap::new();
        for row in rows {
            let attachment = row.into_attachment();
            if let Some(activity_id) = attachment.activity_id {
                grouped
                    .entry(activity_id)
                    .or_insert_with(Vec::new)
                    .push(attachment);
            }
        }

        Ok(grouped)
    }

    pub async fn find_attachment_by_id(
        &self,
        attachment_id: Uuid,
        tenant_id: TenantId,
    ) -> Result<Option<SiteActivityAttachment>, AppError> {
        let row = sqlx::query_as::<_, AttachmentRow>(
            r#"
            SELECT id, tenant_id, activity_id, site_id, storage_key, thumbnail_key,
                original_filename, mime_type, size_bytes, original_bytes, thumbnail_bytes, created_at
            FROM site_activity_attachments
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(attachment_id)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.map(|r| r.into_attachment()))
    }

    // === Dashboard operations ===

    pub async fn get_dashboard_sites(
        &self,
        tenant_id: TenantId,
    ) -> Result<Vec<DashboardSite>, AppError> {
        let sites = sqlx::query_as::<_, DashboardSiteRow>(
            r#"
            SELECT 
                s.id, s.tenant_id, s.project_type, s.name, s.customer_name, s.location, 
                s.status, s.start_date, s.end_date, s.estimated_days,
                COUNT(DISTINCT sa.user_id) as assigned_users,
                COALESCE(SUM(te.hours), 0)::FLOAT as total_hours
            FROM sites s
            LEFT JOIN site_assignments sa ON s.id = sa.site_id AND s.tenant_id = sa.tenant_id
            LEFT JOIN time_entries te ON s.id = te.site_id AND s.tenant_id = te.tenant_id
            WHERE s.tenant_id = $1 AND s.deleted_at IS NULL
            GROUP BY s.id
            ORDER BY 
                CASE s.status 
                    WHEN 'active' THEN 1 
                    WHEN 'planned' THEN 2 
                    WHEN 'completed' THEN 3
                    WHEN 'archived' THEN 4
                    ELSE 5
                END,
                s.start_date ASC NULLS LAST
            "#,
        )
        .bind(tenant_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(sites.into_iter().map(|s| s.into_dashboard_site()).collect())
    }

    pub async fn get_project_labor_summary(
        &self,
        tenant_id: TenantId,
        site_id: SiteId,
    ) -> Result<ProjectLaborSummary, AppError> {
        let row = sqlx::query_as::<_, ProjectLaborSummaryRow>(
            r#"
            SELECT
                COALESCE(SUM(hours), 0)::FLOAT8 AS total_hours,
                COUNT(*)::INT8 AS entry_count,
                COALESCE(SUM(CASE WHEN work_type = 'site' THEN hours ELSE 0 END), 0)::FLOAT8 AS site_hours,
                COALESCE(SUM(CASE WHEN work_type = 'workshop' THEN hours ELSE 0 END), 0)::FLOAT8 AS workshop_hours,
                MAX(work_date) AS last_work_date
            FROM time_entries
            WHERE tenant_id = $1 AND site_id = $2
            "#,
        )
        .bind(tenant_id.0)
        .bind(site_id.0)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.into_summary())
    }

    pub async fn list_site_history_report(
        &self,
        tenant_id: TenantId,
        filter: &SiteHistoryReportFilter,
    ) -> Result<Vec<SiteHistoryReportRow>, AppError> {
        let rows = sqlx::query_as::<_, SiteHistoryReportRowDb>(
            r#"
            WITH labor AS (
                SELECT
                    te.site_id,
                    COALESCE(SUM(te.hours), 0)::FLOAT8 AS total_hours,
                    COUNT(DISTINCT te.user_id)::INT8 AS worker_count
                FROM time_entries te
                WHERE te.tenant_id = $1
                GROUP BY te.site_id
            ),
            materials AS (
                SELECT
                    se.site_id,
                    COUNT(DISTINCT se.material_id)::INT8 AS distinct_material_count,
                    COUNT(*)::INT8 AS withdrawal_count
                FROM stock_entries se
                WHERE se.tenant_id = $1 AND se.entry_type = 'withdrawn'
                GROUP BY se.site_id
            )
            SELECT
                s.id AS site_id,
                s.project_type,
                s.name,
                s.customer_name,
                s.status,
                s.start_date,
                s.end_date,
                s.estimated_days,
                s.budget_amount_cents,
                s.billing_reference,
                s.quote_reference,
                COALESCE(l.total_hours, 0)::FLOAT8 AS total_hours,
                COALESCE(l.worker_count, 0)::INT8 AS worker_count,
                COALESCE(m.distinct_material_count, 0)::INT8 AS distinct_material_count,
                COALESCE(m.withdrawal_count, 0)::INT8 AS withdrawal_count,
                CASE
                    WHEN s.budget_amount_cents IS NOT NULL AND (s.billing_reference IS NOT NULL OR s.quote_reference IS NOT NULL) THEN 'invoice_ready'
                    WHEN s.budget_amount_cents IS NOT NULL AND (COALESCE(l.total_hours, 0) > 0 OR COALESCE(m.withdrawal_count, 0) > 0) THEN 'budget_vs_actual'
                    WHEN s.budget_amount_cents IS NOT NULL THEN 'budget_only'
                    WHEN COALESCE(l.total_hours, 0) > 0 OR COALESCE(m.withdrawal_count, 0) > 0 THEN 'actuals_only'
                    ELSE 'none'
                END AS cost_basis
            FROM sites s
            LEFT JOIN labor l ON l.site_id = s.id
            LEFT JOIN materials m ON m.site_id = s.id
            WHERE s.tenant_id = $1
              AND s.deleted_at IS NULL
              AND s.status IN ('completed', 'archived')
              AND ($2::text IS NULL OR s.customer_name ILIKE '%' || $2 || '%')
              AND ($3::text IS NULL OR s.project_type = $3)
              AND ($4::uuid IS NULL OR EXISTS (
                    SELECT 1 FROM time_entries te
                    WHERE te.tenant_id = s.tenant_id AND te.site_id = s.id AND te.user_id = $4
                ))
              AND ($5::date IS NULL OR s.end_date >= $5 OR s.start_date >= $5)
              AND ($6::date IS NULL OR s.start_date <= $6 OR s.end_date <= $6)
              AND ($7::float8 IS NULL OR COALESCE(l.total_hours, 0) >= $7)
              AND ($8::float8 IS NULL OR COALESCE(l.total_hours, 0) <= $8)
              AND (
                    $9::text IS NULL OR
                    CASE
                        WHEN s.budget_amount_cents IS NOT NULL AND (s.billing_reference IS NOT NULL OR s.quote_reference IS NOT NULL) THEN 'invoice_ready'
                        WHEN s.budget_amount_cents IS NOT NULL AND (COALESCE(l.total_hours, 0) > 0 OR COALESCE(m.withdrawal_count, 0) > 0) THEN 'budget_vs_actual'
                        WHEN s.budget_amount_cents IS NOT NULL THEN 'budget_only'
                        WHEN COALESCE(l.total_hours, 0) > 0 OR COALESCE(m.withdrawal_count, 0) > 0 THEN 'actuals_only'
                        ELSE 'none'
                    END = $9
              )
            ORDER BY COALESCE(s.end_date, s.start_date) DESC NULLS LAST, s.name ASC
            "#,
        )
        .bind(tenant_id.0)
        .bind(filter.customer.as_deref())
        .bind(filter.project_type.as_ref().map(|value| value.as_str()))
        .bind(filter.worker_id.map(|value| value.0))
        .bind(filter.date_from)
        .bind(filter.date_to)
        .bind(filter.duration_min_hours)
        .bind(filter.duration_max_hours)
        .bind(filter.cost_basis.as_deref())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(SiteHistoryReportRowDb::into_row)
            .collect())
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
    project_type: String,
    name: String,
    customer_name: String,
    location: Option<String>,
    description: Option<String>,
    status: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    estimated_days: Option<i32>,
    budget_amount_cents: Option<i64>,
    billing_reference: Option<String>,
    billing_notes: Option<String>,
    quote_reference: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SiteRow {
    fn into_site(self) -> Site {
        Site {
            id: SiteId(self.id),
            tenant_id: TenantId(self.tenant_id),
            project_type: self
                .project_type
                .parse()
                .unwrap_or(ProjectType::ExternalSite),
            name: self.name,
            customer_name: self.customer_name,
            location: self.location,
            description: self.description,
            status: self.status.parse().unwrap_or(SiteStatus::Planned),
            start_date: self.start_date,
            end_date: self.end_date,
            estimated_days: self.estimated_days,
            budget_amount_cents: self.budget_amount_cents,
            billing_reference: self.billing_reference,
            billing_notes: self.billing_notes,
            quote_reference: self.quote_reference,
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
            work_type: self.work_type.parse().unwrap_or(WorkType::Site),
            hours: self.hours,
            work_date: self.work_date,
            notes: self.notes,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct ActivityRow {
    id: Uuid,
    tenant_id: Uuid,
    site_id: Uuid,
    user_id: Uuid,
    creator_name: String,
    activity_type: String,
    content: Option<String>,
    photo_url: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct AttachmentRow {
    id: Uuid,
    tenant_id: Uuid,
    activity_id: Option<Uuid>,
    site_id: Uuid,
    storage_key: String,
    thumbnail_key: Option<String>,
    original_filename: String,
    mime_type: String,
    size_bytes: i64,
    original_bytes: Option<Vec<u8>>,
    thumbnail_bytes: Option<Vec<u8>>,
    created_at: DateTime<Utc>,
}

impl ActivityRow {
    fn into_activity(self) -> Activity {
        use crate::modules::sites::domain::ActivityType;
        Activity {
            id: ActivityId(self.id),
            tenant_id: TenantId(self.tenant_id),
            site_id: SiteId(self.site_id),
            user_id: UserId(self.user_id),
            creator_name: self.creator_name,
            can_delete: false,
            activity_type: self.activity_type.parse().unwrap_or(ActivityType::Note),
            content: self.content,
            photo_url: self.photo_url,
            attachments: Vec::new(),
            created_at: self.created_at,
        }
    }
}

impl AttachmentRow {
    fn into_attachment(self) -> SiteActivityAttachment {
        SiteActivityAttachment {
            id: self.id,
            tenant_id: TenantId(self.tenant_id),
            activity_id: self.activity_id.map(ActivityId),
            site_id: SiteId(self.site_id),
            storage_key: self.storage_key,
            thumbnail_key: self.thumbnail_key,
            original_filename: self.original_filename,
            mime_type: self.mime_type,
            size_bytes: self.size_bytes,
            original_bytes: self.original_bytes,
            thumbnail_bytes: self.thumbnail_bytes,
            created_at: self.created_at,
        }
    }
}

/// Dashboard site summary for open sites view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSite {
    pub id: SiteId,
    pub tenant_id: TenantId,
    pub project_type: ProjectType,
    pub name: String,
    pub customer_name: String,
    pub location: Option<String>,
    pub status: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub estimated_days: Option<i32>,
    pub assigned_users: i64,
    pub total_hours: f64,
}

#[derive(Debug, FromRow)]
struct DashboardSiteRow {
    id: Uuid,
    tenant_id: Uuid,
    project_type: String,
    name: String,
    customer_name: String,
    location: Option<String>,
    status: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    estimated_days: Option<i32>,
    assigned_users: i64,
    total_hours: Option<f64>,
}

#[derive(Debug, FromRow)]
struct ProjectLaborSummaryRow {
    total_hours: f64,
    entry_count: i64,
    site_hours: f64,
    workshop_hours: f64,
    last_work_date: Option<NaiveDate>,
}

#[derive(Debug, FromRow)]
struct SiteHistoryReportRowDb {
    site_id: Uuid,
    project_type: String,
    name: String,
    customer_name: String,
    status: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    estimated_days: Option<i32>,
    budget_amount_cents: Option<i64>,
    billing_reference: Option<String>,
    quote_reference: Option<String>,
    total_hours: f64,
    worker_count: i64,
    distinct_material_count: i64,
    withdrawal_count: i64,
    cost_basis: String,
}

impl DashboardSiteRow {
    fn into_dashboard_site(self) -> DashboardSite {
        DashboardSite {
            id: SiteId(self.id),
            tenant_id: TenantId(self.tenant_id),
            project_type: self
                .project_type
                .parse()
                .unwrap_or(ProjectType::ExternalSite),
            name: self.name,
            customer_name: self.customer_name,
            location: self.location,
            status: self.status,
            start_date: self.start_date,
            end_date: self.end_date,
            estimated_days: self.estimated_days,
            assigned_users: self.assigned_users,
            total_hours: self.total_hours.unwrap_or(0.0),
        }
    }
}

impl ProjectLaborSummaryRow {
    fn into_summary(self) -> ProjectLaborSummary {
        ProjectLaborSummary {
            total_hours: self.total_hours,
            entry_count: self.entry_count,
            site_hours: self.site_hours,
            workshop_hours: self.workshop_hours,
            last_work_date: self.last_work_date,
        }
    }
}

impl SiteHistoryReportRowDb {
    fn into_row(self) -> SiteHistoryReportRow {
        SiteHistoryReportRow {
            site_id: SiteId(self.site_id),
            project_type: self
                .project_type
                .parse()
                .unwrap_or(ProjectType::ExternalSite),
            name: self.name,
            customer_name: self.customer_name,
            status: self.status,
            start_date: self.start_date,
            end_date: self.end_date,
            estimated_days: self.estimated_days,
            budget_amount_cents: self.budget_amount_cents,
            billing_reference: self.billing_reference,
            quote_reference: self.quote_reference,
            total_hours: self.total_hours,
            worker_count: self.worker_count,
            distinct_material_count: self.distinct_material_count,
            withdrawal_count: self.withdrawal_count,
            cost_basis: self.cost_basis,
        }
    }
}
