use sqlx::PgPool;

use crate::common::error::AppError;
use crate::common::types::{SiteId, TenantId, UserId, SiteStatus};
use crate::modules::iam::domain::user_preferences::{
    UpdatePreferences, UserPreferenceRecord,
};
use crate::modules::iam::infrastructure::user_preferences_repository::UserPreferencesRepository;
use crate::modules::sites::infrastructure::site_repository::SiteRepository;

/// Service for user preferences operations with validation
pub struct UserPreferencesService {
    preferences_repo: UserPreferencesRepository,
    site_repo: SiteRepository,
}

impl UserPreferencesService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            preferences_repo: UserPreferencesRepository::new(pool.clone()),
            site_repo: SiteRepository::new(pool),
        }
    }

    /// Get user's preferences
    pub async fn get_preferences(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<UserPreferenceRecord, AppError> {
        self.preferences_repo.get_or_create(user_id, tenant_id).await
    }

    /// Set active site with validation
    /// Returns error if site doesn't exist, doesn't belong to tenant, or is archived/deleted
    pub async fn set_active_site(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        site_id: SiteId,
    ) -> Result<UserPreferenceRecord, AppError> {
        // Validate site exists and belongs to tenant
        let site = self
            .site_repo
            .find_site_by_id(tenant_id, site_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Site not found".to_string()))?;

        // Check if site is archived
        if site.status == SiteStatus::Archived {
            return Err(AppError::Validation(
                "Cannot set archived site as active".to_string(),
            ));
        }

        // Update preferences with the site_id
        let update = UpdatePreferences {
            active_site_id: Some(site_id.0.to_string()),
        };

        self.preferences_repo.update(user_id, tenant_id, &update).await
    }

    /// Clear active site
    pub async fn clear_active_site(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<UserPreferenceRecord, AppError> {
        // Clear the active site
        self.preferences_repo
            .clear_active_site(user_id, tenant_id)
            .await?;

        // Return updated preferences
        self.preferences_repo.get_or_create(user_id, tenant_id).await
    }

    /// Validate and auto-clear if active site is invalid
    /// Called when retrieving preferences to ensure validity
    pub async fn get_validated_preferences(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<UserPreferenceRecord, AppError> {
        let prefs = self.preferences_repo.get_or_create(user_id, tenant_id).await?;

        // If no active site, return as-is
        let Some(active_site_id_str) = &prefs.preferences.active_site_id else {
            return Ok(prefs);
        };

        // Parse site ID
        let site_id = SiteId::parse(active_site_id_str)
            .map_err(|_| AppError::Validation("Invalid site ID in preferences".to_string()))?;

        // Validate site still exists and is valid
        match self.site_repo.find_site_by_id(tenant_id, site_id).await? {
            Some(site) if site.status != SiteStatus::Archived => {
                // Site is valid, return preferences
                Ok(prefs)
            }
            Some(_) => {
                // Site is archived, auto-clear
                self.preferences_repo
                    .clear_active_site(user_id, tenant_id)
                    .await?;
                self.preferences_repo.get_or_create(user_id, tenant_id).await
            }
            None => {
                // Site not found (deleted or never existed), auto-clear
                self.preferences_repo
                    .clear_active_site(user_id, tenant_id)
                    .await?;
                self.preferences_repo.get_or_create(user_id, tenant_id).await
            }
        }
    }
}
