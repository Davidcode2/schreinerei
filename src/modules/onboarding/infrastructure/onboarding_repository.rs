use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::str::FromStr;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::TenantId;
use crate::modules::onboarding::domain::{
    CreateOnboardingSession, InviteStatus, OnboardingSession, OnboardingStatus, OrganizationInvite,
};

#[derive(Clone)]
pub struct OnboardingRepository {
    pool: PgPool,
}

impl OnboardingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_session_by_slug(
        &self,
        organization_slug: &str,
    ) -> Result<Option<OnboardingSession>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, organization_slug, admin_email, selected_plan, status, payment_provider, payment_id, checkout_url
            FROM onboarding_sessions
            WHERE organization_slug = $1
            "#,
        )
        .bind(organization_slug)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;

        row.map(session_from_row).transpose()
    }

    pub async fn insert_session(
        &self,
        command: &CreateOnboardingSession,
        organization_slug: &str,
    ) -> Result<OnboardingSession, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO onboarding_sessions (
                organization_name,
                organization_slug,
                admin_email,
                admin_name,
                selected_plan,
                status
            )
            VALUES ($1, $2, $3, $4, $5, 'pending_payment')
            RETURNING id, organization_slug, admin_email, selected_plan, status, payment_provider, payment_id, checkout_url
            "#,
        )
        .bind(command.normalized_organization_name())
        .bind(organization_slug)
        .bind(command.normalized_admin_email())
        .bind(command.normalized_admin_name())
        .bind(command.normalized_selected_plan())
        .fetch_one(&self.pool)
        .await
        .map_err(database_error)?;

        session_from_row(row)
    }

    pub async fn update_session_checkout(
        &self,
        session_id: Uuid,
        provider: &str,
        payment_id: &str,
        checkout_url: &str,
    ) -> Result<OnboardingSession, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE onboarding_sessions
            SET payment_provider = $2,
                payment_id = $3,
                checkout_url = $4,
                status = 'pending_payment'
            WHERE id = $1
              AND tenant_id IS NULL
              AND status = 'pending_payment'
            RETURNING id, organization_slug, admin_email, selected_plan, status, payment_provider, payment_id, checkout_url
            "#,
        )
        .bind(session_id)
        .bind(provider)
        .bind(payment_id)
        .bind(checkout_url)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?
        .ok_or_else(|| AppError::Conflict("Onboarding session cannot be updated".to_string()))?;

        session_from_row(row)
    }

    pub async fn record_payment_event(
        &self,
        provider: &str,
        provider_event_id: &str,
        payment_id: &str,
        raw_payload: &Value,
        payment_status: Option<&str>,
    ) -> Result<bool, AppError> {
        let inserted_id: Option<uuid::Uuid> = sqlx::query_scalar(
            r#"
            INSERT INTO payment_events (
                provider,
                provider_event_id,
                payment_id,
                raw_payload,
                payment_status
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (provider, provider_event_id) DO NOTHING
            RETURNING id
            "#,
        )
        .bind(provider)
        .bind(provider_event_id)
        .bind(payment_id)
        .bind(raw_payload)
        .bind(payment_status)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;

        Ok(inserted_id.is_some())
    }

    pub async fn update_session_payment_status(
        &self,
        provider: &str,
        payment_id: &str,
        update: SessionPaymentUpdate,
    ) -> Result<Option<String>, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE onboarding_sessions
            SET
                status = $3,
                error_code = $4,
                error_message = $5,
                completed_at = CASE
                    WHEN $3 = 'payment_confirmed' AND completed_at IS NULL THEN NOW()
                    ELSE completed_at
                END
            WHERE payment_provider = $1
              AND payment_id = $2
              AND tenant_id IS NULL
              AND status IN ('pending_payment', 'payment_failed', 'payment_confirmed')
            RETURNING status
            "#,
        )
        .bind(provider)
        .bind(payment_id)
        .bind(update.status.as_str())
        .bind(update.error_code.as_deref())
        .bind(update.error_message.as_deref())
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;

        if let Some(row) = row {
            return Ok(Some(row.try_get("status").map_err(database_error)?));
        }

        self.find_session_status_by_payment(provider, payment_id)
            .await
    }

    pub async fn find_session_status_by_payment(
        &self,
        provider: &str,
        payment_id: &str,
    ) -> Result<Option<String>, AppError> {
        sqlx::query_scalar(
            r#"
            SELECT status
            FROM onboarding_sessions
            WHERE payment_provider = $1
              AND payment_id = $2
            "#,
        )
        .bind(provider)
        .bind(payment_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)
    }

    pub async fn claim_session_for_provisioning(
        &self,
        provider: &str,
        payment_id: &str,
    ) -> Result<Option<ProvisioningSession>, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE onboarding_sessions
            SET status = 'provisioning',
                error_code = NULL,
                error_message = NULL
            WHERE payment_provider = $1
              AND payment_id = $2
              AND status IN ('payment_confirmed', 'keycloak_failed')
            RETURNING id, organization_name, organization_slug, admin_email, admin_name,
                      tenant_id, keycloak_organization_id, keycloak_organization_alias
            "#,
        )
        .bind(provider)
        .bind(payment_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;

        row.map(provisioning_session_from_row).transpose()
    }

    pub async fn ensure_tenant_for_session(
        &self,
        session_id: Uuid,
        keycloak_realm: &str,
        organization_name: &str,
        organization_slug: &str,
    ) -> Result<TenantId, AppError> {
        let mut transaction = self.pool.begin().await.map_err(database_error)?;

        let row = sqlx::query(
            r#"
            SELECT tenant_id
            FROM onboarding_sessions
            WHERE id = $1
            FOR UPDATE
            "#,
        )
        .bind(session_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(database_error)?
        .ok_or_else(|| AppError::NotFound("Onboarding session not found".to_string()))?;

        let tenant_id: Option<Uuid> = row.try_get("tenant_id").map_err(database_error)?;
        if let Some(tenant_id) = tenant_id {
            transaction.commit().await.map_err(database_error)?;
            return Ok(TenantId(tenant_id));
        }

        let tenant_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO tenants (keycloak_realm, name, slug)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(keycloak_realm)
        .bind(organization_name)
        .bind(organization_slug)
        .fetch_one(&mut *transaction)
        .await
        .map_err(database_error)?;

        sqlx::query(
            r#"
            UPDATE onboarding_sessions
            SET tenant_id = $2
            WHERE id = $1
            "#,
        )
        .bind(session_id)
        .bind(tenant_id)
        .execute(&mut *transaction)
        .await
        .map_err(database_error)?;

        transaction.commit().await.map_err(database_error)?;
        Ok(TenantId(tenant_id))
    }

    pub async fn save_keycloak_organization(
        &self,
        session_id: Uuid,
        tenant_id: TenantId,
        organization_id: &str,
        organization_alias: &str,
    ) -> Result<(), AppError> {
        let organization_uuid = Uuid::parse_str(organization_id).map_err(|_| {
            AppError::Validation("Keycloak organization ID must be a UUID".to_string())
        })?;
        let mut transaction = self.pool.begin().await.map_err(database_error)?;

        sqlx::query(
            r#"
            UPDATE tenants
            SET keycloak_organization_id = $2,
                keycloak_organization_alias = $3
            WHERE id = $1
            "#,
        )
        .bind(tenant_id.0)
        .bind(organization_uuid)
        .bind(organization_alias)
        .execute(&mut *transaction)
        .await
        .map_err(database_error)?;

        sqlx::query(
            r#"
            UPDATE onboarding_sessions
            SET keycloak_organization_id = $2,
                keycloak_organization_alias = $3
            WHERE id = $1
            "#,
        )
        .bind(session_id)
        .bind(organization_id)
        .bind(organization_alias)
        .execute(&mut *transaction)
        .await
        .map_err(database_error)?;

        transaction.commit().await.map_err(database_error)?;
        Ok(())
    }

    pub async fn complete_provisioning(&self, session_id: Uuid) -> Result<String, AppError> {
        sqlx::query_scalar(
            r#"
            UPDATE onboarding_sessions
            SET status = 'completed',
                error_code = NULL,
                error_message = NULL,
                completed_at = COALESCE(completed_at, NOW())
            WHERE id = $1
              AND tenant_id IS NOT NULL
              AND keycloak_organization_id IS NOT NULL
              AND keycloak_organization_alias IS NOT NULL
            RETURNING status
            "#,
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?
        .ok_or_else(|| AppError::Conflict("Onboarding session cannot be completed".to_string()))
    }

    pub async fn mark_keycloak_failed(
        &self,
        session_id: Uuid,
        error_code: &str,
        error_message: &str,
    ) -> Result<String, AppError> {
        sqlx::query_scalar(
            r#"
            UPDATE onboarding_sessions
            SET status = 'keycloak_failed',
                error_code = $2,
                error_message = $3
            WHERE id = $1
            RETURNING status
            "#,
        )
        .bind(session_id)
        .bind(error_code)
        .bind(error_message)
        .fetch_one(&self.pool)
        .await
        .map_err(database_error)
    }

    pub async fn find_tenant_invite_context(
        &self,
        tenant_id: TenantId,
    ) -> Result<TenantInviteContext, AppError> {
        let row = sqlx::query(
            r#"
            SELECT keycloak_organization_id::text AS keycloak_organization_id,
                   keycloak_organization_alias
            FROM tenants
            WHERE id = $1
            "#,
        )
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?
        .ok_or_else(|| AppError::NotFound("Tenant not found".to_string()))?;

        let keycloak_organization_id: Option<String> = row
            .try_get("keycloak_organization_id")
            .map_err(database_error)?;
        let keycloak_organization_alias: Option<String> = row
            .try_get("keycloak_organization_alias")
            .map_err(database_error)?;

        Ok(TenantInviteContext {
            keycloak_organization_id: keycloak_organization_id.ok_or_else(|| {
                AppError::Validation("Tenant is missing Keycloak organization ID".to_string())
            })?,
            keycloak_organization_alias: keycloak_organization_alias.ok_or_else(|| {
                AppError::Validation("Tenant is missing Keycloak organization alias".to_string())
            })?,
        })
    }

    pub async fn upsert_pending_invite(
        &self,
        tenant_id: TenantId,
        email: &str,
        role: &str,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<OrganizationInvite, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO organization_invites (
                tenant_id,
                email,
                role,
                token,
                expires_at
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id, lower(email)) WHERE status = 'pending'
            DO UPDATE SET
                role = EXCLUDED.role,
                token = EXCLUDED.token,
                keycloak_invitation_id = NULL,
                expires_at = EXCLUDED.expires_at,
                updated_at = NOW()
            RETURNING id, tenant_id, email, role, token, status, expires_at, created_at
            "#,
        )
        .bind(tenant_id.0)
        .bind(email)
        .bind(role)
        .bind(token)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
        .map_err(database_error)?;

        organization_invite_from_row(row)
    }

    pub async fn mark_invite_keycloak_sent(
        &self,
        invite_id: Uuid,
        keycloak_invitation_id: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE organization_invites
            SET keycloak_invitation_id = $2
            WHERE id = $1
            "#,
        )
        .bind(invite_id)
        .bind(keycloak_invitation_id)
        .execute(&self.pool)
        .await
        .map_err(database_error)?;

        Ok(())
    }

    pub async fn mark_invite_failed(&self, invite_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE organization_invites
            SET status = 'failed'
            WHERE id = $1
            "#,
        )
        .bind(invite_id)
        .execute(&self.pool)
        .await
        .map_err(database_error)?;

        Ok(())
    }

    pub async fn find_invite_by_token(
        &self,
        token: &str,
    ) -> Result<Option<OrganizationInvite>, AppError> {
        self.expire_invite_by_token(token).await?;

        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, email, role, token, status, expires_at, created_at
            FROM organization_invites
            WHERE token = $1
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(database_error)?;

        row.map(organization_invite_from_row).transpose()
    }

    async fn expire_invite_by_token(&self, token: &str) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE organization_invites
            SET status = 'expired'
            WHERE token = $1
              AND status = 'pending'
              AND expires_at <= NOW()
            "#,
        )
        .bind(token)
        .execute(&self.pool)
        .await
        .map_err(database_error)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TenantInviteContext {
    pub keycloak_organization_id: String,
    pub keycloak_organization_alias: String,
}

#[derive(Debug, Clone)]
pub struct ProvisioningSession {
    pub id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub admin_email: String,
    pub admin_name: Option<String>,
    pub tenant_id: Option<TenantId>,
    pub keycloak_organization_id: Option<String>,
    pub keycloak_organization_alias: Option<String>,
}

fn session_from_row(row: sqlx::postgres::PgRow) -> Result<OnboardingSession, AppError> {
    let status: String = row.try_get("status").map_err(database_error)?;
    let status = OnboardingStatus::from_str(&status).map_err(AppError::Database)?;

    Ok(OnboardingSession {
        id: row.try_get("id").map_err(database_error)?,
        organization_slug: row.try_get("organization_slug").map_err(database_error)?,
        admin_email: row.try_get("admin_email").map_err(database_error)?,
        selected_plan: row.try_get("selected_plan").map_err(database_error)?,
        status,
        payment_provider: row.try_get("payment_provider").map_err(database_error)?,
        payment_id: row.try_get("payment_id").map_err(database_error)?,
        checkout_url: row.try_get("checkout_url").map_err(database_error)?,
    })
}

pub struct SessionPaymentUpdate {
    status: OnboardingStatus,
    error_code: Option<String>,
    error_message: Option<String>,
}

impl SessionPaymentUpdate {
    pub fn confirmed() -> Self {
        Self {
            status: OnboardingStatus::PaymentConfirmed,
            error_code: None,
            error_message: None,
        }
    }

    pub fn failed(provider_status: &str) -> Self {
        Self {
            status: OnboardingStatus::PaymentFailed,
            error_code: Some("payment_not_confirmed".to_string()),
            error_message: Some(format!("Mollie payment status: {provider_status}")),
        }
    }
}

fn database_error(error: sqlx::Error) -> AppError {
    AppError::Database(format!("Onboarding database error: {error}"))
}

fn organization_invite_from_row(
    row: sqlx::postgres::PgRow,
) -> Result<OrganizationInvite, AppError> {
    let status: String = row.try_get("status").map_err(database_error)?;

    Ok(OrganizationInvite {
        id: row.try_get("id").map_err(database_error)?,
        tenant_id: row.try_get("tenant_id").map_err(database_error)?,
        email: row.try_get("email").map_err(database_error)?,
        role: row.try_get("role").map_err(database_error)?,
        token: row.try_get("token").map_err(database_error)?,
        status: InviteStatus::from_str(&status).map_err(AppError::Database)?,
        expires_at: row.try_get("expires_at").map_err(database_error)?,
        created_at: row.try_get("created_at").map_err(database_error)?,
    })
}

fn provisioning_session_from_row(
    row: sqlx::postgres::PgRow,
) -> Result<ProvisioningSession, AppError> {
    let tenant_id: Option<Uuid> = row.try_get("tenant_id").map_err(database_error)?;

    Ok(ProvisioningSession {
        id: row.try_get("id").map_err(database_error)?,
        organization_name: row.try_get("organization_name").map_err(database_error)?,
        organization_slug: row.try_get("organization_slug").map_err(database_error)?,
        admin_email: row.try_get("admin_email").map_err(database_error)?,
        admin_name: row.try_get("admin_name").map_err(database_error)?,
        tenant_id: tenant_id.map(TenantId),
        keycloak_organization_id: row
            .try_get("keycloak_organization_id")
            .map_err(database_error)?,
        keycloak_organization_alias: row
            .try_get("keycloak_organization_alias")
            .map_err(database_error)?,
    })
}
