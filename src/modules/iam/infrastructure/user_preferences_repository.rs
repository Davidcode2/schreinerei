use chrono::{DateTime, Utc};
use serde_json;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{TenantId, UserId};
use crate::modules::iam::domain::user_preferences::{
    UpdatePreferences, UserPreferenceRecord, UserPreferences,
};

/// Repository for user preferences data access with tenant isolation
pub struct UserPreferencesRepository {
    pool: PgPool,
}

impl UserPreferencesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get or create preferences for a user (returns default if not exists)
    pub async fn get_or_create(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<UserPreferenceRecord, AppError> {
        // Try to insert a new record with default preferences
        let now = Utc::now();
        let id = Uuid::new_v4();
        let default_prefs = UserPreferences::default();
        let prefs_json = serde_json::to_value(&default_prefs)
            .map_err(|e| AppError::Internal(format!("Failed to serialize preferences: {}", e)))?;

        let result = sqlx::query_as::<_, UserPreferenceRow>(
            r#"
            INSERT INTO user_preferences (id, tenant_id, user_id, preferences, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (tenant_id, user_id) DO NOTHING
            RETURNING id, tenant_id, user_id, preferences, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(user_id.0)
        .bind(&prefs_json)
        .bind(now)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        // If we got a row back, it was inserted
        if let Some(row) = result {
            return Ok(row.into_record());
        }

        // Otherwise, the record already existed - fetch it
        let existing = sqlx::query_as::<_, UserPreferenceRow>(
            r#"
            SELECT id, tenant_id, user_id, preferences, created_at, updated_at
            FROM user_preferences
            WHERE tenant_id = $1 AND user_id = $2
            "#,
        )
        .bind(tenant_id.0)
        .bind(user_id.0)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(existing.into_record())
    }

    /// Update preferences for a user
    pub async fn update(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        update: &UpdatePreferences,
    ) -> Result<UserPreferenceRecord, AppError> {
        let update_json = serde_json::to_value(update)
            .map_err(|e| AppError::Internal(format!("Failed to serialize update: {}", e)))?;

        let record = sqlx::query_as::<_, UserPreferenceRow>(
            r#"
            INSERT INTO user_preferences (id, tenant_id, user_id, preferences, created_at, updated_at)
            VALUES (uuid_generate_v4(), $1, $2, $3::jsonb, NOW(), NOW())
            ON CONFLICT (tenant_id, user_id)
            DO UPDATE SET
                preferences = user_preferences.preferences || $3::jsonb,
                updated_at = NOW()
            RETURNING id, tenant_id, user_id, preferences, created_at, updated_at
            "#,
        )
        .bind(tenant_id.0)
        .bind(user_id.0)
        .bind(&update_json)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(record.into_record())
    }

    /// Clear active site for a user (convenience method)
    pub async fn clear_active_site(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<(), AppError> {
        let clear_json = serde_json::json!({"active_site_id": null});

        sqlx::query(
            r#"
            INSERT INTO user_preferences (id, tenant_id, user_id, preferences, created_at, updated_at)
            VALUES (uuid_generate_v4(), $1, $2, $3::jsonb, NOW(), NOW())
            ON CONFLICT (tenant_id, user_id)
            DO UPDATE SET
                preferences = user_preferences.preferences || $3::jsonb,
                updated_at = NOW()
            "#,
        )
        .bind(tenant_id.0)
        .bind(user_id.0)
        .bind(&clear_json)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}

/// Database row representation for SQLx
#[derive(Debug, FromRow)]
struct UserPreferenceRow {
    id: Uuid,
    tenant_id: Uuid,
    user_id: Uuid,
    preferences: serde_json::Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserPreferenceRow {
    fn into_record(self) -> UserPreferenceRecord {
        let preferences: UserPreferences = serde_json::from_value(self.preferences)
            .unwrap_or_else(|_| UserPreferences::default());

        UserPreferenceRecord {
            id: self.id,
            tenant_id: TenantId(self.tenant_id),
            user_id: UserId(self.user_id),
            preferences,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
