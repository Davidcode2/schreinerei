use axum::{
    body::Body,
    extract::State,
    http::{Request, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::auth::jwks::JwksClient;
use crate::auth::jwt::validate_jwt;
use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;

/// Authentication middleware state
#[derive(Clone)]
pub struct AuthState {
    pub jwks_client: JwksClient,
    pub jwt_issuer: String,
}

/// Authentication middleware that validates JWT tokens
pub async fn auth_middleware(
    State(auth_state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Skip auth for health endpoint
    if request.uri().path() == "/health" {
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

    // Convert to AuthenticatedUser
    let auth_user = AuthenticatedUser::from_claims(&claims)?;

    // Inject into request extensions
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

/// Optional authentication - doesn't fail if no token present
pub async fn optional_auth_middleware(
    State(auth_state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    // Try to extract and validate token, but don't fail if missing
    if let Some(auth_header) = request.headers().get(AUTHORIZATION).and_then(|h| h.to_str().ok()) {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            if let Ok(jwks) = auth_state.jwks_client.get_jwks().await {
                if let Ok(claims) = validate_jwt(token, &jwks, &auth_state.jwt_issuer) {
                    if let Ok(auth_user) = AuthenticatedUser::from_claims(&claims) {
                        request.extensions_mut().insert(auth_user);
                    }
                }
            }
        }
    }

    next.run(request).await
}
