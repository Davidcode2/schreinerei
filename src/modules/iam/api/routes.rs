use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{SiteId, UserId};
use crate::modules::iam::application::user_preferences_service::UserPreferencesService;
use crate::modules::iam::application::user_service::{TenantContext, UserService};
use crate::modules::iam::domain::user::{InviteUser, UpdateProfile};
use crate::modules::iam::domain::user_preferences::UserPreferenceRecord;
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::AppState;

/// Create the IAM API router
pub fn create_router() -> Router<AppState> {
    Router::new()
        // Current user endpoints (any authenticated user)
        .route("/api/v1/auth/me", get(get_current_user))
        .route("/api/v1/users/me", patch(update_own_profile))
        // Preferences endpoints
        .route(
            "/api/v1/preferences",
            get(get_preferences).patch(update_preferences),
        )
        // User management endpoints (admin only)
        .route("/api/v1/users", get(list_users))
        .route("/api/v1/users/invite", post(invite_user))
        .route("/api/v1/users/{id}/role", patch(update_user_role))
        .route("/api/v1/users/{id}", get(get_user))
}

/// Response DTO for user data
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub created_at: String,
}

impl From<crate::modules::iam::domain::user::User> for UserResponse {
    fn from(user: crate::modules::iam::domain::user::User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            role: user.role.to_string(),
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

/// Request DTO for inviting a user
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct InviteUserRequest {
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

/// Request DTO for updating role
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UpdateRoleRequest {
    pub role: String,
}

/// Request DTO for updating profile
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
}

/// Response DTO for user preferences
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "frontend/src/types/generated.ts")]
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
#[ts(export, export_to = "frontend/src/types/generated.ts")]
pub struct UpdatePreferencesRequest {
    pub active_site_id: Option<String>,
}

/// GET /api/v1/auth/me - Get current user profile
pub async fn get_current_user(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = UserService::new(
        crate::modules::iam::infrastructure::user_repository::UserRepository::new(state.pool),
    );
    let _ctx = TenantContext::from_auth(&auth);

    let user = service.get_or_create_from_auth(&auth).await?;

    Ok(Json(UserResponse::from(user)))
}

/// GET /api/v1/users - List all users in tenant (admin only)
pub async fn list_users(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = UserService::new(
        crate::modules::iam::infrastructure::user_repository::UserRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let users = service.list_users(&ctx).await?;
    let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(Json(response))
}

/// GET /api/v1/users/{id} - Get user by ID
pub async fn get_user(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = UserService::new(
        crate::modules::iam::infrastructure::user_repository::UserRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let user_id = Uuid::parse_str(&id)
        .map(UserId)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    let user = service.get_user(user_id, &ctx).await?;

    Ok(Json(UserResponse::from(user)))
}

/// POST /api/v1/users/invite - Invite a new user (admin only)
pub async fn invite_user(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<InviteUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = UserService::new(
        crate::modules::iam::infrastructure::user_repository::UserRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let role = request
        .role
        .parse()
        .map_err(|e: String| AppError::Validation(e))?;

    let invite = InviteUser {
        email: request.email,
        name: request.name,
        role,
    };

    let user = service.invite_user(invite, &ctx).await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

/// PATCH /api/v1/users/{id}/role - Update user role (admin only)
pub async fn update_user_role(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = UserService::new(
        crate::modules::iam::infrastructure::user_repository::UserRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let user_id = Uuid::parse_str(&id)
        .map(UserId)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    let new_role = request
        .role
        .parse()
        .map_err(|e: String| AppError::Validation(e))?;

    let user = service.update_role(user_id, new_role, &ctx).await?;

    Ok(Json(UserResponse::from(user)))
}

/// PATCH /api/v1/users/me - Update own profile
pub async fn update_own_profile(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = UserService::new(
        crate::modules::iam::infrastructure::user_repository::UserRepository::new(state.pool),
    );
    let ctx = TenantContext::from_auth(&auth);

    let update = UpdateProfile { name: request.name };

    let user = service.update_profile(update, &ctx).await?;

    Ok(Json(UserResponse::from(user)))
}

/// GET /api/v1/preferences - Get current user's preferences
pub async fn get_preferences(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = UserPreferencesService::new(state.pool.clone());
    let user_service = UserService::new(UserRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);

    let user_id = user_service.get_or_create_user_id_from_auth(&auth).await?;
    let preferences = service
        .get_validated_preferences(user_id, ctx.tenant_id)
        .await?;

    Ok(Json(PreferencesResponse::from(preferences)))
}

/// PATCH /api/v1/preferences - Update user's active site
pub async fn update_preferences(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(request): Json<UpdatePreferencesRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = UserPreferencesService::new(state.pool.clone());
    let user_service = UserService::new(UserRepository::new(state.pool));
    let ctx = TenantContext::from_auth(&auth);

    let user_id = user_service.get_or_create_user_id_from_auth(&auth).await?;

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
