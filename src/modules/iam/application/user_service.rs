use std::sync::Arc;

use async_trait::async_trait;

use crate::auth::extractor::AuthenticatedUser;
use crate::common::error::AppError;
use crate::common::types::{Role, TenantId, UserId};
use crate::modules::iam::domain::user::{CreateUser, InviteUser, UpdateProfile, User};
use crate::modules::iam::infrastructure::user_repository::UserRepository;
use crate::modules::onboarding::infrastructure::keycloak_admin_client::KeycloakAdminClient;
use axum::{extract::FromRequestParts, http::request::Parts};

#[async_trait]
pub trait RealmRoleAssigner: Send + Sync {
    async fn assign_realm_role(&self, user_id: &str, role: Role) -> Result<(), AppError>;
}

#[async_trait]
pub trait KeycloakUserManager: Send + Sync {
    async fn synchronize_realm_role(&self, user_id: &str, role: Role) -> Result<(), AppError>;
    async fn remove_organization_member(
        &self,
        organization_id: &str,
        user_id: &str,
    ) -> Result<(), AppError>;
}

#[async_trait]
impl KeycloakUserManager for KeycloakAdminClient {
    async fn synchronize_realm_role(&self, user_id: &str, role: Role) -> Result<(), AppError> {
        self.synchronize_realm_role(user_id, role).await
    }

    async fn remove_organization_member(
        &self,
        organization_id: &str,
        user_id: &str,
    ) -> Result<(), AppError> {
        self.remove_organization_member(organization_id, user_id)
            .await
    }
}

#[async_trait]
impl RealmRoleAssigner for KeycloakAdminClient {
    async fn assign_realm_role(&self, user_id: &str, role: Role) -> Result<(), AppError> {
        self.assign_realm_role(user_id, role).await
    }
}

/// Context for tenant-scoped operations
#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub email: String,
    pub roles: Vec<Role>,
    /// Realm roles exactly as they appear in the presented JWT.
    pub token_roles: Vec<Role>,
}

impl TenantContext {
    /// Create context from authenticated user
    pub fn from_auth(auth: &AuthenticatedUser) -> Self {
        Self {
            tenant_id: auth.tenant_id,
            user_id: auth.user_id,
            email: auth.email.clone(),
            roles: auth.roles.clone(),
            token_roles: auth.token_roles.clone(),
        }
    }

    /// Check if user has admin role
    pub fn is_admin(&self) -> bool {
        self.roles.iter().any(|r| r.is_admin())
    }

    /// Check if the presented JWT carries the admin role
    pub fn token_is_admin(&self) -> bool {
        self.token_roles.iter().any(|r| r.is_admin())
    }

    /// Reconstruct authenticated user data from request-scoped context.
    pub fn to_auth(&self) -> AuthenticatedUser {
        AuthenticatedUser {
            user_id: self.user_id,
            tenant_id: self.tenant_id,
            email: self.email.clone(),
            roles: self.roles.clone(),
            token_roles: self.token_roles.clone(),
        }
    }
}

impl<S> FromRequestParts<S> for TenantContext
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth = parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or(AppError::Unauthorized("Not authenticated".to_string()))?;

        Ok(Self::from_auth(&auth))
    }
}

/// Service for user management operations
pub struct UserService {
    user_repo: UserRepository,
    role_assigner: Option<Arc<dyn RealmRoleAssigner>>,
    keycloak_manager: Option<Arc<dyn KeycloakUserManager>>,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self {
            user_repo,
            role_assigner: None,
            keycloak_manager: None,
        }
    }

    pub fn new_with_role_assigner(
        user_repo: UserRepository,
        role_assigner: Arc<dyn RealmRoleAssigner>,
    ) -> Self {
        Self {
            user_repo,
            role_assigner: Some(role_assigner),
            keycloak_manager: None,
        }
    }

    pub fn new_with_keycloak_manager(
        user_repo: UserRepository,
        keycloak_manager: Arc<dyn KeycloakUserManager>,
    ) -> Self {
        Self {
            user_repo,
            role_assigner: None,
            keycloak_manager: Some(keycloak_manager),
        }
    }

    pub fn new_with_keycloak_client(
        user_repo: UserRepository,
        client: Arc<KeycloakAdminClient>,
    ) -> Self {
        Self {
            user_repo,
            role_assigner: Some(client.clone()),
            keycloak_manager: Some(client),
        }
    }

    /// Get or create user from Keycloak JWT authentication
    /// Syncs user from Keycloak to local database
    pub async fn get_or_create_from_auth(
        &self,
        auth: &AuthenticatedUser,
    ) -> Result<User, AppError> {
        self.get_or_create_from_ctx(&TenantContext::from_auth(auth))
            .await
    }

    /// Get or create user from request-scoped tenant context.
    pub async fn get_or_create_from_ctx(&self, ctx: &TenantContext) -> Result<User, AppError> {
        let tenant_id = ctx.tenant_id;
        let invited_role = self
            .user_repo
            .find_pending_invite_role(tenant_id, &ctx.email)
            .await?;

        // Check if user exists
        if let Some(user) = self
            .user_repo
            .find_by_keycloak_id(&ctx.user_id.to_string(), tenant_id)
            .await?
        {
            self.sync_realm_role_if_needed(&user, ctx).await?;
            return Ok(user);
        }

        if let Some(user) = self
            .user_repo
            .claim_pending_by_email(tenant_id, &ctx.email, &ctx.user_id.to_string())
            .await?
        {
            let user = self
                .apply_invited_role_if_needed(user, tenant_id, invited_role)
                .await?;
            if invited_role.is_some() {
                self.user_repo
                    .mark_pending_invite_accepted(tenant_id, &ctx.email)
                    .await?;
            }
            self.sync_realm_role_if_needed(&user, ctx).await?;
            return Ok(user);
        }

        // Create new user from auth
        let create_user = CreateUser {
            keycloak_user_id: ctx.user_id.to_string(),
            email: ctx.email.clone(),
            name: None,
            role: invited_role.unwrap_or_else(|| {
                if ctx.is_admin() {
                    Role::Admin
                } else {
                    Role::Employee
                }
            }),
        };

        let user = self.user_repo.create(&create_user, tenant_id).await?;
        if invited_role.is_some() {
            self.user_repo
                .mark_pending_invite_accepted(tenant_id, &ctx.email)
                .await?;
        }
        self.sync_realm_role_if_needed(&user, ctx).await?;
        Ok(user)
    }

    async fn apply_invited_role_if_needed(
        &self,
        user: User,
        tenant_id: TenantId,
        invited_role: Option<Role>,
    ) -> Result<User, AppError> {
        let Some(invited_role) = invited_role else {
            return Ok(user);
        };

        if user.role == invited_role {
            return Ok(user);
        }

        self.user_repo
            .update_role(user.id, invited_role, tenant_id)
            .await
    }

    async fn sync_realm_role_if_needed(
        &self,
        user: &User,
        ctx: &TenantContext,
    ) -> Result<(), AppError> {
        if !user.is_admin()
            || ctx.token_is_admin()
            || user.keycloak_user_id != ctx.user_id.to_string()
        {
            return Ok(());
        }

        let Some(role_assigner) = &self.role_assigner else {
            return Ok(());
        };

        role_assigner
            .assign_realm_role(&user.keycloak_user_id, Role::Admin)
            .await
    }

    /// Resolve tenant-local user id from authenticated identity.
    pub async fn get_or_create_user_id_from_auth(
        &self,
        auth: &AuthenticatedUser,
    ) -> Result<UserId, AppError> {
        let user = self
            .get_or_create_from_ctx(&TenantContext::from_auth(auth))
            .await?;
        Ok(user.id)
    }

    /// Resolve tenant-local user id from request-scoped context.
    pub async fn get_or_create_user_id_from_ctx(
        &self,
        ctx: &TenantContext,
    ) -> Result<UserId, AppError> {
        let user = self.get_or_create_from_ctx(ctx).await?;
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
        let target = self.management_target(user_id, ctx).await?;
        if target.role == new_role {
            return Ok(target);
        }

        let manager = self.keycloak_manager.as_ref().ok_or_else(|| {
            AppError::Internal("Keycloak user management is not configured".to_string())
        })?;
        manager
            .synchronize_realm_role(&target.keycloak_user_id, new_role)
            .await?;
        self.user_repo
            .update_role(user_id, new_role, ctx.tenant_id)
            .await
    }

    pub async fn delete_user(&self, user_id: UserId, ctx: &TenantContext) -> Result<(), AppError> {
        let target = self.management_target(user_id, ctx).await?;
        let organization_id = self.user_repo.tenant_organization_id(ctx.tenant_id).await?;
        let manager = self.keycloak_manager.as_ref().ok_or_else(|| {
            AppError::Internal("Keycloak user management is not configured".to_string())
        })?;
        manager
            .remove_organization_member(&organization_id, &target.keycloak_user_id)
            .await?;
        self.user_repo.soft_delete(user_id, ctx.tenant_id).await
    }

    async fn management_target(
        &self,
        user_id: UserId,
        ctx: &TenantContext,
    ) -> Result<User, AppError> {
        if !ctx.is_admin() {
            return Err(AppError::Forbidden("Admin access required".to_string()));
        }
        let target = self.get_user(user_id, ctx).await?;
        if !target.can_be_managed_by(&ctx.user_id.to_string()) {
            return Err(AppError::Conflict("User cannot be managed".to_string()));
        }
        Ok(target)
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
