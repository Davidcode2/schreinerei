use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{TenantId, UserId, Role};

/// Authenticated user extracted from JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub email: String,
    pub roles: Vec<Role>,
}

impl AuthenticatedUser {
    /// Create from JWT claims
    pub fn from_claims(claims: &crate::auth::jwt::Claims) -> Result<Self, AppError> {
        let user_id = Uuid::parse_str(&claims.sub)
            .map(UserId)
            .map_err(|e| AppError::Auth(format!("Invalid user ID in token: {}", e)))?;

        let tenant_id = Uuid::parse_str(&claims.organization)
            .map(TenantId)
            .map_err(|e| AppError::Auth(format!("Invalid organization ID in token: {}", e)))?;

        let roles = claims.realm_access.roles.iter()
            .filter_map(|r| r.parse::<Role>().ok())
            .collect();

        Ok(Self {
            user_id,
            tenant_id,
            email: claims.email.clone(),
            roles,
        })
    }

    /// Check if user has admin role
    pub fn is_admin(&self) -> bool {
        self.roles.iter().any(|r| r.is_admin())
    }

    /// Check if user has employee role
    pub fn is_employee(&self) -> bool {
        self.roles.iter().any(|r| r.is_employee())
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or(AppError::Unauthorized("Not authenticated".to_string()))
    }
}
