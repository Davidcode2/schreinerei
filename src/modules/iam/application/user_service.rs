use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{Role, TenantId, UserId};
use crate::modules::iam::domain::user::{CreateUser, InviteUser, UpdateProfile, User};
use crate::modules::iam::infrastructure::user_repository::UserRepository;

/// Context for tenant-scoped operations
#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub email: String,
    pub roles: Vec<Role>,
}

impl TenantContext {
    /// Create context from authenticated user
    pub fn from_auth(auth: &AuthenticatedUser) -> Self {
        Self {
            tenant_id: auth.tenant_id,
            user_id: auth.user_id,
            email: auth.email.clone(),
            roles: auth.roles.clone(),
        }
    }

    /// Check if user has admin role
    pub fn is_admin(&self) -> bool {
        self.roles.iter().any(|r| r.is_admin())
    }
}

/// Service for user management operations
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    /// Get or create user from Keycloak JWT authentication
    /// Syncs user from Keycloak to local database
    pub async fn get_or_create_from_auth(
        &self,
        auth: &AuthenticatedUser,
    ) -> Result<User, AppError> {
        let tenant_id = auth.tenant_id;

        // Check if user exists
        if let Some(user) = self
            .user_repo
            .find_by_keycloak_id(&auth.user_id.to_string(), tenant_id)
            .await?
        {
            return Ok(user);
        }

        // Create new user from auth
        let create_user = CreateUser {
            keycloak_user_id: auth.user_id.to_string(),
            email: auth.email.clone(),
            name: None,
            role: if auth.is_admin() {
                Role::Admin
            } else {
                Role::Employee
            },
        };

        self.user_repo.create(&create_user, tenant_id).await
    }

    /// Resolve tenant-local user id from authenticated identity.
    pub async fn get_or_create_user_id_from_auth(
        &self,
        auth: &AuthenticatedUser,
    ) -> Result<UserId, AppError> {
        let user = self.get_or_create_from_auth(auth).await?;
        Ok(user.id)
    }

    /// Get current user by ID
    pub async fn get_user(&self, user_id: UserId, ctx: &TenantContext) -> Result<User, AppError> {
        self.user_repo
            .find_by_id(user_id, ctx.tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    /// List all users in tenant (admin only)
    pub async fn list_users(&self, ctx: &TenantContext) -> Result<Vec<User>, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        self.user_repo.list(ctx.tenant_id).await
    }

    /// Invite new user (admin only)
    /// Note: Actual email sending is out of scope for V1
    pub async fn invite_user(
        &self,
        invite: InviteUser,
        ctx: &TenantContext,
    ) -> Result<User, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        invite.validate()?;

        // Create user with pending status (no keycloak_user_id yet)
        // In V1, we create a placeholder user
        let create_user = CreateUser {
            keycloak_user_id: format!("pending-{}", uuid::Uuid::new_v4()),
            email: invite.email,
            name: invite.name,
            role: invite.role,
        };

        self.user_repo.create(&create_user, ctx.tenant_id).await
    }

    /// Update user role (admin only)
    pub async fn update_role(
        &self,
        user_id: UserId,
        new_role: Role,
        ctx: &TenantContext,
    ) -> Result<User, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }

        // Prevent admin from demoting themselves
        if user_id == ctx.user_id && new_role != Role::Admin {
            return Err(AppError::Validation(
                "Cannot demote yourself from admin".to_string(),
            ));
        }

        self.user_repo
            .update_role(user_id, new_role, ctx.tenant_id)
            .await
    }

    /// Update own profile
    pub async fn update_profile(
        &self,
        update: UpdateProfile,
        ctx: &TenantContext,
    ) -> Result<User, AppError> {
        self.user_repo
            .update_profile(ctx.user_id, update.name, ctx.tenant_id)
            .await
    }
}
