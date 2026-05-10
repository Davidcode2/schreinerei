use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::extractor::AuthenticatedUser;
use crate::auth::jwks::JwksClient;
use crate::auth::jwt::validate_jwt;
use crate::common::error::AppError;
use crate::common::types::{Role, TenantId, UserId};

/// Authentication middleware state
#[derive(Clone)]
pub struct AuthState {
    pub jwks_client: JwksClient,
    pub jwt_issuer: String,
    pub pool: PgPool,
}

/// Look up tenant ID by organization alias
async fn find_tenant_by_org_alias(pool: &PgPool, org_alias: &str) -> Result<Uuid, AppError> {
    let tenant_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM tenants WHERE keycloak_organization_alias = $1")
            .bind(org_alias)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    tenant_id.ok_or_else(|| {
        AppError::Auth(format!(
            "No tenant found for organization alias: {}",
            org_alias
        ))
    })
}

/// Authentication middleware that validates JWT tokens
pub async fn auth_middleware(
    State(auth_state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Skip auth for public endpoints
    if is_public_endpoint(request.uri().path()) {
        return Ok(next.run(request).await);
    }

    // Extract Bearer token
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid Authorization header format".to_string()))?;

    // Get JWKS
    let jwks = auth_state.jwks_client.get_jwks().await?;

    // Validate JWT
    let claims = validate_jwt(token, &jwks, &auth_state.jwt_issuer)?;

    // Get organization alias from claims
    let org_alias = claims
        .organization_alias()
        .ok_or_else(|| AppError::Auth("No organization membership in token".to_string()))?;

    // Look up tenant by organization alias
    let tenant_id = find_tenant_by_org_alias(&auth_state.pool, org_alias).await?;

    // Parse user ID
    let user_id = Uuid::parse_str(&claims.sub)
        .map(UserId)
        .map_err(|e| AppError::Auth(format!("Invalid user ID in token: {}", e)))?;

    // Parse roles
    let roles = claims
        .realm_access
        .roles
        .iter()
        .filter_map(|r| r.parse::<Role>().ok())
        .collect();

    // Create authenticated user
    let auth_user = AuthenticatedUser {
        user_id,
        tenant_id: TenantId(tenant_id),
        email: claims.email.clone(),
        roles,
    };

    // Inject into request extensions
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

fn is_public_endpoint(path: &str) -> bool {
    path == "/health"
        || path == "/api/v1/onboarding/sessions"
        || path == "/api/v1/onboarding/webhooks/mollie"
        || path.starts_with("/api/v1/onboarding/invites/")
}

/// Optional authentication - doesn't fail if no token present
pub async fn optional_auth_middleware(
    State(auth_state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    // Try to extract and validate token, but don't fail if missing
    if let Some(auth_header) = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            if let Ok(jwks) = auth_state.jwks_client.get_jwks().await {
                if let Ok(claims) = validate_jwt(token, &jwks, &auth_state.jwt_issuer) {
                    if let Some(org_alias) = claims.organization_alias() {
                        if let Ok(tenant_id) =
                            find_tenant_by_org_alias(&auth_state.pool, org_alias).await
                        {
                            if let Ok(user_id) = Uuid::parse_str(&claims.sub).map(UserId) {
                                let roles = claims
                                    .realm_access
                                    .roles
                                    .iter()
                                    .filter_map(|r| r.parse::<Role>().ok())
                                    .collect();

                                let auth_user = AuthenticatedUser {
                                    user_id,
                                    tenant_id: TenantId(tenant_id),
                                    email: claims.email.clone(),
                                    roles,
                                };

                                request.extensions_mut().insert(auth_user);
                            }
                        }
                    }
                }
            }
        }
    }

    next.run(request).await
}
