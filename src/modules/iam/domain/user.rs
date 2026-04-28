use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{TenantId, UserId, Role};

/// User aggregate representing a user in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub tenant_id: TenantId,
    pub keycloak_user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Check if user has admin role
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Check if user has employee role
    pub fn is_employee(&self) -> bool {
        self.role.is_employee()
    }
}

/// Command to create a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub keycloak_user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: Role,
}

impl CreateUser {
    /// Validate the create user command
    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_empty() {
            return Err("Email is required".to_string());
        }
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        if self.keycloak_user_id.is_empty() {
            return Err("Keycloak user ID is required".to_string());
        }
        Ok(())
    }
}

/// Command to update a user's role (admin only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRole {
    pub user_id: UserId,
    pub new_role: Role,
}

/// Command to update a user's own profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfile {
    pub name: Option<String>,
}

/// Command to invite a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteUser {
    pub email: String,
    pub name: Option<String>,
    pub role: Role,
}

impl InviteUser {
    /// Validate the invite user command
    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_empty() {
            return Err("Email is required".to_string());
        }
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        Ok(())
    }
}
