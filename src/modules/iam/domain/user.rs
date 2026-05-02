use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::{Role, TenantId, UserId};

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn test_user(role: Role) -> User {
        User {
            id: UserId::new(),
            tenant_id: TenantId::new(),
            keycloak_user_id: "kc-123".to_string(),
            email: "test@example.com".to_string(),
            name: Some("Test User".to_string()),
            role,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn user_is_admin_returns_true_for_admin_role() {
        let user = test_user(Role::Admin);
        assert!(user.is_admin());
    }

    #[test]
    fn user_is_admin_returns_false_for_employee_role() {
        let user = test_user(Role::Employee);
        assert!(!user.is_admin());
    }

    #[test]
    fn user_is_employee_returns_true_for_employee_role() {
        let user = test_user(Role::Employee);
        assert!(user.is_employee());
    }

    #[test]
    fn user_is_employee_returns_false_for_admin_role() {
        let user = test_user(Role::Admin);
        assert!(!user.is_employee());
    }

    #[test]
    fn create_user_validate_succeeds_with_valid_data() {
        let cmd = CreateUser {
            keycloak_user_id: "kc-123".to_string(),
            email: "test@example.com".to_string(),
            name: Some("Test User".to_string()),
            role: Role::Employee,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn create_user_validate_fails_with_empty_email() {
        let cmd = CreateUser {
            keycloak_user_id: "kc-123".to_string(),
            email: "".to_string(),
            name: None,
            role: Role::Employee,
        };
        assert_eq!(cmd.validate(), Err("Email is required".to_string()));
    }

    #[test]
    fn create_user_validate_fails_with_invalid_email() {
        let cmd = CreateUser {
            keycloak_user_id: "kc-123".to_string(),
            email: "invalid-email".to_string(),
            name: None,
            role: Role::Employee,
        };
        assert_eq!(cmd.validate(), Err("Invalid email format".to_string()));
    }

    #[test]
    fn create_user_validate_fails_with_empty_keycloak_id() {
        let cmd = CreateUser {
            keycloak_user_id: "".to_string(),
            email: "test@example.com".to_string(),
            name: None,
            role: Role::Employee,
        };
        assert_eq!(
            cmd.validate(),
            Err("Keycloak user ID is required".to_string())
        );
    }

    #[test]
    fn invite_user_validate_succeeds_with_valid_data() {
        let cmd = InviteUser {
            email: "invite@example.com".to_string(),
            name: Some("New User".to_string()),
            role: Role::Employee,
        };
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn invite_user_validate_fails_with_empty_email() {
        let cmd = InviteUser {
            email: "".to_string(),
            name: None,
            role: Role::Employee,
        };
        assert_eq!(cmd.validate(), Err("Email is required".to_string()));
    }

    #[test]
    fn invite_user_validate_fails_with_invalid_email() {
        let cmd = InviteUser {
            email: "no-at-sign".to_string(),
            name: None,
            role: Role::Employee,
        };
        assert_eq!(cmd.validate(), Err("Invalid email format".to_string()));
    }
}
