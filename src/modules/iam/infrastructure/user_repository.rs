use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::common::error::AppError;
use crate::common::types::{Role, TenantId, UserId};
use crate::modules::iam::domain::user::{CreateUser, User};

/// Repository for user data access with tenant isolation
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find a user by ID within a specific tenant
    /// Returns None if user not found or not in the same tenant
    pub async fn find_by_id(
        &self,
        id: UserId,
        tenant_id: TenantId,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at
            FROM users
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|row| row.into_user()))
    }

    /// Find a user by Keycloak user ID within a specific tenant
    pub async fn find_by_keycloak_id(
        &self,
        keycloak_user_id: &str,
        tenant_id: TenantId,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at
            FROM users
            WHERE keycloak_user_id = $1 AND tenant_id = $2
            "#,
        )
        .bind(keycloak_user_id)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|row| row.into_user()))
    }

    pub async fn find_by_email(
        &self,
        email: &str,
        tenant_id: TenantId,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at
            FROM users
            WHERE tenant_id = $1
              AND lower(email) = lower($2)
            "#,
        )
        .bind(tenant_id.0)
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|row| row.into_user()))
    }

    pub async fn claim_pending_user_by_email(
        &self,
        email: &str,
        keycloak_user_id: &str,
        tenant_id: TenantId,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            UPDATE users
            SET keycloak_user_id = $3,
                updated_at = NOW()
            WHERE tenant_id = $1
              AND lower(email) = lower($2)
              AND keycloak_user_id LIKE 'pending-%'
            RETURNING id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at
            "#,
        )
        .bind(tenant_id.0)
        .bind(email)
        .bind(keycloak_user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.map(|row| row.into_user()))
    }

    /// List all users in a tenant
    pub async fn list(&self, tenant_id: TenantId) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at
            FROM users
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(tenant_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(users.into_iter().map(|row| row.into_user()).collect())
    }

    /// Create a new user in the database
    pub async fn create(
        &self,
        create_user: &CreateUser,
        tenant_id: TenantId,
    ) -> Result<User, AppError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let user = sqlx::query_as::<_, UserRow>(
            r#"
            INSERT INTO users (id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id.0)
        .bind(&create_user.keycloak_user_id)
        .bind(&create_user.email)
        .bind(&create_user.name)
        .bind(create_user.role.to_string())
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique constraint") {
                AppError::Validation("User already exists in this tenant".to_string())
            } else {
                AppError::Database(e.to_string())
            }
        })?;

        Ok(user.into_user())
    }

    /// Update a user's role within a tenant
    pub async fn update_role(
        &self,
        id: UserId,
        role: Role,
        tenant_id: TenantId,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            UPDATE users
            SET role = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at
            "#,
        )
        .bind(role.to_string())
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user.into_user())
    }

    /// Find or create user by Keycloak ID
    /// Returns existing user or creates new one from Keycloak info
    pub async fn find_or_create_by_keycloak_id(
        &self,
        keycloak_user_id: &str,
        tenant_id: TenantId,
        email: &str,
        role: Role,
    ) -> Result<User, AppError> {
        if let Some(user) = self
            .find_by_keycloak_id(keycloak_user_id, tenant_id)
            .await?
        {
            return Ok(user);
        }

        let create = CreateUser {
            keycloak_user_id: keycloak_user_id.to_string(),
            email: email.to_string(),
            name: None,
            role,
        };

        self.create(&create, tenant_id).await
    }

    /// Get local user ID from Keycloak user ID
    pub async fn get_local_user_id(
        &self,
        keycloak_user_id: &str,
        tenant_id: TenantId,
    ) -> Result<Uuid, AppError> {
        let user = self
            .find_by_keycloak_id(keycloak_user_id, tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found in local database".to_string()))?;
        Ok(user.id.0)
    }

    /// Update a user's profile (name) within a tenant
    pub async fn update_profile(
        &self,
        id: UserId,
        name: Option<String>,
        tenant_id: TenantId,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            UPDATE users
            SET name = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, keycloak_user_id, email, name, role, created_at, updated_at
            "#,
        )
        .bind(&name)
        .bind(id.0)
        .bind(tenant_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user.into_user())
    }
}

/// Database row representation for SQLx
#[derive(Debug, FromRow)]
struct UserRow {
    id: Uuid,
    tenant_id: Uuid,
    keycloak_user_id: String,
    email: String,
    name: Option<String>,
    role: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserRow {
    fn into_user(self) -> User {
        User {
            id: UserId(self.id),
            tenant_id: TenantId(self.tenant_id),
            keycloak_user_id: self.keycloak_user_id,
            email: self.email,
            name: self.name,
            role: self.role.parse().unwrap_or(Role::Employee),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
