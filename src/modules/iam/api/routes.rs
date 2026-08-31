use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use ts_rs::TS;
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{Role, SiteId, UserId};
use crate::modules::iam::application::test_data_service::{TestDataService, TestDataStatus};
use crate::modules::iam::application::user_preferences_service::UserPreferencesService;
use crate::modules::iam::application::user_service::{TenantContext, UserService};
use crate::modules::iam::domain::user::UpdateProfile;
use crate::modules::iam::domain::user_preferences::UserPreferenceRecord;
use crate::modules::iam::infrastructure::test_data_repository::TestDataRepository;
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::modules::onboarding::application::OrganizationInviteService;
use crate::modules::onboarding::infrastructure::keycloak_admin_client::KeycloakAdminClient;
use crate::modules::onboarding::infrastructure::onboarding_repository::OnboardingRepository;
use crate::AppState;

/// Create the IAM API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Current user endpoints (any authenticated user)
        .route("/api/v1/auth/me", get(get_current_user))
        .route("/api/v1/users/me", patch(update_own_profile))
        .route(
            "/api/v1/settings/billing",
            get(get_billing_settings).patch(update_billing_settings),
        )
        .route(
            "/api/v1/settings/test-data",
            get(get_test_data_status)
                .post(install_test_data)
                .delete(remove_test_data),
        )
        // Preferences endpoints
        .route(
            "/api/v1/preferences",
            get(get_preferences).patch(update_preferences),
        )
        // User management endpoints (admin only)
        .route("/api/v1/users", get(list_users))
        .route("/api/v1/users/invites", get(list_pending_invites))
        .route("/api/v1/users/invite", post(invite_user))
        .route("/api/v1/users/{id}/role", patch(update_user_role))
        .route("/api/v1/users/{id}", get(get_user).delete(delete_user))
}

fn user_service(state: &AppState) -> UserService {
    let repository = UserRepository::new(state.pool.clone());
    match KeycloakAdminClient::from_config(&state.config) {
        Ok(client) => UserService::new_with_keycloak_client(repository, Arc::new(client)),
        Err(_) => UserService::new(repository),
    }
}

/// Response DTO for user data
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub is_original_admin: bool,
    pub can_manage: bool,
    pub created_at: String,
}

impl UserResponse {
    pub fn from_user(user: crate::modules::iam::domain::user::User, actor_subject: &str) -> Self {
        let can_manage = user.can_be_managed_by(actor_subject);
        Self {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            role: user.role.to_string(),
            is_original_admin: user.is_original_admin,
            can_manage,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

/// Request DTO for inviting a user
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct InviteUserRequest {
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct InviteUserResponse {
    pub id: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub invite_url: String,
    pub organization_alias: String,
    pub expires_at: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct PendingInviteResponse {
    pub id: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub expires_at: String,
    pub created_at: String,
}

/// Request DTO for updating role
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdateRoleRequest {
    pub role: String,
}

/// Request DTO for updating profile
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
}

/// Response DTO for user preferences
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct PreferencesResponse {
    pub active_site_id: Option<String>,
}

impl From<UserPreferenceRecord> for PreferencesResponse {
    fn from(record: UserPreferenceRecord) -> Self {
        Self {
            active_site_id: record.preferences.active_site_id,
        }
    }
}

/// Request DTO for updating preferences
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdatePreferencesRequest {
    pub active_site_id: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct BillingSettingsResponse {
    pub default_hourly_rate_cents: Option<i64>,
    pub billing_tax_mode: String,
    pub sender_name: String,
    pub sender_address: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct UpdateBillingSettingsRequest {
    pub default_hourly_rate_cents: Option<i64>,
    pub billing_tax_mode: Option<String>,
    pub sender_name: Option<String>,
    pub sender_address: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "generated.ts")]
pub struct TestDataStatusResponse {
    pub installed: bool,
    pub state: String,
    pub removed_records: i64,
    pub retained_records: i64,
}

impl From<TestDataStatus> for TestDataStatusResponse {
    fn from(status: TestDataStatus) -> Self {
        Self {
            installed: status.installed,
            state: status.state.as_str().to_string(),
            removed_records: status.removed_records,
            retained_records: status.retained_records,
        }
    }
}

/// GET /api/v1/auth/me - Get current user profile
pub async fn get_current_user(
    State(state): State<AppState>,
    ctx: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = user_service(&state);
    let user = service.get_or_create_from_ctx(&ctx).await?;

    Ok(Json(UserResponse::from_user(
        user,
        &ctx.user_id.to_string(),
    )))
}

/// GET /api/v1/users - List all users in tenant (admin only)
pub async fn list_users(
    State(state): State<AppState>,
    ctx: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = user_service(&state);

    let users = service.list_users(&ctx).await?;
    let actor_subject = ctx.user_id.to_string();
    let response: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse::from_user(user, &actor_subject))
        .collect();

    Ok(Json(response))
}

/// GET /api/v1/users/invites - List pending invites in tenant (admin only)
pub async fn list_pending_invites(
    State(state): State<AppState>,
    ctx: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    if !ctx.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let invites = OnboardingRepository::new(state.pool)
        .list_pending_invites(ctx.tenant_id)
        .await?;

    let response = invites
        .into_iter()
        .map(|invite| PendingInviteResponse {
            id: invite.id.to_string(),
            email: invite.email,
            role: invite.role,
            status: invite.status.to_string(),
            expires_at: invite.expires_at.to_rfc3339(),
            created_at: invite.created_at.to_rfc3339(),
        })
        .collect::<Vec<_>>();

    Ok(Json(response))
}

/// GET /api/v1/users/{id} - Get user by ID
pub async fn get_user(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = user_service(&state);

    let user_id = Uuid::parse_str(&id)
        .map(UserId)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    let user = service.get_user(user_id, &ctx).await?;

    Ok(Json(UserResponse::from_user(
        user,
        &ctx.user_id.to_string(),
    )))
}

/// POST /api/v1/users/invite - Invite a new user (admin only)
pub async fn invite_user(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<InviteUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !ctx.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let role: Role = request.role.parse().map_err(AppError::Validation)?;
    let keycloak = KeycloakAdminClient::from_config(&state.config)?;
    let service = OrganizationInviteService::new(
        OnboardingRepository::new(state.pool),
        keycloak,
        state.config.frontend_public_url,
        state.config.keycloak_organization_invite_ttl_seconds,
    );

    let invite = service
        .generate_invite(ctx.tenant_id, request.email, request.name, role)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(InviteUserResponse {
            id: invite.id.to_string(),
            email: invite.email,
            role: invite.role,
            status: invite.status.to_string(),
            invite_url: invite.invite_url,
            organization_alias: invite.organization_alias,
            expires_at: invite.expires_at.to_rfc3339(),
        }),
    ))
}

/// PATCH /api/v1/users/{id}/role - Update user role (admin only)
pub async fn update_user_role(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = user_service(&state);

    let user_id = Uuid::parse_str(&id)
        .map(UserId)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    let new_role = request
        .role
        .parse()
        .map_err(|e: String| AppError::Validation(e))?;

    let user = service.update_role(user_id, new_role, &ctx).await?;

    Ok(Json(UserResponse::from_user(
        user,
        &ctx.user_id.to_string(),
    )))
}

/// DELETE /api/v1/users/{id} - Remove a manageable user
pub async fn delete_user(
    State(state): State<AppState>,
    ctx: TenantContext,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let user_id = Uuid::parse_str(&id)
        .map(UserId)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;
    user_service(&state).delete_user(user_id, &ctx).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// PATCH /api/v1/users/me - Update own profile
pub async fn update_own_profile(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = user_service(&state);

    let update = UpdateProfile { name: request.name };

    let user = service.update_profile(update, &ctx).await?;

    Ok(Json(UserResponse::from_user(
        user,
        &ctx.user_id.to_string(),
    )))
}

pub async fn get_billing_settings(
    State(state): State<AppState>,
    ctx: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    if !ctx.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let row = sqlx::query(
        r#"
        SELECT default_hourly_rate_cents, billing_tax_mode, billing_sender_name, billing_sender_address, name
        FROM tenants
        WHERE id = $1
        "#,
    )
    .bind(ctx.tenant_id.0)
    .fetch_one(&state.pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))?;

    let tenant_name: String = row
        .try_get("name")
        .map_err(|error| AppError::Database(error.to_string()))?;
    let sender_name = normalize_optional_text(
        row.try_get::<Option<String>, _>("billing_sender_name")
            .map_err(|error| AppError::Database(error.to_string()))?
            .as_deref(),
    )
    .unwrap_or(tenant_name);
    let sender_address = normalize_optional_text(
        row.try_get::<Option<String>, _>("billing_sender_address")
            .map_err(|error| AppError::Database(error.to_string()))?
            .as_deref(),
    );
    let billing_tax_mode: String = row
        .try_get("billing_tax_mode")
        .map_err(|error| AppError::Database(error.to_string()))?;
    let default_hourly_rate_cents: Option<i64> = row
        .try_get("default_hourly_rate_cents")
        .map_err(|error| AppError::Database(error.to_string()))?;

    Ok(Json(BillingSettingsResponse {
        default_hourly_rate_cents,
        billing_tax_mode,
        sender_name,
        sender_address,
    }))
}

pub async fn update_billing_settings(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<UpdateBillingSettingsRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !ctx.is_admin() {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    if let Some(rate) = request.default_hourly_rate_cents {
        if rate < 0 {
            return Err(AppError::Validation(
                "Default hourly rate cannot be negative".to_string(),
            ));
        }
    }

    let billing_tax_mode = request
        .billing_tax_mode
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("standard");

    if billing_tax_mode != "standard" && billing_tax_mode != "kleinunternehmer" {
        return Err(AppError::Validation(
            "Billing tax mode must be standard or kleinunternehmer".to_string(),
        ));
    }

    let sender_name = normalize_optional_text(request.sender_name.as_deref());
    let sender_address = normalize_optional_text(request.sender_address.as_deref());

    let row = sqlx::query(
        r#"
        UPDATE tenants
        SET default_hourly_rate_cents = $2,
            billing_tax_mode = $3,
            billing_sender_name = $4,
            billing_sender_address = $5
        WHERE id = $1
        RETURNING default_hourly_rate_cents, billing_tax_mode, billing_sender_name, billing_sender_address, name
        "#,
    )
    .bind(ctx.tenant_id.0)
    .bind(request.default_hourly_rate_cents)
    .bind(billing_tax_mode)
    .bind(sender_name)
    .bind(sender_address)
    .fetch_one(&state.pool)
    .await
    .map_err(|error| AppError::Database(error.to_string()))?;

    let tenant_name: String = row
        .try_get("name")
        .map_err(|error| AppError::Database(error.to_string()))?;
    let response_sender_name = normalize_optional_text(
        row.try_get::<Option<String>, _>("billing_sender_name")
            .map_err(|error| AppError::Database(error.to_string()))?
            .as_deref(),
    )
    .unwrap_or(tenant_name);
    let response_sender_address = normalize_optional_text(
        row.try_get::<Option<String>, _>("billing_sender_address")
            .map_err(|error| AppError::Database(error.to_string()))?
            .as_deref(),
    );
    let response_tax_mode: String = row
        .try_get("billing_tax_mode")
        .map_err(|error| AppError::Database(error.to_string()))?;
    let response_hourly_rate: Option<i64> = row
        .try_get("default_hourly_rate_cents")
        .map_err(|error| AppError::Database(error.to_string()))?;

    Ok(Json(BillingSettingsResponse {
        default_hourly_rate_cents: response_hourly_rate,
        billing_tax_mode: response_tax_mode,
        sender_name: response_sender_name,
        sender_address: response_sender_address,
    }))
}

pub async fn get_test_data_status(
    State(state): State<AppState>,
    context: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = test_data_service(&state);
    Ok(Json(TestDataStatusResponse::from(
        service.status(&context).await?,
    )))
}

pub async fn install_test_data(
    State(state): State<AppState>,
    context: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = test_data_service(&state);
    Ok(Json(TestDataStatusResponse::from(
        service.install(&context).await?,
    )))
}

pub async fn remove_test_data(
    State(state): State<AppState>,
    context: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = test_data_service(&state);
    Ok(Json(TestDataStatusResponse::from(
        service.remove(&context).await?,
    )))
}

fn test_data_service(state: &AppState) -> TestDataService {
    TestDataService::new(
        TestDataRepository::new(state.pool.clone()),
        user_service(state),
    )
}

fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(ToOwned::to_owned)
}

/// GET /api/v1/preferences - Get current user's preferences
pub async fn get_preferences(
    State(state): State<AppState>,
    ctx: TenantContext,
) -> Result<impl IntoResponse, AppError> {
    let service = UserPreferencesService::new(state.pool.clone());
    let user_service = user_service(&state);

    let user_id = user_service.get_or_create_user_id_from_ctx(&ctx).await?;
    let preferences = service
        .get_validated_preferences(user_id, ctx.tenant_id)
        .await?;

    Ok(Json(PreferencesResponse::from(preferences)))
}

/// PATCH /api/v1/preferences - Update user's active site
pub async fn update_preferences(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(request): Json<UpdatePreferencesRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = UserPreferencesService::new(state.pool.clone());
    let user_service = user_service(&state);

    let user_id = user_service.get_or_create_user_id_from_ctx(&ctx).await?;

    let preferences = match request.active_site_id {
        Some(site_id_str) => {
            // Parse and validate site_id
            let site_id = SiteId::parse(&site_id_str)
                .map_err(|_| AppError::Validation("Invalid site ID".to_string()))?;

            service
                .set_active_site(user_id, ctx.tenant_id, site_id)
                .await?
        }
        None => {
            // Clear active site
            service.clear_active_site(user_id, ctx.tenant_id).await?
        }
    };

    Ok(Json(PreferencesResponse::from(preferences)))
}
