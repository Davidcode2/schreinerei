use crate::common::error::AppError;
use crate::common::types::{SiteId, UserId, Role};
use crate::common::events::EventType;
use crate::modules::iam::application::user_service::TenantContext;
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::modules::sites::domain::{
    Site, TimeEntry, Activity, CreateSite, UpdateSite, CreateTimeEntry, AssignUser, CreateActivity,
    SiteCreatedPayload, SiteStatusChangedPayload, UserAssignedToSitePayload, TimeEntryCreatedPayload,
};
use crate::modules::sites::infrastructure::site_repository::{SiteRepository, DashboardSite};
use sqlx::PgPool;

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

    // === Dashboard operations ===

    pub async fn get_dashboard(
        &self,
        ctx: &TenantContext,
    ) -> Result<Vec<DashboardSite>, AppError> {
        self.site_repo.get_dashboard_sites(ctx.tenant_id).await
    }
}
