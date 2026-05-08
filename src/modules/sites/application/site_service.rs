use crate::common::error::AppError;
use crate::common::events::EventType;
use crate::common::types::{ActivityId, Role, SiteId, TimeEntryId, UserId};
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::modules::sites::domain::{
    work_type_requires_project_link, Activity, ActivityAttachmentMetadata, AssignUser,
    CreateActivity, CreateSite, CreateTimeEntry, Site, SiteActivityAttachment, SiteCreatedPayload,
    SiteStatusChangedPayload, TimeEntry, TimeEntryCreatedPayload, UpdateSite, UpdateTimeEntry,
    UserAssignedToSitePayload,
};
use crate::modules::sites::infrastructure::site_repository::{DashboardSite, SiteRepository};
use sqlx::PgPool;
use uuid::Uuid;

const MAX_UPLOAD_SIZE_BYTES: usize = 10 * 1024 * 1024;
const THUMBNAIL_MAX_EDGE: u32 = 320;

pub struct UploadPhotoCommand {
    pub site_id: SiteId,
    pub mime_type: String,
    pub original_bytes: Vec<u8>,
    pub original_filename: String,
}

pub struct UploadPhotoResult {
    pub attachment_id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub photo_url: String,
    pub thumbnail_url: Option<String>,
}

/// Service for site business logic
pub struct SiteService {
    site_repo: SiteRepository,
    pool: PgPool,
}

impl SiteService {
    pub fn new(site_repo: SiteRepository) -> Self {
        let pool = site_repo.pool();
        Self { site_repo, pool }
    }

    pub fn validate_upload_payload(mime_type: &str, byte_len: usize) -> Result<(), AppError> {
        let allowed = ["image/jpeg", "image/png", "image/webp", "application/pdf"];
        if !allowed.contains(&mime_type) {
            return Err(AppError::Validation(
                "Unsupported attachment MIME type".to_string(),
            ));
        }
        if byte_len == 0 {
            return Err(AppError::Validation(
                "Uploaded attachment is empty".to_string(),
            ));
        }
        if byte_len > MAX_UPLOAD_SIZE_BYTES {
            return Err(AppError::Validation(
                "Uploaded attachment exceeds size limit".to_string(),
            ));
        }
        Ok(())
    }

    fn extension_for_mime(mime_type: &str) -> Result<&'static str, AppError> {
        match mime_type {
            "image/jpeg" => Ok("jpg"),
            "image/png" => Ok("png"),
            "image/webp" => Ok("webp"),
            "application/pdf" => Ok("pdf"),
            _ => Err(AppError::Validation(
                "Unsupported attachment MIME type".to_string(),
            )),
        }
    }

    fn should_generate_thumbnail(mime_type: &str) -> bool {
        mime_type.starts_with("image/")
    }

    fn generate_thumbnail_bytes(bytes: &[u8], mime_type: &str) -> Result<Vec<u8>, AppError> {
        let img = image::load_from_memory(bytes)
            .map_err(|e| AppError::Validation(format!("Invalid image payload: {e}")))?;
        let thumb = img.thumbnail(THUMBNAIL_MAX_EDGE, THUMBNAIL_MAX_EDGE);

        let mut out = std::io::Cursor::new(Vec::new());
        let format = match mime_type {
            "image/jpeg" => image::ImageFormat::Jpeg,
            "image/png" => image::ImageFormat::Png,
            "image/webp" => image::ImageFormat::WebP,
            _ => {
                return Err(AppError::Validation(
                    "Unsupported image MIME type".to_string(),
                ))
            }
        };

        thumb
            .write_to(&mut out, format)
            .map_err(|e| AppError::Internal(format!("Thumbnail encoding failed: {e}")))?;
        Ok(out.into_inner())
    }

    fn build_attachment_urls(attachment_id: Uuid, mime_type: &str) -> (String, Option<String>) {
        let thumbnail_url = Self::should_generate_thumbnail(mime_type)
            .then(|| format!("/api/v1/attachments/{attachment_id}/thumbnail"));
        (
            format!("/api/v1/attachments/{attachment_id}"),
            thumbnail_url,
        )
    }

    fn extract_attachment_id(photo_url: &str) -> Result<Uuid, AppError> {
        let attachment_id = photo_url
            .strip_prefix("/api/v1/attachments/")
            .and_then(|value| value.split('/').next())
            .ok_or_else(|| {
                AppError::Internal(
                    "Stored photo URL does not reference a protected attachment".to_string(),
                )
            })?;

        Uuid::parse_str(attachment_id).map_err(|_| {
            AppError::Internal("Stored photo URL contains an invalid attachment id".to_string())
        })
    }

    fn to_attachment_metadata(attachment: SiteActivityAttachment) -> ActivityAttachmentMetadata {
        let (url, thumbnail_url) =
            Self::build_attachment_urls(attachment.id, &attachment.mime_type);
        ActivityAttachmentMetadata {
            id: attachment.id,
            filename: attachment.original_filename,
            mime_type: attachment.mime_type,
            url,
            thumbnail_url,
        }
    }

    fn can_delete_activity(activity: &Activity, requester_id: UserId) -> bool {
        matches!(
            activity.activity_type,
            crate::modules::sites::domain::ActivityType::Note
                | crate::modules::sites::domain::ActivityType::Photo
        ) && activity.user_id == requester_id
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

    fn validate_time_entry_project_link(
        work_type: crate::common::types::WorkType,
        site_id: Option<SiteId>,
    ) -> Result<(), AppError> {
        if work_type_requires_project_link(work_type) && site_id.is_none() {
            return Err(AppError::Validation(
                "Project link is required for productive work".to_string(),
            ));
        }

        Ok(())
    }

    fn resolve_updated_time_entry_state(
        existing: &TimeEntry,
        update: &UpdateTimeEntry,
    ) -> (crate::common::types::WorkType, Option<SiteId>) {
        (
            update.work_type.unwrap_or(existing.work_type),
            update.site_id.unwrap_or(existing.site_id),
        )
    }

    // === Site operations ===

    pub async fn create_site(
        &self,
        create: CreateSite,
        ctx: &TenantContext,
    ) -> Result<Site, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        create.validate()?;

        let site = self.site_repo.create_site(&create, ctx.tenant_id).await?;

        // Emit SiteCreated event
        let event = SiteCreatedPayload {
            site_id: site.id,
            name: site.name.clone(),
            customer_name: site.customer_name.clone(),
        }
        .into_event(ctx.tenant_id);

        self.site_repo.publish_event(&event).await?;

        Ok(site)
    }

    pub async fn update_site(
        &self,
        site_id: SiteId,
        update: UpdateSite,
        ctx: &TenantContext,
    ) -> Result<Site, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        let old_site = self
            .site_repo
            .find_site_by_id(ctx.tenant_id, site_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        let site = self
            .site_repo
            .update_site(ctx.tenant_id, site_id, &update)
            .await?;

        // Emit SiteStatusChanged event if status changed
        if let Some(new_status) = &update.status {
            if old_site.status != *new_status {
                let event = SiteStatusChangedPayload {
                    site_id: site.id,
                    old_status: old_site.status.to_string(),
                    new_status: new_status.to_string(),
                    changed_by: ctx.user_id,
                }
                .into_event(ctx.tenant_id);

                self.site_repo.publish_event(&event).await?;

                // Create activity for status change
                let local_user_id = self.resolve_local_user_id(ctx).await?;
                let activity_content = serde_json::json!({
                    "old_status": old_site.status.to_string(),
                    "new_status": new_status.to_string(),
                })
                .to_string();

                sqlx::query!(
                    r#"
                    INSERT INTO site_activities (id, tenant_id, site_id, user_id, activity_type, content, photo_url, created_at)
                    VALUES ($1, $2, $3, $4, $5, $6, NULL, $7)
                    "#,
                    uuid::Uuid::new_v4(),
                    ctx.tenant_id.0,
                    site.id.0,
                    local_user_id.0,
                    "status_change",
                    activity_content,
                    chrono::Utc::now(),
                )
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            }
        }

        Ok(site)
    }

    pub async fn get_site(&self, site_id: SiteId, ctx: &TenantContext) -> Result<Site, AppError> {
        self.site_repo
            .find_site_by_id(ctx.tenant_id, site_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Site not found".to_string()))
    }

    pub async fn list_sites(
        &self,
        status: Option<String>,
        ctx: &TenantContext,
    ) -> Result<Vec<Site>, AppError> {
        self.site_repo.list_sites(ctx.tenant_id, status).await
    }

    /// Delete a site (soft delete)
    /// Returns Conflict error if there are active reservations
    pub async fn delete_site(&self, site_id: SiteId, ctx: &TenantContext) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // Verify site exists
        self.site_repo
            .find_site_by_id(ctx.tenant_id, site_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        // Check for active reservations
        let active_count = self
            .site_repo
            .count_active_reservations(site_id, ctx.tenant_id)
            .await?;
        if active_count > 0 {
            return Err(AppError::Conflict(format!(
                "Cannot delete: {} active reservation(s) exist",
                active_count
            )));
        }

        // Perform soft delete
        self.site_repo.delete_site(site_id, ctx.tenant_id).await
    }

    // === Assignment operations ===

    pub async fn assign_user(
        &self,
        site_id: SiteId,
        assign: AssignUser,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // Verify site exists
        let _site = self.get_site(site_id, ctx).await?;

        self.site_repo
            .assign_user(ctx.tenant_id, site_id, assign.user_id, assign.role)
            .await?;

        // Emit UserAssignedToSite event
        let event = UserAssignedToSitePayload {
            site_id,
            user_id: assign.user_id,
            role: assign.role.to_string(),
            assigned_by: ctx.user_id,
        }
        .into_event(ctx.tenant_id);

        self.site_repo.publish_event(&event).await?;

        Ok(())
    }

    pub async fn remove_assignment(
        &self,
        site_id: SiteId,
        user_id: UserId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        self.site_repo
            .remove_assignment(ctx.tenant_id, site_id, user_id)
            .await
    }

    pub async fn list_assignments(
        &self,
        site_id: SiteId,
        ctx: &TenantContext,
    ) -> Result<Vec<crate::modules::sites::domain::SiteAssignment>, AppError> {
        // Verify site exists
        let _site = self.get_site(site_id, ctx).await?;

        self.site_repo
            .list_assignments(ctx.tenant_id, site_id)
            .await
    }

    // === Time entry operations ===

    pub async fn create_time_entry(
        &self,
        create: CreateTimeEntry,
        ctx: &TenantContext,
    ) -> Result<TimeEntry, AppError> {
        create.validate()?;
        Self::validate_time_entry_project_link(create.work_type, create.site_id)?;

        // If site_id is provided, verify site exists
        if let Some(site_id) = create.site_id {
            let _site = self.get_site(site_id, ctx).await?;
        }

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        let entry = self
            .site_repo
            .create_time_entry(ctx.tenant_id, local_user_id, &create)
            .await?;

        // Emit TimeEntryCreated event
        let event = TimeEntryCreatedPayload {
            site_id: create.site_id,
            user_id: ctx.user_id,
            hours: create.hours,
            work_type: create.work_type.to_string(),
            work_date: create.work_date.to_string(),
        }
        .into_event(ctx.tenant_id);

        self.site_repo.publish_event(&event).await?;

        Ok(entry)
    }

    pub async fn list_time_entries(
        &self,
        site_id: Option<SiteId>,
        user_id: Option<UserId>,
        ctx: &TenantContext,
    ) -> Result<Vec<TimeEntry>, AppError> {
        // If site_id is provided, verify site exists
        if let Some(site_id) = site_id {
            let _site = self.get_site(site_id, ctx).await?;
        }

        self.site_repo
            .list_time_entries(ctx.tenant_id, site_id, user_id)
            .await
    }

    pub async fn list_my_time_entries(
        &self,
        ctx: &TenantContext,
    ) -> Result<Vec<TimeEntry>, AppError> {
        let local_user_id = self.resolve_local_user_id(ctx).await?;
        self.site_repo
            .list_time_entries(ctx.tenant_id, None, Some(local_user_id))
            .await
    }

    pub async fn get_time_entry(
        &self,
        entry_id: TimeEntryId,
        ctx: &TenantContext,
    ) -> Result<TimeEntry, AppError> {
        self.site_repo
            .find_time_entry_by_id(ctx.tenant_id, entry_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))
    }

    pub async fn update_time_entry(
        &self,
        entry_id: TimeEntryId,
        update: UpdateTimeEntry,
        ctx: &TenantContext,
    ) -> Result<TimeEntry, AppError> {
        // Validate update
        update.validate()?;

        // Fetch existing entry to check ownership
        let existing = self
            .site_repo
            .find_time_entry_by_id(ctx.tenant_id, entry_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))?;

        // Check ownership: only the owner or admin can edit
        let local_user_id = self.resolve_local_user_id(ctx).await?;
        if existing.user_id != local_user_id && !ctx.is_admin() {
            return Err(AppError::Forbidden(
                "Can only edit own time entries".to_string(),
            ));
        }

        let (work_type, site_id) = Self::resolve_updated_time_entry_state(&existing, &update);
        Self::validate_time_entry_project_link(work_type, site_id)?;

        if let Some(site_id) = site_id {
            let _site = self.get_site(site_id, ctx).await?;
        }

        self.site_repo
            .update_time_entry(ctx.tenant_id, entry_id, &update)
            .await
    }

    pub async fn delete_time_entry(
        &self,
        entry_id: TimeEntryId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        // Fetch existing entry to check ownership
        let existing = self
            .site_repo
            .find_time_entry_by_id(ctx.tenant_id, entry_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))?;

        // Check ownership: only the owner or admin can delete
        let local_user_id = self.resolve_local_user_id(ctx).await?;
        if existing.user_id != local_user_id && !ctx.is_admin() {
            return Err(AppError::Forbidden(
                "Can only delete own time entries".to_string(),
            ));
        }

        self.site_repo
            .delete_time_entry(ctx.tenant_id, entry_id)
            .await
    }

    // === Activity operations ===

    pub async fn create_activity(
        &self,
        create: CreateActivity,
        ctx: &TenantContext,
    ) -> Result<Activity, AppError> {
        // Validate the command
        create.validate()?;

        // Verify site exists in same tenant
        let _site = self.get_site(create.site_id, ctx).await?;

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        let activity = self
            .site_repo
            .create_activity(ctx.tenant_id, local_user_id, &create)
            .await?;

        let attachments = if create.attachment_ids.is_empty() {
            Vec::new()
        } else {
            self.site_repo
                .link_activity_attachments(
                    ctx.tenant_id,
                    create.site_id,
                    activity.id,
                    &create.attachment_ids,
                )
                .await?
                .into_iter()
                .map(Self::to_attachment_metadata)
                .collect()
        };

        // Publish ActivityAdded event
        let event = crate::common::events::DomainEvent::new(
            EventType::ActivityAdded,
            ctx.tenant_id,
            "Site",
            activity.site_id.to_string(),
            serde_json::json!({
                "site_id": activity.site_id.to_string(),
                "user_id": activity.user_id.to_string(),
                "activity_type": activity.activity_type.as_str(),
            }),
        );

        self.site_repo.publish_event(&event).await?;

        Ok(Activity {
            attachments,
            can_delete: Self::can_delete_activity(&activity, local_user_id),
            ..activity
        })
    }

    pub async fn list_activities(
        &self,
        site_id: SiteId,
        limit: i32,
        ctx: &TenantContext,
    ) -> Result<Vec<Activity>, AppError> {
        // Verify site exists in same tenant
        let _site = self.get_site(site_id, ctx).await?;

        let local_user_id = self.resolve_local_user_id(ctx).await?;
        let activities = self
            .site_repo
            .list_activities(ctx.tenant_id, site_id, limit)
            .await?;
        let activity_ids: Vec<_> = activities.iter().map(|activity| activity.id).collect();
        let attachments_by_activity = self
            .site_repo
            .list_activity_attachments(ctx.tenant_id, site_id, &activity_ids)
            .await?;

        Ok(activities
            .into_iter()
            .map(|activity| Activity {
                can_delete: Self::can_delete_activity(&activity, local_user_id),
                attachments: attachments_by_activity
                    .get(&activity.id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(Self::to_attachment_metadata)
                    .collect(),
                ..activity
            })
            .collect())
    }

    pub async fn delete_activity(
        &self,
        site_id: SiteId,
        activity_id: ActivityId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        let existing = self
            .site_repo
            .find_activity_by_id(ctx.tenant_id, site_id, activity_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Activity not found".to_string()))?;

        let local_user_id = self.resolve_local_user_id(ctx).await?;
        if !Self::can_delete_activity(&existing, local_user_id) {
            return Err(AppError::Forbidden(
                "Can only delete your own note or photo activities".to_string(),
            ));
        }

        if let Some(photo_url) = existing.photo_url.as_deref() {
            let attachment_id = Self::extract_attachment_id(photo_url)?;
            self.site_repo
                .delete_attachment_by_id(ctx.tenant_id, site_id, attachment_id)
                .await?;
        }

        self.site_repo
            .delete_activity(ctx.tenant_id, site_id, activity_id)
            .await
    }

    pub async fn upload_site_attachment(
        &self,
        cmd: UploadPhotoCommand,
        ctx: &TenantContext,
    ) -> Result<UploadPhotoResult, AppError> {
        let mime_type = cmd.mime_type.clone();
        let original_filename = cmd.original_filename.clone();

        Self::validate_upload_payload(&cmd.mime_type, cmd.original_bytes.len())?;

        let _site = self.get_site(cmd.site_id, ctx).await?;

        let extension = Self::extension_for_mime(&cmd.mime_type)?;
        let original_key = format!("{}.{}", Uuid::new_v4(), extension);
        let thumbnail_key = Self::should_generate_thumbnail(&cmd.mime_type)
            .then(|| format!("{}.{}", Uuid::new_v4(), extension));
        let thumbnail_bytes = if Self::should_generate_thumbnail(&cmd.mime_type) {
            Some(Self::generate_thumbnail_bytes(
                &cmd.original_bytes,
                &cmd.mime_type,
            )?)
        } else {
            None
        };

        let attachment_id = Uuid::new_v4();

        self.site_repo
            .create_activity_attachment(
                ctx.tenant_id,
                &SiteActivityAttachment {
                    id: attachment_id,
                    tenant_id: ctx.tenant_id,
                    activity_id: None,
                    site_id: cmd.site_id,
                    storage_key: original_key,
                    thumbnail_key,
                    original_filename: original_filename.clone(),
                    mime_type: mime_type.clone(),
                    size_bytes: cmd.original_bytes.len() as i64,
                    original_bytes: Some(cmd.original_bytes),
                    thumbnail_bytes,
                    created_at: chrono::Utc::now(),
                },
            )
            .await?;

        let (photo_url, thumbnail_url) = Self::build_attachment_urls(attachment_id, &mime_type);

        Ok(UploadPhotoResult {
            attachment_id,
            filename: original_filename,
            mime_type,
            photo_url,
            thumbnail_url,
        })
    }

    pub async fn upload_photo_attachment(
        &self,
        cmd: UploadPhotoCommand,
        ctx: &TenantContext,
    ) -> Result<UploadPhotoResult, AppError> {
        self.upload_site_attachment(cmd, ctx).await
    }

    // === Dashboard operations ===

    pub async fn get_dashboard(&self, ctx: &TenantContext) -> Result<Vec<DashboardSite>, AppError> {
        self.site_repo.get_dashboard_sites(ctx.tenant_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::SiteService;
    use crate::common::types::{ActivityId, SiteId, TenantId, TimeEntryId, UserId, WorkType};
    use crate::modules::sites::domain::{Activity, ActivityType, TimeEntry, UpdateTimeEntry};
    use chrono::Utc;
    use image::GenericImageView;

    fn existing_time_entry(work_type: WorkType, site_id: Option<SiteId>) -> TimeEntry {
        TimeEntry {
            id: TimeEntryId::new(),
            tenant_id: TenantId::new(),
            site_id,
            user_id: UserId::new(),
            work_type,
            hours: 8.0,
            work_date: chrono::Local::now().date_naive(),
            notes: None,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn validate_time_entry_project_link_requires_project_for_productive_work() {
        let error = SiteService::validate_time_entry_project_link(WorkType::Workshop, None)
            .expect_err("productive workshop time should require a project");

        assert_eq!(
            error.to_string(),
            "Validation error: Project link is required for productive work"
        );
    }

    #[test]
    fn resolve_updated_time_entry_state_can_clear_project_for_overhead_work() {
        let existing = existing_time_entry(WorkType::Site, Some(SiteId::new()));
        let update = UpdateTimeEntry {
            work_type: Some(WorkType::Travel),
            site_id: Some(None),
            ..UpdateTimeEntry::default()
        };

        let (work_type, site_id) =
            SiteService::resolve_updated_time_entry_state(&existing, &update);

        assert_eq!(work_type, WorkType::Travel);
        assert_eq!(site_id, None);
    }

    #[test]
    fn activity_response_can_delete_for_owned_note_and_photo_only() {
        let owner_id = UserId::new();

        let note = sample_activity(ActivityType::Note, owner_id);
        let photo = sample_activity(ActivityType::Photo, owner_id);
        let other_user_note = sample_activity(ActivityType::Note, UserId::new());

        assert!(SiteService::can_delete_activity(&note, owner_id));
        assert!(SiteService::can_delete_activity(&photo, owner_id));
        assert!(!SiteService::can_delete_activity(
            &other_user_note,
            owner_id
        ));
    }

    #[test]
    fn activity_response_can_delete_rejects_status_change_even_for_owner() {
        let owner_id = UserId::new();
        let status_change = sample_activity(ActivityType::StatusChange, owner_id);

        assert!(!SiteService::can_delete_activity(&status_change, owner_id));
    }

    fn sample_activity(activity_type: ActivityType, user_id: UserId) -> Activity {
        Activity {
            id: ActivityId::new(),
            tenant_id: TenantId::new(),
            site_id: SiteId::new(),
            user_id,
            creator_name: "Max Mustermann".to_string(),
            can_delete: false,
            activity_type,
            content: Some("hello".to_string()),
            photo_url: None,
            attachments: Vec::new(),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn upload_validation_rejects_bad_mime_and_oversize() {
        let bad_mime = SiteService::validate_upload_payload("text/plain", 1024);
        assert!(bad_mime.is_err());

        let oversized = SiteService::validate_upload_payload("image/jpeg", 11 * 1024 * 1024);
        assert!(oversized.is_err());
    }

    #[test]
    fn upload_photo_rejects_empty_payload() {
        let empty = SiteService::validate_upload_payload("image/jpeg", 0);
        assert!(empty.is_err());
    }

    #[test]
    fn thumbnail_generation_produces_bounded_image() {
        let img = image::DynamicImage::new_rgb8(1200, 800);
        let mut bytes = std::io::Cursor::new(Vec::new());
        img.write_to(&mut bytes, image::ImageFormat::Png).unwrap();

        let thumb =
            SiteService::generate_thumbnail_bytes(&bytes.into_inner(), "image/png").unwrap();
        let decoded = image::load_from_memory(&thumb).unwrap();
        let (w, h) = decoded.dimensions();

        assert!(w <= 320);
        assert!(h <= 320);
        assert!(!thumb.is_empty());
    }

    #[test]
    fn attachment_urls_use_attachment_id_paths() {
        let id = uuid::Uuid::new_v4();
        let (photo_url, thumb_url) = SiteService::build_attachment_urls(id, "image/jpeg");
        assert_eq!(photo_url, format!("/api/v1/attachments/{id}"));
        assert_eq!(
            thumb_url,
            Some(format!("/api/v1/attachments/{id}/thumbnail"))
        );
    }

    #[test]
    fn upload_photo_urls_do_not_use_pending_marker() {
        let id = uuid::Uuid::new_v4();
        let (photo_url, thumb_url) = SiteService::build_attachment_urls(id, "image/jpeg");
        assert!(!photo_url.contains("pending-upload"));
        assert!(!thumb_url.unwrap_or_default().contains("pending-upload"));
    }

    #[test]
    fn document_attachment_upload_accepts_pdf_payload() {
        let result = SiteService::validate_upload_payload("application/pdf", 1024);
        assert!(result.is_ok());
    }

    #[test]
    fn document_attachment_upload_skips_thumbnail_for_pdf() {
        let id = uuid::Uuid::new_v4();
        let (_url, thumbnail_url) = SiteService::build_attachment_urls(id, "application/pdf");
        assert!(thumbnail_url.is_none());
    }

    #[test]
    fn delete_activity_photo_cleanup_extracts_attachment_id_from_protected_url() {
        let id = uuid::Uuid::new_v4();

        let parsed = SiteService::extract_attachment_id(&format!("/api/v1/attachments/{id}"))
            .expect("parse protected attachment url");

        assert_eq!(parsed, id);
    }
}
