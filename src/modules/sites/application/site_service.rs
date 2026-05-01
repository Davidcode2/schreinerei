use crate::common::error::AppError;
use crate::common::types::{SiteId, UserId, Role, TimeEntryId};
use crate::common::events::EventType;
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::modules::sites::domain::{
    Site, TimeEntry, Activity, CreateSite, UpdateSite, CreateTimeEntry, UpdateTimeEntry, AssignUser, CreateActivity,
    SiteCreatedPayload, SiteStatusChangedPayload, UserAssignedToSitePayload, TimeEntryCreatedPayload,
    SiteActivityAttachment,
};
use crate::modules::sites::infrastructure::site_repository::{SiteRepository, DashboardSite};
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
        let allowed = ["image/jpeg", "image/png", "image/webp"];
        if !allowed.contains(&mime_type) {
            return Err(AppError::Validation("Unsupported image MIME type".to_string()));
        }
        if byte_len == 0 {
            return Err(AppError::Validation("Uploaded image is empty".to_string()));
        }
        if byte_len > MAX_UPLOAD_SIZE_BYTES {
            return Err(AppError::Validation("Uploaded image exceeds size limit".to_string()));
        }
        Ok(())
    }

    fn extension_for_mime(mime_type: &str) -> Result<&'static str, AppError> {
        match mime_type {
            "image/jpeg" => Ok("jpg"),
            "image/png" => Ok("png"),
            "image/webp" => Ok("webp"),
            _ => Err(AppError::Validation("Unsupported image MIME type".to_string())),
        }
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
            _ => return Err(AppError::Validation("Unsupported image MIME type".to_string())),
        };

        thumb
            .write_to(&mut out, format)
            .map_err(|e| AppError::Internal(format!("Thumbnail encoding failed: {e}")))?;
        Ok(out.into_inner())
    }

    fn build_attachment_urls(attachment_id: Uuid) -> (String, Option<String>) {
        (
            format!("/api/v1/attachments/{attachment_id}"),
            Some(format!("/api/v1/attachments/{attachment_id}/thumbnail")),
        )
    }

    async fn resolve_local_user_id(&self, ctx: &TenantContext) -> Result<UserId, AppError> {
        let user_repo = UserRepository::new(self.pool.clone());
        let user = user_repo
            .find_or_create_by_keycloak_id(
                &ctx.user_id.to_string(),
                ctx.tenant_id,
                &ctx.email,
                if ctx.is_admin() { Role::Admin } else { Role::Employee },
            )
            .await?;
        Ok(user.id)
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
        }.into_event(ctx.tenant_id);

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

        let old_site = self.site_repo.find_site_by_id(ctx.tenant_id, site_id).await?
            .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        let site = self.site_repo.update_site(ctx.tenant_id, site_id, &update).await?;

        // Emit SiteStatusChanged event if status changed
        if let Some(new_status) = &update.status {
            if old_site.status != *new_status {
                let event = SiteStatusChangedPayload {
                    site_id: site.id,
                    old_status: old_site.status.to_string(),
                    new_status: new_status.to_string(),
                    changed_by: ctx.user_id,
                }.into_event(ctx.tenant_id);

                self.site_repo.publish_event(&event).await?;

                // Create activity for status change
                let local_user_id = self.resolve_local_user_id(ctx).await?;
                let activity_content = serde_json::json!({
                    "old_status": old_site.status.to_string(),
                    "new_status": new_status.to_string(),
                }).to_string();

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

    pub async fn get_site(
        &self,
        site_id: SiteId,
        ctx: &TenantContext,
    ) -> Result<Site, AppError> {
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
    pub async fn delete_site(
        &self,
        site_id: SiteId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // Verify site exists
        self.site_repo.find_site_by_id(ctx.tenant_id, site_id).await?
            .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        // Check for active reservations
        let active_count = self.site_repo.count_active_reservations(site_id, ctx.tenant_id).await?;
        if active_count > 0 {
            return Err(AppError::Conflict(
                format!("Cannot delete: {} active reservation(s) exist", active_count)
            ));
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
        }.into_event(ctx.tenant_id);

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

        self.site_repo.remove_assignment(ctx.tenant_id, site_id, user_id).await
    }

    pub async fn list_assignments(
        &self,
        site_id: SiteId,
        ctx: &TenantContext,
    ) -> Result<Vec<crate::modules::sites::domain::SiteAssignment>, AppError> {
        // Verify site exists
        let _site = self.get_site(site_id, ctx).await?;

        self.site_repo.list_assignments(ctx.tenant_id, site_id).await
    }

    // === Time entry operations ===

    pub async fn create_time_entry(
        &self,
        create: CreateTimeEntry,
        ctx: &TenantContext,
    ) -> Result<TimeEntry, AppError> {
        create.validate()?;

        // If site_id is provided, verify site exists
        if let Some(site_id) = create.site_id {
            let _site = self.get_site(site_id, ctx).await?;
        }

        let local_user_id = self.resolve_local_user_id(ctx).await?;

        let entry = self.site_repo
            .create_time_entry(ctx.tenant_id, local_user_id, &create)
            .await?;

        // Emit TimeEntryCreated event
        let event = TimeEntryCreatedPayload {
            site_id: create.site_id,
            user_id: ctx.user_id,
            hours: create.hours,
            work_type: create.work_type.to_string(),
            work_date: create.work_date.to_string(),
        }.into_event(ctx.tenant_id);

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

        self.site_repo.list_time_entries(ctx.tenant_id, site_id, user_id).await
    }

    pub async fn list_my_time_entries(
        &self,
        ctx: &TenantContext,
    ) -> Result<Vec<TimeEntry>, AppError> {
        let local_user_id = self.resolve_local_user_id(ctx).await?;
        self.site_repo.list_time_entries(ctx.tenant_id, None, Some(local_user_id)).await
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
        let existing = self.site_repo
            .find_time_entry_by_id(ctx.tenant_id, entry_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))?;

        // Check ownership: only the owner or admin can edit
        let local_user_id = self.resolve_local_user_id(ctx).await?;
        if existing.user_id != local_user_id && !ctx.is_admin() {
            return Err(AppError::Forbidden("Can only edit own time entries".to_string()));
        }

        // If site_id is being set (not None), verify site exists
        if let Some(Some(site_id)) = update.site_id {
            let _site = self.get_site(site_id, ctx).await?;
        }

        self.site_repo.update_time_entry(ctx.tenant_id, entry_id, &update).await
    }

    pub async fn delete_time_entry(
        &self,
        entry_id: TimeEntryId,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        // Fetch existing entry to check ownership
        let existing = self.site_repo
            .find_time_entry_by_id(ctx.tenant_id, entry_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Time entry not found".to_string()))?;

        // Check ownership: only the owner or admin can delete
        let local_user_id = self.resolve_local_user_id(ctx).await?;
        if existing.user_id != local_user_id && !ctx.is_admin() {
            return Err(AppError::Forbidden("Can only delete own time entries".to_string()));
        }

        self.site_repo.delete_time_entry(ctx.tenant_id, entry_id).await
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

        let activity = self.site_repo
            .create_activity(ctx.tenant_id, local_user_id, &create)
            .await?;

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

        Ok(activity)
    }

    pub async fn list_activities(
        &self,
        site_id: SiteId,
        limit: i32,
        ctx: &TenantContext,
    ) -> Result<Vec<Activity>, AppError> {
        // Verify site exists in same tenant
        let _site = self.get_site(site_id, ctx).await?;

        self.site_repo.list_activities(ctx.tenant_id, site_id, limit).await
    }

    pub async fn upload_photo_attachment(
        &self,
        cmd: UploadPhotoCommand,
        ctx: &TenantContext,
    ) -> Result<UploadPhotoResult, AppError> {
        Self::validate_upload_payload(&cmd.mime_type, cmd.original_bytes.len())?;

        let _site = self.get_site(cmd.site_id, ctx).await?;

        let extension = Self::extension_for_mime(&cmd.mime_type)?;
        let original_key = format!("{}.{}", Uuid::new_v4(), extension);
        let thumbnail_key = format!("{}.{}", Uuid::new_v4(), extension);
        let thumbnail_bytes = Self::generate_thumbnail_bytes(&cmd.original_bytes, &cmd.mime_type)?;

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
                    thumbnail_key: Some(thumbnail_key),
                    original_filename: cmd.original_filename,
                    mime_type: cmd.mime_type,
                    size_bytes: cmd.original_bytes.len() as i64,
                    original_bytes: Some(cmd.original_bytes),
                    thumbnail_bytes: Some(thumbnail_bytes),
                    created_at: chrono::Utc::now(),
                },
            )
            .await?;

        let (photo_url, thumbnail_url) = Self::build_attachment_urls(attachment_id);

        Ok(UploadPhotoResult {
            attachment_id,
            photo_url,
            thumbnail_url,
        })
    }

    // === Dashboard operations ===

    pub async fn get_dashboard(
        &self,
        ctx: &TenantContext,
    ) -> Result<Vec<DashboardSite>, AppError> {
        self.site_repo.get_dashboard_sites(ctx.tenant_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::SiteService;
    use image::GenericImageView;

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

        let thumb = SiteService::generate_thumbnail_bytes(&bytes.into_inner(), "image/png").unwrap();
        let decoded = image::load_from_memory(&thumb).unwrap();
        let (w, h) = decoded.dimensions();

        assert!(w <= 320);
        assert!(h <= 320);
        assert!(!thumb.is_empty());
    }

    #[test]
    fn attachment_urls_use_attachment_id_paths() {
        let id = uuid::Uuid::new_v4();
        let (photo_url, thumb_url) = SiteService::build_attachment_urls(id);
        assert_eq!(photo_url, format!("/api/v1/attachments/{id}"));
        assert_eq!(thumb_url, Some(format!("/api/v1/attachments/{id}/thumbnail")));
    }

    #[test]
    fn upload_photo_urls_do_not_use_pending_marker() {
        let id = uuid::Uuid::new_v4();
        let (photo_url, thumb_url) = SiteService::build_attachment_urls(id);
        assert!(!photo_url.contains("pending-upload"));
        assert!(!thumb_url.unwrap_or_default().contains("pending-upload"));
    }
}
