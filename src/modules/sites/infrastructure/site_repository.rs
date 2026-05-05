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
            INSERT INTO sites (id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
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
            SELECT id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
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
                    SELECT id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
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
                    SELECT id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
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
                updated_at = NOW()
            WHERE id = $10 AND tenant_id = $11
            RETURNING id, tenant_id, project_type, name, customer_name, location, description, status, start_date, end_date, estimated_days, created_at, updated_at
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
        // Build dynamic update query based on which fields are provided
        // For notes: we need to handle Option<Option<String>> where:
        // - None = don't update notes field
        // - Some(None) = set notes to null
        // - Some(Some(value)) = set notes to value
        let notes_update = update.notes.clone();
        let should_update_notes = notes_update.is_some();
        let notes_value = notes_update.flatten();

        let entry = sqlx::query_as::<_, TimeEntryRow>(
            r#"
            UPDATE time_entries
            SET 
                site_id = COALESCE($1, site_id),
                work_type = COALESCE($2, work_type),
                hours = COALESCE($3, hours),
                work_date = COALESCE($4, work_date),
                notes = CASE 
                    WHEN $5::boolean THEN $6 
                    ELSE notes 
                END
            WHERE id = $7 AND tenant_id = $8
            RETURNING id, tenant_id, site_id, user_id, work_type, hours, work_date, notes, created_at
            "#
        )
        .bind(update.site_id.as_ref().and_then(|opt| opt.as_ref().map(|s| s.0)))  // Flatten Option<Option<SiteId>> to Option<Uuid>
        .bind(update.work_type.as_ref().map(|wt| wt.as_str()))
        .bind(update.hours)
        .bind(update.work_date)
        .bind(should_update_notes)  // Flag to indicate if notes should be updated
        .bind(notes_value)  // The actual notes value (None = null, Some(value) = value)
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
            WHERE s.tenant_id = $1 AND s.status IN ('planned', 'active')
            GROUP BY s.id
            ORDER BY 
                CASE s.status 
                    WHEN 'active' THEN 1 
                    WHEN 'planned' THEN 2 
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
