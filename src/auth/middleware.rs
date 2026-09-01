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

    let jwt_roles: Vec<Role> = claims
        .realm_access
        .roles
        .iter()
        .filter_map(|r| r.parse::<Role>().ok())
        .collect();
    let roles = resolve_authoritative_roles(
        &auth_state.pool,
        TenantId(tenant_id),
        &claims.sub,
        &claims.email,
        jwt_roles.clone(),
    )
    .await?;

    // Create authenticated user
    let auth_user = AuthenticatedUser {
        user_id,
        tenant_id: TenantId(tenant_id),
        email: claims.email.clone(),
        roles,
        token_roles: jwt_roles,
    };

    // Inject into request extensions
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

fn is_public_endpoint(path: &str) -> bool {
    path == "/health"
        || path == "/api/v1/onboarding/sessions"
        || path.starts_with("/api/v1/onboarding/sessions/")
        || path == "/api/v1/onboarding/webhooks/mollie"
        || path.starts_with("/api/v1/onboarding/invites/")
}

async fn resolve_authoritative_roles(
    pool: &PgPool,
    tenant_id: TenantId,
    keycloak_user_id: &str,
    email: &str,
    jwt_roles: Vec<Role>,
) -> Result<Vec<Role>, AppError> {
    let exact: Option<(String, bool)> = sqlx::query_as(
        "SELECT role, deleted_at IS NOT NULL FROM users WHERE tenant_id = $1 AND keycloak_user_id = $2",
    )
    .bind(tenant_id.0)
    .bind(keycloak_user_id)
    .fetch_optional(pool)
    .await
    .map_err(|error| AppError::Internal(format!("Database error: {error}")))?;

    if let Some((role, deleted)) = exact {
        if deleted {
            return Err(AppError::Unauthorized(
                "User account is deleted".to_string(),
            ));
        }
        return parse_authoritative_role(role);
    }

    let pending: Option<String> = sqlx::query_scalar(
        r#"
        SELECT role FROM users
        WHERE tenant_id = $1 AND lower(email) = lower($2)
          AND keycloak_user_id LIKE 'pending-%' AND deleted_at IS NULL
        ORDER BY CASE WHEN keycloak_user_id LIKE 'pending-onboarding-admin-%' THEN 0 ELSE 1 END,
                 created_at
        LIMIT 1
        "#,
    )
    .bind(tenant_id.0)
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(|error| AppError::Internal(format!("Database error: {error}")))?;

    pending
        .map(parse_authoritative_role)
        .unwrap_or(Ok(jwt_roles))
}

fn parse_authoritative_role(role: String) -> Result<Vec<Role>, AppError> {
    role.parse::<Role>()
        .map(|role| vec![role])
        .map_err(|error| AppError::Internal(format!("Invalid local user role: {error}")))
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
                                let jwt_roles: Vec<Role> = claims
                                    .realm_access
                                    .roles
                                    .iter()
                                    .filter_map(|r| r.parse::<Role>().ok())
                                    .collect();
                                if let Ok(roles) = resolve_authoritative_roles(
                                    &auth_state.pool,
                                    TenantId(tenant_id),
                                    &claims.sub,
                                    &claims.email,
                                    jwt_roles.clone(),
                                )
                                .await
                                {
                                    let auth_user = AuthenticatedUser {
                                        user_id,
                                        tenant_id: TenantId(tenant_id),
                                        email: claims.email.clone(),
                                        roles,
                                        token_roles: jwt_roles,
                                    };
                                    request.extensions_mut().insert(auth_user);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::resolve_authoritative_roles;
    use crate::common::error::AppError;
    use crate::common::types::{Role, TenantId};
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn local_role_overrides_stale_jwt_role(pool: PgPool) {
        let tenant_id = insert_tenant(&pool, "local-role").await;
        let subject = Uuid::new_v4();
        insert_user(&pool, tenant_id, &subject.to_string(), "employee", false).await;

        let roles = resolve_authoritative_roles(
            &pool,
            TenantId(tenant_id),
            &subject.to_string(),
            "user@example.com",
            vec![Role::Admin],
        )
        .await
        .unwrap();

        assert_eq!(roles, vec![Role::Employee]);
    }

    #[sqlx::test]
    async fn pending_local_role_overrides_jwt_before_claim(pool: PgPool) {
        let tenant_id = insert_tenant(&pool, "pending-role").await;
        insert_user(&pool, tenant_id, "pending-invite", "admin", false).await;

        let roles = resolve_authoritative_roles(
            &pool,
            TenantId(tenant_id),
            &Uuid::new_v4().to_string(),
            "USER@example.com",
            vec![Role::Employee],
        )
        .await
        .unwrap();

        assert_eq!(roles, vec![Role::Admin]);
    }

    #[sqlx::test]
    async fn deleted_exact_identity_is_rejected(pool: PgPool) {
        let tenant_id = insert_tenant(&pool, "deleted-auth").await;
        let subject = Uuid::new_v4();
        insert_user(&pool, tenant_id, &subject.to_string(), "admin", true).await;

        let result = resolve_authoritative_roles(
            &pool,
            TenantId(tenant_id),
            &subject.to_string(),
            "user@example.com",
            vec![Role::Admin],
        )
        .await;

        assert!(matches!(result, Err(AppError::Unauthorized(_))));
    }

    async fn insert_tenant(pool: &PgPool, suffix: &str) -> Uuid {
        sqlx::query_scalar(
            "INSERT INTO tenants (keycloak_realm, name, slug) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(format!("auth-{suffix}"))
        .bind(suffix)
        .bind(format!("auth-{suffix}"))
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn insert_user(pool: &PgPool, tenant_id: Uuid, subject: &str, role: &str, deleted: bool) {
        sqlx::query(
            r#"
            INSERT INTO users (tenant_id, keycloak_user_id, email, role, deleted_at)
            VALUES ($1, $2, 'user@example.com', $3, CASE WHEN $4 THEN NOW() ELSE NULL END)
            "#,
        )
        .bind(tenant_id)
        .bind(subject)
        .bind(role)
        .bind(deleted)
        .execute(pool)
        .await
        .unwrap();
    }
}
