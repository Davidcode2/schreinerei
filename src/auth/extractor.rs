use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};

use crate::common::error::AppError;
use crate::common::types::{Role, TenantId, UserId};

/// Authenticated user extracted from JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub email: String,
    pub roles: Vec<Role>,
}

impl AuthenticatedUser {
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

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or(AppError::Unauthorized("Not authenticated".to_string()))
    }
}
